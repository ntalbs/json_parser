# JSON parser

JSON parser written in Rust.
This is a toy project to implement a basic JSON parser without external dependencies.

This can be used as a library to parse JSON. 
If you have a JSON string, you can parse it by:

```rust
let input = "... JSON string ...";

match Json::from_str(input) {
    Ok(json) => ...,
    Err(errs) => ...,
}
```

`Json::from_str(...)` returns `Result<Json, Vec<Error>>`.
Once it parsed the string into `Json` successfully, you can use `Json` inside `Result`.

`Json` is an enum represent JSON.

```rust
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Obj(Vec<(String, Json)>),
    Arr(Vec<Json>),
}
```

This also includes a CLI that parse JSON from `stdin` and print out. 
For example, the command below will print the syntax highlighed JSON to console.

```console
cargo run < tests/inputs/list-lambda.json
```
