# MaterialSpace Scientific Assumptions & Limitations

MaterialSpace provides deterministic, simplified engineering physics calculations for educational experimentation. This document explicitly outlines what each model assumes and what physical effects are **not** simulated.

---

## 1. Linear Thermal Expansion Assumptions & Boundaries

### What the Model Assumes:
1. **Linear Isotropic Expansion**: The expansion is identical in all coordinate directions ($x, y, z$) with a single scalar CTE $\alpha$.
2. **Constant Coefficient ($\alpha$)**: The expansion coefficient does not vary across the simulated temperature range.
3. **Unconstrained Expansion**: The material is free to expand without rigid fixtures, thermal clamping, or induced thermal stresses.
4. **No Phase Transformations**: The specimen remains in the same solid crystallographic phase throughout the thermal cycle.

### Limitations:
- Real CTEs vary non-linearly over wide cryogenic or elevated temperature spans.
- Unidirectional fiber-reinforced composites (e.g. CFRP) exhibit strong anisotropic thermal expansion (often slightly negative axially and positive transversely).
- Constrained components will develop severe thermal stresses ($\sigma = -E \alpha \Delta T$) which can cause buckling or yielding.

---

## 2. 1D Steady-State Heat Conduction Assumptions & Boundaries

### What the Model Assumes:
1. **One-Dimensional Flow**: Conduction occurs only along a single spatial dimension $x$; lateral edges are adiabatic (perfectly insulated).
2. **Steady-State Thermal Equilibrium**: Temperature at every internal coordinate is constant over time ($\partial T / \partial t = 0$).
3. **Uniform Thermal Conductivity ($k$)**: Thermal conductivity is treated as independent of local temperature across the wall.
4. **Zero Internal Generation**: No internal heat generation ($q''' = 0$) from electrical Joule heating, chemical reaction, or radiation absorption.
5. **Zero Contact Resistance**: Perfect thermal contact between the material face and boundary temperatures.

### Limitations:
- Does not model unsteady transient thermal heating/cooling curves (governed by thermal diffusivity $\alpha_{diff} = k / (\rho c)$).
- Neglects convective boundary layer heat transfer and external radiative radiation ($q = \varepsilon \sigma (T^4 - T_\infty^4)$).
- Real multi-layer or complex 3D shapes experience multidimensional heat spreading.

---

## 3. Uniaxial Stress & Strain (Hooke's Law) Assumptions & Boundaries

### What the Model Assumes:
1. **Pure Axial Loading**: The specimen is subject only to collinear tensile or compressive axial loads; bending moments, torsion, and transverse shears are zero.
2. **Uniform Stress Distribution**: Stress $\sigma = F/A$ is perfectly uniform across the cross section (Saint-Venant's principle).
3. **Linear Elasticity**: Material deforms elastically and reversibly according to Hooke's Law ($\sigma = E \varepsilon$).
4. **Small Strain Formulation**: Deformations are small enough that cross-sectional necking and true strain geometry changes are negligible.

### Limitations:
- Does not simulate post-yield plastic deformation, strain hardening, necking, ductile void growth, or brittle cleavage.
- Does not account for geometric stress concentration factors ($K_t$) at notches, holes, fillets, or weld toes.
- Does not model Euler column buckling under compressive loads on slender geometries.
- Does not predict cyclic fatigue life ($S-N$ curves), creep at elevated temperatures, or fracture mechanics stress intensity ($K_{Ic}$).

---

## 4. Heat Capacity Assumptions & Boundaries

### What the Model Assumes:
1. **Sensible Heat Only**: Energy change is purely sensible ($\Delta Q = m c \Delta T$) without latent heat of melting, vaporization, or allotropic transformation.
2. **Constant Specific Heat ($c$)**: The specific heat capacity at constant pressure $c_p$ is assumed constant over the temperature delta.
3. **Lumped Thermal System**: Assumes uniform internal body temperature (Biot number $Bi < 0.1$).

### Limitations:
- Specific heat drops rapidly toward zero near absolute zero ($0\text{ K}$) according to the Debye $T^3$ model.
- High-temperature phase transformations require substantial latent heats of transformation ($\Delta H_{trans}$).
