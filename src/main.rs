fn main() {
    println!("Hello, world!");
    let mut nums = vec![1, 5, 3, 2, 4, -1];
    bubblesort(&mut nums);
    println!("{:?}", nums);
}

fn bubblesort(vec: &mut [i32]) {
    let n = vec.len();
    let mut newn = 1;
    while newn >= 1 {
        newn = 0;
        for i in 0..n - 1 {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                newn = i;
            }
        }
    }
}
