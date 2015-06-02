fn main() {
    //"Write a program that prints the numbers from 1 to 100. 
    //But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”. 
    //For numbers which are multiples of both three and five print “FizzBuzz”."
    println!("Hello FizzBuzz!");

    for i in 1..101 {
        if i % 5 == 0 && i % 3 == 0 { 
            println!("{} -> FizzBuzz!", i);
        }
        else if i % 3 == 0 { 
            println!("{} -> Fizz!", i);
        }
        else if i % 5 == 0 { 
            println!("{} -> Buzz!", i);
        }
        else { 
            println!("{}", i);
        }
        // None => { break }
    }
}
