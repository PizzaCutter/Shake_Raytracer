extern crate image;
use image::{ DynamicImage, GenericImageView, GenericImage, ImageBuffer, RgbImage };

pub mod vector;
use vector::Vector3;

use smath;

pub struct Color {
        pub red: i32,
        pub green: i32,
        pub blue: i32,
}

pub struct Sphere 
{
    pub center: Vector3,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene 
{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

pub struct Ray{
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray{
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray{
        Ray {
            origin: Vector3::Zero(),
            direction: Vector3::Zero(),
        }
    }
}

fn main() {
    println!("Hello, world!");

    
    // let mut x:i32 = 42;
    // println!("{:#b}", x);
    // x = -42;

    // println!("{:#b}", x);


    // let mask:i32 = x >> 31;
    // x = x ^ mask;
    // x = x - mask;

    // println!("{:#b}", x);


//     let mut value:f32 = 16.3511315;
//     println!("{:#b}", value.to_bits());
//    // value = -16.3511315;
//     //println!("{:#b}", value.to_bits());

//     let mut thing:i32 = value.to_bits() as i32;
//     let mask:i32 = thing >> 31;
//     thing = thing ^ mask;
//     thing = thing - mask;

//     //println!("{:#b}", thing);

    
//     let mut bits:u32 = value.to_bits() as u32;
//     println!("{:#b}", bits);
    
//     bits = bits & 0b11111111111111000000000000000000;

//     println!("{:#b}", bits);

    let mut x:f32 = 2.1234567890;
    //println!("{:.32}", x);
    //println!("{:.32}", smath::smath::trunc(x));

    x = 3.40282347e+38_f32;

    // println!("{:.32}", x);
    // println!("{:.32}", smath::smath::trunc(x));

    x = 3.40282347e+38_f32 - 0.123456789123456_f32;

    //println!("{:.32}", x);
    //println!("{:.32}", smath::smath::trunc(x));


    let temp = -1.5;
    println!("{:.32}", temp);
    println!("{:.32}", smath::smath::absf(temp));
}

pub fn render(scene: &Scene) -> DynamicImage 
{
    return DynamicImage::new_rgb8(scene.width, scene.height);
}

#[test]
fn test_can_render_scene() 
{
    let scene = Scene
    {
        width : 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: vector::Vector3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 102,
                green: 255,
                blue: 102,
            },
        },
    };

    let dynamic_image: DynamicImage = render(&scene);
    
    assert_eq!(scene.width, dynamic_image.dimensions().0);
    assert_eq!(scene.height, dynamic_image.dimensions().1);
}
