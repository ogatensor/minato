### [[Execution Quality in OFAs]]

## Navigation
- [[#Overview|Overview]]
- [[#Price Improvement|Price Improvement]]
- [[#Factors Affecting Price Improvement|Factors Affecting Price Improvement]]
- [[#Comparison Across OFA Systems|Comparison Across OFA Systems]]
- [[#Timing Considerations|Timing Considerations]]

In the context of OFA systems, execution quality refers to the overall effectiveness and desirability of the trade execution experienced by users. The paper focuses on evaluating execution quality through the lens of "price improvement", which is defined as the difference between the realized price of a swap and a counterfactual baseline price.

Specifically, the paper highlights a few key aspects of execution quality in OFA systems:

1. **[[Price Improvement]]**: This is the primary metric used to assess execution quality. It captures how much better (or worse) the realized price is compared to a simulated baseline trade.
2. **[[Factors Affecting Price Improvement]]**: Routing Efficiency, Gas Optimization, and Priority Fee Settings.
3. **[[Comparison Across OFA Systems]]**: The framework allows for a systematic comparison of execution quality across different OFA implementations, such as 1Inch Fusion, UniswapX, 1Inch Aggregator, and Uniswap Classic.
4. **[[Timing Considerations]]**: The paper also evaluates price improvement at different time offsets from the settlement block, to account for factors like transaction ordering and blockchain state changes.