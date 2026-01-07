using System;

namespace PhoenixAirCleanupSimulator.Models
{
    public class TreeSequestration
    {
        private const double Co2PerTreeKgPerYear = 18.0; // 2025 desert-adapted average
        private const double Pm25PerTreeKgPerYear = 0.45;

        public double AnnualCo2Kg(int trees) => trees * Co2PerTreeKgPerYear;
        public double AnnualPm25Kg(int trees) => trees * Pm25PerTreeKgPerYear;

        public double EcoImpactScore(double co2Kg, double pm25Kg)
        {
            return (co2Kg / 10.0) + (pm25Kg / 0.1); // Weighted health/CO2 equivalent
        }
    }
}
