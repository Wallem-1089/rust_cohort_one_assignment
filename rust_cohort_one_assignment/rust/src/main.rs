fn main() {
    /*Defining the variables */
    let name = "Walter";
    let mat_number = "ENG2203987";
    let fav_emoji = '😂';
    let quotient = 5.6/6.0;
    /*Printing out the variables */
    println!("{name}");
    println!("{mat_number}");
    println!("{fav_emoji}");
    println!("{quotient}");

    /*Calling the functions */
    print_name("Walter");
    print_mat_number("ENG2203987");
    print_fav_emoji(&'😂');
    print_division(&5.6,&6.0);
    
}
/*Defining the functions */
fn print_name(name:&str){
    /*Function to print out name */
    println!("My name is {}",name); }
fn print_mat_number(number:&str){
    /*Function to print out mat number */
    println!("My mat number is {}",number); }
fn print_fav_emoji(emoji:&char){
    /*Function to print out favourite emoji */
    println!("My favourite emoji is {}",emoji); }
fn print_division(number:&f64,second_number:&f64){
    /*Function to print out the quotient */
    let numerator = number;
    let denominator = second_number;
    let result = numerator/denominator;
    println!("{result}");
}