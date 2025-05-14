//Pure functions

pub fn add(x: i32, y: i32) -> i32{
    x+y//The function is pure same input gives the same output

    //The -> specifies the return datatype


    //pub is the access modifier for allowing for public access
}

//Higher order functions
pub fn apply(value: i32, func: fn(i32)->i32)->i32{
    println!("Inside apply({}, function", value);
    func(value)
}

pub fn increment(n: i32) -> i32{
    println!("Inside increment({})",n);
    n+1
}


pub fn decrement(n: i32) -> i32{
    println!("Inside decrement({})",n);
    n-1
}

//Example of a function used with apply
pub fn square(n: i32)-> i32{
    println!("Inside Square ({})",n);
    n*n
}