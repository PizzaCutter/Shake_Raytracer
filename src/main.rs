extern crate image;
use image::{ DynamicImage, GenericImageView, GenericImage, ImageBuffer, RgbImage, Pixel, Rgba };

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
    pub radius: f32,
    pub color: Color,
}

pub struct Scene 
{
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub sphere: Sphere,
}

pub struct Ray{
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray{
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray{
        //TODO: get own to_radians and tan function
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();

        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let sensor_x = ((((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0) * fov_adjustment;

        let mut direction = Vector3 { x: sensor_x, y: sensor_y, z: -1.0 };
        direction.Normalize();
        
        Ray {
            origin: Vector3::Zero(),
            direction: direction,
        }
    }
}

pub trait Intersectable 
{
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere
{
    fn intersect(&self, ray: &Ray) -> bool
    {
        //Create a line segment between the ray origin and the center of the sphere
        let l:vector::Vector3 = self.center - ray.origin;

        //Use 1 as a hypotenuse and find the length of the adjacent side
        let adj2 = Vector3::Dot(l, ray.direction);
        
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than)  (l.length() * l.length*()) - (adj2 * adj2)
        let d2 = Vector3::Dot(l, l) - (adj2 * adj2);

        return d2 < (self.radius * self.radius);
    }
}

fn main() {
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
    
    dynamic_image.save("Render_01.jpg");


    let mut vec = Vector3{x: 10.0, y:30.0, z: 50.0 };
    println!("Magnitude: {}", vec.Magnitude());
    println!("Thing: {}", smath::smath::sqrt_fast(3500.0));
    vec.Normalize();
    println!("x: {}, y: {}, z: {}", vec.x, vec.y, vec.z);
}

pub fn render(scene: &Scene) -> DynamicImage 
{
    let mut image:DynamicImage = DynamicImage::new_rgb8(scene.width, scene.height);

    let black_pixel = Rgba::from_channels(0,0,0,0);
    let color_pixel = Rgba::from_channels(255,0,0,255);

    for x in 0..scene.width 
    {
        for y in 0..scene.height
        {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray)
            {
               image.put_pixel(x, y, color_pixel);
            } else
            {
                image.put_pixel(x,y, black_pixel);
            }
        }
    }


    return image; 
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

    dynamic_image.save("Render_01.jpg");
}
