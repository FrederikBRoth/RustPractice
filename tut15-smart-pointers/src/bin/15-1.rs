fn main() {
    //We can use the Box object to store normally stack allocated variables to the heap instead. This
    //has the pros of storing large data on the heap, while only using a small pointer on the stack
    //reducing the time it takes to copy stuff around
    let b = Box::new(5);
    println!("b = {}", b);

    //This is kinda weird though since storing just a normal value on the heap is not helpfull
    //While not used commonly, you can use Boxes to store recursive data types due to the Box
    //constant sizing, while a recursive data type potentially could be infinite

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
}
