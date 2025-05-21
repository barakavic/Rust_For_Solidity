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

    // another example

    let y = String::from("hello");// s comes into scope

    takes_ownership(y);// s value moves into the function
                        //Its no longer valid here



    let x = 5;// x comes into scope

    makes_copy(x); // x moves into function
                    // i32 is a copy , its okay to use x after


    

}//s, then y, then x goes out of scope


fn takes_ownership(sth_string: String){//Some string comes into scope

    println!("{}", sth_string);


}//Sth string goes out of scope and drop is called. Backing memory is freed

fn makes_copy(some_integer: i32){// some integer comes into scope

    println!("{}",some_integer);


}// Here, some integer goes into scope nothing major happens