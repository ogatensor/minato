Defining the "realized price" (p):

This is the actual price at which the swap was executed on the OFA interface.
It takes into account the input token amount (i), output token amount (o), gas used (g), base fee per gas (b), and priority fee per gas (f).
The formula is: p = o / (i + g(b + f))

## Mathematical Derivation

To derive the realized price in the context of an Automated Market Maker (AMM), we first need to define the mathematical structure in which the AMM operates. This structure will include the variables and parameters that influence the price calculation in an AMM environment.

### Mathematical Structure Definition

1. **Variables:**
   - $i$: Input token amount provided by the trader.
   - $o$: Output token amount received by the trader.
   - $g$: Gas used for the transaction.
   - $b$: Base fee per gas.
   - $f$: Priority fee per gas.

2. **Parameters:**
   - The AMM's pricing algorithm, which might be a function of the current liquidity pool state, the input and output token types, and other market conditions.

### Realized Price Derivation

Given the above structure, the realized price $p$ in an AMM can be derived by considering how the AMM handles transaction costs, including gas fees. The formula for the realized price, considering the gas costs, is typically given by:

$$
p = \frac{o}{i + g(b + f)}
$$

This formula calculates the effective price per input token, taking into account the gas costs paid in terms of the input token. Here's the breakdown:

- $\frac{o}{i}$ represents the basic exchange rate offered by the AMM, ignoring transaction costs.
- $g(b + f)$ represents the total gas cost of the transaction, converted into the input token's units.

### Derivation Steps:

1. **Calculate the total cost of gas in input token units:**
   - The total gas cost is the product of the gas used and the sum of the base and priority gas fees: $g \times (b + f)$.

2. **Adjust the input amount by adding the gas cost:**
   - The effective input amount considering the gas cost is $i + g(b + f)$.

3. **Compute the realized price:**
   - The realized price is the ratio of the output token amount to the adjusted input amount: $p = \frac{o}{i + g(b + f)}$.

This derivation provides a clear understanding of how transaction costs are integrated into the price calculation in an AMM setting, ensuring that the realized price reflects both the market conditions and the cost of executing the trade.
