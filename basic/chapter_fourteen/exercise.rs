struct Wrapper<T> {
    value: T,
}

fn main() {
    exercise();
    let data_wrapper = Wrapper::data("Foo").value;

    println!("{:#?}", data_wrapper)
}

fn exercise() {
    let mut string_data: Vec<&str> = Vec::new();
    string_data.push("nama");
    println!("{:?}", string_data);
}

impl<T> Wrapper<T> {
    pub fn data(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::data(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::data("Foo").value, "Foo");
    }
}
