# Docs

Each request handler should be prefixxed with `#[approck::request]` macro.

## Basic annotations 

```rust

// if this was in src/user/edit.rs, the URI would be /user/edit, 

#[approck::http]
pub async fn handler(req: &handler::req) -> handler::res {
    "hello"
}


#[approck::request(GET /foo/bar)]
pub async fn handler(req: &handler::req) -> handler::res {
    "hello"
}



// if this was in src/user/edit.rs, the URI would be /user/edit/save

#[approck::request(POST ./save; post = { a: String, b: String}; )]
pub async fn save(req: &save::req) -> save::res {
    "hello"
}

```

## GET+POST

```rust


#[approck::request(
    GET|POST /edit/foo-bar-baz/{id}?name=String&age=u32;
    post = {
        a: String,
        b: String,
    };
    websocket = true;
    put = stream;
    data = bytes;
    max_request_size: 1024;
)]
pub async fn edit(req: &edit::req) -> edit::res {
    req.path.id;
    req.qs.name;
    req.qs.age;
    req.data.a;
    req.data.b;
    req.body; // stream
    format!("goodbye, {}", req.qs.a)
}

```

## Ajax handler

```rust

#[approck::request(
    POST ./ajax/save;
    post = {
        gsid: String,
        name: String,
    };
    
)]

#[approck::request(
    PUT ./ajax/upload;
    put<stream>;
    put<bytes> {
        max-request-size: 1024;
    };
)]

#[approck::request(
    PUT ./ajax/upload;
    put<bytes> {
        max-request-size: 1024;
    };
)]



```









```

