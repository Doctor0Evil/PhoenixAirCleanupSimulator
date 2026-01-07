namespace PhoenixAirCleanupSimulator.Models
{
    public class DACCapture
    {
        private const double KgCo2PerKwh = 0.4; // Mid-range real MOF 2025-2026

        public double AnnualCo2Kg(double powerKw) => powerKw * KgCo2PerKwh * 8760;

        public double NetEnergyGjPerTon(double grossKwhPerTon = 2000) // ~2 GJ/ton benchmark
            => grossKwhPerTon * 3.6 / 1000.0; // kWh to GJ
    }
}
