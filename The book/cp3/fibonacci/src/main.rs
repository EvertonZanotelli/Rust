use  std::io;

fn main() {

    loop {
        println!("Choose whitch Fibonacci number to reach");

        //Getting user imput
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");
        
        //Converting 
        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", fibo(number));
    }
}

fn fibo(num: u64) -> u64 {
    if num < 2 {
        1
    } else {
        fibo(num -1) + fibo(num - 2)
    }
}