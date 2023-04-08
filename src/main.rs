fn main() {
    let mut nums = vec![300, 1, 5, 3, 2, 4, 234, -22, 4];
    cocktailshakersort(&mut nums);
    println!("{:?}", nums);
}

fn cocktailshakersort(vec: &mut [i32]) {
    let mut end = vec.len() - 1;
    let mut start = 0;

    while start < end {
        for i in start..end {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
            }
        }
        end -= 1;

        for i in (start..end).rev() {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
            }
        }
        start += 1;
    }
}