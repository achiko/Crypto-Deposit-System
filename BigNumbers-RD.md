# Big Numbers in Rust

# num_bigint

https://docs.rs/num-bigint/latest/num_bigint/index.html




## Introduction

This document describes how to use big numbers in Rust. It is based on the [BigInt](https://docs.rs/num-bigint/latest/num_bigint/) crate, which is a Rust implementation of arbitrary-precision integers.

## Motivation

The Rust ecosystem has a rich set of libraries for working with numbers. However, there is no standard library implementation of arbitrary-precision integers. This makes it difficult to use in applications that require high-precision calculations.

## Solution

The `BigNumbers` crate provides a wrapper around the `num-bigint` crate that provides a standard interface for working with big numbers. It is designed to be easy to use and to integrate with existing Rust code.

## Usage

To use the `BigNumbers` crate, add the following to your `Cargo.toml` file:

```toml
[dependencies]
big-numbers = "0.1.0"
```

Then, import the `BigNumbers` module in your code:

```rust
use big_numbers::BigNumbers;
```

## Examples

### Creating a BigNumber

You can create a `BigNumber` from a string or a byte array:

```rust
let num = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num = BigNumbers::from_bytes(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
```

### Converting to a String

You can convert a `BigNumber` to a string:

```rust
let num = BigNumbers::from_str("1234567890123456789012345678901234567890");
let str = num.to_str();
```

### Converting to a Byte Array

You can convert a `BigNumber` to a byte array:

```rust
let num = BigNumbers::from_str("1234567890123456789012345678901234567890");
let bytes = num.to_bytes();
```

### Adding BigNumbers

You can add two `BigNumbers` together:

```rust
let num1 = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num2 = BigNumbers::from_str("9876543210987654321098765432109876543210");
let sum = num1.add(&num2);
```

### Subtracting BigNumbers

You can subtract one `BigNumber` from another:

```rust
let num1 = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num2 = BigNumbers::from_str("9876543210987654321098765432109876543210");
let diff = num1.sub(&num2);
```

### Multiplying BigNumbers

You can multiply two `BigNumbers` together:

```rust
let num1 = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num2 = BigNumbers::from_str("9876543210987654321098765432109876543210");
let prod = num1.mul(&num2);
```

### Dividing BigNumbers

You can divide one `BigNumber` by another:

```rust
let num1 = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num2 = BigNumbers::from_str("9876543210987654321098765432109876543210");
let quotient = num1.div(&num2);
```

### Modulo BigNumbers

You can calculate the remainder of one `BigNumber` divided by another:

```rust
let num1 = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num2 = BigNumbers::from_str("9876543210987654321098765432109876543210");
let remainder = num1.mod(&num2);
```

### Exponentiating BigNumbers

You can raise one `BigNumber` to the power of another:

```rust
let num1 = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num2 = BigNumbers::from_str("9876543210987654321098765432109876543210");
let result = num1.pow(&num2);
```

### Comparing BigNumbers

You can compare two `BigNumbers` for equality:

```rust
let num1 = BigNumbers::from_str("1234567890123456789012345678901234567890");
let num2 = BigNumbers::from_str("9876543210987654321098765432109876543210");
let result = num1.eq(&num2);
```
