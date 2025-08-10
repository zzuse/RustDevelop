use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division)
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong)
        }
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
