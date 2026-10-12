// original: 0x00683EC0 gated_vec3_accumulate
//! Adds a scaled source vector to the target vector only when the scale
//! squared exceeds the configured threshold and the target is not flagged.

use core::hint::black_box;

const THRESHOLD_VA: u32 = 0x00FE8660;

lf_checker_rt::export!(thiscall, rw_fn_00683ec0(target: u32, source: u32) -> () {
    let scale = f32::from_bits(lf_checker_rt::xmm_word(2, 0));
    let threshold = unsafe { lf_checker_rt::global::<f32>(THRESHOLD_VA).read() };
    let scale_squared = black_box(scale) * black_box(scale);
    let flags = unsafe { core::ptr::read_unaligned((target as *const u8).add(4)) };

    if scale_squared > threshold && flags & 0x10 == 0 {
        let target_floats = target as *mut f32;
        let source_floats = source as *const f32;
        let scale_x = black_box(scale);
        let source_x = unsafe { core::ptr::read_unaligned(source_floats.add(4)) };
        let current_x = unsafe { core::ptr::read_unaligned(target_floats.add(4)) };
        let next_x = black_box(black_box(scale_x) * black_box(source_x)) + black_box(current_x);
        unsafe { core::ptr::write_unaligned(target_floats.add(4), next_x) };

        let source_y = unsafe { core::ptr::read_unaligned(source_floats.add(5)) };
        let current_y = unsafe { core::ptr::read_unaligned(target_floats.add(5)) };
        let next_y = black_box(current_y) + black_box(black_box(source_y) * black_box(scale));
        unsafe { core::ptr::write_unaligned(target_floats.add(5), next_y) };

        let source_z = unsafe { core::ptr::read_unaligned(source_floats.add(6)) };
        let current_z = unsafe { core::ptr::read_unaligned(target_floats.add(6)) };
        let next_z = black_box(current_z) + black_box(black_box(source_z) * black_box(scale));
        unsafe { core::ptr::write_unaligned(target_floats.add(6), next_z) };
    }
});
