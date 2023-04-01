use std::ops::Deref;
fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //You can use Box as a reference also

    let boxy = Box::from(x);
    assert_eq!(5, *boxy);

    //We can make our own smart pointer akin to Box<T>
    //However it needs to be said that a homemade box type needs to implement the Deref trait to be
    //able to be derefenced
    struct MyBox<T>(T);
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
}
