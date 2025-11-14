// Three popular sorting algorithms implemented in Rust

/// Bubble Sort - Simple comparison-based sorting algorithm
/// Time Complexity: O(n²) average and worst case, O(n) best case
/// Space Complexity: O(1)
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // If no swaps occurred, array is already sorted
        if !swapped {
            break;
        }
    }
}

/// Quick Sort - Efficient divide-and-conquer sorting algorithm
/// Time Complexity: O(n log n) average case, O(n²) worst case
/// Space Complexity: O(log n) due to recursion
pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quick_sort_helper(arr, 0, len - 1);
}

fn quick_sort_helper(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        
        if pivot_index > 0 {
            quick_sort_helper(arr, low, pivot_index - 1);
        }
        quick_sort_helper(arr, pivot_index + 1, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    
    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

/// Merge Sort - Efficient divide-and-conquer sorting algorithm
/// Time Complexity: O(n log n) in all cases
/// Space Complexity: O(n)
pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    
    merge_sort(&mut left);
    merge_sort(&mut right);
    
    merge(arr, &left, &right);
}

fn merge(arr: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn print_array(name: &str, arr: &[i32]) {
    println!("{}: {:?}", name, arr);
}

fn main() {
    println!("=== Three Popular Sorting Algorithms in Rust ===\n");
    
    // Test Bubble Sort
    println!("1. Bubble Sort");
    let mut arr1 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("   Original array: {:?}", arr1);
    bubble_sort(&mut arr1);
    println!("   Sorted array:   {:?}\n", arr1);
    
    // Test Quick Sort
    println!("2. Quick Sort");
    let mut arr2 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("   Original array: {:?}", arr2);
    quick_sort(&mut arr2);
    println!("   Sorted array:   {:?}\n", arr2);
    
    // Test Merge Sort
    println!("3. Merge Sort");
    let mut arr3 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("   Original array: {:?}", arr3);
    merge_sort(&mut arr3);
    println!("   Sorted array:   {:?}\n", arr3);
    
    // Test with edge cases
    println!("=== Edge Cases ===\n");
    
    // Empty array
    let mut empty: Vec<i32> = vec![];
    bubble_sort(&mut empty);
    print_array("Empty array (bubble sort)", &empty);
    
    // Single element
    let mut single = vec![42];
    quick_sort(&mut single);
    print_array("Single element (quick sort)", &single);
    
    // Already sorted
    let mut sorted = vec![1, 2, 3, 4, 5];
    merge_sort(&mut sorted);
    print_array("Already sorted (merge sort)", &sorted);
    
    // Reverse sorted
    let mut reverse = vec![5, 4, 3, 2, 1];
    bubble_sort(&mut reverse);
    print_array("Reverse sorted (bubble sort)", &reverse);
    
    // Duplicates
    let mut duplicates = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    quick_sort(&mut duplicates);
    print_array("With duplicates (quick sort)", &duplicates);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
        
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
        
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr1 = vec![42];
        bubble_sort(&mut arr1);
        assert_eq!(arr1, vec![42]);
        
        let mut arr2 = vec![42];
        quick_sort(&mut arr2);
        assert_eq!(arr2, vec![42]);
        
        let mut arr3 = vec![42];
        merge_sort(&mut arr3);
        assert_eq!(arr3, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr1 = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr1);
        assert_eq!(arr1, vec![1, 2, 3, 4, 5]);
        
        let mut arr2 = vec![1, 2, 3, 4, 5];
        quick_sort(&mut arr2);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]);
        
        let mut arr3 = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr3);
        assert_eq!(arr3, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr1 = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr1);
        assert_eq!(arr1, vec![1, 2, 3, 4, 5]);
        
        let mut arr2 = vec![5, 4, 3, 2, 1];
        quick_sort(&mut arr2);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]);
        
        let mut arr3 = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr3);
        assert_eq!(arr3, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr1 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        bubble_sort(&mut arr1);
        assert_eq!(arr1, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
        
        let mut arr2 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        quick_sort(&mut arr2);
        assert_eq!(arr2, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
        
        let mut arr3 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        merge_sort(&mut arr3);
        assert_eq!(arr3, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}
