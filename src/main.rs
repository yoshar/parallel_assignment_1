use std::{thread, mem};
use std::sync::{mpsc};
use std::time::{Instant};
// use primal::is_prime;
// use rand::Rng;

fn main() {
    let start = Instant::now();

    let (sum, n_primes, list_primes) = list_primes(100000000, 8);

    let duration = start.elapsed();


    println!("Sum: {:?}", sum);
    println!("Number of Primes: {:?}", n_primes);
    println!("Top 10 Primes: {:?}", list_primes);
    println!("Time elapsed: {:?}", duration);
}

fn list_primes(n: i32, nthreads: i32) -> (i64, i32, [i32; 10]) {
    let mut sum: i64 = 0;
    let mut n_primes: i32 = 0;
    
    let mut max_list = [0; 10];
    let mut count: usize = 0;
    let mut min: i32 = 0;

    let (sender, reciever) = mpsc::channel();

    for i in 0..nthreads {
        let sender_n = sender.clone();
        thread::spawn(move || {
            prime_calc(nthreads, i as i32, n, sender_n);

        });
    }

    mem::drop(sender);

    for recieved in reciever {
        for i in recieved {
            n_primes += 1;
            sum += i as i64;

            if count < 9 {
                max_list[count] = i;
                count += 1;
            }

            if i > min {
                max_list.sort();
                max_list[0] = i;
                min = minimum(max_list[0] , max_list[1]);
            }
        }
    }

    max_list.sort();

    (sum, n_primes, max_list)
}

// nthreads = total number of threads
// thread_number = which thread is this
// n = what number to stop at
fn prime_calc(nthreads: i32, thread_number: i32, n: i32, sender: mpsc::Sender<Vec<i32>>) {
    let mut send: Vec<i32> = vec![]; 

    let mut i = thread_number;
    while i < n {
        if determ_miller_test(i) {
            send.push(i);
        }

        // sets offset based on thread number
        if i == thread_number {
            i = i + 1 + nthreads + thread_number;
        }
        else { 
            //incremetns by 16 to skip over what other threads have done and the even number
            i += 2 * nthreads; 
        }
    }
    
    //sends back a vector of all primes found by this thread
    sender.send(send).unwrap();
}

//helper for sorting the final list of 10 primes
fn minimum(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }

    b
}

// slow is_prime function on wikipedia
// fn custom_is_prime(n: i32) -> bool {
//     if n == 2 || n == 3 {
//         return true;
//     }
    
//     if n % 2 == 0 || n % 3 == 0 || n <= 1{
//         return false;
//     }
    
//     for i in (5..((n as f64).sqrt() as i32 + 1)).step_by(6) {
//         if n % i == 0 || n % (i + 2) == 0 {
//             return false;
//         }
//     }

//     true
// }

// power function so that the 
fn power (x_arg: i32, y_arg: i32, p_arg: i32) -> u64 {
    let mut res: u64 = 1;
    let mut x = x_arg as u64;
    let mut y = y_arg as u64;
    let p = p_arg as u64;

    x = x % p;
    while y > 0 {
        if y % 2 == 1 {
            res = (res * x) % p;
        }

        y = y / 2;
        x = (x * x) % p;
    }

    res
}

fn witness(n: i32, arg_s: i32, d: i32, a: i32) -> bool {
    let mut x = power(a, d, n);
    let mut s = arg_s as u64;
    let mut y: u64 = 0;

    while s > 0{
        y = (x * x) % (n as u64);
        if y == 1 && x != 1 && x != ((n - 1) as u64){
            return false;
        }

        x = y;

        s = s - 1;
    }

    if y != 1 {
        return false;
    }

    true
}

fn determ_miller_test(n: i32) -> bool {
    if (n % 2 == 0 && n != 2) || n < 2 || (n % 3 == 0 && n != 3) {
        return false;
    }

    if n <= 3 {
        return true;
    }

    let mut d = n / 2;
    let mut s = 1;

    while d % 2 == 0 {
        d = d / 2;
        s += 1;
    }

    if n < 1373653 {
        return witness(n, s, d, 2) && witness(n, s, d, 3);
    }

    if n < 9080191 {
        return witness(n, s, d, 31) && witness(n, s, d, 73);
    }

    if n < 2147483647 {
        return witness(n, s, d, 2) && witness(n, s, d, 7) && witness(n, s, d, 61);
    }

    witness(n, s, d, 2) && witness(n, s, d, 3) && witness(n, s, d, 5) && witness(n, s, d, 7) && witness(n, s, d, 11) && witness(n, s, d, 13) && witness(n, s, d, 17)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_primes() {
        let (_sum, n_primes, _list) = list_primes(100000000, 8);
        assert_eq!(n_primes, 5761455, "Incorrect number of primes");
    }

    #[test]
    fn sum_of_primes() {
        let (sum, _n_primes, _list) = list_primes(100000000, 8);
        assert_eq!(sum, 279209790387276, "Incorrect sum of primes");
    }

    #[test]
    fn list_of_primes() {
        let (_sum, _n_primes, list) = list_primes(100000000, 8);
        let test_list = [99999787, 99999821, 99999827, 99999839, 99999847, 
            99999931, 99999941, 99999959, 99999971, 99999989];
        for i in 0..10 {
            assert!(list[i] == test_list[i], "Incorrect list of max 10 primes");
        }
    }

    // #[test]
    // fn prime_check() {
    //     assert!(custom_is_prime(13), "is_prime rejected a prime");
    //     assert!(!custom_is_prime(10), "is_prime accepted a non-prime");
    // }

    #[test]
    fn miller_test() {
        assert!(determ_miller_test(13), "miller rejected a prime");
        assert!(!determ_miller_test(10), "miller accepted a composite");
    }
}