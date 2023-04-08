fn main() {
    let mut nums = vec![300, 1, 5, 3, 2, 4, 234, -22, 4];
    cocktailshakersort(&mut nums);
    println!("{:?}", nums);
}

fn cocktailshakersort(vec: &mut [i32]) {
    let mut end = vec.len() - 1;
    let mut start = 0;
    let mut swapped = true;

    while start < end && swapped {
        swapped = false;
        for i in start..end {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        end -= 1;
        swapped = false;
        for i in (start..end).rev() {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
        start += 1;
    }
}
