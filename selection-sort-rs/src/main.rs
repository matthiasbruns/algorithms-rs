fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    selection_sort(&mut arr);
    println!("{:?}", arr);
    bench_million_iterations();
}

fn selection_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    // find the minimum element in the unsorted array
    // and swap it with the first element

    for i in 1..arr.len() {
        for j in i..arr.len() {
            // check if the current element is less than our minimum element
            if arr[j] < arr[i - 1] {
                // swap the elements
                arr.swap(i - 1, j);
            }
        }
    }
}

fn bench_million_iterations() {
    // start timer
    let start = std::time::Instant::now();
    for _ in 0..1000000 {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        selection_sort(&mut arr);
    }
    // stop timer
    let duration = start.elapsed();
    println!(
        "\nTime elapsed in bench_million_iterations() is: {:?}",
        duration
    );
}
