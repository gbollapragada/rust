// private by default use pub to explicitly make it public and visible with imports
#[derive(Debug)]
pub struct AddressTwo {
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pin: String,
    pub coordinate: (u8, u8),
    pub phone: String,
}

impl AddressTwo {
    pub fn new(
        address1_in: String,
        address2_in: String,
        city_in: String,
        state_in: String,
        country_in: String,
        pin_in: String,
        phone_in: String,
        coordinate_in: (u8, u8),
    ) -> Self {
        Self {
            address1: address1_in,
            address2: address2_in,
            city: city_in,
            state: state_in,
            country: country_in,
            pin: pin_in,
            phone: phone_in,
            coordinate: coordinate_in,
        } // return
    }
}
