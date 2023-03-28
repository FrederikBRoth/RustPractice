#[cfg(not(test))]
pub const OS_RELEASE_FILE_PATH: &str = "/etc/os-release";
#[cfg(test)]
pub const OS_RELEASE_FILE_PATH: &str = "/tmp/fake-etc-os-release-file";

use std::option::Option;
fn main() {
    #[derive(Debug)]
    enum IpAddrVersion {
        V4,
        V6,
    }
    #[derive(Debug)]
    struct IpAddr {
        version: IpAddrVersion,
        address: String,
    }
    //This is the tut for enums and patterns
    let home = IpAddr {
        version: IpAddrVersion::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        version: IpAddrVersion::V6,
        address: String::from("::1"),
    };

    println!("Ip: {:?}", home);

    //You could alternatively just add the information to the enums instead of creating a struct to
    //make the association more concise

    enum IpAddrConcise {
        V4(String),
        V6(String),
    }

    let home = IpAddrConcise::V4(String::from("127.0.0.1"));

    //Enums can even have differing content in their "constructors"

    enum IpAddrDiffering {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrDiffering::V4(127, 0, 0, 1);

    //You can even use structs as values inside the enums. Pretty litty
    //
    //Lets look at another example

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    //this could also be done as 4 different structs, however that would not take advantage of how
    //an enum is under the same overlaying type.

    //You can also, just like Structs, impl functions to enums in the exact same way that structs
    //are done

    //The Option Enum

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
