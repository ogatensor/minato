# Wuxing Elements and Realized Price Parameters System

## Elemental Mapping
Each parameter in the realized price formula is associated with a Wuxing element, providing a philosophical insight into its role in the formula. Below are the mappings and links to their detailed definitions:

### Input Token Amount ($i$) - Wood (木)
- **Element**: Wood (木)
- **Characteristics**: Growth, expansion, upward movement.
- **Role in Formula**: Represents the initial resource put into the trade.
- **Link**: [[Realized Price Definition#Input Token|Input Token]]
- **Tag**: #Wood_Element [[Input Token Amount ($i$)]]

### Output Token Amount ($o$) - Fire (火)
- **Element**: Fire (火)
- **Characteristics**: Transformation, change, energy.
- **Role in Formula**: The result of the swap process.
- **Link**: [[Realized Price Definition#Output Token|Output Token]]
- **Tag**: #Fire_Element [[Output Token Amount ($o$)]]

### Base Fee per Gas ($b$) - Earth (土)
- **Element**: Earth (土)
- **Characteristics**: Stability, foundation, grounding.
- **Role in Formula**: An underlying cost that ensures transaction processing.
- **Link**: [[Realized Price Definition#Base Fee|Base Fee]]
- **Tag**: #Earth_Element [[Base Fee per Gas ($b$)]]

### Priority Fee per Gas ($f$) - Metal (金)
- **Element**: Metal (金)
- **Characteristics**: Precision, value, refinement.
- **Role in Formula**: An additional cost to gain faster execution.
- **Link**: [[Realized Price Definition#Priority Fee|Priority Fee]]
- **Tag**: #Metal_Element [[Priority Fee per Gas ($f$)]]

### Gas Used ($g$) - Water (水)
- **Element**: Water (水)
- **Characteristics**: Fluidity, adaptability, flow.
- **Role in Formula**: A variable cost depending on network conditions.
- **Link**: [[Realized Price Definition#Gas Used|Gas Used]]
- **Tag**: #Water_Element [[Gas Used ($g$)]]

## Interaction Model
Explore how these elements interact within the formula and influence the realized price through generative and controlling relationships.

## Formula Integration
Detailed breakdown of how each element influences the realized price formula:
```math
p = \frac{o}{i + g(b + f)}
```

This structured system with tags allows for quick referencing and visualization in Obsidian, making it easier to navigate between the philosophical and practical aspects of the realized price derivation.

To integrate the elemental mappings with other equations in the codebase, we can extend the Obsidian formatting to include mappings for each relevant formula. Below, I'll outline the additional elemental mappings for the equations found in the provided code snippets.

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

### Baseline Generation Function
From the README.md, the baseline function is:
```math
B: (i, t) \rightarrow (o', g')
```
**Elemental Mappings**:
- **Input Amount ($i$)** - Wood (木)
  - **Element**: Wood (木)
  - **Characteristics**: Initiation, resource input.
  - **Role in Formula**: Starts the baseline generation process.
  - **Tag**: #Wood_Element [[Input Amount ($i$)]]

- **Output Amount ($o'$)** - Fire (火)
  - **Element**: Fire (火)
  - **Characteristics**: Transformation, result.
  - **Role in Formula**: Represents the outcome of the baseline process.
  - **Tag**: #Fire_Element [[Output Amount ($o'$)]]

- **Gas Used ($g'$)** - Water (水)
  - **Element**: Water (水)
  - **Characteristics**: Flexibility, essential operation.
  - **Role in Formula**: Reflects the adaptability in simulating different network conditions.
  - **Tag**: #Water_Element [[Gas Used ($g'$)]]

### Taylor Series Expansion for Price Improvement Attribution
From the README.md, the Taylor series expansion used for price improvement attribution is:
```math
\pi = \left.\frac{\partial p}{\partial o}\right|_{o',g',f'} \cdot \frac{o - o'}{p'} + \left.\frac{\partial p}{\partial g}\right|_{o',g',f'} \cdot \frac{g - g'}{p'} + \left.\frac{\partial p}{\partial f}\right|_{o',g',f'} \cdot \frac{f - f'}{p'} + R(x, x')
```
**Elemental Mappings**:
- **Output Token Differential ($\frac{o - o'}{p'}$)** - Fire (火)
  - **Element**: Fire (火)
  - **Characteristics**: Transformation, impact.
  - **Role in Formula**: Reflects the transformative impact on price due to output changes.
  - **Tag**: #Fire_Element [[Output Token Differential ($\frac{o - o'}{p'}$)]]

- **Gas Differential ($\frac{g - g'}{p'}$)** - Water (水)
  - **Element**: Water (水)
  - **Characteristics**: Adaptability, cost variation.
  - **Role in Formula**: Adapts the price based on variations in gas usage.
  - **Tag**: #Water_Element [[Gas Differential ($\frac{g - g'}{p'}$)]]

- **Fee Differential ($\frac{f - f'}{p'}$)** - Metal (金)
  - **Element**: Metal (金)
  - **Characteristics**: Precision, cost adjustment.
  - **Role in Formula**: Refines the price based on priority fee adjustments.
  - **Tag**: #Metal_Element [[Fee Differential ($\frac{f - f'}{p'}$)]]

## Elemental Mapping Table
| Element | Characteristics | Role in Formula | Link | Tag |
| --- | --- | --- | --- | --- |
| Wood (木) | Growth, expansion, upward movement | Represents the initial resource put into the trade | [[Realized Price Definition#Input Token|Input Token]] | #Wood_Element [[Input Token Amount ($i$)]] |
| Fire (火) | Transformation, change, energy | The result of the swap process | [[Realized Price Definition#Output Token|Output Token]] | #Fire_Element [[Output Token Amount ($o$)]] |
| Earth (土) | Stability, foundation, grounding | An underlying cost that ensures transaction processing | [[Realized Price Definition#Base Fee|Base Fee]] | #Earth_Element [[Base Fee per Gas ($b$)]] |
| Metal (金) | Precision, value, refinement | An additional cost to gain faster execution | [[Realized Price Definition#Priority Fee|Priority Fee]] | #Metal_Element [[Priority Fee per Gas ($f$)]] |
| Water (水) | Fluidity, adaptability, flow | A variable cost depending on network conditions | [[Realized Price Definition#Gas Used|Gas Used]] | #Water_Element [[Gas Used ($g$)]] |

## Price Improvement Formula Elemental Mappings
| Element | Characteristics | Role in Formula | Tag |
| --- | --- | --- | --- |
| Wood (木) | Growth, expansion, optimization | Enhances the efficiency of routing to maximize output | #Wood_Element [[Routing Efficiency ($\pi_{routing}$)]] |
| Water (水) | Adaptability, flow, minimization | Reduces gas costs, adapting to network conditions | #Water_Element [[Gas Optimization ($\pi_{gas}$)]] |
| Metal (金) | Precision, value, speed | Adjusts fees to optimize transaction speed | #Metal_Element [[Priority Fee Optimization ($\pi_{fee}$)]] |

## Baseline Generation Function Elemental Mappings
| Element | Characteristics | Role in Formula | Tag |
| --- | --- | --- | --- |
| Wood (木) | Initiation, resource input | Starts the baseline generation process | #Wood_Element [[Input Amount ($i$)]] |
| Fire (火) | Transformation, result | Represents the outcome of the baseline process | #Fire_Element [[Output Amount ($o'$)]] |
| Water (水) | Flexibility, essential operation | Reflects the adaptability in simulating different network conditions | #Water_Element [[Gas Used ($g'$)]] |

## Taylor Series Expansion Elemental Mappings
| Element | Characteristics | Role in Formula | Tag |
| --- | --- | --- | --- |
| Fire (火) | Transformation, impact | Reflects the transformative impact on price due to output changes | #Fire_Element [[Output Token Differential ($\frac{o - o'}{p'}$)]] |
| Water (水) | Adaptability, cost variation | Adapts the price based on variations in gas usage | #Water_Element [[Gas Differential ($\frac{g - g'}{p'}$)]] |
| Metal (金) | Precision, cost adjustment | Refines the price based on priority fee adjustments | #Metal_Element [[Fee Differential ($\frac{f - f'}{p'}$)]] |

These mappings can be added to the Obsidian documentation to provide a comprehensive view of how traditional Wuxing elements are integrated into various financial metrics and formulas across the codebase. This approach not only aids in understanding the mathematical models but also enriches them with philosophical insights, making the analysis more robust and multidimensional.
These mappings can be added to the Obsidian documentation to provide a comprehensive view of how traditional Wuxing elements are integrated into various financial metrics and formulas across the codebase. This approach not only aids in understanding the mathematical models but also enriches them with philosophical insights, making the analysis more robust and multidimensional.