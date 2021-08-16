use core::cmp::max;
fn max_len(a: &str, b: &str) -> u32{
    max(a.len() as u32, b.len() as u32)
}

fn pad_str_0(input: String, len: u32)-> String{
    let mut padding = String::new();
    for _ in 1..len{
        padding.push('0')
    }
    padding + &input
}

fn add_str(val_1: &String, val_2: &String) -> String{
    let mut result = String::new();
    let (rest, cur_sum) = val_1
            .chars().
            rev().
            zip(val_2.chars().rev()).fold((0, String::new()), |(rest: u32, cur_sum: String), (val_1: u32, val_2: u32)| {
        let val_1: u32 = val_1.into().unwrap();
        let val_2: u32 = val_2.into().unwrap();
        let new_val = add + val_1 + val_2;
        cur_sum.push(new_val % 10);
        let new_rest = new_val / 10;
        
        (new_rest, cur_sum)
    });
    
    cur_sum.push(rest).chars().reverse().collect;
    
}

fn add(x: i32, y: i32) -> i32 {
    unimplemented!()
}


fn main() {
    println!("Hello, world!");
}
