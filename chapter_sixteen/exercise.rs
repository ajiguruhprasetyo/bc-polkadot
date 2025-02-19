fn main() {
    exercise();
    exercise_struct();
}

// memory less because not copy data and owner data
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn exercise_struct() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}

fn exercise() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let string3 = "xyz";
    let result;
    let result2 = longest(string1.as_str(), string3);

    {
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is '{}'", result);
    println!("The longest string 2 is '{}'", result2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
