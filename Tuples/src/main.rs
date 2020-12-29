fn main() {
    let rgb: (u8, u8, u8) = get_rgb(255, 255, 255);

    println!("Red ({}), green({}), blue({})", rgb.0, rgb.1, rgb.2);

    // empty tuples returned by functions
    let testfunc: () = myfunc();
    println!("My function is {:?}", testfunc);

    // tuple -- to acces use 0 baed index
    let person_tuple1 = (1, "Ganesh", "43", "Waterloo", "N2J068");
    let person_tuple2 = (2, "Akhil", "11", "Waterloo", "N2J068");

    println!("person_tuple1.0(name) : {}", person_tuple1.1);
    println!("person_tuple2 all data : {:?}", person_tuple2);

    // nested tuples
    let person_tuple3 = (2, "Akhil", "11", "Waterloo", "N2J068", (255, 255));
    println!("person_tuple3 coordinates : {:?}", (person_tuple3.5));
    println!(
        "person_tuple3 coordinates : {}, {}",
        (person_tuple3.5).0,
        (person_tuple3.5).1
    );
}

fn get_rgb(red: u8, green: u8, blue: u8) -> (u8, u8, u8) {
    (red, green, blue)
}

fn myfunc() {
    ()
}
