fn main() {
    //To preface this multithreading adventure, i would like to explain closures. Closures are
    //
    //Closures are effectively anonymous functions that can be passed as an argument or saves a
    //variable. A really cool way to use it is in a combo with an Option. You can do a unwrap or
    //else and pass a closure as an argument. If the unwrap is successful the value of the Some is
    //returned. If not, the closure is executed
    #[derive(Debug)]
    enum Shirt {
        Long,
        Short,
    }

    struct Inventory {
        shirts: Vec<Shirt>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<Shirt>) -> Shirt {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> Shirt {
            let mut num_long = 0;
            let mut num_short = 0;

            for s in &self.shirts {
                match s {
                    Shirt::Long => num_long += 1,
                    Shirt::Short => num_short += 1,
                }
            }

            if num_long > num_short {
                Shirt::Long
            } else {
                Shirt::Short
            }
        }
    }

    let store = Inventory {
        shirts: vec![Shirt::Short, Shirt::Short, Shirt::Long],
    };

    let user = Some(Shirt::Long);

    let chosen = store.giveaway(user);
    println!("{:?}", chosen);

    let uncertain_user = None;
    let chosen2 = store.giveaway(uncertain_user);
    println!("{:?}", chosen2);
}
