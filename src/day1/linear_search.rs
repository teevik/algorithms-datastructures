pub fn linear_search<T: PartialEq>(haystack: &[T], needle: &T) -> bool {
    for element in haystack {
        if element == needle {
            return true;
        }
    }

    false
}

#[test]
fn test() {
    let foo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

    assert_eq!(linear_search(&foo, &69), true);
    assert_eq!(linear_search(&foo, &1336), false);
    assert_eq!(linear_search(&foo, &69420), true);
    assert_eq!(linear_search(&foo, &69421), false);
    assert_eq!(linear_search(&foo, &1), true);
    assert_eq!(linear_search(&foo, &0), false);
} 