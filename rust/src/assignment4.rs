fn main(){
    use std::io;
    /*Assignment 1 - 12 days of Christmas */
    /*Defining the variables */
    let first_day = "On the first day of Christmas\nMy true love sent to me\nA partridge in a pear tree";
    let second_day = "On the second day of Christmas\nMy true love sent to me\nTwo turtle-doves\nAnd a partridge in a pear tree";
    let third_day = "On the third day of Christmas\nMy true love sent to me\nThree French hens\nTwo turtle-doves\nAnd a partridge in a pear tree";
    let fourth_day = "On the fourth day of Christmas\nMy true love sent to me\nFour calling birds\nThree French hens\nTwo turtle-doves\nAnd a partridge in a pear tree";
    let fifth_day = "On the fifth day of Christmas\nMy true love sent to me\nFive golden rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtle-doves\nAnd a partridge in a pear tree";
    let sixth_day = "On the sixth day of Christmas\nMy true love sent to me\nSix geese a laying\nFive golden rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtle-doves\nAnd a partridge in a pear tree";
    let seventh_day = "On the seventh day of Christmas\nMy true love sent to me\nSeven swans a swimming\nSix geese a-laying";
    let five_golden_rings = "Five golden rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtle-doves\nAnd a partridge in a pear tree";
    let eigth_day = "On the eighth day of Christmas\nMy true love sent to me\nEight maids a milking\nSeven swans a swimming\nSix geese a-laying";
    let ninth_day = "On the ninth day of Christmas\nMy true love sent to me\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying";
    let tenth_day = "On the tenth day of Christmas\nMy true love sent to me\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying";
    let eleventh_day = "On the 11th day of Christmas\nMy true love sent to me\nI sent 11 pipers piping\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying";
    let twelveth_day = "On the 12th day of Christmas\nMy true love sent to me\n12 drummers drumming\nEleven pipers piping\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying";


    let mut counter = 0; /*Defining our loop counter variable */
    /*Defining an array to contain all the variables */
    let counter_list = [first_day,second_day,third_day,fourth_day,fifth_day,sixth_day,seventh_day,five_golden_rings,eigth_day,ninth_day,tenth_day,eleventh_day,twelveth_day]; 
    for n in counter_list{
        counter += 1; /*Counter variable increases by 1 at each loop */
        /*If less than index number 8 print the first 7 variables */
        if counter < 8{
            println!("{n}");
            println!("{}","");
        }
        /*If greater than index number 8 print the variables after it */
        else if counter > 8 { 
            println!("{}",counter_list[7]); /*prints the five golden rings line before each variable greater than the 7th day */
            println!("{}","");
            println!("{n}");
            println!("{}","");
        }
    }
    /*Assignment 2 Grading each student with Name and Score*/
    // Get number of students
    let num_students = loop {
        println!("How many students are in the class? (minimum 10)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse::<u32>() {
            Ok(num) if num >= 10 => break num,
            Ok(_) => println!("Please enter a number of 10 or greater"),
            Err(_) => println!("Please enter a valid number"),
        }
    };

    // Process each student
    for i in 1..=num_students {
        println!("\nStudent #{}", i);
        
        // Get student name
        println!("Enter student name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read name");
        let name = name.trim().to_string();

        // Get student score
        let score = loop {
            println!("Enter score for {} (0-100):", name);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read score");
            
            match input.trim().parse::<u32>() {
                Ok(num) if num <= 100 => break num,
                Ok(_) => println!("Score must be between 0 and 100"),
                Err(_) => println!("Please enter a valid number"),
            }
        };

        // Assign grade based on score
        let (grade, description) = if score >= 70 && score <= 100 {
            ("A", "Excellent")
        } else if score >= 60 {
            ("B", "Very Good")
        } else if score >= 55 {
            ("C", "Good")
        } else if score >= 45 {
            ("D", "Satisfactory")
        } else if score >= 40 {
            ("E", "Pass")
        } else {
            ("F", "Fail")
        };

        // Display results
        println!("Student: {}", name);
        println!("Score: {}", score);
        println!("Grade: {} ({})", grade, description);
    }

    println!("\nGrade processing complete!");
}
