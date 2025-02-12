fn main() {
    println!("this data {}", exercise_one(5, 3));
    println!("this result name is {}", exercise_two("mynamed"));
    println!("this animal stay in {}", exercise_three("chicken"));
}

// exercise 1
pub fn exercise_one(data: i32, data2: i32) -> i32 {
    if data > data2 {
        data
    } else {
        data2
    }
}

// exercise 2
pub fn exercise_two(string_data: &str) -> &str {
    if string_data == "myname" {
        "myname"
    } else if string_data == "name" {
        "name"
    } else {
        "uname"
    }
}

// exercise 3
pub fn exercise_three(name_animal: &str) -> &'static str {
    let identity = if name_animal == "crocodile" {
        1
    } else if name_animal == "chicken" {
        2
    } else if name_animal == "snake" {
        3
    } else {
        0
    };

    let environment = if identity == 1 {
        "Coffeeshop"
    } else if identity == 2 {
        "House"
    } else if identity == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    environment
}

#[cfg(test)]
mod tests {
    use super::*;

    // exercise 2
    #[test]
    fn myname_is_myname() {
        assert_eq!(exercise_two("myname"), "myname");
    }

    #[test]
    fn default_to_uname() {
        assert_eq!(exercise_two("joe"), "uname");
    }

    #[test]
    fn name_to_name() {
        assert_eq!(exercise_two("name"), "name");
    }
}
