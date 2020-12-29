#[allow(unused_variables)] // don't use in production only for hello_work and samples
#[allow(unused_assignments)]

fn main() {
    println!("Hello, World!");

    let some_bool_data: bool = true; // true or false
    let mut some_bool_data2 = true;

    // some_bool_data = false; cannot change un mutable variables
    some_bool_data2 = false; // this variable is mutable as its defined during initializaton with mut keyworkd

    let some_i8_data: i8 = 10; // holds 8 bit signed integers ranging from -127 to +127
    println!("The MIN for i8 is : {}", std::i8::MIN);
    println!("The MAX for i8 is : {}", std::i8::MAX);

    // Other types include u8 for unsigned 0 to 255
    let some_u8_data: u8 = 10;
    println!("The MIN for u8 is : {}", std::u8::MIN);
    println!("The MAX for u8 is : {}", std::u8::MAX);

    // default is i32
    let some_int32_data = 10;

    // can take the system / os / architecture default by using isize or usize eiter 32 or 64
    let some_int_data: isize = 10;
    println!("The MIN for isize is : {}", std::isize::MIN);
    println!("The MAX for isize is : {}", std::isize::MAX);

    // float f32 or f64(default)
    let some_float32_data: f32 = 10.1;

    // charecter data char
    let some_char_data: char = 'a'; // can hold all asci and non asci data use single quote
}
