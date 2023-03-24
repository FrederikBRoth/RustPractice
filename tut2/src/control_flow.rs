pub mod control_flow_module {
    pub fn ifelse() {
        let number = 5;

        if number < 5 {
            println!("Yoo that shits correct");
        } else {
            println!("Yoooo that shit sucks");
        }
        //Always be explicit with your statements and
    }

    pub fn ifelseif(x: u32) {
        let number = x;

        if number % 4 == 0 {
            println!("Number is divisble by 4");
        } else if number % 2 == 0 {
            println!("Number is divisble by 2");
        } else {
            println!("Number is not divible by 4 or 2");
        }
    }
    pub fn iflet() {
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("{number}");
    }

    pub fn loops() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {result}")
    }

    pub fn looplabels() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }

    pub fn whileloop() {
        let mut number = 3;

        while number != 0 {
            println!("{number}");
            number -= 1;
        }

        println!("ITS TIME");
    }

    pub fn forloop() {
        let a = [20, 10, 30, 40];
        let mut index = 0;

        for element in a {
            println!("{element}");
        }
    }
}
