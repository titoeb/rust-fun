fn is_even(x: i32) -> bool {
    return x & 2 == 0;
}

fn greater_then(limit: i32) -> impl Fn(i32) -> bool {
    return move |y| y > limit;
}

fn main() {
    // sum of all even squares < 500
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else if is_even(i) {
            sum += i * i;
        }
    }
    println!("sum: {}", sum);

    // Alternative implementation not using a distinc function:
    let above_limit = greater_then(limit);
    let is_even = |x: i32| -> bool {
        return x & 2 == 0;
    };
    for i in 0.. {
        let isq = i * i;
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    let sum2 = (0..)
        .map(|x| {
            return x * x;
        })
        .take_while(|&x| {
            return x < limit;
        })
        .filter(|x: &i32| {
            return is_even(*x);
        })
        .fold(0, |sum, x| {
            return sum + x;
        });

    println!("{}", sum2);
}
