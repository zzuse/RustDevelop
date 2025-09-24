fn print_for_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn print_for_each_elements(elements: &Vec<String>) {
    elements.iter().for_each(|el| println!("{} {}", el, el));
}

fn print_map_for_each_elements(elements: &Vec<String>) {
    elements
        .iter()
        .map(|el| format!("{} {} {}", el, el, el))
        .for_each(|el| println!("{} {}", el, el));
}

fn print_map_for_each_elements_slices(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{} {}", el, el));
}

fn shorten_strings(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn shorten_strings_slice(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut colors_iter = colors.iter();

    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());

    print_for_elements(&colors);
    print_for_each_elements(&colors);
    print_map_for_each_elements(&colors);
    // prefer slices for adapability
    print_map_for_each_elements_slices(&colors);
    shorten_strings(&mut colors);
    println!("{:#?}", colors);
    shorten_strings_slice(&mut colors);
    println!("{:#?}", colors);
    println!("{:#?}", to_uppercase(&colors));
}
