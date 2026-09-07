# MaterialSpace Provenance & Data Sources

MaterialSpace prioritizes authentic, traceable scientific data. Every property in the database retains its specific provenance source ID and test conditions.

---

## Authoritative Reference Sources

### 1. NASA Ames Thermal Protection Systems Expert (TPSX)
- **Source ID**: `nasa_tpsx`
- **Organization**: National Aeronautics and Space Administration (NASA Ames Research Center)
- **Database URL**: [https://tpsx.arc.nasa.gov](https://tpsx.arc.nasa.gov)
- **Scope**: Thermal protection systems (TPS), ceramic tiles (LI-900), carbon ablators (PICA), high-temperature ceramic matrix composites (C/SiC, C/C), and aerogels.

### 2. NIST Chemistry WebBook (SRD 69)
- **Source ID**: `nist_webbook`
- **Organization**: National Institute of Standards and Technology (NIST)
- **Database URL**: [https://webbook.nist.gov/chemistry/](https://webbook.nist.gov/chemistry/)
- **Scope**: Thermophysical properties, standard state specific heat capacities, enthalpies of transition, melting points, and boiling points of pure elemental metals and substances.

### 3. MatWeb Online Materials Database
- **Source ID**: `matweb`
- **Organization**: MatWeb LLC / Material Data Resources
- **Database URL**: [https://www.matweb.com](https://www.matweb.com)
- **Scope**: Commercial engineering materials, aerospace aluminum alloys (6061, 7075, 2024), titanium alloys, superalloys (Inconel 718, 625), steels, and engineering thermoplastics.

### 4. ASM International Materials Handbook Series
- **Source ID**: `asm_handbook`
- **Organization**: ASM International (The Materials Information Society)
- **Database URL**: [https://www.asminternational.org](https://www.asminternational.org)
- **Scope**: Peer-reviewed mechanical properties (Young's modulus, 0.2% offset yield strength, ultimate tensile strength) across structural alloys, tempers, and composites.

---

## Scientific Integrity Policy

- **No Hallucinated Data**: If an authoritative property measurement is unrecorded for a specific material grade, it is explicitly preserved as `null` and displayed in the application UI as `Data unavailable`.
- **SI Base Unit Normalization**: All raw values are normalized into standard SI units with original unit traceability preserved.
