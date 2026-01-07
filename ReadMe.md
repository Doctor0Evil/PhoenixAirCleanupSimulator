# PhoenixAirCleanupSimulator

PhoenixAirCleanupSimulator is a .NET 8 console application that turns real Phoenix–Mesa air-quality context into **actionable intervention scenarios** for 2026 and beyond, with a focus on ozone, PM₂.₅, and solar-backed cleanup capacity. It is framed around Phoenix–Mesa’s 4th‑worst national ozone ranking, CarbonCapture’s Mesa DAC plant, and the region’s >300 sunny days per year.

***

## Why Phoenix, Why Now

- The American Lung Association’s 2025 “State of the Air” report ranks the **Phoenix–Mesa metro 4th‑worst in the nation for ozone pollution**, with Maricopa County averaging about 54.8 unhealthy ozone days per year. This represents a worsening trend compared with earlier years.
- Phoenix benefits from **over 300 sunny days annually**, making it one of the most favorable U.S. metros for solar-powered infrastructure and continuous clean-energy operation.
- CarbonCapture Inc. has leased an 83,000 ft² facility in **Mesa, AZ**, projected at full capacity to manufacture about **4,000 DAC modules per year**, equating to roughly **2 megatons of CO₂ removal capacity annually**.

PhoenixAirCleanupSimulator encodes this context into a structured CSV “data shard” so each simulated node corresponds to a real or plausible corridor, canyon, or DAC deployment zone in the Phoenix–Mesa region.

***

## Core Interventions Modeled

### 1. Expanded Urban Tree Canopy

The simulator represents desert‑appropriate urban tree projects (e.g., along freeways and in heat‑vulnerable neighborhoods) with a configurable per‑tree sequestration rate and PM₂.₅ deposition benefit, using ~18 kg CO₂ per mature tree per year as a typical reference value within published urban‑forestry ranges.

- Nodes along I‑10, I‑17, Loop 202, and dense neighborhoods can be assigned thousands of trees.  
- The model converts tree counts into annual CO₂ and PM₂.₅ removal and aggregates into an **EcoImpactScore** that heavily weights PM₂.₅ due to health impacts.

**Impact level:** High – relatively low‑energy, durable sequestration with cooling and air‑quality co‑benefits in vulnerable corridors.

### 2. Solar-Powered Electrostatic Filters

High‑efficiency electrostatic or HEPA‑class filtration units (often rated above 90% for PM₂.₅ in controlled flows) are mapped onto lampposts, sound walls, and street canyons in traffic‑dense corridors.

- Key parameters: module flow rate (m³/h), removal efficiency, and baseline local PM₂.₅ levels (µg/m³).
- Phoenix’s strong solar resource enables near‑continuous module operation with minimal or net‑zero grid emissions, especially along major freeway segments and downtown street canyons.

**Impact level:** High – direct particulate mass removal where residents experience the worst roadway pollution.

### 3. Modular DAC via Mesa Facility

DAC nodes are tied conceptually to CarbonCapture’s Mesa manufacturing plant, which is expected to produce modular DAC units totaling around **2 megatons of annual CO₂ removal capacity** at full build‑out.

- The DACCapture model uses:
  - Capture rate in kg CO₂ per hour per kW.
  - Regeneration energy in GJ per ton CO₂, constrained within current DAC design study ranges.
- Phoenix’s high solar irradiance implies a large share of DAC energy can be supplied by renewables, pushing the system toward net‑negative operation over time.

**Impact level:** High – scalable, durable CO₂ removal capacity anchored in a local manufacturing ecosystem.

### 4. Hybrid Highway Nodes (Trees + Filters)

Hybrid nodes combine TreeSequestration (long‑lived CO₂/PM₂.₅ benefits) and FilterRemoval (immediate PM₂.₅ reduction) at sites such as I‑10 East, I‑17 segments, and Loop 202 near key population centers.

- These locations mirror real Phoenix freeway corridors that routinely experience some of the region’s worst pollution episodes.
- The hybrid approach targets both acute exposure and long‑term climate benefits at the same physical nodes.

**Impact level:** High – multi‑pollutant reduction exactly where people breathe the highest traffic‑related burdens.

***

## Data Shard: Phoenix Nodes CSV

The simulator expects a qpudatashard‑style CSV describing modeled nodes and their baseline conditions.

