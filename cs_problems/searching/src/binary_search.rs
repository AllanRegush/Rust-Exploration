use crate::linear_search::SearchError;

pub struct BinarySearch;

impl BinarySearch {
    pub fn search<Type>(haystack: &[Type], needle: Type) -> Result<usize, SearchError>
        where
        Type: Ord
    {
        let mut low = 0;
        let mut high = haystack.len();
        
        loop {
            if low >= high {
                break;
            }
            
            let mid = (low + high) / 2;
            let guess = &haystack[mid];
            
            if *guess == needle {
                return Ok(mid)
            }
            
            if *guess > needle {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
            
        }
        Err(SearchError::NotFound)
    }
}

#[test]
fn binary_search_works() {
    let values = vec![1, 2, 3, 4, 5, 7];
    let result = match BinarySearch::search::<usize>(&values, 2) {
        Ok(result) => result,
        _ => unreachable!(),
    };
    assert_eq!(1, result);
}

