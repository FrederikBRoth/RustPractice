fn main() {
    //You can specify exactly behavior when the cleanup of the object occures. This is the same as
    //when in C++ you have to specify destructors with delete operators in order to free memory
    //
    //Rust does all that for you, but you might still want to specify said above behavior anyways
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("This smart pointer is dropped: {}", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("Stuff"),
    };

    //You can also drop memory early using 'drop(T)'
    drop(c);
    let b = CustomSmartPointer {
        data: String::from("More stuff"),
    };
}
