use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    exercise_one::make_sausage();

    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

mod exercise_one {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    //add pub on function .. if you need call this function
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

mod delicious_snacks {
    pub use self::fruits::APPLE as fruit;
    pub use self::veggies::CARROT as veggie;

    mod fruits {
        #[allow(dead_code)]
        pub const PEAR: &'static str = "Pear";
        #[allow(dead_code)]
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        #[allow(dead_code)]
        pub const CUCUMBER: &'static str = "Cucumber";
        #[allow(dead_code)]
        pub const CARROT: &'static str = "Carrot";
    }
}
