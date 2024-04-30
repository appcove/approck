
`swc-linux-x64-gnu compile --extensions 'mts' --out-file-extension 'mjs' --out-dir . --config-file lib/approck-example/.swcrc lib/approck-example/src/`



```rust

#[approck::http(
    GET /foo/bar/edit?code=i32&items=Vec<String>; 
    return Page(crate::ui::Layout);
)]

pub async fn page(req: &page::Request) -> page::Response {
    
    if req.qs.code == 10 {
        // do something with id
    }

    for item in req.qs.items {
        // do something with each item
    }

    let mut rval = crate::ui::Layout::new();
    rval.add(html!(
        <div>
            <h1>Foo</h1>
            <p>Bar</p>
        </div>
    ));
    
    page::response::Page(rval)
}

```




```rust
#[approck::http(
    GET /foo/bar/edit;
    upgrade = websocket;
)]
pub async fn page1(_req: &page1::Request) -> page1::Response {

    // authenticate with get params or session etc..

    // create variables if needed

    //_req.websocket;
    "fo".to_string();
    
    _req.websocket_handler(async move |socket| {
        // refer to those variaables above here
        // this runs for the duration of the websocket connnection


    })
}



```

