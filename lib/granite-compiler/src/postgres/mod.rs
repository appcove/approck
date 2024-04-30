mod codegen;
mod parser;

use crate::tokenator::TokenError;

use quote::quote;

pub struct QueryArg {
    pub arg_number: u32,
    pub arg_ident: Option<syn::Ident>,
    pub value_tokens: proc_macro2::TokenStream,
}

pub struct RowField {
    pub ident: syn::Ident,
    pub ty: FieldType,
}

#[allow(non_camel_case_types)]
pub enum FieldType {
    String,
    bool,
    i32,
    i64,
}

#[derive(Default)]
struct QueryBundleBuilder {
    db_ident: Option<syn::Ident>,
    args: Option<Vec<QueryArg>>,
    row: Option<Vec<RowField>>,
    query: Option<String>,
}

impl QueryBundleBuilder {
    fn build(self) -> Result<QueryBundle, TokenError> {
        Ok(QueryBundle {
            db_ident: self
                .db_ident
                .ok_or_else(|| TokenError::new_call_site("missing `db` instruction"))?,
            args: self.args.unwrap_or_default(),
            row: self
                .row
                .ok_or_else(|| TokenError::new_call_site("missing `row` instruction"))?,
            query: self
                .query
                .ok_or_else(|| TokenError::new_call_site("missing query"))?,
        })
    }
}

pub struct QueryBundle {
    pub db_ident: syn::Ident,
    pub args: Vec<QueryArg>,
    pub row: Vec<RowField>,
    pub query: String,
}

impl QueryBundle {
    pub fn new(input: proc_macro2::TokenStream) -> Result<Self, TokenError> {
        self::parser::parse_into_query_bundle(input)
    }

    pub fn pg_execute(self) -> proc_macro2::TokenStream {
        let query = &self.query;
        let quote_args = self.quote_args();

        quote! {
            {
                use granite::ResultExt;

                //let result:Result<Vec<_>, granite_postgres::PgError> = async {
                async {
                    // prepare the same query
                    let sql = #query;

                    let db_rows = client.query(sql, &[#quote_args]).await.amend(|e| e.add_context(sql))?;

                    granite::Result::Ok(())
                }
            }
        }
    }

    pub fn pg_value(self) -> proc_macro2::TokenStream {
        quote! {}
    }

    pub fn pg_value_vec(self) -> proc_macro2::TokenStream {
        quote! {}
    }

    pub fn pg_row(self) -> proc_macro2::TokenStream {
        let query = &self.query;
        let quote_client = &self.db_ident;
        let quote_row = self.quote_row();
        let quote_row_construct = self.quote_row_construct();
        let quote_args = self.quote_args();

        quote! {
            {
                use granite::ResultExt;
                use granite_postgres::DB;
                #quote_row
                let db = &#quote_client;

                //let result:Result<Vec<_>, granite_postgres::PgError> = async {
                async {
                    // prepare the same query
                    let sql = #query;

                    let mut db_rows = db.query(sql, &[#quote_args]).await.amend(|e| e.add_context(sql))?;

                    let row = match db_rows.len() {
                        0 => {
                            panic!("expected 1 row, got 0");
                        },
                        1 => {
                            let db_row = db_rows.pop().unwrap();
                            #quote_row_construct
                        },
                        n => {
                            panic!("expected 1 row, got {n}");
                        }
                    };

                    granite::Result::Ok(row)
                }
            }
        }
    }

    pub fn pg_row_vec(self) -> proc_macro2::TokenStream {
        let query = &self.query;
        let quote_db = &self.db_ident;
        let quote_row = self.quote_row();
        let quote_row_construct = self.quote_row_construct();
        let quote_args = self.quote_args();

        quote! {
            {
                use granite::ResultExt;
                use granite_postgres::DB;
                #quote_row

                //let result:Result<Vec<_>, granite_postgres::PgError> = async {
                async {
                    // prepare the same query
                    let sql = #query;

                    let db_rows = #quote_db.query(sql, &[#quote_args]).await.amend(|e| e.add_context(sql))?;

                    let mut rows:Vec<Row> = Vec::with_capacity(db_rows.len());

                    for db_row in db_rows.into_iter() {
                        rows.push(#quote_row_construct);
                    }

                    granite::Result::Ok(rows)
                }
            }
        }
    }

    fn quote_args(&self) -> proc_macro2::TokenStream {
        let query_arg_tokens = self
            .args
            .iter()
            .map(|arg| {
                let value_tokens = &arg.value_tokens;
                quote! {
                    (#value_tokens) as &(dyn granite_postgres::ToSql + Sync),
                }
            })
            .collect::<Vec<_>>();
        quote!(#(#query_arg_tokens)*)
    }

    fn quote_row(&self) -> proc_macro2::TokenStream {
        let row_fields = self
            .row
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let ty = match &field.ty {
                    FieldType::String => quote! { String },
                    FieldType::bool => quote! { bool },
                    FieldType::i32 => quote! { i32 },
                    FieldType::i64 => quote! { i64 },
                };
                quote! {
                    #ident: #ty,
                }
            })
            .collect::<Vec<_>>();
        quote! {
            #[derive(Debug)]
            pub struct Row {
                #(#row_fields)*
            }
        }
    }

    fn quote_row_construct(&self) -> proc_macro2::TokenStream {
        let row_fields = self
            .row
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let ident = &field.ident;
                quote! {
                    #ident: db_row.get(#i),
                }
            })
            .collect::<Vec<_>>();
        quote! {
            Row {
                #(#row_fields)*
            }
        }
    }
}
