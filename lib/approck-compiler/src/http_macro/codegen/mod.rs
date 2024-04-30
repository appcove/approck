#[cfg(test)]
mod tests;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

mod path;
mod post_form;
mod query_string;

use crate::ParamType;

/// One coherent place to aggregate all the tokens that will be used to generate the final code
/// Glossary:
/// * mrf = 'mod request function'
struct CodeGenTokens {
    mod_items: TokenStream,
    wrap_fn_sig_inputs: TokenStream,
    wrap_fn_items: TokenStream,
    wrap_fn_return: TokenStream,
    request_fn_params: TokenStream,
    request_fn_sig_inputs: TokenStream,
    derive_debug: TokenStream,
}

impl Default for CodeGenTokens {
    fn default() -> Self {
        Self {
            mod_items: quote! {},
            wrap_fn_items: quote! {},
            wrap_fn_sig_inputs: quote! {},
            wrap_fn_return: quote! {},
            request_fn_params: quote! {},
            request_fn_sig_inputs: quote! {},
            derive_debug: quote! {},
        }
    }
}

impl CodeGenTokens {
    /// Unless not needed, each statement should end in a `;`
    fn mod_items(&mut self, tokens: TokenStream) {
        self.mod_items.append_all(tokens);
    }

    /// MUST END IN A `,`  
    /// Add tokens to the wrap function signature.  
    fn wrap_fn_sig_inputs(&mut self, tokens: TokenStream) {
        self.wrap_fn_sig_inputs.append_all(tokens);
    }

    /// Unless not needed, each statement should end in a `;`
    fn wrap_fn_items(&mut self, tokens: TokenStream) {
        self.wrap_fn_items.append_all(tokens);
    }

    /// MUST END IN A `,`
    /// Adds argument tokens to the `let response = request(...)` call
    fn request_fn_params(&mut self, tokens: TokenStream) {
        self.request_fn_params.append_all(tokens);
    }

    /// MUST END IN A `,`
    /// Adds tokens to the request function signature
    fn request_fn_sig_inputs(&mut self, tokens: TokenStream) {
        self.request_fn_sig_inputs.append_all(tokens);
    }
}

