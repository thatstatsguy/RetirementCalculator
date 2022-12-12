namespace client.Data.API;

public record APISubmission(
    List<ContributionSubmission> calculation_parameters, 
    double gross_salary,
    double expected_roi);
