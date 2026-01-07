namespace PhoenixAirCleanupSimulator.Models
{
    public class ElectrostaticFilter
    {
        private const double Efficiency = 0.95; // Commercial units
        private const double FlowM3PerHourPerKw = 2000.0; // Scaled from urban pilots

        public double AnnualPm25Kg(double powerKw, double baselineUgM3)
        {
            double flowM3Year = powerKw * FlowM3PerHourPerKw * 8760;
            double inletKgYear = (baselineUgM3 / 1e6) * flowM3Year; // µg/m³ to kg
            return Efficiency * inletKgYear;
        }
    }
}
