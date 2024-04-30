use quote::quote;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt::Debug;
use std::path::PathBuf;

pub mod codegen;
pub mod http_macro;
pub mod http_routing;
pub mod tokenator;

/// This represents an [approck:http()] function in-place in a web project
/// It knows where the code is, and owns the inner function struct
pub struct HttpModule {
    pub rel_path: String,
    pub fn_line: usize,
    pub rust_ident: String,
    pub inner: HttpModuleInner,
}

/// This is the whole parsed contents of the approck_http attribute that comes
/// from the ... part of #[approck::http(...)]
///
/// It is not aware of where it exists in a web project because when called from a
/// proc_macro that info is not available
pub struct HttpModuleInner {
    pub methods: Vec<Method>,
    pub path: Vec<(u8, PathPart)>,
    pub query_string: Option<Vec<QueryStringPart>>,
    pub derive_debug: bool,
    pub return_types: ReturnTypes,
    pub mod_name: String,
    pub mod_ident: syn::Ident,
    pub mod_post_type: PostType,
    pub mod_request_fn: syn::ItemFn,
    pub mod_request_fn_params: Vec<Param>,
    pub mod_request_fn_return: RequestFunctionReturnType,
    pub mod_items_remaining: Vec<syn::Item>,
}

impl HttpModuleInner {
    pub fn has_path_captures(&self) -> bool {
        self.path
            .iter()
            .any(|p| matches!(p.1, PathPart::Capture { .. }))
    }
    pub fn iter_path_captures(&self) -> impl Iterator<Item = (u8, String, &PathPartCapture)> {
        self.path.iter().filter_map(|p| match &p.1 {
            PathPart::Capture { name, capture } => Some((p.0, name.clone(), capture)),
            _ => None,
        })
    }
    pub fn get_wrap_fn_app_trait_list(&self) -> Vec<String> {
        let mut rval = Vec::new();
        for param in self.mod_request_fn_params.iter() {
            match &param.param_type {
                // TODO: deduplicate this list
                ParamType::App(paths) => {
                    for path in paths {
                        rval.push(path.to_owned());
                    }
                }
                ParamType::Document => {
                    rval.push("::approck::traits::DocumentModule".to_string());
                }
                ParamType::Postgres => {
                    rval.push("::granite_postgres::PostgresModule".to_string());
                }
                ParamType::Redis => {
                    rval.push("::granite_redis::RedisModule".to_string());
                }
                _ => {}
            }
        }
        rval
    }

    #[allow(clippy::single_match)]
    pub fn get_request_fn_app_trait_list(&self) -> Vec<String> {
        let mut rval = Vec::new();
        for param in self.mod_request_fn_params.iter() {
            match &param.param_type {
                ParamType::App(paths) => {
                    for path in paths {
                        rval.push(path.to_owned());
                    }
                }
                _ => {}
            }
        }
        rval
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// This represnts a chunk of the path
/// Can be extended with varaibles and placeholders, like /user/{id:i32}/edit
///
/// This ordering is critical to the functionality of the router.  For example:
///    `/user/{id:i32}/`
///    `/user/{username:String}/`
///    `/user/add`
///
/// As per Ord on PathPart, the literal `add` will come first
/// As per Ord on PathPartCapture, the int will come first and the String will come last
///    this is so the String doesn't consume integers before int has a chance to grab them

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum PathPart {
    Index,
    Literal(String),
    Capture {
        name: String,
        capture: PathPartCapture,
    },
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum PathPartCapture {
    i8,
    u8,
    i32,
    u32,
    i64,
    u64,
    usize,
    String,
}

impl PathPartCapture {
    pub fn index(&self) -> usize {
        match self {
            PathPartCapture::i8 => 0,
            PathPartCapture::u8 => 1,
            PathPartCapture::i32 => 2,
            PathPartCapture::u32 => 3,
            PathPartCapture::i64 => 4,
            PathPartCapture::u64 => 5,
            PathPartCapture::usize => 6,
            PathPartCapture::String => 7,
        }
    }
}

impl Ord for PathPart {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PathPart::Index, PathPart::Index) => Ordering::Equal,
            (PathPart::Index, _) => Ordering::Less,
            (_, PathPart::Index) => Ordering::Greater,

            (PathPart::Literal(a), PathPart::Literal(b)) => a.cmp(b),
            (PathPart::Literal(_), _) => Ordering::Less,
            (_, PathPart::Literal(_)) => Ordering::Greater,

            (
                PathPart::Capture {
                    name: name1,
                    capture: capture1,
                },
                PathPart::Capture {
                    name: name2,
                    capture: capture2,
                },
            ) => match capture1.index().cmp(&capture2.index()) {
                Ordering::Equal => name1.cmp(name2),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            },
        }
    }
}

impl PartialOrd for PathPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq)]
pub struct QueryStringPart {
    pub name: String,
    pub value: QueryStringValue,
}

#[derive(Debug, PartialEq)]
pub enum QueryStringValue {
    Require(QueryStringValueType),
    Option(QueryStringValueType),
    Vec(QueryStringValueType),
    HashSet(QueryStringValueType),
    NoValue,
}
// seems more important to match the types exactly than to keep the names camel case
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum QueryStringValueType {
    String,
    i32,
    u32,
    i64,
    u64,
    f32,
    f64,
}

