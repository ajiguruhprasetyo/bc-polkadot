use std::result::Result::Ok;
use std::result::Result::Err;

fn main(){}

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
   if b == 0 {
       Err(DivisionError::DivideByZero)
   } else if a % b ==0 {
       Ok(a / b)
   } else {
       Err(DivisionError::NotDivisible(NotDivisibleError {
           dividend: a,
           divisor: b,
       }))
   }
}

fn _result_with_list() -> Result<Vec<i32>, DivisionError>  {
    let numbers = vec![27, 297, 38502, 81];
    let _division_results = numbers.into_iter().map(|n| divide(n, 27));
    let mut vect : Vec<i32> = Vec::new();

    for result in _division_results {
        match result {
            Ok(n) => vect.push(n),
            Err(e) => return Err(e),
        }
    }
    Ok(vect)
}

fn _list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    let mut v: Vec<Result<i32, DivisionError>> = Vec::new();
    for result in division_results {
        v.push(result);
    }
    v
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn capitalize_words_vector(_words: &[&str]) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    let word_iterator = _words.iter();
    for word in word_iterator {
        vector.push(capitalize_first(word));
    }
    vector
}

pub fn capitalize_words_string(_words: &[&str]) -> String {
    let mut data: String = String::new();
    let word_iterator = _words.iter();
    for word in word_iterator {
        data += &capitalize_first(word);
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_capitalize_first()  {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty_capitalize_first() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }

    #[test]
    fn iterate_string_vec() {
        let my_favorite_fruit = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];
        let mut iteration_fruit = my_favorite_fruit.iter();
        assert_eq!(iteration_fruit.next(), Some(&"banana"));
        assert_eq!(iteration_fruit.next(), Some(&"custard apple"));
        assert_eq!(iteration_fruit.next(), Some(&"avocado"));
        assert_eq!(iteration_fruit.next(), Some(&"peach"));
        assert_eq!(iteration_fruit.next(), Some(&"raspberry"));
        assert_eq!(iteration_fruit.next(), None);
    }

    #[test]
    fn test_success_devide() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", _result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", _list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }

}
