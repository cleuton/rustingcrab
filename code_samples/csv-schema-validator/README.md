# csv-schema-validator

## Version 0.1.1

[![Crates.io](https://img.shields.io/crates/v/csv-schema-validator.svg)](https://crates.io/crates/csv-schema-validator) [![Documentation](https://docs.rs/csv-schema-validator/badge.svg)](https://docs.rs/csv-schema-validator)

A Rust library for validating CSV record data based on rules defined directly in your structs using the `#[derive(ValidateCsv)]` macro.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
csv-schema-validator = "0.1.1"
serde = { version = "1.0", features = ["derive"] }
csv = "1.3"
regex = "1.11"
once_cell = "1.21"
```

## Quick Start

```rust
use serde::Deserialize;
use csv::Reader;
use csv_schema_validator::{ValidateCsv, ValidationError};

// Define your struct with validation annotations
#[derive(Debug, Deserialize, ValidateCsv)]
struct Record {
    #[validate(range(min = 0.0, max = 100.0))]
    grade: f64,

    #[validate(regex = r"^[A-Z]{3}\d{4}$")]
    code: String,

    #[validate(required, length(min=10, max=50))]
    name: Option<String>,

    #[validate(custom = "length_validator")]
    comments: String,
}

// Custom validator: comments must be at most 50 characters
fn length_validator(s: &str) -> Result<(), String> {
    if s.len() <= 50 {
        Ok(())
    } else {
        Err("Comments too long".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("data.csv")?;
    for result in reader.deserialize() {
        let rec: Record = result?;
        rec.validate_csv()?;
        println!("Record valid: {:?}", rec);
    }
    Ok(())
}
```

## Usage

### Range Validation (since 0.1.0)

```rust
#[validate(range(min = 0.0, max = 100.0))]
grade: f64,
```

Ensures that `grade` is between 0.0 and 100.0 (inclusive).

### Regex Validation (since 0.1.0)

```rust
#[validate(regex = r"^[A-Z]{3}\d{4}$")]
code: String,
```

Validates the field against a regular expression.

### Required Validation (since 0.1.0)

```rust
#[validate(required)]
name: Option<String>,
```

Ensures that the `Option` is not `None`.

### Custom Validation (since 0.1.0)

```rust
#[validate(custom = "path::to::func")]
comments: String,
```

Calls your custom function `fn(&T) -> Result<(), String>` for additional checks.

### Length (since 0.1.1)

```rust
#[validate(required, length(min = 10, max = 50))]
name: Option<String>,
```

### Struct check

The macro validates the type it is annotating, only strucs with named fields are allowed: 

```rust
use serde::Deserialize;
use csv_schema_validator::ValidateCsv;

#[derive(Deserialize, ValidateCsv)]
struct TupleStruct(f64, String);

#[derive(Deserialize, ValidateCsv)]
enum Status {
    Success { code: f64, message: String },
    Error(f64, String),
    Unknown,
}

fn main() {
    let record = TupleStruct(42.0, "ABC1234".to_string());
    let s = Status::Success { code: 200.0, message: "OK".into() };
    let _ = record.validate_csv();
    let _ = s.validate_csv();
}
```

Trying to compile this code will result in errors: 

```shell
cargo run
error: only structs with named fields (e.g., `struct S { a: T }`) are supported
 --> src/main.rs:5:19
  |
5 | struct TupleStruct(f64, String);
  |                   ^^^^^^^^^^^^^

error: only structs are supported
  --> src/main.rs:8:1
   |
8  | / enum Status {
9  | |     Success { code: f64, message: String },
10 | |     Error(f64, String),
11 | |     Unknown,
12 | | }
   | |_^

```

### Complete example

This is an example which reads a csv file: 

`Cargo.toml`:

```toml
[package]
name = "use-csv-validator"
version = "0.1.1"
edition = "2021"

[dependencies]
csv = "1.1"
serde = { version = "1.0", features = ["derive"] }
csv-schema-validator = "0.1.1"
```

`src/main.rs`:

```rust
use std::error::Error;
use csv::ReaderBuilder;
use serde::Deserialize;
use csv_schema_validator::{ValidateCsv, ValidationError};

/// Custom validator: ensure comments string isn't too long
fn length_validation(s: &str) -> Result<(), String> {
    if s.len() <= 20 {
        Ok(())
    } else {
        Err("Comments too long".into())
    }
}

#[derive(Debug, Deserialize, ValidateCsv)]
struct TestRecord {
    // grade must be between 0.0 and 100.0
    #[validate(range(min = 0.0, max = 100.0))]
    grade: f64,

    // code must be 3 uppercase letters followed by 4 digits
    #[validate(regex = r"^[A-Z]{3}\d{4}$")]
    code: String,

    // name is required (must be Some)
    #[validate(required)]
    name: Option<String>,

    // comments uses our custom length validator
    #[validate(custom = "length_validation")]
    comments: String,

    // more needs to be a valid string with length range
    #[validate(length(min = 1, max = 20))]
    more: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // open the CSV file placed alongside Cargo.toml
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("data.csv")?;

    // for each record, deserialize and validate
    for (i, result) in reader.deserialize::<TestRecord>().enumerate() {
        let record = result?;
        match record.validate_csv() {
            Ok(()) => println!("Line {}: Record is valid: {:?}", i + 1, record),
            Err(errors) => {
                eprintln!("Line {}: Validation errors:", i + 1);
                for ValidationError { field, message } in errors {
                    eprintln!("  Field `{}`: {}", field, message);
                }
            }
        }
    }

    Ok(())
}

```

`data.csv`: 

```csv
grade,code,name,comments,more
85.5,XYZ1234,Alice,All good,ok
90.0,XYZ5678,Bob,Too long comment indeed,ok
95.0,xWF9101,Charlie,code,ok
110.0,XYZ2345,Dave,range,ok
34.0,XYZ6789,,name,ok
78.0,XYZ7890,Frank,more,too long field indeed
f34s,XYZ3456,Eve,comments,ok
```

Running this example will generate these messages: 

```shell
Line 1: Record is valid: TestRecord { grade: 85.5, code: "XYZ1234", name: Some("Alice"), comments: "All good", more: Some("ok") }
Line 2: Validation errors:
  Field `comments`: Comments too long
Line 3: Validation errors:
  Field `code`: does not match the expected pattern
Line 4: Validation errors:
  Field `grade`: value out of expected range: 0 to 100
Line 5: Validation errors:
  Field `name`: mandatory field
Line 6: Validation errors:
  Field `more`: length out of expected range: 1 to 20
Error: Error(Deserialize { pos: Some(Position { byte: 230, line: 8, record: 7 }), err: DeserializeError { field: Some(0), kind: ParseFloat(ParseFloatError { kind: Invalid }) } })
```

## Why Use This Crate?

* **Declarative API:** Define validation rules directly in your struct.
* **Zero Runtime Overhead:** All checks are generated at compile time.
* **Seamless Serde & CSV Integration:** Works directly with `serde` and `csv` crates.
* **Clear Error Messages:** Each failure reports the field and reason.

## Comparison with csv Crate Validations

While the `csv` crate provides low‑level parsing and some helper methods, this derive‑based approach offers:

* **Field‑Level Declarative Rules:** Annotate each struct field with its own validation, rather than writing imperative checks after parsing.
* **Type‑Safety & Integration:** Leverages your existing `serde::Deserialize` types, so you get compile‑time guarantees on types and validations in one place.
* **Custom Validators:** Easily plug in custom functions per field without manual looping or error‑handling boilerplate.
* **Centralized Error Collection:** Automatically collects all errors into a single `Vec<ValidationError>`, instead of ad‑hoc early exits.
* **Reusable Across Projects:** Define your struct once, reuse validations in different contexts (CLI, web server, batch jobs) with the same guarantees.

By contrast, using the `csv` crate directly may require manual loops over records and explicit `match`/`if` chains for each validation, leading to more boilerplate and potential for missing checks.

## Compatibility

* This crate requires the Rust standard library (it is **not** compatible with `#![no_std]` environments).
* Rust **1.56+**
* `serde` **1.0**
* `csv` **1.3**
* `regex` **1.11**

## Contributing

Feel free to open issues and submit pull requests. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

## Links

* **Released on crates.io:** [csv-schema-validator](https://crates.io/crates/csv-schema-validator)
* **API Documentation:** [docs.rs](https://docs.rs/csv-schema-validator)
* **Source Code:** [https://github.com/cleuton/rustingcrab/tree/main/code_samples/csv-schema-validator](https://github.com/cleuton/rustingcrab/tree/main/code_samples/csv-schema-validator)
