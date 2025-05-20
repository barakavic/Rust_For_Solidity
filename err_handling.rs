/*

There are 2 types of errors in rusty Recoverable and unrecoverable

//Recoverable Errors 
Solved with the Result enum

fundamental way to use result is with match
enum Result<T. E>{

Ok(T),#T: Success Type
Err(E),#E: Error Type

In the main you ensure there a match statement



}

//Unrecoverable Errors
Use panic!()


*/

pub fn safe_divide(numerator: i32, denominator: i32) -> Result<i32, String>{

    if denominator == 0{
        //If an error occurs return a String Error

        Err("Cannot divide by zero!".to_string())

    }
    else {
        //If Successful return Ok containing integer unit
        Ok(numerator/denominator)
    }
}

pub fn access_array_element(arr: &[i32], index: usize) -> i32{
    if index >= arr.len(){


        panic!("index Out of Bounds: Array has length {} but index was {}",arr.len(), index);
    }
    arr[index]
}