# Numerical Precision Concerns for Orderflow Auction Analysis Considerations 

"If you’re working with real money, floating point errors can be disastrous (it’s common to use an integer type including pennies as the last two digits, or a fixed-point library for financial calculations)." - Wolverson, H. (2023). Rust Brain Teasers. Pragmatic Bookshelf.

Rust, like many other languages, uses the IEEE-754 standard to represent floating-point numbers, which can lead to small discrepancies between the expected and actual output.

# Further Reading 

IEE-754 Floating Point Standard:
https://en.wikipedia.org/wiki/IEEE_754

RUG—Arbitrary Precision Numbers crate:
https://lib.rs/crates/rug

f128 crate:
https://lib.rs/crates/f128

fixed crate:
https://docs.rs/fixed/1.10.0/fixed/

