namespace client.Data.API;

/// <summary>
/// Captures calculated results by profile/scenario set up by the user
/// </summary>
/// <param name="Name"></param>
/// <param name="InvestmentReturns"></param>
public record ProfileResults(string Name, double[] InvestmentReturns);