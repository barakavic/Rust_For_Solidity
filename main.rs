use std::collections::btree_map::Values;



mod closures;

mod  functional;// Declaring the functional module

fn main(){
    println!("This the main function");



    let a = 23;
    let b = 45;
    let sum = functional::add(a, b);
    println!("The sum of {} and {} is :{}", a, b, sum);




    println!("\n ---Higher Order Function---");
    let num = 10;
    let incremented = functional::apply(num, functional::increment);
    println!("{} incremented is {}", num, incremented);

    let decremented = functional::apply(num, functional::decrement);
    println!("\n{} decremented is: {}",num, decremented);

    let squared = functional::apply(num,functional::square);
    println!("{} squared is: {}", num, squared);


    let result1 = functional::divide(10.0, 2.0);
    match result1{
        Some(value) => println!("Result of 10.0 / 2.0 : {}",value),
        None => println!("cannot divide by zero!"),

    }


    let result2 = functional::divide(5.0, 0.0);
    match result2 {
        Some(value) => println!("Result of 5.0 / 0.0: {}", value),
        None => println!("Cannot divide by zero!"),
        
    }

    
        let multiplier = 3;
        let multiply_by_multiplier = |x|x * multiplier;// closure definition

        let result = multiply_by_multiplier(5);
        println!("Result: {}", result);//Output: 15


        let numbers = vec![1,2,3];
        let multiplied: Vec<_> = numbers.iter().map(|n|n*multiplier).collect();
        println!("Multiplied {:?}",multiplied);

        let applied_result = functional::apply_closure(10, |y| y*multiplier);
        println!("Applied result: {}", applied_result);
    



        //Closures

        let mut counter = 0;
        
        let mut increment_counter = ||{//Note the 'mut' here on the closure itself

        counter +=1;

        println!("Counter Incremented to: {}", counter);


        };

        increment_counter();
        increment_counter();

        println!("Final Counter value: {}", counter);

}


