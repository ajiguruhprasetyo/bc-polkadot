fn main() {
    exercise_one();
    exercise_two();
    exercise_three();
    exercise_four();
    exercise_five();
    exercise_six();
}

fn exercise_one() {
    let is_deleted = false;
    if is_deleted {
        println!("The file is deleted");
    } else {
        println!("The file is not deleted");
    }

    let is_traveled = true;
    if is_traveled {
        println!("The person has traveled");
    } else {
        println!("The person has not traveled");
    }
}

fn exercise_two() {
    // check alphabet or numeric need to use single quotes and also 1 character only
    let string_one = 'd';

    if string_one.is_alphabetic() {
        println!("Alphabet");
    } else if string_one.is_numeric() {
        println!("Numeric");
    } else {
        println!("Neither alphabet nor numeric");
    }
    let string_two = '3';
    if string_two.is_alphabetic() {
        println!("Alphabet");
    } else if string_two.is_numeric() {
        println!("Numeric");
    } else {
        println!("Neither alphabet nor numeric");
    }
}

fn exercise_three() {
    // array_data the data index 1 = 3 is [1,2,3] and also so type 100 times
    let array_data = [3; 100];
    // code :? to show the array data
    println!("result array {:?}", array_data);

    if array_data.len() >= 100 {
        println!("a lot of data");
    } else {
        println!("a bit of data");
        panic!("Array not big enough, more elements needed")
    }

    let repeat_char = "aji".repeat(100);
    // code :? to show the repeat data
    println!("result repeat {:?}", repeat_char);

    if array_data.len() >= 100 {
        println!("a lot of data");
    } else {
        println!("a bit of data");
        panic!("Array not big enough, more elements needed")
    }
}

fn exercise_four() {
    let array_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice_data = &array_data[1..6];
    println!("{:?}", slice_data);
}

fn exercise_five() {
    let object_data = ("sutelo", 5);
    let (name, age) = object_data;
    println!("Name: {}, Age: {}", name, age);
}

fn exercise_six() {
    let numb = (2, 5, 10);
    let index_first = numb.0;
    let index_second = numb.1;
    let index_third = numb.2;

    println!("Index data : {}", index_first);
    println!("Index data : {}", index_second);
    println!("Index data : {}", index_third);
}

#[cfg(test)]
mod tests {

    #[test]
    fn handling_slice_array() {
        let array_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let slice_data = &array_data[1..7];
        assert_eq!(slice_data, [2, 3, 4, 5, 6, 7]);
    }
}
