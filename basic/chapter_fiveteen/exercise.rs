fn main() {
    exercise();
}

trait AppendBar {
    fn append_bar(self) -> Self;
}

trait AppendBarVec {
    fn append_bar_vec(self) -> Self;
}

pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

pub trait LicensedV2 {
    fn licensing_info_v2(&self) -> String {
        "some information".to_string()
    }
}

struct _SomeSoftware {
    _version_number: i32,
}

struct _OtherSoftware {
    _version_number: String,
}

struct SomeSoftwareV2 {}

struct OtherSoftwareV2 {}

impl Licensed for _SomeSoftware {}
impl Licensed for _OtherSoftware {}

impl LicensedV2 for SomeSoftwareV2 {}
impl LicensedV2 for OtherSoftwareV2 {}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        self + "Bar"
    }
}

impl AppendBarVec for Vec<String> {
    fn append_bar_vec(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn compare_license_types(software: impl LicensedV2, software_two: impl LicensedV2) -> bool {
    software.licensing_info_v2() == software_two.licensing_info_v2()
}

fn exercise() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("string concatenan: {}", s);

    let s_vec = vec![String::from("Foo")];
    let s_vec = s_vec.append_bar_vec();
    println!("string vec concatenan: {:?}", s_vec);

    let some_software = SomeSoftwareV2 {};
    let other_software = OtherSoftwareV2 {};
    let validate = compare_license_types(some_software, other_software);
    println!("validate data {}", validate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar_vec();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = _SomeSoftware { _version_number: 1 };
        let other_software = _OtherSoftware {
            _version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
