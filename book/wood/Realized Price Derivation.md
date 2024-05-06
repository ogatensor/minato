## Realized Price Derivation

### [[Realized Price Definition]]

Defining the "realized price" ($p$):

This is the actual price at which the swap was executed on the OFA interface. It takes into account the input token amount ($i$), output token amount ($o$), gas used ($g$), base fee per gas ($b$), and priority fee per gas ($f$). The formula is:
$$
p = \frac{o}{i + g(b + f)}
$$

### Table of Contents

- [[#Realized Price Definition|Realized Price Definition]]
- [[#Mathematical Derivation|Mathematical Derivation]]
- [[#Mathematical Structure Definition|Mathematical Structure Definition]]
- [[#Realized Price Derivation|Realized Price Derivation]]
- [[#Derivation Steps|Derivation Steps]]
- [[#Dialogues & Analysis|Dialogues & Analysis]]
- [[#Understanding Realized Price Derivation in Context|Understanding Realized Price Derivation in Context]]

### Mathematical Derivation

To derive the realized price in the context of an Automated Market Maker (AMM), we first need to define the mathematical structure in which the AMM operates. This structure will include the variables and parameters that influence the price calculation in an AMM environment.

#### Mathematical Structure Definition

The mathematical structure for the AMM includes:

- Input token amount ($i$): [[Realized Price Definition#Input Token|Input Token]]
- Output token amount ($o$): [[Realized Price Definition#Output Token|Output Token]]
- Gas used ($g$): [[Realized Price Definition#Gas Used|Gas Used]]
- Base fee per gas ($b$): [[Realized Price Definition#Base Fee|Base Fee]]
- Priority fee per gas ($f$): [[Realized Price Definition#Priority Fee|Priority Fee]]

#### Realized Price Derivation

Given the above structure, the realized price $p$ in an AMM can be derived by considering how the AMM handles transaction costs, including gas fees. The formula for the realized price, considering the gas costs, is typically given by:

$$
p = \frac{o}{i + g(b + f)}
$$

This formula calculates the effective price per input token, taking into account the gas costs paid in terms of the input token. Here's the breakdown:

- $\frac{o}{i}$ represents the basic exchange rate offered by the AMM, ignoring transaction costs.
- $g(b + f)$ represents the total gas cost of the transaction, converted into the input token's units.

#### Derivation Steps

1. **Calculate the total cost of gas in input token units:** [[Realized Price Definition#Gas Cost Calculation|Gas Cost Calculation]]
   - The total gas cost is the product of the gas used and the sum of the base and priority gas fees: $g \times (b + f)$.

2. **Adjust the input amount by adding the gas cost:** [[Realized Price Definition#Input Adjustment|Input Adjustment]]
   - The effective input amount considering the gas cost is $i + g(b + f)$.

3. **Compute the realized price:** [[Realized Price Definition#Realized Price Computation|Realized Price Computation]]
   - The realized price is the ratio of the output token amount to the adjusted input amount: $p = \frac{o}{i + g(b + f)}$.

This derivation provides a clear understanding of how transaction costs are integrated into the price calculation in an AMM setting, ensuring that the realized price reflects both the market conditions and the cost of executing the trade.

### Dialogues & Analysis

It's interesting to consider how Wuxing theory might provide a framework for interpreting the realized price expression. Here's a possible approach, keeping in mind that this involves applying a traditional philosophical concept to a modern financial calculation:

**Wuxing Elements and their Correspondences**

|Element|Traditional Associations|Possible Interpretation in Realized Price|
|---|---|---|
|Wood (木)|Growth, expansion, upward movement|Input token amount ($i$): [[Realized Price Definition#Input Token|Input Token]](#Input_Token), as it represents the initial resource put into the trade|
|Fire (火)|Transformation, change, energy|Output token amount ($o$): [[Realized Price Definition#Output Token|Output Token]](#Output_Token), the result of the swap process|
|Earth (土)|Stability, foundation, grounding|Base fee per gas ($b$): [[Realized Price Definition#Base Fee|Base Fee]](#Base_Fee), an underlying cost that ensures transaction processing|
|Metal (金)|Precision, value, refinement|Priority fee per gas ($f$): [[Realized Price Definition#Priority Fee|Priority Fee]](#Priority_Fee), an additional cost to gain faster execution|
|Water (水)|Fluidity, adaptability, flow|Gas used ($g$): [[Realized Price Definition#Gas Used|Gas Used]](#Gas), a variable cost depending on network conditions|

# Wuxing Elements and Realized Price Parameters System
---

## Elemental Mapping
- **Wood (木)**: Represents growth and expansion.
  - **Mapped to**: Input token amount ($i$)
  - **Reasoning**: The input token is the fundamental resource that initiates the trade, embodying the potential for growth in the transaction.
  - **Link**: [[Realized Price Definition#Input Token|Input Token]]
- **Fire (火)**: Symbolizes transformation and energy.
  - **Mapped to**: Output token amount ($o$)
  - **Reasoning**: The output token is the result of the swap process, representing the transformation of the input token into another form.
  - **Link**: [[Realized Price Definition#Output Token|Output Token]]
- **Earth (土)**: Denotes stability and foundation.
  - **Mapped to**: Base fee per gas ($b$)
  - **Reasoning**: The base fee provides a stable cost foundation for processing the transaction on the blockchain.
  - **Link**: [[Realized Price Definition#Base Fee|Base Fee]]
- **Metal (金)**: Indicates precision and refinement.
  - **Mapped to**: Priority fee per gas ($f$)
  - **Reasoning**: The priority fee refines the transaction processing speed by paying extra, ensuring a faster or more timely execution.
  - **Link**: [[Realized Price Definition#Priority Fee|Priority Fee]]
- **Water (水)**: Reflects fluidity and adaptability.
  - **Mapped to**: Gas used ($g$)
  - **Reasoning**: Gas usage varies with network conditions, reflecting the adaptable nature of transaction costs.
  - **Link**: [[Realized Price Definition#Gas Used|Gas Used]]

## Interaction Model
- **Generative Relationships**:
  - **Wood fuels Fire**: The input token amount ($i$) directly influences the output token amount ($o$), as more input can potentially yield more output, depending on the market conditions and AMM algorithms.
- **Controlling Relationships**:
  - **Metal cuts Wood**: Higher priority fees ($f$) can decrease the effective input by increasing the cost, thus controlling the potential growth initiated by the input token.
  - **Earth bears Metal**: The base fee ($b$) supports the priority fee by providing a foundational cost upon which the priority fee can operate.
  - **Water nourishes Wood**: The adaptability in gas usage ($g$) can help facilitate the transaction, ensuring that the input token is processed efficiently under varying network conditions.

## Formula Integration
- **Realized Price Formula**: $$ p = \frac{o}{i + g(b + f)} $$
- **Elemental Influence on Formula**:
  - **Wood and Fire**: The ratio $\frac{o}{i}$ (output to input) is directly influenced by the amount of input token available.
  - **Metal and Earth**: The sum $g(b + f)$ (total gas cost) in the denominator showcases how the transaction fees (Metal and Earth) control and stabilize the realized price by adding to the input cost.
  - **Water's Role**: The variable $g$ (gas used) adapts the formula to current network conditions, affecting the overall cost and thus the price.

This structured system provides a comprehensive view of how traditional Wuxing elements can be interpreted within the context of a modern financial metric like the realized price in an AMM setting. This approach not only aids in understanding the financial model but also enriches it with philosophical insights, making the analysis more robust and multidimensional.

----


**Analyzing the Expression through Wuxing**

- **Interaction of Elements:** The formula combines these elements in a dynamic manner:
    
    - **Wood fuels Fire:** The input token (Wood) drives the transformation into the output token (Fire).
    - **Metal refines Fire:** The priority fee (Metal) can shape the speed and outcome of the transformative process (Fire).
    - **Earth supports all:** The base fee (Earth) is a fundamental platform upon which the transaction rests.
    - **Water adapts:** The gas used (Water) reflects changing circumstances influencing the trade's flow.
- **Generative and Controlling Cycles:** The formula can be seen to embody aspects of both the generative (生) and controlling (克) cycles of Wuxing:
    
    - **Generative:** Wood's expansion generates Fire's transformation.
    - **Controlling:** Metal's precision can manage the intensity of Fire.

**Limitations** It's important to note that Wuxing theory is primarily philosophical and symbolic. Applying it to a precise mathematical formula requires a degree of creative interpretation.

**Let's Discuss**

Would you like to delve deeper into how specific elements of Wuxing could be mapped onto the formula, or consider potential ways to refine the analogy?

[[#Dialogues & Analysis|Dialogues & Analysis]]

**Absolutely! Let's analyze the parameters within the realized price rational expression in terms of the Sheng (generating) and Ke (controlling) cycles of Wuxing theory.**

**Reminder: The Realized Price Formula**

$$
p = \frac{o}{i + g(b + f)}
$$

**Sheng Cycle (#Sheng_Cycle)**

The Sheng cycle (#Sheng_Cycle) represents a relationship where one element nourishes or supports the growth of another. Here's how we could see this within the formula:

- **Input token amount ($i$): [[Realized Price Definition#Input Token|Input Token]](#Input_Token) generates Output token amount ($o$): [[Realized Price Definition#Output Token|Output Token]](#Output_Token):** The core function of the AMM is to transform the input token into the output token. Larger values of 'i' drive a larger 'o', even if not proportionally. This reflects a generative relationship.
- **Gas used ($g$): [[Realized Price Definition#Gas Used|Gas Used]](#Gas) facilitating the process:** While gas represents a cost, it's essential for the transaction to occur. Without gas, the swap from input to output tokens could not take place. In a way, it fuels the process.

**Ke Cycle (#Ke_Cycle)**

The Ke cycle (#Ke_Cycle) embodies a controlling or regulating relationship. We can identify this dynamic within the formula:

- **Base fee per gas ($b$): [[Realized Price Definition#Base Fee|Base Fee]](#Base_Fee) and Priority fee per gas ($f$): [[Realized Price Definition#Priority Fee|Priority Fee]](#Priority_Fee) controlling cost:** These fees directly impact the denominator. Larger fees (especially with higher gas usage) reduce the realized price 'p'. This shows a controlling influence on the overall outcome.
- **Output token amount ($o$): [[Realized Price Definition#Output Token|Output Token]](#Output_Token) inversely related to cost:** While not explicitly within the cost portion of the formula, the concept of the realized price highlights that a higher output 'o' has a mitigating effect against the transaction costs to some degree.

**Mathematical Properties and [[Wuxing]](#Wuxing)**

Here's where it gets even more interesting:

- **Division as Ke (#Division_as_Ke):** The act of division itself could be seen as a form of control. The numerator (output) is constrained by the denominator (input + costs).
- **Asymptotic Behavior (#Asymptotic_Behavior):** Rational expressions often exhibit asymptotic behavior. If transaction costs increase dramatically, the realized price will approach a limit dictated by those costs. This reflects a strong controlling influence on potential gains.

**Important Notes:**

- **Dynamic Interaction (#Dynamic_Interaction):** The variables are not fixed; they change in relation to market conditions and trader choices. Thus, the Sheng and Ke relationships continually shift.
- **Multiple Interpretations (#Multiple_Interpretations):** Depending on your focus, you might see other potential Sheng and Ke relationships within the structure of the formula.

**Let's continue! Would you like to...**

- Explore how the concept of balance (阴阳) within Wuxing theory might be reflected in the price calculation for optimal trading?
- Analyze the formula in different market conditions (high/low gas, volatile tokens, etc.) to see how the Sheng/Ke dynamics change?

Let me know where you want to take the analysis!

### Understanding Realized Price Derivation in Context

The realized price derivation, as detailed in the document "Realized Price Derivation" ([Realized Price Derivation](book/wood/Realized Price Derivation.md)), is a crucial component of the broader analysis presented across various sections of the paper. Here's how it integrates and interacts with other key concepts:

#### Connection with Mathematical Formulations

The formula for realized price:
```markdown:book/wood/Realized Price Derivation.md
startLine: 32
endLine: 34
```
is also reflected in the mathematical formulations in the README, emphasizing its importance in calculating transaction costs and price improvements:

```40:46:README.md
1. **Price Definition**:
   - For transactions where the input or output token is the gas token (e.g., ETH/WETH):
     $$p = \frac{o}{i + g(b + f)}$$

   - For transactions where gas is internalized by the OFA:
     $$p = \frac{o}{i}$$
```

#### Role in Price Improvement Framework

The realized price is foundational in the framework for assessing price improvements in OFAs, as outlined in [Framework for Price Improvement in OFAs](book/earth/Framework for Price Improvement in OFAs.md). It helps in comparing the actual transaction prices against baseline or counterfactual prices, which are crucial for evaluating the efficiency of different OFA systems.

#### Impact on Execution Quality

Understanding the realized price is essential for assessing the execution quality in OFAs ([Execution Quality in OFAs](book/fire/Execution Quality in OFAs.md)). It directly influences the primary metric of price improvement, which measures the effectiveness of trade executions in OFA systems.

#### Numerical Precision and Calculations

The precision in calculating the realized price is critical, as discussed in [Numerical Precision Concerns for Orderflow Auction Analysis Considerations](book/wood/Numerical Precision Concerns for Orderflow Auction Analysis Considerations.md). Accurate computation ensures reliable assessments of OFA performance and supports robust financial analysis.

#### Integration with Alloy Library

For developers implementing these concepts using Rust, the [Alloy library](book/metal/alloy-rs.md) provides tools for interacting with Ethereum blockchains, which can be useful for simulations and real-time calculations involving realized prices and other financial metrics.

By understanding the derivation and application of the realized price within these contexts, one can appreciate its central role in the analytical frameworks used for evaluating OFAs, thereby enhancing the strategic decisions regarding trade executions and system optimizations.
By understanding the derivation and application of the realized price within these contexts, one can appreciate its central role in the analytical frameworks used for evaluating OFAs, thereby enhancing the strategic decisions regarding trade executions and system optimizations.
