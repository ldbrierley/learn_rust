// Make a prgram that prints out all number starting form 0 to infinity.

fn main() {
    for number in (1..) {
        if number % 3 == 0 {
            if number % 5 == 0 {
                println!("fizzbuzz")
            } else {
                println!("fizz")
            }
        } else if number % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", number);
        }
    }
}
