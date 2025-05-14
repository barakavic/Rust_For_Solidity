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

}
