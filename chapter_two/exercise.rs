fn main() {
    exercise_one();
    exercise_two(2);
    exercise_three();
    exercise_four(50);
    exercise_five(5);
}

// exercise 1
fn exercise_one() {
    let first_name = "aji ";
    let last_name = "prasetyo";
    let result = format!("{} {}", first_name, last_name);
    println!("Exercise no params function : {}", result);
}

// exercise 2
fn exercise_two(numbers: i32) {
    // use scenario 1
    for index in 0..numbers {
        println!("Exercise with looping function : {}", index + 1);
    }

    // use scenario 2
    let numbers: i32 = numbers as i32;
    for index2 in 0..numbers {
        println!("Exercise with looping shadowing function : {}", index2 + 1);
    }
}

// exercise 3
fn exercise_three() {
    let numbers = 5;

    for index in 0..numbers {
        println!(
            "Exercise with no params and looping function : {}",
            index + 1
        );
    }
}

// exercise 4
fn exercise_four(hpp_price: i32) {
    println!("Your sale price is {}", sale_price(hpp_price));
}

// exercise 5
fn exercise_five(numbers: i32) {
    let answer = square(numbers);
    println!("The square of 5 is {}", answer);
}

fn sale_price(price: i32) -> i32 {
    if even_number(price) {
        return price - 10;
    } else {
        return price - 3;
    }
}

fn even_number(numbers: i32) -> bool {
    return numbers % 2 == 0;
}

fn square(numbers: i32) -> i32 {
    return numbers * numbers;
}
