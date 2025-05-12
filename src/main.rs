//Outputting in rust
// Type inference is automatic in rust


fn demo_immutable_variable(){

    let age = 30;
    println!("the value of the age is :{} ",age);
    
}


fn demo_mutable_variable(){
    let mut score = 0;
    println!("Initial score: {}", score);


    score = score + 10;
    println!("Score after gaining points: {}", score);

}


fn demo_var_shadowing(){
    let x =5;
    println!("The initial value of x is: {}", x);

    let  x = x + 1;
    println!("The shadowed value of x is: {}", x);

    println!("The value of x is: {}", x);

}


 fn type_inference(){

    let greeting = "Hello".to_string();//Explicitly creating a string
    let _another_greeting: String  = String::from("Hi");// Another way to create a string

    let age : u32 = 30;//Unsigned 32 bit integer
    let temperature: i16 = -5; // Signed 16-bit integer
    let big_number: i64 = 9_000_000_000;// Signed 64-bit integer with separator

    let byte: u8 = 0b1111_0000; // Unsigned 8 bit integer in binary


    println!("Age: {}", age);
    println!("Temperature: {}", temperature);
    println!("Big Number:{}", big_number);
    println!("Byte: {}", byte);
    println!("{}",greeting);
    println!("{}", _another_greeting);
    



   



 }

 fn demo_tuples(){
    println!("---Tuples");
    let person: (&str, i32, bool) = ("Alice", 30, true);


    //Accessing tuple elements using indexing
    let name = person.0;
    let age = person.1;
    let is_active = person.2;


    println!("Name :{}", name);
    println!("Age :{}", age);
    println!("Is active: {}", is_active);

    //Tuple destructuring
    let (person_name, person_age, person_active) = person;
    println!("Name:{}, Age:{}, Active:{}", person_name, person_age, person_active);

    //Unit tuple (Empty tuple)
    let empty_tuple = ();
    println!("Empty tuple: {:?}", empty_tuple);// Outputs empty tuple
    let person : (&str, i32, bool) = ("Alice", 30 , true);
    println!("Person tuple {:?}", person);
 }





 fn demo_arrays(){
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers array: {:?}", numbers);
 }


 // Arithmetic operations between similar types
fn demo_arithmetic_operators(){

    println!("---Integer operations---");
    let a= 30;
    let b = 3;

    println!("a + b = {}", a+b);
    println!("a - b = {}", a-b);
    println!("a * b = {}", a*b);
    println!("a / b = {}", a/b);
    println!("a % b = {}", a%b);

    let c = 5.0;
    let d = 2.0;
    println!("c/d = {}", c/d);
}

fn demo_mixed_type_operations(){
    println!("\n ---Operations between different numeric types ---");
    let integer: i32 = 10;
    let float: f64 = 3.5;

    //This would result in a CT error
    //let result = integer + float;

    //Explicitly convert the integer to a float
    let integer_as_float = integer as f64;
    let result_float = integer_as_float + float;
    println!("integer as float + float :{}",result_float)

}

//Assignment operators
fn demo_assignment_operators(){
    println!("---Assignment Operators---");
    let mut counter = 0;
    println!("INitial counter: {}", counter);

    counter +=1;
    println!("Counter after +=1: {}", counter);

    counter *=5;
    println!("Counter after *=5:{}", counter);
}

//Logical operators
fn demo_logical_operators(){
    println!("\n---Logical operators---");
    let p= true;
    let q = false;

    println!("p && q: {}", p && q);
    println!("p || q; {}", p||q);
    println!("!p: {}", !p);

}

fn demo_bitwise_xor(){
    println!("\n--- Bitwise XOR Operations ---");
    let a: u8 = 0b00110011;
    let b : u8 = 0b01010101;

    let result = a ^ b;

    println!("a: {:08b}({})",a,a);
    println!("b: {:08b}({})", b, b);

    println!("a^b :{:08b}({})", result, result);
}

fn main(){
    println!("---Demo immutable variable---");
    demo_immutable_variable();


    println!("---Demo immutable variable---");
    demo_mutable_variable();

    println!("---Demo Variable shadowing---");
    demo_var_shadowing();

    println!("---Type inferencing---");
    type_inference();

    println!("---Rust Tuples---");
    demo_tuples();

    println!("---Rust Arrays---");
    demo_arrays();

    println!("---same type arithmetic operations---");
    demo_arithmetic_operators();

    println!("---Different type arithmetic operations---");
    demo_mixed_type_operations();

    demo_bitwise_xor();
}

