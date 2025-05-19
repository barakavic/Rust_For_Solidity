// Rust Iterators  defines a standard eay to produce a sequence of values one at a time

/*
trait iterator{

    type Item;// Associate type

    fn next(&mut self) -> Option<Self::Item>;
}

.iter() creates an iterator over the collection that yields immutable references to each item(&Item)
#It allows you to read the elements without modifying the original

*/


//.take(n)#Takes the first n items of the original iterator
//.skip(n)#Skips the first n items of an iterator
//.sum()#Calculates the sum of the iterable elements
//.prod()#Product of the iterables

pub fn iterators(){
    let numbers = vec![1,2,3];
    let iterator = numbers.iter();

    for number in iterator{
        println!("The number is :{}", number);
    }


}

//.iter_mut() creates a mutable reference to each item(&mut Item)
//This allows to modify elements of the original collection through the iterator

pub fn iterator_mut(){
    let mut numbers = vec![1,2,3];
    let iterator_mut = numbers.iter_mut();

    for number in iterator_mut{
        *number += 10;// Dereference the mutable reference to modify the value

    }
    println!("Modified numbers: {:?}", numbers);
}

//Iterator adapters: Lazy methods -> Dont do anything till the resulting iterator is consumed
// ..map(closure)
pub fn map(){
    let numbers = vec![1,2,3];
    let doubled: Vec<_> = numbers.iter().map(|x|x*2).collect();
    println!("Doubled numbers{:?}", doubled);
}

