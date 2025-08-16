use std::fs;
use std::io::Error;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            println!("{:#?}", text_that_was_read);
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed)
        }
    }

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
