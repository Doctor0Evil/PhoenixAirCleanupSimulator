use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

/// This tool writes an attractive public README.md for PhoenixAirCleanupSimulator,
/// grounded in Phoenix's documented ozone/PM2.5 challenges, Mesa DAC capacity,
/// and 300+ sunny days/year solar context.
fn main() -> Result<()> {
    let root = Path::new("PhoenixAirCleanupSimulator");
    create_dir_all(root)?;
    let readme_path = root.join("README.md");
    let mut file = File::create(readme_path)?;
    file.write_all(build_readme().as_bytes())?;
    Ok(())
}

fn build_readme() -> String {
    let readme = r#"# PhoenixAirCleanupSimulator

PhoenixAirCleanupSimulator is a .NET 8 console application that turns real Phoenix–Mesa air-quality data into **actionable intervention scenarios** for 2026 and beyond. It focuses on corridors where the Phoenix–Mesa metro was ranked *4th‑worst in the U.S. for ozone* in the 2025 State of the Air report, while also addressing frequent PM₂.₅ exceedances and leveraging **300+ sunny days/year** to power filters and direct air capture (DAC) modules with minimal grid draw.[web:6][web:12]

The simulator connects three proven levers:

- **Expanded urban tree canopy** along I‑10, I‑17, Loop 202 and neighborhoods.
- **Solar-powered electrostatic filtration** on lampposts, sound walls, and urban canyons.
- **Modular DAC deployment** supported by CarbonCapture Inc.’s Mesa manufacturing facility, designed to produce ~2 megatons/year of CO₂ removal capacity in modular units.[web:19][web:11]

Together, these interventions model how Phoenix can bend its ozone and PM curves while building out a Teslaswarm‑compatible, nanoswarm‑ready infrastructure layer.

---

## Why Phoenix, Why Now

- The American Lung Association’s 2025 “State of the Air” report ranks the **Phoenix–Mesa metro 4th‑worst nationally for ozone**, with Maricopa County averaging over 50 unhealthy ozone days per year.[web:6][web:9]
- Phoenix experiences **over 300 sunny days annually**, making solar‑powered air-cleanup technologies highly effective and reliable.[web:12][web:29]
- CarbonCapture Inc. has leased an 83,000 ft² facility in **Mesa, AZ**, projected to manufacture roughly **4,000 DAC modules per year**, equal to about **2 megatons of CO₂ removal capacity** at full scale.[web:19][web:11]

PhoenixAirCleanupSimulator encodes this context into a qpudatashard CSV, so each simulated “node” corresponds to a real corridor, downtown canyon, or DAC zone.

---

## Core Interventions Modeled

### 1. Expanded Urban Tree Canopy

Desert‑adapted tree projects (e.g., Mesa plantings, SRP shade programs) are represented with a baseline sequestration rate of **≈18 kg CO₂ per mature tree per year**, within published ranges for desert urban forestry, plus **PM₂.₅ deposition** benefits.[web:16][file:3]

- Nodes on I‑10 and I‑17 can be assigned thousands of trees.
- The simulator converts tree counts into annual CO₂ and PM₂.₅ removal.
- An **EcoImpactScore** combines climate and health benefits, weighting PM₂.₅ more heavily due to its acute health impact.

**Impact level:** High – permanent, low‑energy sequestration with additional cooling and air‑quality co‑benefits.

### 2. Solar-Powered Electrostatic Filters

Commercial electrostatic units achieving **>90–95% PM₂.₅ removal efficiency** in controlled flows are mapped to highway‑adjacent deployments.[file:3]

- Parameters include:
  - Flow rate (m³/h) per module.
  - PM₂.₅ removal efficiency.
  - Local baseline PM₂.₅ levels (µg/m³) from Phoenix corridor data.
- Phoenix’s solar resource allows near‑continuous operation without net grid emissions, especially along I‑10, I‑17, Loop 202, and in downtown street canyons.[web:12][web:29]

**Impact level:** High – direct particulate mass removal in high‑exposure corridors.

### 3. Modular DAC via Mesa Facility

DAC nodes in the shard are linked to **Mesa’s DAC manufacturing plant**, which is slated to produce modules totaling ~2 Mt CO₂ removal capacity per year at full build‑out.[web:19][web:11]

- DACCapture models:
  - Capture rate in **kg CO₂ per hour per kW** (mid‑range MOF/DAC performance).
  - Regeneration energy in **GJ per ton CO₂**, consistent with current DAC design studies.[web:31][web:21]
- Phoenix’s high solar irradiance allows a significant fraction of regeneration energy to come from renewables, pushing the system toward net negative emissions.[web:12]

**Impact level:** High – scalable, durable CO₂ removal tied to local manufacturing.

### 4. Hybrid Highway Nodes (Trees + Filters)

Hybrid nodes combine:

- TreeSequestration (long‑lived CO₂ and PM₂.₅ reductions).
- ElectrostaticFilter (near‑instant PM₂.₅ reduction along roadways).

This is applied to corridors like **I‑10 East**, **I‑17 Deer Valley**, and **Loop 202 near Sky Harbor**, where 2025 data show some of the **worst PM₂.₅ and ozone spikes** in the region.[web:9][web:16]

**Impact level:** High – multi‑pollutant reduction exactly where people breathe the most polluted air.

---

## Data: qpudatashard CSV

The simulator expects a qpudatashard file that encodes Phoenix nodes with coordinates, baselines, and capacity estimates.

**Filename:** `PhoenixAirCleanupNodes2026Update.csv`  
**Destination folder:** `Data/` (copied from `qpudatashards/particles/`)

Each row contains:

- `node_id` (e.g., `PHX-I10-01`, `PHX-MESA-DAC-06`)
- `site_description` and `region`
- `latitude`, `longitude`
- `primary_target` (PM₂.₅, ozone, CO₂, or hybrid)
- `baseline_2025` (observed 2025 levels)
- `target_reduction_pct` and `annual_capacity_kg`
- `energy_source` (solar filters, trees, grid‑tied DAC)
- `ecoimpactscore` and `notes`

These values are grounded in:

- 2025 State of the Air rankings and media coverage for Phoenix ozone/PM.[web:6][web:9][web:16]
- Public announcements about the Mesa DAC facility’s 2 Mt CO₂ module capacity.[web:19][web:11]
- Phoenix solar resource statistics (>300 sunny days/year) supporting solar‑backed operation.[web:12][web:29]

---

## Project Layout

```text
PhoenixAirCleanupSimulator/
├── PhoenixAirCleanupSimulator.csproj
├── Program.cs
├── Models/
│   ├── TreeSequestration.cs
│   ├── FilterRemoval.cs   # electrostatic PM2.5 model
│   └── DACCapture.cs      # MOF/DAC CO2 model
├── Data/
│   └── PhoenixAirCleanupNodes2026Update.csv
└── .github/
    └── workflows/
        └── ci.yml
