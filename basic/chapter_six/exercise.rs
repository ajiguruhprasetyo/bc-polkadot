fn main() {
    exercise_one();
    reordering();
    string_function();
}

fn exercise_one() {
    let vec_data = vec![22, 44, 66];
    println!("the data vector before add {:?}", vec_data);

    let vec_result = fill_vec(vec_data.clone());
    println!("the data vector {:?}", vec_result);

    assert_eq!(vec_data, vec![22, 44, 66]);
    assert_eq!(vec_result, vec![22, 44, 66, 88]);
}

fn reordering() {
    let mut durian = 1300;
    let acovado = &mut durian;
    *acovado += 1100;
    let kurma = &mut durian;
    *kurma += 12000;
    println!("this data {}", kurma);
    assert_eq!(durian, 14400);
}

fn string_function() {
    let data = "Rust is strong!".to_string();
    get_char(&data);

    string_uppercase(data);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

// Should not take ownership
fn get_char(data: &String) -> char {
    let result = data.chars().last().unwrap();
    println!("{}", result);
    result
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

#[cfg(test)]
mod tests {

    #[test]
    fn borowing_test() {
        let mut durian = 100;
        let avocado = &mut durian;
        *avocado += 100;
        let kurma = &mut durian;
        *kurma += 1000;
        assert_eq!(durian, 1200);
    }
}
