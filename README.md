# Inventory
An inventory management program written in rust.

# Running
Rocket requires a specific version of nightly rust to work.

`rustup override set nightly-2017-12-21`

Install the diesel ORM tools

`cargo install diesel_cli`

Generate the databse

`diesel migration run`

install cargo web

`cargo install cargo-web`

Add the emscripten compile target

`rustup target add asmjs-unknown-emscripten`

Compile the client

```
cd client
cargo web build
cd ..
```

And run;

`cargo run`
