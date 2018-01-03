# Inventory
An inventory management program written in rust.

# Running
Rocket requires a specific version of nightly rust to work.

`rustup override set nightly-2017-12-21`

Install the diesel ORM tools

`cargo install diesel_cli`

Generate the databse

`diesel migration run`

And run;

`cargo run`
