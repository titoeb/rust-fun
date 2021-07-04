fn funny_conditional() {
    let temp: i8 = 31;

    // A beautiful conditional.
    if temp > 20 {
        println!("It is really hot!");
    } else if temp < 10 {
        println!("It is really cold!");
    } else {
        println!("It's just ok outside");
    }
    // You can also assign variables with an if-statement.
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("{}", day);

    // We could do the same thing also within the println:
    println!("{}", if temp > 20 { "sunny" } else { "cloudy" });

    // We could also nest the `if`-statements in there:
    println!(
        "{}",
        if temp > 20 {
            if temp > 30 {
                "extremely hot"
            } else {
                "sunny"
            }
        } else {
            "cloudy"
        }
    );
}
fn funny_loops() {
    // The good, old while loop.
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("x={}", x);
    }

    // If we want to iterate infinitely use the `loop`-statement.
    let mut y = 1;
    loop
    // while true
    {
        y *= 2;
        println!("y={}", y);
        if y >= 1 << 10 {
            // 1 << 10 = 2^10
            break;
        }
    }

    // Let's have a look at the `if`-statement:
    for x in 1..11 {
        println!("x={}", x)
    }
    // So interestingly, this for-loop iterates over a range
    // opposed to the classical `for(int i = 1; i<11; i++)`
    // that is well-known from other languages.
    // A typical requirment for a for-loop is not only to know
    // the values you currently iterate over (imagine you iterate)
    // over an array, but also the index that you are currently in.
    // You could do that in the following way.
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statements() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unkown",
        _ => "invalid",
    };
    println!("Country with code {} is {}", country_code, country);
}

fn main() {
    funny_conditional();
    funny_loops();
    match_statements();
}
