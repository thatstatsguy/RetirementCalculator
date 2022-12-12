using System.ComponentModel.DataAnnotations;

namespace client.Data;

public class ContributionSubmission
{
    [Required] public int years;
    [Required] public double percentage_contributions;

    public ContributionSubmission(int years, double percentageContributions)
    {
        this.years = years;
        percentage_contributions = percentageContributions;
    }
}
    