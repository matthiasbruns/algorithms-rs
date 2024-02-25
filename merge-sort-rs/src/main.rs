/// merge_sort arr:
/// s_left = merge_sort(left half of arr);
/// s_right = merge_sort(right half of arr);
/// return merge(s_left, s_right);

fn main() {
    let arr = [64, 34, 25, 12, 22, 11, 90];
    let vec = merge_sort(arr.to_vec());
    println!("{:?}", vec);
    bench_million_iterations();
}

fn merge_sort(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec;
    }

    let half = vec.len() / 2;
    let left = merge_sort(vec[0..half].to_vec());
    let right = merge_sort(vec[half..].to_vec());

    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut l = 0;
    let mut r = 0;
    let mut merged: Vec<i32> = Vec::new();

    while l < left.len() && r < right.len() {
        if left[l] < right[r] {
            merged.push(left[l]);
            l += 1;
        } else {
            merged.push(right[r]);
            r += 1;
        }
    }

    // if sizes are not equal
    while l < left.len() {
        merged.push(left[l]);
        l += 1;
    }

    while r < right.len() {
        merged.push(right[r]);
        r += 1;
    }

    merged
}

fn bench_million_iterations() {
    // start timer
    let start = std::time::Instant::now();
    for _ in 0..1000000 {
        let arr = [64, 34, 25, 12, 22, 11, 90];
        merge_sort(arr.to_vec());
    }
    // stop timer
    let duration = start.elapsed();
    println!(
        "\nTime elapsed in bench_million_iterations() is: {:?}",
        duration
    );
}
