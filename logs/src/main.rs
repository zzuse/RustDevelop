use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            let error_logs = extract_errors(&text_that_was_read.as_str());
            println!("{:#?}", error_logs);
            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors.txt"),
                Err(reason_write_failed) => {
                    println!("Writing of errors.txt failed: {}", reason_write_failed);
                }
            }
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed)
        }
    }

    // alternatives 1
    let text = fs::read_to_string("logs.txt").expect("failed to read logfile");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors_.txt", error_logs.join(",")).expect("failed to write errors_.txt");
    // alternatives 2
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors_.txt", error_logs.join("\n"))?;

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division)
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong)
        }
    }

    match validate_email(String::from("asdf@asdf.com")) {
        Ok(..) => println!("email is valid"),
        Err(reason_this_failed_validation) => {
            println!("{}", reason_this_failed_validation)
        }
    }

    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        // String::from("Olives"),
    ];

    match validate_ingredients(&ingredients) {
        Ok(..) => println!("Ingredients are good to go"),
        Err(error) => println!("Failed validation: {}", error),
    }

    string_test(
        "blue".to_string(),
        &String::from("red"),
        String::from("red").as_str(),
    );
    Ok(())
    // Err(Error::other("something went wrong..."))
}

// Generic Enum Result<T,E> { Ok(T), Err(E)}
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cant divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("too many ingredients"))
    } else {
        Ok(())
    }
}

fn string_test(a: String, b: &String, c: &str) {
    // stack fast, 2-8MB
    // heap slow, big
    let string_slice = &a[1..4];
    println!("a1..4{:#?}", string_slice);
    println!("b{:#?}", b);
    println!("c{:#?}", c);
}

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }
    results
}
