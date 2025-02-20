fn main() {
    exercise_one();
}

fn exercise_one() -> ([i32; 4], Vec<i32>, [i32; 4], Vec<i32>) {
    let array = [10, 20, 35, 60];
    let vector = vec![10, 20, 35, 60]; // using vector macro
    println!("data vector macro {:?}", vector);

    let multiply_by_two = vector_looping(&vector);
    println!("data vector macro multiple by two {:?}", multiply_by_two);

    let arrays = [15, 25, 40, 70];
    let mut vectors: Vec<i32> = Vec::new(); // using vector default
    vectors.push(15);
    vectors.push(25);
    vectors.push(40);
    vectors.push(70);

    println!("data vector non macro {:?}", vectors);

    (array, vector, arrays, vectors)
}

fn vector_looping(v: &Vec<i32>) -> Vec<i32> {
    // The problem is the element here is the reference to the element of the vector
    // for element in v.iter_mut() {
    //     *element *= 2;
    // }

    // vector element using map if you need new value
    v.iter().map(|element| element * 2).collect()
}
