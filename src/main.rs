use std::{thread, mem};
use std::sync::{mpsc};
use std::time::{Instant};

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
    let mut max: i32 = 0;

    let mut low;
    let mut index = 0;
    let mut iter;

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
            if i > max {
                max = i;
                
                if count < 9 {
                    max_list[count] = i;
                    count += 1;
                }
                else {
                    low = max_list[0];
                    iter = 0;

                    for n in max_list {
                        if low >= n {
                            low = n;
                
                            index = iter;
                        }
                
                        iter += 1;
                    }

                    max_list[index] = max;
                }

            }
        }
    }

    (sum, n_primes, max_list)
}

// nthreads = total number of threads
// thread_number = which thread is this
// n = what number to stop at
fn prime_calc(nthreads: i32, thread_number: i32, n: i32, sender: mpsc::Sender<Vec<i32>>) {
    let mut send: Vec<i32> = vec![]; 

    let mut i = thread_number;
    while i < n {
        if is_prime(i) {
            send.push(i);
        }

        if i == thread_number {
            i = i + 1 + nthreads + thread_number;
        }
        else {
            i += 2 * nthreads; 
        }
    }

    sender.send(send).unwrap();
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
    fn prime_check() {
        assert!(is_prime(13), "is_prime rejected a prime");
        assert!(!is_prime(10), "is_prime accepted a non-prime");
    }
}