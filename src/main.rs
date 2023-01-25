use std::path::is_separator;
use std::thread::JoinHandle;
use std::{thread, mem};
use std::sync::{mpsc, RwLock, Arc, Mutex};
use std::time::{Instant};
use threadpool::ThreadPool;

fn main() {
    let start = Instant::now();

    let (sum, n_primes) = list_primes(100000000, 8);

    let duration = start.elapsed();

    println!("Sum: {:?}", sum);
    println!("Number of Primes: {:?}", n_primes);
    println!("Time elapsed: {:?}", duration);
}

fn list_primes(n: i32, nthreads: usize) -> (i64, i32) {
    let mut sum: i64 = 0;
    let mut n_primes: i32 = 0;
    let counter = Arc::new(Mutex::new(5));
    // let mut max_primes = [(1 as i32); 10];
    // let mut max: i32;

    let (sender, reciever) = mpsc::channel();

    let mut handles: Vec<JoinHandle<()>> = vec![];

    // let pool = ThreadPool::new(8);

    for i in 0..nthreads {
        let sender_n = sender.clone();
        thread::spawn(move || {
            prime_calc(nthreads, i as i32, n, sender_n);

        });
    }
    // let (tx, rx) = channel();
    // for i in 0..n { 
    //     let tx = tx.clone();

    //     pool.execute(move || {
    //         if is_prime(i) {
    //             tx.send(i).expect("Pool messed up");
    //         }
            
    //     })
    // }

    mem::drop(sender);
    
    for recieved in reciever {
        sum += recieved as i64;
        n_primes += 1;
        // if recieved > max {
        //     max = recieved;

            
    //     // }
    }

    (sum, n_primes)
}

// nthreads = total number of threads
// thread_number = which thread is this
// n = what number to stop at
fn prime_calc(nthreads: usize, thread_number: i32, n: i32, sender: mpsc::Sender<i32>) {
    // for i in (1..n).skip(1).step_by(nthreads * 2) {
    //     if is_prime(i + thread_number) {
    //         sender.send(i + thread_number).unwrap();
    //     }
    // }
    let mut i = thread_number;
    while i < n {
        if is_prime(i) {
            sender.send(i).unwrap();
        }

        if(i == thread_number) {
            i = i + 1 + nthreads as i32 + thread_number;
        }
        else {
            i += 2 * nthreads as i32; 
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    
    if n % 2 == 0 || n % 3 == 0 || n <= 1{
        return false;
    }
    
    for i in (5..((n as f64).sqrt() as i32 + 1)).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}

// fn prime_calc_sieve(nthreads: usize, n: i32, sender: mpsc::Sender<i32>) {
//     let sqrt_n = (n as f64).sqrt().ceil() as i32;

//     //possibly dynamicall determine this list
//     let starting_primes = Arc::new();

//     let mut threads = Vec::new();

//     // let offset = (n - sqrt_n) / nthreads;

//     //play with this value for optomization
//     let seg_size = 100000;
    
//     for i in 0..nthreads {
//         let starting_primes = starting_primes.clone();

//     }
// }

// fn seg_sieve(starting_primes: Arc<RwLock<Vec<i32>>>, low: i32, high: i32, seg_size: i32) {
//     let mut retval = Vec::new();

//     // let mut low = low;
//     // let mut high = high + seg_size;

//     // let starting_primes = starting_primes.read().unwrap();
//     let sieving_primes = [3, 5, 7, 11, 13];


//     for min in (min_check..max_check).step_by(seg_size) {
//         let s = low / min * min;
//         if  s < low {
//             s += min;
//         }
//         for i in (s..max).step_by(min) {
//             retval[i - low] = 0;
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_primes() {
        let (_sum, n_primes) = list_primes(100000000, 8);
        assert_eq!(n_primes, 5761455, "Incorrect number of primes");
    }

    #[test]
    fn sum_of_primes() {
        let (sum, _n_primes) = list_primes(100000000, 8);
        assert_eq!(sum, 279209790387276, "Incorrect sum of primes");
    }

    #[test]
    fn prime_check() {
        assert!(is_prime(13), "is_prime rejected a prime");
        assert!(!is_prime(10), "is_prime accepted a non-prime");
    }
}

