
use datatypes::CalculationInput;
use rocket::serde::json::{Json};
pub mod financialcalculations;
pub mod datatypes;
use crate::financialcalculations::*;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

//resources: https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/serialization/src/json.rs
#[post("/calculatefuturevalue", format = "json", data = "<calculation_inputs>")]
fn future_value(calculation_inputs: Json<CalculationInput>) -> Json<Vec<f64>> {
    
    let final_result : Vec<f64>= 
        calculation_inputs.calculation_parameters
            .iter()
            //assumes our gross salary doesn't grow with time
            .scan(0., |state, x| {
                *state = *state + calculate_future_value_for_given_periods(x.years.try_into().unwrap(),12,calculation_inputs.expected_roi,x.percentage_contributions,calculation_inputs.gross_salary,*state);// calculate_future_value_for_given_periods
                
                // then, we'll yield the negation of the state
                Some(*state)
            })
            .collect();

    Json(final_result)
}

#[post("/calculatefuturevaluebyyear", format = "json", data = "<calculation_inputs>")]
fn future_value_by_year(calculation_inputs: Json<CalculationInput>) -> Json<Vec<f64>> {

    let mut f :Vec<f64> = Vec::new();
    // TODO - find a clean way to dispose as 
    //we really just want a scaniter type method where the state is maintained
    let _final_result : Vec<f64>= 
        calculation_inputs.calculation_parameters
            .iter()
            //assumes our gross salary doesn't grow with time
            .scan(0., |principle_amount_state, x| {
                let mut per_year_result = 
                    calculate_future_value_for_given_periods_by_year(
                        x.years.try_into().unwrap(),
                        12,
                        calculation_inputs.expected_roi,
                        x.percentage_contributions,
                        calculation_inputs.gross_salary,
                        *principle_amount_state);// calculate_future_value_for_given_periods
                
                *principle_amount_state = 
                    match per_year_result.last() {
                        Some(r) => *r,
                        None => panic!("at the disco")
                        
                    };
                
                f.append(&mut per_year_result);
                Some(*principle_amount_state)
            })
            .collect();

    Json(f)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![future_value])
        .mount("/", routes![future_value_by_year])
}

#[cfg(test)]
mod route_tests {

    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;


    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn future_value_test() {
        
        let request_body = 
            "{
                \"expected_roi\":0.08,
                \"gross_salary\":100000.00,
                \"calculation_parameters\":[{\"years\":5, \"percentage_contributions\":0.20}, {\"years\":35, \"percentage_contributions\":0.13}]
            }";
        
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = 
            client
                .post(uri!(super::future_value))
                .body(request_body)
                .header(ContentType::JSON)
                .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "[122461.42707540125,4602709.696636483]");
    }
    #[test]
    fn future_value_test_by_year() {
        
        let request_body = 
            "{
                \"expected_roi\":0.08,
                \"gross_salary\":100000.00,
                \"calculation_parameters\":[{\"years\":5, \"percentage_contributions\":0.20}, {\"years\":35, \"percentage_contributions\":0.13}]
            }";
        
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = 
            client
                .post(uri!(super::future_value_by_year))
                .body(request_body)
                .header(ContentType::JSON)
                .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let string_response : String = 
            response
            .into_string()
            .unwrap();

        let mut chars = string_response.chars();
        chars.next();
        chars.next_back();
            
        let processed_results : Vec<f64> = 
            chars
                .as_str()
                .split(',')
                .map(|string_to_parse| string_to_parse.parse::<f64>().unwrap())
                .collect();

        assert_eq!(*processed_results.last().unwrap(), 4602709.696636483);
    }

    fn process_by_year_request(request_body : &str) -> f64 {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = 
            client
                .post(uri!(super::future_value_by_year))
                .body(request_body)
                .header(ContentType::JSON)
                .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let string_response : String = 
            response
            .into_string()
            .unwrap();

        let mut chars = string_response.chars();
        chars.next();
        chars.next_back();
            
        let processed_results : Vec<f64> = 
            chars
                .as_str()
                .split(',')
                .map(|string_to_parse| string_to_parse.parse::<f64>().unwrap())
                .collect();
        
                *processed_results.last().unwrap()
    }

    #[test]
    fn future_value_test_by_year_split_calculation() {
        //these two requests should give the same answer
        let request_body1 = 
            "{
                \"expected_roi\":0.08,
                \"gross_salary\":100000.00,
                \"calculation_parameters\":[{\"years\":5, \"percentage_contributions\":0.1}, {\"years\":5, \"percentage_contributions\":0.1}]
            }";
        
        let request_body2 = 
            "{
                \"expected_roi\":0.08,
                \"gross_salary\":100000.00,
                \"calculation_parameters\":[{\"years\":10, \"percentage_contributions\":0.1}]
            }";
        
        let answer1 = process_by_year_request(request_body1);
        let answer2 = process_by_year_request(request_body2);
        
        assert_eq!(answer1.floor(), answer2.floor());
        assert_eq!(answer1.floor(), 152455.0);
    }

}