impl QueryStringValueType {
    pub fn get_type_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            QueryStringValueType::String => quote! { String },
            QueryStringValueType::i32 => quote! { i32 },
            QueryStringValueType::u32 => quote! { u32 },
            QueryStringValueType::i64 => quote! { i64 },
            QueryStringValueType::u64 => quote! { u64 },
            QueryStringValueType::f32 => quote! { f32 },
            QueryStringValueType::f64 => quote! { f64 },
        }
    }
}

#[derive(Debug, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct ReturnTypes {
    pub Bytes: bool,
    pub Text: bool,
    pub Empty: bool,
    pub HTML: bool,
    pub JavaScript: bool,
    pub CSS: bool,
    pub JSON: bool,
    pub SVG: bool,
    pub NotFound: bool,
    pub Redirect: bool,
    pub WebSocketUpgrade: bool,
    pub Stream: bool,
}

#[derive(Debug, PartialEq)]
pub enum RequestFunctionReturnType {
    ResultResponse,
    Response,
}

pub enum PostType {
    None,
    Struct(PostTypeStruct),
    Enum(PostTypeEnum),
}

pub struct PostTypeStruct {
    token_stream: proc_macro2::TokenStream,
    query_string_parts: Vec<QueryStringPart>,
}

pub struct PostTypeEnum {}

impl Debug for PostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostType::None => write!(f, "None"),
            PostType::Struct(_) => write!(f, "Form(...)"),
            PostType::Enum(_) => write!(f, "Enum(...)"),
        }
    }
}

impl PostType {
    pub fn is_filled(&self) -> bool {
        !matches!(self, PostType::None)
    }
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub param_name: String,
    pub param_type: ParamType,
}

#[derive(Debug, PartialEq)]
pub enum ParamType {
    App(Vec<String>),      // App
    Document,              // impl Document
    Postgres,              // PostgresCX<'_>
    Redis,                 // RedisCX<'_>
    Path,                  // Path
    Request,               // Request
    QueryString,           // QueryString
    QueryStringOption,     // Option<QueryString>
    PostForm,              // PostForm
    PostFormOption,        // Option<PostForm>
    OptionalParam(String), // Option<v>
}

pub fn get_workspace_path() -> PathBuf {
    let orig_cwd = std::env::current_dir().unwrap();
    let mut cwd = orig_cwd.clone();
    loop {
        if cwd.join(".git").exists() {
            break cwd;
        }
        if !cwd.pop() {
            panic!("Not in a git repo at {}", orig_cwd.display());
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // create a macro to generate a test.  It should take a name identifier and item to assert to true
    macro_rules! assert_true {
        ($name:ident, $item:expr) => {
            #[test]
            fn $name() {
                assert!($item);
            }
        };
    }

    // test that Index is equal to itself
    assert_true!(test_path_part_index_eq, PathPart::Index == PathPart::Index);

    // test that Index is less than a literal
    assert_true!(
        test_path_part_index_lt_literal,
        PathPart::Index < PathPart::Literal("".to_string())
    );

    // test that a literal is > index
    assert_true!(
        test_path_part_literal_gt_index,
        PathPart::Literal("".to_string()) > PathPart::Index
    );

    // test that a literal is equal to itself
    assert_true!(
        test_path_part_literal_eq,
        PathPart::Literal("".to_string()) == PathPart::Literal("".to_string())
    );

    // test that a literal is less than another literal
    assert_true!(
        test_path_part_literal_lt,
        PathPart::Literal("a".to_string()) < PathPart::Literal("b".to_string())
    );

    // test that a capture is > than a literal
    assert_true!(
        test_path_part_capture_gt_literal,
        PathPart::Capture {
            name: "aaa".to_string(),
            capture: PathPartCapture::String
        } > PathPart::Literal("bbb".to_string())
    );

    // test that a literal is < than a capture
    assert_true!(
        test_path_part_literal_lt_capture,
        PathPart::Literal("bbb".to_string())
            < PathPart::Capture {
                name: "aaa".to_string(),
                capture: PathPartCapture::String
            }
    );

    // test that a capture is equal to itself
    assert_true!(
        test_path_part_capture_eq,
        PathPart::Capture {
            name: "aaa".to_string(),
            capture: PathPartCapture::String
        } == PathPart::Capture {
            name: "aaa".to_string(),
            capture: PathPartCapture::String
        }
    );

    // test that an i8 capture is < than a u8 capture
    assert_true!(
        test_path_part_capture_i8_lt_u8,
        PathPart::Capture {
            name: "bbb".to_string(),
            capture: PathPartCapture::i8
        } < PathPart::Capture {
            name: "aaa".to_string(),
            capture: PathPartCapture::u8
        }
    );

    // test that a string capture is > usize capture
    assert_true!(
        test_path_part_capture_string_gt_usize,
        PathPart::Capture {
            name: "aaa".to_string(),
            capture: PathPartCapture::String
        } > PathPart::Capture {
            name: "bbb".to_string(),
            capture: PathPartCapture::usize
        }
    );
}
