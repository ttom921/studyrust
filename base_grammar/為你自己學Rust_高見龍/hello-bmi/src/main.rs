fn main() {
    //let result = calculator::bmi(170, 65);
    let result = bmi_calc::bmi(170, 65);
    match result {
        Ok(bmi) => {
            println!("BMI:{}", bmi);
        }
        Err(message) => {
            println!("{message}")
        }
    }
}
