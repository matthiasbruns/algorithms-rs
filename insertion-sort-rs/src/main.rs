fn main() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
    bench_million_iterations();
}

fn insertion_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    // starting at one for iteration to compare with previous
    for i in 1..arr.len() {
        // current val to compare the sorted array with
        let key = arr[i];
        // current index to decrement and compare with
        let mut j = i;
        // we are not at the start of the array and the previous value is greater than the current
        while j > 0 && arr[j - 1] > key {
            // swap
            arr[j] = arr[j - 1];
            arr[j - 1] = key;
            // decrement
            j -= 1;
        }
    }
}

fn bench_million_iterations() {
    // start timer
    let start = std::time::Instant::now();
    for _ in 0..1000000 {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        insertion_sort(&mut arr);
    }
    // stop timer
    let duration = start.elapsed();
    println!(
        "\nTime elapsed in bench_million_iterations() is: {:?}",
        duration
    );
}
