pub mod area {
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        pub fn can_contain(&self, other: Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        pub fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    pub fn calc_area(width: u32, height: u32) -> u32 {
        width * height
    }

    pub fn calc_area_tuple(dimension: (u32, u32)) -> u32 {
        dimension.0 * dimension.1
    }

    pub fn calc_area_struct(rectangle: &Rectangle) -> u32 {
        rectangle.height * rectangle.width
    }
}
