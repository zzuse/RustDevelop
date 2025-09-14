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

fn main() {
    let colors = vec![
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
}
