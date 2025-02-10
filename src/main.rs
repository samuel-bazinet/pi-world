use std::env;
use pub_sub_server;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments, please only provide a single numerical argument");
    }

    if let Ok(top) = args[1].parse::<usize>() {
        println!("Printing the max prime number under {top}");
        println!("The max prime number is {}", get_max_prime(top));
    } else {
        panic!("Please provide a positive integer as your single argument");
    }
    let configs = pub_sub_server::setup_server("configs/configs.json").unwrap();
    let _ = pub_sub_server::start_listening(configs);
}

fn get_max_prime(max: usize) -> usize {
    let mut primes = vec![];
    for i in 2..=max {
        let mut is_prime = true;
        for &prime in &primes {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    primes.last().copied().unwrap()
}