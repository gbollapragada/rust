#[allow(unused_variables)]

fn main() {
    // Strings in rust are just a collection of u8
    let some_str_data: &str = "Hello"; // string slice
    let some_string_data: String = String::from("World"); // String

    let string_from_str: String = some_str_data.to_string();
    let string_from_str3 = some_str_data.to_string();
    let string_from_str2 = "New String".to_string();

    let string_2 = String::from("New String");
    let string_3 = String::from(some_str_data);
    let string_4 = String::from(some_string_data);

    let str_from_string: &str = &string_2;
    let str_from_string_1 = &string_2;
    let str_from_str_1 = some_str_data;

    // let concat_test_1: String = "One" + " Two"; Does not work
    let concat_test: String = "One".to_string() + &" Two".to_string(); // only in this order; refer borrowing

    let concat_test_1: String = ["one", " ", "two"].concat();
    let concat_test_2 = format!("{} {}", "one", "two");

    let mut mut_sring = String::new();
    mut_sring.push_str("one");
    mut_sring.push_str(str_from_str_1);
    mut_sring.push('a');
}
