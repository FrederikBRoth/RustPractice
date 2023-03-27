mod figure_area;

pub use crate::figure_area::area;
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Vector3D(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Peter"),
        email: String::from("Peter@gmail.com"),
        sign_in_count: 1,
    };

    println!("User1: Username = {}", user1.username);

    //Creating a struct from another struct can be done easily using struct update syntax.
    let user2 = User {
        email: String::from("another@gmail.com"),
        ..user1
    };
    //This is done to replace
    //let user2 = user {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@gmail.com"),
    //    sign_in_count: user1.sign_in_count,
    //}

    //This is fine, but due to how heap allocated objects are moved from on variable to the other,
    //since username is a String it is moved from user1 to user2. This causes user1 to be invalid,
    //inaccessable and deleted

    println!("User1: Username = {}", user2.username);

    //Tuple structs to make different types, i.e Matrix or vector
    let vec = Vector3D(10, 10, 10);

    //Example program structs;
    area::calc_area(20, 30);

    let rect1 = area::Rectangle {
        width: 30,
        height: 30,
    };
    let rect2 = area::Rectangle {
        width: 20,
        height: 10,
    };
    let rect3 = area::Rectangle {
        width: 40,
        height: 20,
    };
    println!("Area of rect = {}", area::calc_area_struct(&rect1));
    println!("Rectangle: {:?}", rect1);

    println!("Area of Rectangle: {}", rect1.area());

    //Multiple parameters in impl functions

    println!("Does rect1 contain rect2: {}", rect1.can_contain(rect2));
    println!("Does rect1 contain rect3: {}", rect1.can_contain(rect3));

    //You can make associated functions that are used as constructors. Can be evoked such as how
    //static functions are evoked in other languages

    let sq = area::Rectangle::square(20);
    println!("Size of square, {:?}", sq);
}

fn build_user(email: String, username: String) -> User {
    //Only works due to the parameters in the function has the same name as the struct fields
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
