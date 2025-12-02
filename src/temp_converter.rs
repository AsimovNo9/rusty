fn farenheit_to_celcius(farenheit: f64) -> f64 {
    (farenheit-32.0) * 5.0/9.0
}

fn celcius_to_farenheit(celcius: f64) -> f64 {
    (celcius*9.0/5.0) + 32.0
}

pub fn run(){
    println!("{}", celcius_to_farenheit(24.0));
    println!("{}", farenheit_to_celcius(100.0));
}