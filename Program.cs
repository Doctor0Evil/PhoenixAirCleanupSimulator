using System;
using System.IO;
using CsvHelper;
using System.Globalization;
using System.Collections.Generic;
using PhoenixAirCleanupSimulator.Models;

record Node(string node_id, string site_description, double latitude, double longitude,
    string primary_target, double baseline_2025, int trees_or_units, double power_kw);

class Program
{
    static void Main()
    {
        var nodes = new List<Node>
        {
            new("PHX-I10-01", "I-10 East", 33.450, -112.050, "PM2.5", 27.5, 2000, 50),
            // Add others from shard...
        };

        var treeModel = new TreeSequestration();
        var filterModel = new ElectrostaticFilter();
        var dacModel = new DACCapture();

        foreach (var n in nodes)
        {
            double co2 = treeModel.AnnualCo2Kg(n.trees_or_units);
            double pm25 = filterModel.AnnualPm25Kg(n.power_kw, n.baseline_2025);
            double dacCo2 = dacModel.AnnualCo2Kg(n.power_kw);

            Console.WriteLine($"{n.node_id}: {co2 + dacCo2:F0} kg CO₂/year, {pm25:F0} kg PM2.5/year removed");
        }
    }
}
