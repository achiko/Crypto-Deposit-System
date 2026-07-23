use num_bigint::{BigUint, ParseBigIntError};

fn main() -> Result<(), ParseBigIntError> {
    // 1. Convert a native Rust integer into a BigUint.
    let native_number = u128::MAX;
    let big_uint_from_number = BigUint::from(native_number);
    println!("From u128:  {big_uint_from_number}");

    // 2. Parse a decimal string into a BigUint.
    let decimal_string = "340282366920938463463374607431768211455009292928383938829838387493948489939399403943993282330000000333443221838393944334559992033";
    let big_uint_from_string = decimal_string.parse::<BigUint>()?;
    println!("From string: {big_uint_from_string}");

    // 3. Calculate 100^6 using a BigUint.
    let base = "100".parse::<BigUint>()?;
    let power_result = base.pow(6);
    println!("100^6:      {power_result}");

    // A BigUint can also be converted back into its decimal string representation.
    let converted_back_to_string = big_uint_from_string.to_string();
    println!("Back to str: {converted_back_to_string}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_u128_to_big_uint() {
        let value = BigUint::from(u128::MAX);

        assert_eq!(value.to_string(), u128::MAX.to_string());
    }

    #[test]
    fn parses_decimal_string_to_big_uint() {
        let decimal_string =
            "34028236692093846346337460743176821145500929292838393882983838749394848";
        let value = decimal_string
            .parse::<BigUint>()
            .expect("the test string should be a valid decimal integer");

        assert_eq!(value.to_string(), decimal_string);
    }
}
