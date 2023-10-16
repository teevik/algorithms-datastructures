use std::cmp::Ordering;

pub fn binary_search<T: Ord>(haystack: &[T], needle: &T) -> bool {
    let mut left = 0;
    let mut right = haystack.len();

    while left < right {
        let middle = (left + right) / 2;

        match haystack[middle].cmp(needle) {
            Ordering::Equal => return true,
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle,
        }
    }

    false
}

#[test]
fn test() {
    let foo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    assert_eq!(binary_search(&foo, &69), true);
    assert_eq!(binary_search(&foo, &1336), false);
    assert_eq!(binary_search(&foo, &69420), true);
    assert_eq!(binary_search(&foo, &69421), false);
    assert_eq!(binary_search(&foo, &1), true);
    assert_eq!(binary_search(&foo, &0), false);
}
