use std::env;

pub mod body;
pub mod nbodies;
pub mod octtree;
pub mod vec3d;

///Searches the environment for arguments, sends those arguments to be processed by parsenum, pushes the processed arguments into a vector, then returns the vector. Failure occurs if environment (not including the first argument) doesn't have exactly three arguments.
pub fn parsearg() -> Vec<u32> {
    let args = env::args().skip(1);
    if args.len() != 2 {
        error("usage: n_body_sim <1 (naive) or 2 (with tree)> <number of bodies to simulate>");
    }

    let mut num = Vec::new();
    for arg in args {
        num.push(parsenum(&arg));
    }

    if num[0] > 2 || num[0] < 1 {
        error("error: first argument must be 1 or 2");
    }
    num
}

///Takes a string, parses it, and unwraps it to an unsigned 32bit integer. Fails and calls error() if resultant parse is not an integer, or exceeds 2^32. Converts u32 to u64 and returns.
fn parsenum(s: &str) -> u32 {
    let n: u32 = s
        .parse()
        .unwrap_or_else(|_| error("error: please provide only integers < 2^32"));

    n
}

///Upon finding an error, this function is called to display a (provided) custom message to the user, then cleanly exit the program. During tests, it instead panics following the message.
fn error(msg: &str) -> ! {
    eprintln!("{:?}", msg);
    #[cfg(test)]
    panic!("error");
    #[cfg(not(test))]
    std::process::exit(1);
}

#[cfg(test)]
mod parsenum_tests {
    use super::*;
    use rand::Rng;

    ///Poor-man's fuzzer - randomly generates and tests 10,000 inputs which are expected to be good inputs for the parsenum function
    #[test]
    fn goodnum() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let num = rng.gen::<u32>();
            assert!(parsenum(&num.to_string()) == num);
        }
    }

    ///Test which hands parsenum a string representation of a float, which should cause it to panic.
    #[test]
    #[should_panic]
    fn floatnum() {
        parsenum("4.0");
    }

    ///Test that hands parsenum a string; should result in panic
    #[test]
    #[should_panic]
    fn strnum() {
        parsenum("imastring");
    }

    ///Test that hands parsenum a string representation of a negative number; should result in panic
    #[test]
    #[should_panic]
    fn negnum() {
        parsenum("-30");
    }

    ///Test that hands parsenum a string representing a number that far exceeds 2^32; should panic.
    #[test]
    #[should_panic]
    fn bignum() {
        parsenum("99999999999999999999999999999999999999999999999999999999999");
    }
}
