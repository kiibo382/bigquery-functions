# List of BigQuery Standard SQL Functions

[This Reference](https://cloud.google.com/bigquery/docs/reference/standard-sql/functions-and-operators) is parsed to generate type definitions.

See [output](./output) for a list of BiqQuery functions.

## Usage
Here's how to process a list of BigQuery functions using Rust.
Fist, add the following to Cargo.toml.
```toml
[dependencies]
bigquery-functions = "0.1.13"
```

Then call the function or type as follows.
```rust
use bigquery_functions::get_bigquery_function_names;

fn main() {
    let function_names = get_bigquery_function_names();
    println!("function_names: {:?}", function_names)
}
```

[docs.rs](https://docs.rs/bigquery-functions/latest/bigquery_functions/#) is also helpful.
