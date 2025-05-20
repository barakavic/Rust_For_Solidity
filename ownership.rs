/*


Rust Ownership 
-Dictates how values are managed in memory 
-Ensures memory is always cleaned up
-No need for garbage collectors #Am talking to you C

Each value in rust has a variable that is its owner
There can only be one owner at a time


//Ownership in functions

*/

pub fn take_ownership(some_string: String){
    println!("Recieved: {}",some_string);

}//Some string goes out of scope and the data is dropped


pub fn gives_ownership() ->String{
    let some_string = String::from("hello world");
    some_string //Ownership moves out of function

}
fn main(){
    let my_s = String::from("unique string");//my_s owsns "unique string"
    take_ownership(my_s);// Ownership moves from my_s to some_string
    //my_s is no longer valid

    let s = gives_ownership();
    println!("s: {}",s);

}//s goes out of scope