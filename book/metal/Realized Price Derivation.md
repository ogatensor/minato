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

#### Wuxing Interpretation of the Realized Price Formula

The realized price formula can be interpreted through the lens of Wuxing theory as follows:

- **Input token amount ($i$): [[Realized Price Definition#Input Token|Input Token]](#Input_Token)** represents the [[Wood (木)]] element, as it initiates the growth and expansion of the trade.
- **Output token amount ($o$): [[Realized Price Definition#Output Token|Output Token]](#Output_Token)** corresponds to the [[Fire (火)]] element, symbolizing the transformation of the input token.
- **Base fee per gas ($b$): [[Realized Price Definition#Base Fee|Base Fee]](#Base_Fee)** aligns with the [[Earth (土)]] element, providing a stable foundation for the transaction.
- **Priority fee per gas ($f$): [[Realized Price Definition#Priority Fee|Priority Fee]](#Priority_Fee)** embodies the [[Metal (金)]] element, refining and optimizing the transaction speed.
- **Gas used ($g$): [[Realized Price Definition#Gas Used|Gas Used]](#Gas)** represents the [[Water (水)]] element, adapting to the fluidity of network conditions.

Within the formula, we can observe both the generative (Sheng) and controlling (Ke) cycles of Wuxing:

- **Sheng Cycle**: The input token amount (Wood) generates the output token amount (Fire), facilitated by the gas used (Water).
- **Ke Cycle**: The priority fee (Metal) and base fee (Earth) control and regulate the overall cost, influencing the realized price.

This interpretation provides a philosophical lens through which we can understand the interactions and dynamics within the realized price formula, enriching our analysis with traditional wisdom.

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

Absolutely! Let's integrate the concept of the counterfactual baseline into this analysis.

**Understanding Counterfactual Baseline in Context**

The counterfactual baseline, as proposed, adds a new dimension to the analysis of trade execution within an AMM setting. Here's how it relates to the broader themes discussed:

* **Comparison to Realized Price:** The fundamental purpose of the counterfactual price is to provide a benchmark against the actual realized price.  This comparison shines a light on potential optimizations that could have been made in routing or gas strategies.
* **Wuxing and Counterfactuals:**
    * **Sheng Cycle Limitation:** The counterfactual highlights a limitation of focusing purely on the generative (Sheng) aspect within the formula. A high realized price due to a large output token amount doesn't guarantee the best possible outcome. 
    * **Ke Cycle Emphasis:** The counterfactual baseline emphasizes the controlling (Ke) forces at play. Optimizations in routing and gas strategies represent a way to control the costs that influence the overall price a trader receives.
* **Execution Quality Measurement:** Unlike the realized price, which is a historical snapshot, the counterfactual baseline enables analysis of trade execution quality. The gap between the counterfactual and realized prices can become a metric for improvement over time.

**Where the Counterfactual Baseline Fits:**

Let's add a section to your existing document structure to highlight this concept:

**Table of Contents (Modified)**

* ... (Existing Sections) ...
* [[#Understanding Realized Price Derivation in Context|Understanding Realized Price Derivation in Context]]
* [[#Understanding Counterfactual Baseline in Context|Understanding Counterfactual Baseline in Context]]

**New Section: Understanding Counterfactual Baseline in Context**

* **Purpose:** Briefly reiterate the purpose of the counterfactual baseline – to model an alternate trade execution scenario for comparison.
* **Relation to Realized Price:** Explain that the counterfactual price enables an assessment of execution quality *relative* to the realized price.
* **Wuxing Integration:** Highlight the emphasis on the controlling (Ke) cycle of Wuxing theory through optimization of gas and routing.
* **Limitations of the Baseline:** Acknowledge the importance of the `baseline_function`. A poorly constructed model will produce meaningless counterfactual prices.

**Let's Expand!**

Here are some directions we could take this analysis further:

* **Baseline Function and Wuxing:** Could we draw more specific analogies between potential optimization techniques within an AMM setting and the elements within Wuxing? For example, could routing optimization be linked to the fluidity of Water?
* **Market Dynamics:**  How might the counterfactual baseline concept help analyze trade execution in the face of rapidly shifting market conditions involving volatility and sudden gas price changes?

Let me know which direction you'd like to explore! 
