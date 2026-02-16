const FREEZING_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_F) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + FREEZING_F
}

fn main() {
    let mut temp_f: f64 = 32.0;

    // First conversion
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}F = {temp_c}C");

    // reverse function 
    let back_to_f = celsius_to_fahrenheit(temp_c);
    println!("{temp_c}C = {back_to_f}F");

    // change next 5 integer temps
    let mut count: i32 = 0;

    loop {
        if count == 5 {
            break;
        }
        temp_f = temp_f + 1.0;
        let c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}F = {c}C");
        count = count + 1;
    }
}
