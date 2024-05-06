# Numerical Precision Concerns for Orderflow Auction Analysis Considerations

## Navigation
- [[#Overview|Overview]]
- [[#Floating-Point Errors|Floating-Point Errors]]
- [[#Rust and IEEE-754 Standard|Rust and IEEE-754 Standard]]
- [[#Ethereum and Floating-Point Types|Ethereum and Floating-Point Types]]
- [[#Further Reading|Further Reading]]

## Overview
This section discusses the importance of numerical precision in financial calculations, particularly when working with real money. It highlights the potential issues that can arise from using floating-point arithmetic, which is the standard in many programming languages, including Rust.

## Floating-Point Errors
The author notes that floating-point errors can be "disastrous" when working with real money. To mitigate this, it is common to use integer types (including pennies as the last two digits) or fixed-point libraries for financial calculations.

## Rust and IEEE-754 Standard
Rust, like many other languages, uses the [[IEEE-754 Standard]] to represent floating-point numbers. This can lead to small discrepancies between the expected and actual output due to the inherent limitations of floating-point arithmetic.

## Ethereum and Floating-Point Types
The IEEE-754 standard for floating-point arithmetic is not applicable in the Ethereum environment, as [[Solidity]] (the programming language used for Ethereum smart contracts) does not have floating-point types. This means that the numerical precision concerns discussed are relevant for safe type conversions, off-chain computations, and simulations that are performed in support or in conjunction with on-chain protocols.

### When You Need Overflow

Sometimes, you want numeric overflow to occur. Many cryptographic and random number generation algorithms assume that integer wrapping will occur. Rust lets you opt in to the behavior with the [[std::num::Wrapping]] facility. A safe version of this program looks like this:

[byte_sized_wrap/src/main.rs](http://media.pragprog.com/titles/hwrustbrain/code/byte_sized_wrap/src/main.rs)

### Detecting Overflow without Crashing

If your program doesn't need wrapping behavior, but you're concerned that you might run into a situation in which you overflow the capacity of a variable, Rust has your back. Rust's numeric types implement a series of checked functions: checked_add, checked_div, checked_mul, checked_sub, and a few others. These functions return an Option that will either contain Some(x) if the operation succeeded or None if an overflow occurred, as shown in this example:

## Further Reading
The section provides links to additional resources for further exploration:

1. **[[IEEE-754 Floating Point Standard]]**: https://en.wikipedia.org/wiki/IEEE_754
2. **[[RUG—Arbitrary Precision Numbers crate]]**: https://lib.rs/crates/rug
3. **[[f128 crate]]**: https://lib.rs/crates/f128
4. **[[fixed crate]]**: https://docs.rs/fixed/1.10.0/fixed/
5. **[[Alloy]]**: https://alloy-rs.github.io/alloy/alloy/index.html
6. **[[std::num::Wrapping]]**: [https://doc.rust-lang.org/std/num/struct.Wrapping.html](https://doc.rust-lang.org/std/num/struct.Wrapping.html)
7. **[[Two's Complement]]**: [https://en.wikipedia.org/wiki/Two%27s_complement](https://en.wikipedia.org/wiki/Two%27s_complement)
8. **[[Cargo Profiles]]**: [https://doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html)
9. **[[Rust Data Types]]**: [https://doc.rust-lang.org/book/ch03-02-data-types.html](https://doc.rust-lang.org/book/ch03-02-data-types.html)