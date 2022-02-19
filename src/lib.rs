use std::mem;

mod vec3;
use vec3::{dot, unit_vector, Vec3};

const CANVAS_SIZE: usize = 1280 * 720 * 4;

type Float = f32;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

fn hit_sphere(center: &Vec3, radius: Float, ray: &Ray) -> Float {
    let oc = ray.origin.vsub(center);
    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&oc, &ray.direction);
    let c = dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn point_at_parameter(ray: &Ray, t: Float) -> Vec3 {
    ray.origin.vadd(&ray.direction.fmul(t))
}

fn color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(
        &Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        ray,
    );

    if t > 0.0 {
        let normal = unit_vector(&point_at_parameter(ray, t).vsub(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }));
        return Vec3 {
            x: normal.x + 1.0,
            y: normal.y + 1.0,
            z: normal.z + 1.0,
        }
        .fmul(0.5);
    }

    let unit_direction = unit_vector(&ray.direction);

    let u = 0.5 * (unit_direction.y + 1.0);

    let one = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let two = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    one.fmul(1.0 - u).vadd(&two.fmul(u))
}

#[no_mangle]
pub extern "C" fn alloc() -> *mut u8 {
    let mut plain_old_memory = Vec::with_capacity(CANVAS_SIZE);
    let ptr = plain_old_memory.as_mut_ptr();

    mem::forget(ptr);

    ptr
}

#[no_mangle]
pub extern "C" fn paint(buf: *mut u8, width: usize, height: usize) {
    let canvas = unsafe { std::slice::from_raw_parts_mut(buf, CANVAS_SIZE) };

    // image
    let aspect_ratio: Float = 16.0 / 9.0;
    let image_width = 1280;
    let image_height = 720; // image_wdith / aspect ratio
    
    // camera
    let viewport_height = 2.0;
    let viewport_width = 3.5; // aspect_ratio * viewport_height
    let focal_length = 1.0;

    let lower_left_corner = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };

    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };

    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };

    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    for j in 0..height {
        for i in 0..width {
            let u: Float = i as Float / width as Float;
            let v: Float = j as Float / height as Float;

            let direction = lower_left_corner
                .vadd(&horizontal.fmul(u))
                .vadd(&vertical.fmul(v));

            let col = color(&Ray {
                origin: origin,
                direction: direction,
            });

            let index = ((height - j - 1) * width * 4) + (i * 4);

            canvas[index] = (255.99 * col.x) as u8;
            canvas[index + 1] = (255.99 * col.y) as u8;
            canvas[index + 2] = (255.99 * col.z) as u8;
            canvas[index + 3] = 255; // alpha channel
        }
    }
}
