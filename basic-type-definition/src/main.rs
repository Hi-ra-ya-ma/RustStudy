fn main() {
    let x = String::from("Hello");
    let len = string_length(&x);
    println!("len is {}", len);
    println!("x is {}", x);
}

fn string_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn main2() {
    let x = 100;
    println!("x is {}", x);

    x = 200; // Error!
    println!("x is {}", x);
}
