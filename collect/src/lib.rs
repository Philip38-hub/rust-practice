pub fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    
    while n > 1 {
        let mut new_n = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i; // track the last swap index
            }
        }
        n = new_n; // reduce the range for the next pass
    }
}

