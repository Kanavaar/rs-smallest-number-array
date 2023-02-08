//const TEST: u32 = 60 * 60 *3;
//
//fn is_even(num: i64) -> bool {
//    match num % 2 {
//        0 => true,
//        _ => false,
//    }
//}
//
//fn tuples() {
//    let tup: (i32, f64, &str) = (69, 42.0, "Ich liebe mich");
//    let (tup_num_one, tup_float, a_string) = tup;
//    let a = tup.0;
//    let b = tup.1;
//    let c = tup.2;
//    println!("{}{}{}{}{}{}", tup_num_one, tup_float, a_string, a, b, c);
//}
//
//fn arrays() {
//    let a: [i32; 5] = [1, 2, 3, 4, 5];
//    let array_pos_0 = a[0];
//    println!("{array_pos_0}");
//}
//
//

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
    //println!("{TEST}");
    //let answer: isize = "42".trim().parse().expect("Not a number");
    //println!("{answer}");
    //for i in 0..=16 {
    //    if is_even(i) == true {
    //        println!("{i}");
    //    }
    //}
    //tuples();
    //arrays();

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
