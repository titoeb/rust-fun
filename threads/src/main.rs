use std::sync::Arc;
use std::thread;

fn my_thread_fun(thread_num: usize, my_data: Arc<Vec<i32>>) -> i32 {
    let my_data_point = my_data[thread_num];
    println!(
        "I am thread {} and my data is {}",
        thread_num, my_data_point
    );
    my_data_point
}
fn main() {
    //let large_data: Vec<i32> = (1..=100).rev().collect();
    let large_data = Arc::new((1..=100).rev().collect());
    let mut handles = vec![];

    for thread_num in 0..10 {
        let my_large_data: Arc<Vec<i32>> = Arc::clone(&large_data);
        let handle = thread::spawn(move || my_thread_fun(thread_num, my_large_data));
        handles.push(handle);
    }

    for handle in handles {
        println!("Return value was: {}", handle.join().unwrap());
    }
}
