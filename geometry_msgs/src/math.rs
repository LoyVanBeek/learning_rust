
pub trait Addable {
    fn add(&a: Self, &b: Self) -> Self; //Self is implementor type (see http://rustbyexample.com/trait.html)
}