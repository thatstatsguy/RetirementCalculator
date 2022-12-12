use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SingleCalcInput{
    pub years : u32,
    pub percentage_contributions: f64
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CalculationInput {
    pub expected_roi : f64,
    pub gross_salary : f64,
    pub calculation_parameters : Vec<SingleCalcInput>
    
}