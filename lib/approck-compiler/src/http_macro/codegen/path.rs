use crate::PathPartCapture;
use quote::quote;
use syn::Ident;

pub fn process(
    codegen_tokens: &mut super::CodeGenTokens,
    path_captures: &Vec<(u8, String, &crate::PathPartCapture)>,
) {
    // --------------------------------------------------------------------------------------------
    // handle `Path`

    let mut path_fields = Vec::new();
    let mut path_assign = Vec::new();

    for (level, name, capture) in path_captures {
        let value_type = match &capture {
            PathPartCapture::i8 => quote! { i8 },
            PathPartCapture::u8 => quote! { u8 },
            PathPartCapture::i32 => quote! { i32 },
            PathPartCapture::u32 => quote! { u32 },
            PathPartCapture::i64 => quote! { i64 },
            PathPartCapture::u64 => quote! { u64 },
            PathPartCapture::usize => quote! { usize },
            PathPartCapture::String => quote! { String },
        };

        let name_ident = Ident::new(name, proc_macro2::Span::call_site());
        let arg_ident = Ident::new(
            &format!("capture_level{}_{}", level, name),
            proc_macro2::Span::call_site(),
        );

        path_fields.push(quote! {
            pub #name_ident: #value_type,
        });

        path_assign.push(quote! {
            #name_ident: #arg_ident,
        });

        codegen_tokens.wrap_fn_sig_inputs(quote! {
            #arg_ident: #value_type,
        });
    }

    let derive_debug = &codegen_tokens.derive_debug;
    codegen_tokens.mod_items(quote! {
        #derive_debug
        pub struct Path {
            #( #path_fields )*
        }
    });
    codegen_tokens.wrap_fn_items(quote! {
        let path = Path {
            #( #path_assign )*
        };
    });
}
