fn main() {
    let shadow_number = 7;
    let shadow_number = shadow_number + 2;
    {
        let shadow_number = 20 + 3;
        println!("number in block {}", shadow_number);
    }
    println!("this number plus two {}", shadow_number);
}