- **Filename:** `PhoenixAirCleanupNodes2026Update.csv`  
- **Default location:** `Data/` (for example, copied from a `qpudatashards/particles/` source directory in the repository).

Each row encodes:

- `node_id` (e.g., `PHX-I10-01`, `PHX-MESA-DAC-01`) and human‑readable `site_description`.  
- Geographic attributes: `region`, `latitude`, `longitude`.  
- Air‑quality attributes: `primary_target` (PM₂.₅, ozone, CO₂, hybrid), `baseline_2025` pollution or CO₂ levels, and `target_reduction_pct`.
- Capacity and implementation fields: `annual_capacity_kg`, `energy_source` (trees, solar filters, grid‑tied or solar‑tied DAC), `ecoimpactscore`, and free‑text `notes`.

Values are chosen to be consistent with:

- 2025 State of the Air ozone rankings and supporting coverage for Phoenix–Mesa.[4][1]
- Public information on the Mesa DAC facility’s projected 2 Mt CO₂ annual module capacity.[2]
- Phoenix solar statistics indicating >300 sunny days per year and high solar irradiance.[3]

***

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
```

- `PhoenixAirCleanupSimulator.csproj` targets .NET 8, treats warnings as errors, and ensures the CSV shard is copied to the build output so the simulator can load it at runtime.
- `.github/workflows/ci.yml` configures GitHub Actions to build (and optionally test) the project on each push and pull request.

### Model overview

- **TreeSequestration.cs**  
  - Uses a configurable baseline of about 18 kg CO₂/year per mature desert tree, with a ramp‑up factor for years to maturity, and includes PM₂.₅ deposition estimates.
  - Returns annual CO₂ and PM₂.₅ masses and a combined EcoImpactScore that strongly emphasizes PM₂.₅ reduction.

- **FilterRemoval.cs**  
  - Represents fixed electrostatic/HEPA modules with parameters for airflow (m³/h), efficiency (0–1], and ambient PM₂.₅ (µg/m³).
  - Converts these into hourly and annual PM₂.₅ mass removed and assigns a health‑weighted score (for example, points per 0.1 kg PM₂.₅ removed).

- **DACCapture.cs**  
  - Models MOF/DAC CO₂ capture with tunable capture rate (kg CO₂/h/kW) and regeneration energy (GJ/tCO₂).[5]
  - Computes annual capture and a net eco‑impact score that subtracts an energy‑intensity penalty while staying within realistic DAC performance ranges.

***

## Running the Simulator

### Requirements

- .NET 8 SDK or later installed.  
- Git and standard build tools.  
- `Data/PhoenixAirCleanupNodes2026Update.csv` present in the `Data` directory.

### Steps

```bash
git clone <your-repo-url>.git
cd PhoenixAirCleanupSimulator
dotnet restore
dotnet build --configuration Release
dotnet run --configuration Release
```

On each run, the simulator will:

- Load the Phoenix data shard and enumerate nodes (e.g., I‑10, I‑17, Loop 202, downtown canyons, Mesa DAC clusters).
- Report baseline 2025 air‑quality values and capacities, then estimate annual CO₂ and PM₂.₅ removal for trees, filters, and DAC modules at each node.
- Compute and print eco‑impact scores per node and summarize total projected benefits for the 2026‑scale portfolio of interventions.

***

## Integration with Teslaswarm & Cybercore-Brain

PhoenixAirCleanupSimulator is designed to interoperate conceptually with Teslaswarm and Cybercore‑Brain control layers described for Phoenix‑focused infrastructure planning.

- Each CSV node can act as a nanoswarm cluster or endpoint with known power draw and pollutant‑removal curves, enabling higher‑level optimizers to allocate power and hardware under grid and safety constraints.
- Phoenix’s strong solar resource and the Mesa DAC plant’s projected output make it plausible to deploy a Teslaswarm‑mediated air‑cleanup layer that is:
  - Grounded in current DAC and filtration technologies.
  - Targeted at the metro area’s worst ozone and PM hot spots, as documented in recent air‑quality assessments.

By combining evidence‑informed environmental modeling, a structured Phoenix node shard, and CI‑backed .NET infrastructure, PhoenixAirCleanupSimulator aims to provide a transparent, reproducible foundation for planning real‑world interventions that can measurably improve Phoenix air quality in 2026 and beyond.
