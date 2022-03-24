pub struct LinearSearch;

#[derive(Debug)]
pub enum SearchError {
    NotFound,
}

impl LinearSearch {
    pub fn search<Type>(haystack: &[Type], needle: Type) -> Result<usize,SearchError> 
        where
        Type: Ord,
    {
        for i in 0..haystack.len() {
            if haystack[i] == needle {
                return Ok(i)
            }
        }
        Err(SearchError::NotFound)
    }
}

#[test]
fn search_works() {
    let things = vec![4, 3, 2, 1];
    let result = match LinearSearch::search::<usize>(&things, 1) {
        Ok(result) => result,
        Err(_) => unreachable!(),
    };
    assert_eq!(result, 3);
}

