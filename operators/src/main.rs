fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("a={}", a);
    a = a + 1;
    a -= 2;
    println!("a={}", a);
    println!("remainder of {} by {} is {}", a, 3, a % 3);

    let a_cubed = i32::pow(a, 3);
    println!("a_cubed={}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bit-wise operators
    // In rust bit-wise operators are only available for integers.
    let c = 1 | 2;
    println!("1|2 == {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 42.0; // This should be true.

    // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; // true
}

fn main() {
    // Main function
    operators();
}
