use err_handling::{access_array_element, safe_divide};

mod err_handling;

fn main(){
    println!("--Recoverable Errors with result---");

    //Successful Division
    match err_handling::safe_divide(28, 4) {

        Ok(value) => println!("\n 28/4 {}",value),
        Err(e) => println!("Error during division {}", e),
        
    }

    match err_handling::safe_divide(5, 0) {
    Ok(value) =>println!("5/0 = {}", value),
    Err(e) => println!("Error During division: {}",e),
        
    }

    println!("\n--- Unrecoverable Errors With panic! ---- ");
    let my_array = [10,20,30];

    //This Call will work
    let val = access_array_element(&my_array, 1);
    println!("Value at index 1 :{}", val);

    //This call wont
    //access_array_element(&my_array, 5)
}