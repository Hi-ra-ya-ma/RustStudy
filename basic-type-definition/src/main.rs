fn main() {
    let x = String::from("Hello");
    println!("x is {}", x);
}

fn block() {
    let x = 100;
    println!("x is {}", x); // 100
    {
        let x = 200;
        println!("x is {}", x); // 200
    }
    println!("x is {}", x); // 100
}
