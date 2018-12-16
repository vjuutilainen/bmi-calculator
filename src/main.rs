use std::io;

fn prompt_number(question: &str) -> f64 {
    let number: f64;

    loop {
        println!("{}", question);

        // String type is a growable UTF-8 encoded bit of text.
        // ´new´ is an associated function of the String type
        let mut number_input = String::new();

        // io::Result is an enumeration, `Ok` or `Err`
        // if it's `Err`, `expect` will cause a crash
        io::stdin()
            .read_line(&mut number_input)
            .expect("Failed to read line");

        // trim, convert decimals and try to parse the number
        // if it's Ok, break the loop, if Err, continue
        match number_input.trim().replace(",", ".").parse() {
            Ok(parsed_input) => {
                number = parsed_input;
                break;
            }
            Err(_) => continue,
        };
    }

    number
}

fn main() {
    let weight: f64 = prompt_number("What is your weight in kilograms?");
    let height: f64 = prompt_number("What is your height in meters?");
    let result = weight / height.powf(2.0);

    // formatting the index to include one decimal
    println!(
        "With {} kg weight and {} m height your body mass index is {:.1}",
        weight, height, result
    );
}
