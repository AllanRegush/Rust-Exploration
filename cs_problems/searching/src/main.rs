mod linear_search;
mod binary_search;
use crate::linear_search::LinearSearch;
fn main() {
    let things = vec![4, 2, 5, 3, 1];
    let result = match LinearSearch::search::<usize>(&things, 1) {
        Ok(result) => result,
        Err(err) => unreachable!(),
    };
    println!("Hello, world! {}", result);
}
