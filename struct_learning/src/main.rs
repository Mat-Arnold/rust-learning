



// fn main() 
// {
//     let width_1 = 30;
//     let height_1 = 50;

//     println!
//     (
//         "The area of the rectangel is {} square pixels.",
//         area(width_1,height_1)
//     );
// }

// fn area(width: u32, height: u32) -> u32
// {
//     return width * height;
// }

// fn main() 
// {
//     let dimensions_px = (30, 50);

//     println!
//     (
//         "The area of the rectangel is {} square pixels.",
//         area(dimensions_px)
//     );
// }

// fn area(dimensions_px: (u32, u32)) -> u32
// {
//     return dimensions_px.0 * dimensions_px.1;
// }

#[derive(Debug)]
struct Rectangle 
{
    width: u32,
    height: u32,

}
impl Rectangle
{
    fn area(&self) -> u32
    {
        return self.width * self.height;
    }
}

enum IpAddressKind
{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() 
{
    let rectangle_1 = Rectangle
    {
        width: 30,
        height: 50,
    };

    let four = IpAddressKind::V4(127, 0, 0, 1);
    let six = IpAddressKind::V6(String::from("::1"));


    println!
    (
        "The area of the rectangel is {} square pixels.",
        rectangle_1.area()
    );

    println!("rectangle_1 is {:?}",rectangle_1)
}

// fn area(rectangle: &Rectangle) -> u32
// {
//     return rectangle.width * rectangle.height;
// }

fn route(ip_kind: &IpAddressKind)
{

}