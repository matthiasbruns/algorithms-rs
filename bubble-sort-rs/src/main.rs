/// This is how you would implement a bubble sort algorithm in Rust
///
/// Definition:
/// Bubble sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order.
/// The pass through the list is repeated until the list is sorted.
///
/// Time Complexity:
/// O(n^2)
fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    // print array before sorting
    for i in 0..arr.len() {
        print!("{} ", arr[i]);
    }
    println!();

    bubble_sort(&mut arr);

    // print array after sorting
    for i in 0..arr.len() {
        print!("{} ", arr[i]);
    }
    println!();

    bench_million_iterations();
}

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn bench_million_iterations() {
    // start timer
    let start = std::time::Instant::now();
    for _ in 0..1000000 {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
    }
    // stop timer
    let duration = start.elapsed();
    println!(
        "\nTime elapsed in bench_million_iterations() is: {:?}",
        duration
    );
}
