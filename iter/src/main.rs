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

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    // into_iter use values, if you put & ref, it will move only refs
    vec_a.into_iter().for_each(|el| vec_b.push(el));
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
    println!("Shortern vector: {:#?}", colors);
    shorten_strings_slice(&mut colors);
    println!("Shorten Slice: {:#?}", colors);
    println!("Upper: {:#?}", to_uppercase(&colors));

    let mut destination = vec![];
    move_elements(colors, &mut destination);
    println!("Destinations: {:#?}", destination);
}
