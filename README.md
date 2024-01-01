# Function AI Crate

The 'function_ai' crate provides a Rust procedural macro that allows you to transform any function into a function that returns its own definition as a string. This is useful for sending the function's code to a large language model for further processing.

## Installation

Add the following to your 'Cargo.toml' file:

```toml
[dependencies]
function_ai = "0.1.0"
```

Then run cargo build to download and compile the function_ai crate.

## Usage

To use the function_ai macro, simply annotate your function with #[function_ai].

use function_ai::function_ai;

```
#[function_ai]
fn example_function(arg: i32) -> i32 {
  arg \* 2
}
```

When you call example_function(), instead of returning arg * 2, it will return a string containing the source code of the example_function.

## Limitations

The #[function_ai] macro currently does not support functions with complex control flow like loops or conditionals. It only supports simple function definitions.

```
use function_ais::function_ai;

#[function_ai]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    assert_eq!(add(2, 3), "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}");
}
```