fn param_token_stream(param: &crate::Param) -> proc_macro2::TokenStream {
    let name = Ident::new(&param.param_name, proc_macro2::Span::call_site());
    match &param.param_type {
        ParamType::App(_) => quote! { app: &'static APP, },
        ParamType::Document => quote! { mut #name: impl Document, },
        ParamType::Postgres => quote! { mut #name: DBCX<'_>, },
        ParamType::Redis => quote! { mut #name: RedisCX<'_>, },
        ParamType::Path => quote! { #name: Path, },
        ParamType::Request => quote! { #name: &mut Request<'_>, },
        ParamType::QueryString => quote! { #name: QueryString, },
        ParamType::QueryStringOption => quote! { #name: Option<QueryString>, },
        ParamType::PostForm => quote! { #name: PostForm, },
        ParamType::PostFormOption => quote! { #name: Option<PostForm>, },
        ParamType::OptionalParam(t) => {
            let t = Ident::new(t, proc_macro2::Span::call_site());
            quote! { #name: Option<#t>, }
        }
    }
}

pub fn expand(mod_bundle: crate::HttpModuleInner) -> TokenStream {
    let mut codegen_tokens = CodeGenTokens::default();

    // --------------------------------------------------------------------------------------------
    // handle `derive_debug;`

    if mod_bundle.derive_debug {
        codegen_tokens.derive_debug = quote! {
            #[derive(Debug)]
        };
    }

    // --------------------------------------------------------------------------------------------
    // handle `Path`

    if mod_bundle.has_path_captures() {
        let path_captures = &mod_bundle.iter_path_captures().collect();
        self::path::process(&mut codegen_tokens, path_captures);
    }

    if let Some(qs_parts) = &mod_bundle.query_string {
        self::query_string::process(&mut codegen_tokens, qs_parts);
    }

    // handle code gen for post type
    match &mod_bundle.mod_post_type {
        crate::PostType::None => {}
        crate::PostType::Struct(post_type_struct) => {
            self::post_form::process(&mut codegen_tokens, post_type_struct);
        }
        crate::PostType::Enum(_) => {
            unimplemented!("enum post types are not yet supported");
        }
    }

    // --------------------------------------------------------------------------------------------
    // handle `request(*params)`

    for param in &mod_bundle.mod_request_fn_params {
        codegen_tokens.request_fn_sig_inputs(param_token_stream(param));

        match &param.param_type {
            crate::ParamType::App(_) => {
                // TODO: insert the trait_paths
                codegen_tokens.request_fn_params(quote! { app, });
            }

            crate::ParamType::Document => {
                codegen_tokens.mod_items(quote! {
                    use approck::traits::Document;
                });
                codegen_tokens.wrap_fn_items(quote! {
                    // LUKE: here is where document is created, need to think of a way to make this more flexible
                    let document = app.get_document();
                });
                codegen_tokens.request_fn_params(quote! { document, });
            }

            crate::ParamType::Postgres => {
                codegen_tokens.mod_items(quote! {
                    use granite_postgres::DBCX;
                });
                codegen_tokens.wrap_fn_items(quote! {
                    let dbcx = app.postgres_dbcx().await?;
                });
                codegen_tokens.request_fn_params(quote! { dbcx, });
            }

            crate::ParamType::Redis => {
                codegen_tokens.mod_items(quote! {
                    use granite_redis::{RedisCX, AsyncCommands};
                });
                codegen_tokens.wrap_fn_items(quote! {
                    let redis_cx = app.redis_dbcx().await?;
                });
                codegen_tokens.request_fn_params(quote! { redis_cx, });
            }

            crate::ParamType::Path => {
                codegen_tokens.request_fn_params(quote! { path, });
            }

            // the Request type is simply an alias and passthrough
            crate::ParamType::Request => {
                codegen_tokens.mod_items(quote! {
                    type Request<'a> = approck::server::Request<'a>;
                });
                codegen_tokens.request_fn_params(quote! {
                    &mut req,
                });
            }

            // handle QueryString
            crate::ParamType::QueryString => {
                codegen_tokens.wrap_fn_items(quote! {
                    let qs = QueryString::parse(&req).amend(|e| e
                        .set_uri(req.uri_string())
                        .add_context("QueryString::parse()")
                    )?;
                });
                codegen_tokens.request_fn_params(quote! {
                    qs,
                });
            }

            // handle Option<QueryString>
            crate::ParamType::QueryStringOption => {
                codegen_tokens.wrap_fn_items(quote! {
                    let qs = if req.has_query_string() {
                        Some(QueryString::parse(&req).amend(|e|
                            e.set_uri(req.uri_string()).add_context("QueryString::parse()")
                        )?)
                    } else {
                        None
                    };
                });
                codegen_tokens.request_fn_params(quote! {
                    qs,
                });
            }

            // handler PostForm
            crate::ParamType::PostForm => {
                codegen_tokens.wrap_fn_items(quote! {
                    let post_form = PostForm::parse(&mut req).await.amend(|e| e
                        .set_uri(req.uri_string())
                        .add_context("PostForm::parse()")
                    )?;
                });
                codegen_tokens.request_fn_params(quote! {
                    post_form,
                });
            }

            // handler Option<PostForm>
            crate::ParamType::PostFormOption => {
                codegen_tokens.wrap_fn_items(quote! {
                    let post_form = if req.is_post() {
                        Some(PostForm::parse(&mut req).await.amend(|e| e
                            .set_uri(req.uri_string())
                            .add_context("PostForm::parse()")
                        )?)
                    } else {
                        None
                    };
                });
                codegen_tokens.request_fn_params(quote! {
                    post_form,
                });
            }

            // any other type is an Option gets None
            crate::ParamType::OptionalParam(_) => {
                codegen_tokens.request_fn_params(quote! {
                    None,
                });
            }
        }
    }

    // --------------------------------------------------------------------------------------------
    // handle return_types

    {
        let mut mods = Vec::new();
        let mut arms = Vec::new();
        let mut variants = Vec::new();

        if mod_bundle.return_types.Bytes {
            mods.push(quote! {
                ::use approck::server::response::Bytes;
            });
            variants.push(quote! {
                Bytes(approck::server::response::Bytes),
            });
            arms.push(quote! {
                Response::Bytes(v) => approck::server::response::Response::Bytes(v),
            });
        }

        if mod_bundle.return_types.Text {
            mods.push(quote! {
                use ::approck::server::response::Text;
            });
            variants.push(quote! {
                Text(approck::server::response::Text),
            });
            arms.push(quote! {
                Response::Text(v) => approck::server::response::Response::Text(v),
            });
        }

        if mod_bundle.return_types.Empty {
            mods.push(quote! {
                use ::approck::server::response::Empty;
            });
            variants.push(quote! {
                Empty(approck::server::response::Empty),
            });
            arms.push(quote! {
                Response::Empty(v) => approck::server::response::Response::Empty(v),
            });
        }

        if mod_bundle.return_types.HTML {
            mods.push(quote! {
                use ::approck::server::response::HTML;
            });
            variants.push(quote! {
                HTML(approck::server::response::HTML),
            });
            arms.push(quote! {
                Response::HTML(v) => approck::server::response::Response::HTML(v),
            });
        }

        if mod_bundle.return_types.JavaScript {
            mods.push(quote! {
                use ::approck::server::response::JavaScript;
            });
            variants.push(quote! {
                JavaScript(approck::server::response::JavaScript),
            });
            arms.push(quote! {
                Response::JavaScript(v) => approck::server::response::Response::JavaScript(v),
            });
        }

        if mod_bundle.return_types.CSS {
            mods.push(quote! {
                use ::approck::server::response::CSS;
            });
            variants.push(quote! {
                CSS(approck::server::response::CSS),
            });
            arms.push(quote! {
                Response::CSS(v) => approck::server::response::Response::CSS(v),
            });
        }

        if mod_bundle.return_types.JSON {
            mods.push(quote! {
                use ::approck::server::response::JSON;
            });
            variants.push(quote! {
                JSON(approck::server::response::JSON),
            });
            arms.push(quote! {
                Response::JSON(v) => approck::server::response::Response::JSON(v),
            });
        }

        if mod_bundle.return_types.SVG {
            mods.push(quote! {
                use ::approck::server::response::SVG;
            });
            variants.push(quote! {
                SVG(approck::server::response::SVG),
            });
            arms.push(quote! {
                Response::SVG(v) => approck::server::response::Response::SVG(v),
            });
        }

        if mod_bundle.return_types.NotFound {
            mods.push(quote! {
                use ::approck::server::response::NotFound;
            });
            variants.push(quote! {
                NotFound(approck::server::response::NotFound),
            });
            arms.push(quote! {
                Response::NotFound(v) => approck::server::response::Response::NotFound(v),
            });
        }

        if mod_bundle.return_types.Redirect {
            mods.push(quote! {
                use ::approck::server::response::Redirect;
            });
            variants.push(quote! {
                Redirect(approck::server::response::Redirect),
            });
            arms.push(quote! {
                Response::Redirect(v) => approck::server::response::Response::Redirect(v),
            });
        }

        if mod_bundle.return_types.WebSocketUpgrade {
            mods.push(quote! {
                use ::approck::server::websocket::{WebSocket, Message as WebSocketMessage, MessageData as WebSocketMessageData, MessageDataRef as WebSocketMessageDataRef};
                use ::approck::server::response::WebSocketUpgrade;
            });
            variants.push(quote! {
                WebSocketUpgrade(approck::server::response::WebSocketUpgrade),
            });
            arms.push(quote! {
                Response::WebSocketUpgrade(v) => approck::server::response::Response::WebSocketUpgrade(v),
            });
        }

        if mod_bundle.return_types.Stream {
            mods.push(quote! {
                use ::approck::server::response::Stream;
                use ::futures::StreamExt;

            });
            variants.push(quote! {
                Stream(approck::server::response::Stream),
            });
            arms.push(quote! {
                Response::Stream(v) => approck::server::response::Response::Stream(v),
            });
        }

        // generate actual code
        codegen_tokens.mod_items(quote! {
            #(#mods)*
        });

        codegen_tokens.mod_items(quote! {
            pub enum Response {
                #(#variants)*
            }
        });

        codegen_tokens.wrap_fn_return = quote! {
            Ok(match response {
                #(#arms)*
            })
        };
    }

    // --------------------------------------------------------------------------------------------
    // actual code generation

    let mod_items = &mod_bundle.mod_items_remaining;
    let mod_ident = &mod_bundle.mod_ident;
    let mod_tokens = codegen_tokens.mod_items;

    let wrap_fn_traits =
        crate::codegen::quote_traits("APP", &mod_bundle.get_wrap_fn_app_trait_list());
    let wrap_fn_sig_inputs = codegen_tokens.wrap_fn_sig_inputs;
    let wrap_fn_items = codegen_tokens.wrap_fn_items;
    let wrap_fn_return = codegen_tokens.wrap_fn_return;

    let request_fn_params = codegen_tokens.request_fn_params;
    let request_fn_vis = &mod_bundle.mod_request_fn.vis.to_token_stream();
    let request_fn_async = mod_bundle.mod_request_fn.sig.asyncness.to_token_stream();
    let request_fn_sig_inputs = &codegen_tokens.request_fn_sig_inputs;
    let request_fn_block = &mod_bundle.mod_request_fn.block.to_token_stream();

    // only include APP trait list if it's not empty
    let request_fn_traits = {
        let trait_list = mod_bundle.get_request_fn_app_trait_list();
        if trait_list.is_empty() {
            quote! {}
        } else {
            crate::codegen::quote_traits("APP", &trait_list)
        }
    };

    // Can be either `-> Response` or `-> Result<Response>`
    let request_fn_sig_output = match &mod_bundle.mod_request_fn_return {
        crate::RequestFunctionReturnType::Response => {
            quote! { -> Response }
        }
        crate::RequestFunctionReturnType::ResultResponse => {
            quote! { -> approck::Result<Response> }
        }
    };

    // When invoking the request() function, we need to place a `?` after it if it returns a Result, otherwise not
    let request_fn_invoke = match &mod_bundle.mod_request_fn_return {
        crate::RequestFunctionReturnType::Response => {
            quote! {
                request(#request_fn_params).await
            }
        }
        crate::RequestFunctionReturnType::ResultResponse => {
            quote! {
                request(#request_fn_params).await.amend(|e| e
                    .set_uri(req.uri_string())
                    .add_context("error propagated out from request()")
                )?
            }
        }
    };

    // Construct the expanded code
    let expanded_code = quote! {
        pub mod #mod_ident {
            use approck::ResultExt;
            #mod_tokens
            pub async fn wrap<#wrap_fn_traits>(app: &'static APP, mut req: approck::server::Request<'_>, #wrap_fn_sig_inputs) -> approck::Result<approck::server::response::Response> {
                #wrap_fn_items
                let response = #request_fn_invoke;
                #wrap_fn_return
            }
            #request_fn_vis #request_fn_async fn request <#request_fn_traits>(#request_fn_sig_inputs) #request_fn_sig_output {
                #request_fn_block
            }
            #(#mod_items)*
        }
    };

    //panic!("Expanded code: \n {}", expanded_code.to_string());

    expanded_code
}
