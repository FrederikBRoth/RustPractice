pub mod area {
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
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
