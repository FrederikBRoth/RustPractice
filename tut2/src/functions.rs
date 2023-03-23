pub mod functions_module {
    pub fn test() {
        println!("yoooooo");
    }

    pub fn parameter(x: i32) {
        println!("This is x: {x}");
    }

    pub fn statements() {
        let y = {
            let x = 3;
            //When a line has a semicolon its a statement, while not it is an expression
            x + 1
        };

        println!("Nested variable: {y}");
    }
    pub fn return_variable() -> i32 {
        10
    }
}
