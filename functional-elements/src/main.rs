fn map_for_each(fruits: Vec<String>, fun: fn(&String) -> usize) -> Vec<usize>{
    fruits.iter().map(|fruit| {fun(fruit)}).collect()
}
fn main() {
    let fruits = vec![
        String::from("Orange"),
        String::from("Apple"),
        String::from("Banana"),
        String::from("Grape"),
    ];

    let lengths = map_for_each(fruits, |fruit: &String| -> usize {
        fruit.len()
    });

    println!("{:?}", lengths);
}
