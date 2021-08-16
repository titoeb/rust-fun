use std::fmt;
use std::ops::Deref;

struct MyBox<T: fmt::Display>(T);

impl<T> MyBox<T>
where
    T: fmt::Display,
{
    fn new(x: T) -> MyBox<T> {
        Self(x)
    }
}
impl<T> Deref for MyBox<T>
where
    T: fmt::Display,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> fmt::Display for MyBox<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl<T> Drop for MyBox<T>
where
    T: fmt::Display,
{
    fn drop(&mut self) {
        println!("Dropping MyBox with data {}", self.0);
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    {
        let a = MyBox::new(x);
        assert_eq!(5, *a);
    }
    println!("... and testing the rest ...");
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}
