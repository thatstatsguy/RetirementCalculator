

fn future_value_ordinary_annuity (payment : f64, yearly_interest_rate : f64, compounding_interval : f64, years : f64) -> f64{
    let interest_per_compounding_period = yearly_interest_rate / compounding_interval as f64;
    let numerator = (1. + interest_per_compounding_period).powf(years * compounding_interval) - 1.;
    (payment * numerator)/interest_per_compounding_period
}

fn compound_interest(principle_amount : f64, expected_yearly_roi : f64, compounding_frequency : i32, years : i32) -> f64{
    principle_amount * (1. + expected_yearly_roi/(compounding_frequency as f64)).powi(compounding_frequency * years)
} 
    

/// calculates the future value of an annuity + principle amount over a given period
/// years - number of years contributions are being made to a retirement annuity
/// compounding_frequency - assumption on how frequently compounding is done.
/// expected_yearly_roi - assumption on yield of fund per year
/// gross_salary - salary prior to deductions. Used to calculate annuity contributions.
/// principle_amount - prior amounts as starting value of fund. 
/// # Examples
///
/// ```
/// use server::financialcalculations::calculate_future_value_for_given_periods;
///
/// let result = calculate_future_value_for_given_periods(years, compounding_frequency, expected_yearly_roi, percentage_contributions, gross_salary, principle_amount);
/// assert_eq!(result, );
/// ```
pub fn calculate_future_value_for_given_periods(
    years: i32,
    compounding_frequency : i32, 
    expected_yearly_roi : f64,
    percentage_contributions: f64, 
    gross_salary: f64, 
    principle_amount : f64,
) -> f64 {

    let future_value_principle_amount: f64 = 
        compound_interest(principle_amount,expected_yearly_roi, compounding_frequency, years);
    
    let yearly_contributions = gross_salary * percentage_contributions;
    let monthly_contributions = yearly_contributions / 12.;
    
    //https://www.youtube.com/watch?v=s1aJWw2zOek
    let future_value_annuity : f64 = 
        future_value_ordinary_annuity(monthly_contributions, expected_yearly_roi, 12., years as f64);

    future_value_annuity + future_value_principle_amount

}


/// Performs the same calculate as calculate_future_value_for_given_periods, 
/// but outputs future values by year
pub fn calculate_future_value_for_given_periods_by_year(
    years: i32,
    compounding_frequency : i32, 
    expected_yearly_roi : f64,
    percentage_contributions: f64, 
    gross_salary: f64, 
    principle_amount : f64,
) -> Vec<f64> {

    let future_value_principle_amount = 
        compound_interest_by_year(principle_amount,expected_yearly_roi, compounding_frequency, years);
    
    let yearly_contributions = gross_salary * percentage_contributions;
    let monthly_contributions = yearly_contributions / 12.;
    
    //https://www.youtube.com/watch?v=s1aJWw2zOek
    let future_value_annuity = 
        future_value_ordinary_annuity_by_year(monthly_contributions, expected_yearly_roi, 12., years as f64);
    
    //add on the principle amount 
    future_value_principle_amount
        .iter()
        .zip(future_value_annuity.iter())
        .map(|(fv_principle_amount,fv_annuity_amount)| 
            
            (fv_annuity_amount + fv_principle_amount))
        .collect()
}

/// Calculates an ordinary annuity, but shows growth in annuity value over time (per year)
pub fn future_value_ordinary_annuity_by_year (payment : f64, yearly_interest_rate : f64, compounding_interval : f64, years : f64) -> Vec<f64>{
    (1  .. years as i32 + 1)
    .collect::<Vec<i32>>()
    .into_iter()
    .map(|f| f as f64)
    .collect::<Vec<f64>>()
    .iter()
    .map(|y|
        future_value_ordinary_annuity(payment, yearly_interest_rate, compounding_interval, *y)
    )
    .collect()
}


/// Calculates an ordinary annuity, but shows compound interest value over time (per year)
fn compound_interest_by_year(principle_amount : f64, expected_yearly_roi : f64, compounding_frequency : i32, years : i32) -> Vec<f64>{
    (1  .. years as i32 + 1)
    .collect::<Vec<i32>>()
    .into_iter()
    .map(|f| f as f64)
    .collect::<Vec<f64>>()
    .iter()
    .map(|y|
        compound_interest(principle_amount, expected_yearly_roi, compounding_frequency, *y as i32)
    )
    .collect()

}

#[cfg(test)]
mod helper_function_tests {
    use crate::financialcalculations::*;

    #[test]
    fn test_annuity() {
        let result = future_value_ordinary_annuity(1000., 0.048, 4., 10.);
        assert_eq!(result.floor(), 50955.);
    }

    //shows annuity growth over time - growing to final point
    #[test]
    fn test_annuity_by_year() {
        let result = future_value_ordinary_annuity_by_year(1000., 0.048, 4., 10.);
        
        let final_calculated_value = result.last();
        
        match final_calculated_value {
            Some(r) => assert_eq!((*r).floor(), 50955. as f64),
            None => panic!("At the disco")
        }
    }
    //shows annuity growth over time - growing to final point
    #[test]
    fn test_compound_interest_by_year() {
        let result = compound_interest_by_year(1000., 0.12, 12, 10);
        
        let final_calculated_value = result.last();
        
        match final_calculated_value {
            Some(r) => assert_eq!((*r).floor(), 3300. as f64),
            None => panic!("At the disco")
        }
    }

    #[test]
    fn test_compound_interest() {
        let result = compound_interest(1000., 0.12, 12, 10);
        assert_eq!(result.floor(), 3300.);
    }

}
