# Wuxing Correspondence Table

## Elemental Mappings

| Parameter/Equation | Wuxing Element | Characteristics | Role |
| --- | --- | --- | --- |
| [[Input Token Amount ($i$)]] | [[Wood (木)]] | Growth, expansion, upward movement | Represents the initial resource put into the trade |
| [[Output Token Amount ($o$)]] | [[Fire (火)]] | Transformation, change, energy | The result of the swap process |
| [[Base Fee per Gas ($b$)]] | [[Earth (土)]] | Stability, foundation, nourishment | Provides a stable foundation for the transaction |
| [[Priority Fee per Gas ($f$)]] | [[Metal (金)]] | Precision, value, speed | Refines and optimizes the transaction speed |
| [[Gas Used ($g$)]] | [[Water (水)]] | Adaptability, flow, minimization | Adapts to the fluidity of network conditions |
| [[Routing Efficiency ($\pi_{routing}$)]] | [[Wood (木)]] | Growth, expansion, optimization | Enhances the efficiency of routing to maximize output |
| [[Gas Optimization ($\pi_{gas}$)]] | [[Water (水)]] | Adaptability, flow, minimization | Reduces gas costs, adapting to network conditions |
| [[Priority Fee Optimization ($\pi_{fee}$)]] | [[Metal (金)]] | Precision, value, speed | Adjusts fees to optimize transaction speed |

## Wuxing Interpretation of Formulas/Processes

### Realized Price Formula
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

### Price Improvement Formula
From the README.md, we have the price improvement formula:
```math
\pi = \pi_{routing} + \pi_{gas} + \pi_{fee}
```

**Elemental Mappings**:
- **Routing Efficiency ($\pi_{routing}$)** - Wood (木)
  - **Element**: Wood (木)
  - **Characteristics**: Growth, expansion, optimization.
  - **Role in Formula**: Enhances the efficiency of routing to maximize output.
  - **Tag**: #Wood_Element [[Routing Efficiency ($\pi_{routing}$)]]

- **Gas Optimization ($\pi_{gas}$)** - Water (水)
  - **Element**: Water (水)
  - **Characteristics**: Adaptability, flow, minimization.
  - **Role in Formula**: Reduces gas costs, adapting to network conditions.
  - **Tag**: #Water_Element [[Gas Optimization ($\pi_{gas}$)]]

- **Priority Fee Optimization ($\pi_{fee}$)** - Metal (金)
  - **Element**: Metal (金)
  - **Characteristics**: Precision, value, speed.
  - **Role in Formula**: Adjusts fees to optimize transaction speed.
  - **Tag**: #Metal_Element [[Priority Fee Optimization ($\pi_{fee}$)]]

