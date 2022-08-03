use  std::io;

fn main() {

    loop {
        println!("Write a degree temperature in fahrenheit to convert to Celsius");

        //Getting user imput
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read temperature");
        
        //Converting string to float
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Formula
        let temperature = (temperature - 32.0) * 5.0 / 9.0;
        
        println!("The provided temperature in Celsius is {temperature}");
    }
}