pub fn insert<Type>(slice: &mut [Type], value: Type, index: usize) {
    slice[index..].rotate_right(1);
    slice[index] = value;
}

#[derive(Debug)]
pub enum FindError {
    NotFound,
}

pub fn find<Type>(haystack: &[Type], needle: Type) -> Result<usize, FindError>
where
    Type: std::cmp::Ord + std::fmt::Display,
{
    for (index, item) in haystack.iter().enumerate() {
        if *item == needle {
            return Ok(index)
        }
    }
    Err(FindError::NotFound)
}

pub fn main() {
    let mut values = [1, 2, 3];
    insert::<usize>(&mut values, 2, 0);
    println!("{:?}", values);
    let result = match find::<usize>(&values, 2) {
        Ok(result) => result,
        Err(why) => panic!("{:?}", why),
    };
    println!("Index: {}", result);
}