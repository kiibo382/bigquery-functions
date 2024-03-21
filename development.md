## Step1

Update the output files by running the following commands.

```
cargo run
make cp-output
```

## Step2

Update the version number in the README.md file.

## Step3

Update the version number in the bigquery-functions-types-macros/Cargo.toml file.
Next, publish the bigquery-functions-types-macros crate.

```
make publish-types-macros
```

## Step4

Update the version number in the bigquery-functions/Cargo.toml file.
Next, publish the bigquery-functions crate.

```
make publish
```
