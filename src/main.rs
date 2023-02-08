use rand::Rng;

fn smallest_num_array(arr: [i64; 10]) -> i64 {
    let mut i = 0;
    let mut smallest_num: i64 = arr[0];
    loop {
        if i == 10 {
            return smallest_num;
        } else {
            if arr[i] < smallest_num {
                smallest_num = arr[i];
            }

        };
        i += 1;
    }
}

fn main() {
    let a = rand::thread_rng().gen_range(1..=100);
    let b = rand::thread_rng().gen_range(1..=100);
    let c = rand::thread_rng().gen_range(1..=100);
    let d = rand::thread_rng().gen_range(1..=100);
    let e = rand::thread_rng().gen_range(1..=100);
    let f = rand::thread_rng().gen_range(1..=100);
    let g = rand::thread_rng().gen_range(1..=100);
    let h = rand::thread_rng().gen_range(1..=100);
    let i = rand::thread_rng().gen_range(1..=100);
    let j = rand::thread_rng().gen_range(1..=100);

    println!("The smalles number is {}.", smallest_num_array([a,b,c,d,e,f,g,h,i,j]));
}
