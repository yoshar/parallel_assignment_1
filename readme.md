# Parallel Computing Assignment 1
This is a multithreaded application which sums, counts and lists the maximum 10 primes below a limit, in the scope of the assignment this is 10<sup>8</sup>. This was written in Spring 2023 for the Parallel Computing class at UCF.

This is a fairly fast iterative implementation, however if you are looking at this in a future class I would direct you to implementing a sieve.

## To Compile and Run
cargo run

Test cases, set at values for 10<sup>8</sup>
cargo test

## Reasoning and Design
I use a mpsc pattern to assign each thread a starting number and the limit, those threads then check the primality of each number within that range and according to their offset. The threads then return a vector the found primes to the receiver. The receiver preforms all counting and adding operations needed. 

The two important parts are the offset and the primality check I used. For the offset each thread is given its starting number then it will increment by 1 + n_threads + thread_id (ex. thread 1 will start on 1 and then move to 11), this ensures that all threads will now be on an odd number and offset from each other. For the rest of the iteration each thread will increment by 16 (for 8 threads) to skip over what each thread is doing or will do and to skip the next odd number. This effectively reduces n by half as even numbers, excluding 2, cannot be prime. Doing the indices this way ensures that each thread is doing close to the same amount of work, without micro-managing them which would slow them down.

The primality test I used is a deterministic version the miller-rabin test. I talk a bit about it in the comments above the method. To use it on any number n it would need to be randomized, however mine is deterministic up to 2147483647, which is more than 10<sup>8</sup>. Within the scope of the project the implementation of the miller-rabin test that I used is deterministic.

## Experimentation Evaluation
There are two separate scalers to the time taken of this algorithm, the multithreading implementation, and the primality test.

I started with a naive approach that takes every prime in a range and checks it. Then each thread will get a range and an offset to use to check them. Without excluding odd numbers 4 threads will always be checking even numbers which are trivial. This led me to trying a thread pool to have the master thread give each number, but this ended up being slow as there is too much overhead where the threads are passing information. I went back to the mpsc and made my indices smarter, this improved run time significantly as I was now only running n/2 instead of n.

After this improvement I went from ~80s to ~21s

Next I used the primal library and saw a huge performance increase using their primality test over mine. I wasn't sure if I was allowed to use that library and it seemed like it was within the scope of the project to code my own so that is what I did.

Primal performance boost ~21s to ~5s

I read in the primal documentation that they used a deterministic version of the miller-rabin test and researched that further and found some info on it and redid a C implementation into rust. This ended up being faster than the one in the primal library.

With all of this I was down from ~80s to ~2.5s

## Current Runtime
System
i7-9700k @ stock (3.6gHz - 4.9gHz boost) 8c/8t
2.38s

## Refrences
[https://en.wikipedia.org/wiki/Primality_test#C,_C++,_C#_&_D](https://en.wikipedia.org/wiki/Primality_test#C,_C++,_C#_&_D)

https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test#Deterministic_up_to_341,550,071,728,321

https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Mathematical_concepts

https://docs.rs/primal/latest/primal/fn.is_prime.html

https://doc.rust-lang.org/book/ch16-02-message-passing.html