use std::{thread, mem};
use std::sync::mpsc;

fn main() {
    let (sum, n_primes) = list_primes(100, 1);
    println!("Sum: {:?}", sum);
    println!("Number of Primes: {:?}", n_primes);
}

fn list_primes(n: i32, nthreads: usize) -> (i64, i32) {
    let mut sum: i64 = 0;
    let mut n_primes: i32 = 0;
    // let mut max_primes = [(1 as i32); 10];
    // let mut max: i32;

    let (sender, reciever) = mpsc::channel();

    for i in 0..nthreads {
        let sender_n = sender.clone();
        thread::spawn(move || {
            prime_calc(nthreads, i as i32, n, sender_n);
        });
    }

    mem::drop(sender);
    
    for recieved in reciever {
        sum += recieved as i64;
        n_primes += 1;
        // if recieved > max {
        //     max = recieved;

            
        // }
    }

    (sum, n_primes)

}

// nthreads = total number of threads
// thread_number = which thread is this
// n = what number to stop at
fn prime_calc(nthreads: usize, thread_number: i32, n: i32, sender: mpsc::Sender<i32>) {
    for i in (thread_number..n).step_by(nthreads) {
        if is_prime(i) {
            sender.send(i).unwrap();
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    
    if n % 2 == 0 || n % 3 == 0 || n == 0 || n == 1{
        return false;
    }
    
    for i in 4..((n as f64).sqrt() as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

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

