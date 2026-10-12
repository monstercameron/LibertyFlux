// original: 0x00B2CDA0 garage_elevation_scaled
//! Follows the object's linked position vector, computes the horizontal
//! radius, sends height and radius to the scripted angle routine, scales
//! the returned single precision angle, and returns it through ST0.

use core::hint::black_box;

const SCALE_VA: u32 = 0x00E98888;

lf_checker_rt::export!(thiscall, rw_fn_00b2cda0(this: u32) -> f32 {
    let object = this as *const u32;
    let nested = unsafe { core::ptr::read_unaligned(object.add(20)) } as *const u32;
    let vector = unsafe { core::ptr::read_unaligned(nested.add(8)) } as *const u32;
    let x = f32::from_bits(unsafe { core::ptr::read_unaligned(vector.add(4)) });
    let y = f32::from_bits(unsafe { core::ptr::read_unaligned(vector.add(5)) });
    let height = f32::from_bits(unsafe { core::ptr::read_unaligned(vector.add(6)) });

    let x_squared = black_box(x) * black_box(x);
    let y_squared = black_box(y) * black_box(y);
    let radius_squared = black_box(x_squared) + black_box(y_squared);
    let radius = black_box(radius_squared).sqrt();
    let height64 = f64::from(height).to_bits();
    let radius64 = f64::from(radius).to_bits();
    let angle_bits = lf_checker_rt::callee_cdecl!(
        1,
        u64,
        height64 as u32,
        (height64 >> 32) as u32,
        radius64 as u32,
        (radius64 >> 32) as u32,
    );
    let angle = f64::from_bits(angle_bits) as f32;
    let scale = unsafe { lf_checker_rt::global::<f32>(SCALE_VA).read() };
    black_box(angle) * black_box(scale)
});
