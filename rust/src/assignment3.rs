fn main(){
    use std::f64;
    /*Add, Divide, Multiply, Subtract, Cgpa */

    /*Funtion for Addition */
    fn addition(arr: &[i32;2]) {
        let array = arr;
        let sum_one= array[0] as i32 + array[1] as i32;
        println!("Addition gives {}", sum_one);
    }

    /*Function for Subtraction */
    fn subtraction(arr: &[i32;2]) {
        let array = arr;
        let sub_one= array[0] as i32 - array[1] as i32;
        println!("Subtraction gives {}", sub_one);
    }

    /*Function for Multiplication */
    fn multiplication(arr: &[i32;2]) {
        let array = arr;
        let mult_one= array[0] as i32 * array[1] as i32;
        println!("Multiplication gives {}", mult_one);

    }

    /*Function for Division */
    fn division(arr: &[i32;2]) {
        let array = arr;
        let div_one= array[0] as i32 / array[1] as i32;
        println!("Division gives {}", div_one);
    }

    /*Function to Calculate CGPA */
    fn calculate_and_display_cgpa(totalgradepoints: f32, totalcredits: f32) -> Result<f32, String> {
        if totalcredits == 0.0 {
            return Err("Total credits cannot be zero".to_string());
        }
    
        let cgpa = (totalgradepoints / totalcredits * 100.0).round() / 100.0;
        println!("Calculated CGPA: {:.2}", cgpa);
    
        Ok(cgpa)
    }

    /*Function to find sin */
    fn sin_value(value:&f64){
        let sin_val = value.sin();
        println!("sin value is {}",sin_val);
    }

    /*Function to find cos */
    fn cos_value(value:&f64){
        let cos_val = value.cos();
        println!("cos value is {}",cos_val);
    }

    /*Function to find tan */
    fn tan_value(value:&f64){
        let tan_val = value.tan();
        println!("tan value is {}",tan_val);
    }

    /*Function to find ln */
    fn ln_value(value:&f64){
        let ln_val = value.ln();
        println!("ln value is {}",ln_val);
    }

    /*Function to convert radians to degree */
    fn deg_value(value:&f64){
        let deg_val = value.to_degrees();
        println!("deg value is {}",deg_val);
    }

     /*Function to convert degree to radians */
     fn rad_value(value:&f64){
        let rad_val = value.to_radians();
        println!("rad value is {}",rad_val);
    }

     /*Function to get percentage */
     fn percent_value(value:&f64){
        let per_val = value* 0.01;
        println!("percentage value is {}",per_val);
    }

     /*Function to find sin inverse */
     fn sin_inverse(value:&f64){
        let sin_inver = value.asin();
        println!("sin inverse value is {}",sin_inver);
    }

    /*Function to find cos inverse */
    fn cos_inverse(value:&f64){
        let cos_inver = value.acos();
        println!("cos inverse value is {}",cos_inver);
    }

    /*Function to find tan inverse */
    fn tan_inverse(value:&f64){
        let tan_inver = value.atan();
        println!("tan inverse value is {}",tan_inver);
    }

    /*Function to find hyperbolic sin */
    fn hyper_sin(value:&f64){
        let hyper_sin = value.sinh();
        println!("hyperbolic sin value is {}",hyper_sin);
    }

    /*Function to find hyperbolic cos */
    fn hyper_cos(value:&f64){
        let hyper_cos = value.cosh();
        println!("hyperbolic cos value is {}",hyper_cos);
    }

    /*Function to find hyperbolic tan */
    fn hyper_tan(value:&f64){
        let hyper_tan = value.tanh();
        println!("hyperbolic sin value is {}",hyper_tan);
    }
    calculate_and_display_cgpa(25.0, 800.0);
    addition(&[2,3]);
    subtraction(&[7,5]);
    multiplication(&[7,5]);
    division(&[6,2]);
    sin_value(&0.5);
    cos_value(&0.5);
    tan_value(&0.5);
    ln_value(&0.5);
    deg_value(&0.5);
    rad_value(&0.5);
    percent_value(&60.0);
    sin_inverse(&0.5);
    cos_inverse(&0.5);
    tan_inverse(&0.5);
    hyper_sin(&0.5);
    hyper_cos(&0.5);
    hyper_tan(&0.5);
}
