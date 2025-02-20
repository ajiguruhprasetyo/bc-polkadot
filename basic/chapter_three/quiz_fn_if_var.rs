fn main() {
    quiz_many_an_apple(30);
}

pub fn quiz_many_an_apple(total_apples: u32) {
    println!(
        "Price for {} apple: {} Rustbucks",
        total_apples,
        calculate_price_of_apples(total_apples)
    );
}

fn calculate_price_of_apples(number_of_apples: u32) -> u32 {
    if number_of_apples > 40 {
        number_of_apples
    } else {
        number_of_apples * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
