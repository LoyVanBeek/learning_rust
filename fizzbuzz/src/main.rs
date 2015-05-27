fn main() {
    //"Write a program that prints the numbers from 1 to 100. 
    //But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”. 
    //For numbers which are multiples of both three and five print “FizzBuzz”."
    println!("Hello FizzBuzz!");

    let mut range = 1..100;

    loop {
        match range.next() {
            Some(i) if i % 5 == 0 && i % 3 == 0 =>  println!("{} -> FizzBuzz!", i),
            Some(i) if i % 3 == 0 =>  println!("{} -> Fizz!", i),
            Some(i) if i % 5 == 0 =>  println!("{} -> Buzz!", i),
            Some(i) =>  println!("{}", i),
            None => { break }
        }
    }
}
