# MaterialSpace Equations & Physical Models

This document details the governing equations, variables, SI units, and mathematical formulations implemented in MaterialSpace.

---

## 1. Linear Thermal Expansion

### Governing Equations

$$\Delta L = \alpha \cdot L_0 \cdot \Delta T$$
$$L = L_0 (1 + \alpha \Delta T)$$
$$\varepsilon_{thermal} = \frac{\Delta L}{L_0} = \alpha \Delta T$$

### Variables & SI Units

| Symbol | Description | SI Unit | Common Display Units |
| :--- | :--- | :--- | :--- |
| $\Delta L$ | Linear change in length | meters ($m$) | $mm$, $\mu m$ |
| $L_0$ | Initial length at reference temperature $T_0$ | meters ($m$) | $mm$, $cm$, $m$ |
| $\alpha$ | Linear coefficient of thermal expansion (CTE) | $K^{-1}$ | $\mu m / (m \cdot K)$, $10^{-6}/^\circ C$ |
| $\Delta T$ | Temperature difference ($T_{final} - T_{initial}$) | Kelvin ($K$) | $K$, $^\circ C$ |
| $L$ | Final specimen length ($L_0 + \Delta L$) | meters ($m$) | $mm$, $m$ |
| $\varepsilon_{thermal}$ | Unconstrained thermal strain | dimensionless | $\%$, $\mu\varepsilon$ |

---

## 2. 1D Steady-State Heat Conduction (Fourier's Law)

### Governing Equations

$$\dot{Q} = k \cdot A \cdot \frac{T_{hot} - T_{cold}}{L}$$
$$q'' = \frac{\dot{Q}}{A} = k \cdot \nabla T$$
$$R_{th} = \frac{L}{k \cdot A}$$
$$\dot{Q} = \frac{\Delta T}{R_{th}}$$

### Variables & SI Units

| Symbol | Description | SI Unit | Common Display Units |
| :--- | :--- | :--- | :--- |
| $\dot{Q}$ | Conductive heat transfer rate | Watts ($W = J/s$) | $W$, $kW$, $MW$ |
| $k$ | Thermal conductivity | $W/(m \cdot K)$ | $W/(m \cdot K)$, $W/(m \cdot ^\circ C)$ |
| $A$ | Cross-sectional heat flow surface area | square meters ($m^2$) | $m^2$, $cm^2$, $mm^2$ |
| $L$ | Conduction wall thickness / path length | meters ($m$) | $mm$, $m$ |
| $T_{hot}$ | Hot side boundary temperature | Kelvin ($K$) | $^\circ C$, $K$ |
| $T_{cold}$ | Cold side boundary temperature | Kelvin ($K$) | $^\circ C$, $K$ |
| $\nabla T$ | Temperature gradient ($\Delta T / L$) | $K/m$ | $^\circ C/mm$, $K/m$ |
| $q''$ | Heat flux | $W/m^2$ | $W/m^2$, $kW/m^2$ |
| $R_{th}$ | Conductive thermal resistance | $K/W$ | $K/W$, $^\circ C/W$ |

---

## 3. Uniaxial Stress, Strain, & Hooke's Law

### Governing Equations

$$\sigma = \frac{F}{A}$$
$$\varepsilon = \frac{\sigma}{E}$$
$$\Delta L = \varepsilon \cdot L_0 = \frac{F \cdot L_0}{A \cdot E}$$
$$FoS = \frac{\sigma_{yield}}{\sigma}$$

### Variables & SI Units

| Symbol | Description | SI Unit | Common Display Units |
| :--- | :--- | :--- | :--- |
| $\sigma$ | Engineering normal stress | Pascals ($Pa = N/m^2$) | $MPa$, $GPa$, $kPa$ |
| $F$ | Applied tensile or compressive axial force | Newtons ($N$) | $N$, $kN$ |
| $A$ | Cross-sectional load-bearing area | square meters ($m^2$) | $mm^2$, $cm^2$ |
| $E$ | Young's modulus of elasticity (stiffness) | Pascals ($Pa$) | $GPa$, $MPa$ |
| $\varepsilon$ | Engineering strain ($\Delta L / L_0$) | dimensionless | $\%$, $\mu\varepsilon$ |
| $L_0$ | Original gauge length | meters ($m$) | $mm$, $m$ |
| $\Delta L$ | Elastic elongation / compression | meters ($m$) | $mm$, $\mu m$ |
| $\sigma_{yield}$ | 0.2% offset yield strength | Pascals ($Pa$) | $MPa$ |
| $FoS$ | Factor of Safety | dimensionless | — |

---

## 4. Sensible Heat Capacity & Thermal Energy Storage

### Governing Equations

$$Q = m \cdot c \cdot \Delta T$$
$$C_{th} = m \cdot c$$
$$\Delta T = \frac{Q}{m \cdot c}$$

### Variables & SI Units

| Symbol | Description | SI Unit | Common Display Units |
| :--- | :--- | :--- | :--- |
| $Q$ | Sensible thermal energy transferred | Joules ($J$) | $J$, $kJ$, $MJ$ |
| $m$ | Total mass of material body | kilograms ($kg$) | $g$, $kg$ |
| $c$ | Specific heat capacity ($c_p$) | $J/(kg \cdot K)$ | $J/(kg \cdot K)$, $kJ/(kg \cdot K)$ |
| $\Delta T$ | Temperature difference ($T_{final} - T_{initial}$) | Kelvin ($K$) | $K$, $^\circ C$ |
| $C_{th}$ | Total thermal mass / bulk heat capacity | $J/K$ | $J/K$, $kJ/K$ |

---

## 5. Multi-Criteria Material Selection Decision Engine

### Normalization (0–100 Scale)

For **higher-is-better** criteria (e.g., stiffness $E$, yield strength $\sigma_y$, thermal conductivity $k$, emissivity $\varepsilon$, melting point $T_m$):

$$S_i = \left( \frac{x_i - x_{min}}{x_{max} - x_{min}} \right) \times 100$$

For **lower-is-better** criteria (e.g., density $\rho$, thermal expansion coefficient $\alpha$, parasitic mass):

$$S_i = \left( \frac{x_{max} - x_i}{x_{max} - x_{min}} \right) \times 100$$

### Weighted Aggregate Score

$$Score_{overall} = \sum_{i=1}^{N} \tilde{w}_i \cdot S_i, \quad \text{where } \tilde{w}_i = \frac{w_i}{\sum_{j=1}^N w_j}$$
