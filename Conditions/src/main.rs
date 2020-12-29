fn main() {
    let some_true_bool: bool = true;

    if some_true_bool == true {
        println!("some_true_bool is {}", some_true_bool);
    } else {
        println!("some_bool is {}", some_true_bool);
    }

    let param_1: u8 = 30;
    let param_2: u8 = 50;

    if param_1 > param_2 || (some_true_bool == true && some_true_bool == true) {
        println!("This is true");
    }

    // inline or ternary operation??
    // return type should be compatable to the assignment
    // will get a comple error if return is not proper type
    let inline_var: u16 = if param_1 == 30 {
        300 // considered as return does not need ";"
    } else if param_2 == 50 {
        500
    } else {
        1000
    };

    println!("Returned : {}", inline_var);

    // match statements
    match some_true_bool {
        true => {
            println!("True");
        }
        false => {
            println!("False");
        }
    }

    match param_1 {
        1 => println!("first condition"),
        2 => println!("second condition"),
        3 => println!("trhird condition"),
        4 => {
            println!("fourth condition ...");
            println!("Do a lot more here");
        }
        5 | 6 => (println!("condion fice or six do the same")),
        _ => (println!("all else")), // catch all here
    }

    //linine match
    let linine_match_var: u16 = match some_true_bool {
        true => 300,
        false => 500,
    };
    println!("returned {}", linine_match_var);
}
