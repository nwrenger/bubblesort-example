fn main() {
    let mut nums = vec![1, 5, 3, 2, 4, -1];
    cocktailshakersort(&mut nums);
    println!("{:?}", nums);
}

fn cocktailshakersort(vec: &mut [i32]) {
    let len = vec.len() - 1;
    let mut swapped = true;
    
    while swapped {
        swapped = false;
        for i in 0..len {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        for i in len..0 {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
