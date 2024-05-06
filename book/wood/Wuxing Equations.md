# Wuxing Elements and Realized Price Parameters System

## Elemental Mapping
Each parameter in the realized price formula is associated with a Wuxing element, providing a philosophical insight into its role in the formula. Below are the mappings and links to their detailed definitions:

### Input Token Amount ($i$) - Wood (木)
- **Element**: Wood (木)
- **Characteristics**: Growth, expansion, upward movement.
- **Role in Formula**: Represents the initial resource put into the trade.
- **Link**: [[Realized Price Definition#Input Token|Input Token]]
- **Tag**: `#Wood_Element`

### Output Token Amount ($o$) - Fire (火)
- **Element**: Fire (火)
- **Characteristics**: Transformation, change, energy.
- **Role in Formula**: The result of the swap process.
- **Link**: [[Realized Price Definition#Output Token|Output Token]]
- **Tag**: `#Fire_Element`

### Base Fee per Gas ($b$) - Earth (土)
- **Element**: Earth (土)
- **Characteristics**: Stability, foundation, grounding.
- **Role in Formula**: An underlying cost that ensures transaction processing.
- **Link**: [[Realized Price Definition#Base Fee|Base Fee]]
- **Tag**: `#Earth_Element`

### Priority Fee per Gas ($f$) - Metal (金)
- **Element**: Metal (金)
- **Characteristics**: Precision, value, refinement.
- **Role in Formula**: An additional cost to gain faster execution.
- **Link**: [[Realized Price Definition#Priority Fee|Priority Fee]]
- **Tag**: `#Metal_Element`

### Gas Used ($g$) - Water (水)
- **Element**: Water (水)
- **Characteristics**: Fluidity, adaptability, flow.
- **Role in Formula**: A variable cost depending on network conditions.
- **Link**: [[Realized Price Definition#Gas Used|Gas Used]]
- **Tag**: `#Water_Element`

## Interaction Model
Explore how these elements interact within the formula and influence the realized price through generative and controlling relationships.

## Formula Integration
Detailed breakdown of how each element influences the realized price formula:
$$ p = \frac{o}{i + g(b + f)} $$

This structured system with tags allows for quick referencing and visualization in Obsidian, making it easier to navigate between the philosophical and practical aspects of the realized price derivation.