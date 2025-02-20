// struct property and type data
struct ColorStruct {
    red: u32,
    green: u32,
    blue: u32,
}

// struct type data only
struct ColorTupleStruct(u32, u32, u32);

#[derive(Debug)]
struct ColorUnitLikeStruct;

#[derive(Debug)]
struct Transaction {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

fn main() {
    let price_sender_package = 1200;
    let sender_country = "wakanda";
    let recipient_country = "papua";
    let _color = ColorStruct {
        red: 255,
        green: 0,
        blue: 0,
    };
    let _color_tuple = ColorTupleStruct(255, 0, 255);
    let _unit = ColorUnitLikeStruct;
    let grams = 12;

    let trx = create_trx();
    let your_trx = Transaction {
        name: String::from("Hacker in Rust"),
        count: 1,
        ..trx
    };

    println!(
        "the color red {} green {} blue {}",
        _color.red, _color.green, _color.blue
    );

    println!(
        "the color red {} green {} blue {}",
        _color_tuple.0, _color_tuple.1, _color_tuple.2
    );

    println!("i have new transaction, name {}, year {}, is_phone {}, is_mobile {}, is_email {}, item number {}, count {}", your_trx.name,your_trx.year,your_trx.made_by_phone,your_trx.made_by_mobile,your_trx.made_by_email,your_trx.item_number,your_trx.count);

    let package_data = Package::new(
        sender_country.to_string(),
        recipient_country.to_string(),
        grams,
    );
    let _validate_country = Package::is_international(&package_data);
    if _validate_country {
        let price_total = Package::get_fee(&package_data, price_sender_package);
        println!(
            "data shipping package sender {} to destination {} with weight {} and amount {}",
            &package_data.sender_country,
            &package_data.recipient_country,
            &package_data.weight_in_grams,
            price_total
        );
    } else {
        panic!("Can not ship a package because sender and recipient country not different.")
    }
}

fn create_trx() -> Transaction {
    Transaction {
        name: String::from("Bob"),
        year: 2025,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 169,
        count: 0,
    }
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            // This is not how you should handle errors in Rust,
            // but we will learn about error handling later.
            panic!("Can not ship a package with weight below 10 grams.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool {
        if self.sender_country != self.recipient_country {
            return true;
        }
        false
    }

    fn get_fee(&self, price_per_gram: u32) -> u32 {
        price_per_gram * self.weight_in_grams
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorStruct {
            red: 0,
            green: 255,
            blue: 0,
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let greens = ColorTupleStruct(0, 255, 0);

        assert_eq!(greens.0, 0);
        assert_eq!(greens.1, 255);
        assert_eq!(greens.2, 0);
    }

    #[test]
    fn unit_like_structs() {
        let color_unit_like_struct = ColorUnitLikeStruct;
        let message = format!("{:?}s are fun!", color_unit_like_struct);

        assert_eq!(message, "ColorUnitLikeStructs are fun!");
    }

    #[test]
    fn transaction() {
        let trx = create_trx();

        let your_trx = Transaction {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..trx
        };

        assert_eq!(your_trx.name, "Hacker in Rust");
        assert_eq!(your_trx.year, trx.year);
        assert_eq!(your_trx.made_by_phone, trx.made_by_phone);
        assert_eq!(your_trx.made_by_mobile, trx.made_by_mobile);
        assert_eq!(your_trx.made_by_email, trx.made_by_email);
        assert_eq!(your_trx.item_number, trx.item_number);
        assert_eq!(your_trx.count, 1);
    }

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Wakanda");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Wakanda");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fee() {
        let sender_country = String::from("Wakanda");
        let recipient_country = String::from("Wakanda");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fee(cents_per_gram), 4500);
        assert_eq!(package.get_fee(cents_per_gram * 2), 9000);
    }
}
