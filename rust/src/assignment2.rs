fn main() {
    /*Defining the variables */
    let student_score: [i32; 10] = [49,48,52,53,54,56,57,58,59,60]; /*stores student records - at least 10 */
    let total_student_score: i32 = student_score.iter().sum(); /*calculates the total */
    println!("This is the total score {}", total_student_score); 
    let average_student_score = total_student_score as f64/ student_score.len() as f64; /*calculates the average */
    println!("This is the average score {}", average_student_score);
    let highest_student = student_score.iter().max().unwrap();/*determines the highest scoring student */
    println!("The highest score is {}", highest_student);
    let mut fail_count = 0;
    /*sorts the students by pass and fail*/
    for element in student_score.iter() {
        if *element < 50 {
            println!("{} failed",element);
            fail_count = fail_count + 1; /*counts how many students failed - Failure starts around <50 */
        }
        else{
            println!("{} passed", element);
        }
    }
    println!("{}", fail_count);
    


    
}