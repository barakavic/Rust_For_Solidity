mod  functional;// Declaring the functional module

fn main(){
    println!("This the main function");



    let a = 23;
    let b = 45;
    let sum = functional::add(a, b);
    println!("The sum of{} and {} is :{}", a, b, sum);
    


}