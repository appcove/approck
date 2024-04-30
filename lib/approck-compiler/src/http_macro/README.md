


## A simple example

```rust

#[approck::http(
    GET /foo/bar?id=i32;
    return String;
)]
async fn foo(qs:Δ) -> Δ {
    format!("foo: {}", qs.id)
}

```

## an example using a mod

```rust
#[approck::http(
    GET /foo/bar?id=i32;
    return String;
)]
mod foo {
    pub async fn get(qs:Δ) -> Δ {
        format!("foo: {}", qs.id)
    }
}



#[approck::http(GET /asset/logo.svg; return SVG;)]
pub async fn svg(_req: svg::Request) -> Response::Response {
    include_str!("logo.svg").to_string()
}

#[approck::http(GET /asset/logo.svg;)]
pub mod logo_svg {
    pub async fn get() -> Response {
        include_str!("logo.svg").to_string()
    }
}



```

