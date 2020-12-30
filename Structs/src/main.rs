/*
    Multi line comments
    comments
    comments
    comments
*/
mod address_two;
use address_two::*;

mod counter;
use counter::*;

#[derive(Debug)]
struct PersonData {
    name: String,
    age: u8,
    address: (
        String,   // address1
        String,   // address2
        String,   // City
        String,   // State
        String,   // Country
        String,   // Pin
        String,   // Phone
        (u8, u8), // Coordinates
    ),
    is_member: bool,
    second_address: AddressTwo,
}

impl PersonData {
    fn is_member(&self) -> bool {
        self.is_member // return
                       /*
                       if self.is_member == true {
                           true
                       } else {
                           false
                       }*/
    }
}

fn main() {
    let mut ganesh_data = PersonData {
        name: "Ganesh".to_string(),
        age: 43,
        address: (
            "123".to_string(),
            "waterloo buildin".to_string(),
            "Kitchner".to_string(),
            "Ontario".to_string(),
            "Canada".to_string(),
            "N2J0C6".to_string(),
            "007".to_string(),
            (4, 5),
        ),
        is_member: true,
        second_address: AddressTwo {
            address1: "add12".to_string(),
            address2: "add22".to_string(),
            city: "city2".to_string(),
            state: "state2".to_string(),
            country: "country2".to_string(),
            pin: "pin2".to_string(),
            phone: "phone2".to_string(),
            coordinate: (0, 0),
        },
    };

    println!("Person data is : {:#?}", ganesh_data);
    println!("Person is a member: {:#?}", ganesh_data.is_member());
    println!("Editing teh struct..");

    ganesh_data.age = 50;
    (ganesh_data.address.7).0 = 50;
    (ganesh_data.address.7).1 = 40;

    println!("Updated person data is : {:#?}", ganesh_data);

    let mut akhil_data = PersonData { ..ganesh_data }; // copy from another struct
    akhil_data.name = "Akhil".to_string();
    akhil_data.age = 11;
    println!("Coppied person data is : {:#?}", akhil_data);
    /*
    This copy does not work
    because
        main.rs(51, 26): value moved here, this occurs as address field
        does not implement the copy triat

        second move of value address, value used here after move above
    let mut maha_data = PersonData {
        name: "Maha".to_string(),
        age: 40,
        ..ganesh_data
    };
    */
    // let mut maha_data = PersonData { ..akhil_data };

    let maha_data = PersonData {
        name: "Maha".to_string(),
        age: 40,
        ..akhil_data
    };
    println!("Coppied person data is : {:#?}", maha_data);

    // initialize with new keyword
    let address_two = AddressTwo::new(
        "address12".to_string(),
        "address22".to_string(),
        "city2".to_string(),
        "state2".to_string(),
        "country2".to_string(),
        "pin2".to_string(),
        "phone2".to_string(),
        (66, 77),
    );

    println!("Address2 is {:#?}", address_two);

    let mut my_counter: Counter = Counter::new();

    for _ in 0..10 {
        my_counter.increment();
        println!("Counter reached {}", my_counter.counter);
    }

    println!("Counter is bigger than 5 : {}", my_counter.is_bigger(5));
}

// decorating with more functionality / augmenting with is_bigger
impl Counter {
    pub fn is_bigger(&self, compare_to: u8) -> bool {
        self.counter > compare_to // if and return
    }
}
