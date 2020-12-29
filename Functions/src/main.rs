fn main() {
    let sum: f32 = some_sum(10.0, 10.0);
    println!("The sum is {}", sum);

    println!(
        "The greater number between 10 and 20 is : {}",
        some_greater(10, 20)
    );

    some_procedure();

    some_str_procedure("Test string litral or text slice");

    // let test_string: String = String::from("TEST String");
    // some_str_procedure(test_string.to_string()); does not work as we need to pass a slice

    let test_string: &str = &String::from("TEST String has to converted to slice before passing");
    some_str_procedure(test_string);
}

fn some_sum(param_a: f32, param_b: f32) -> f32 {
    let sum = param_a + param_b;

    sum
}

fn some_greater(param_a: u8, param_b: u8) -> u8 {
    if param_a > param_b {
        param_a //return
    } else {
        param_b //return
    }
}

fn some_procedure() {
    println!("some_procedure called .. Procedures don't return any thing");
}

fn some_str_procedure(param: &str) {
    println!("This was passed to us : {}", param);
}
