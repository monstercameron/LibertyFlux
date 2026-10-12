// original: 0x005E4690 effect_track_update_floats
/// Add the first scalar input to the object's running line height, optionally
/// flush a completed line, update the secondary height from the second scalar
/// input, and append the selected values to the object's indexed record arrays.
///
/// XMM1 supplies the first scalar and XMM2 supplies the second. A flush occurs
/// only when the updated line height exceeds the object's limit and the old
/// height is nonzero. The flush helper is observed as a scripted thiscall.
/// The cap used after a flush is loaded from the image.
///
/// Original: `0x005E4690` (thiscall, object in ECX; scalar inputs in XMM1/XMM2).
lf_checker_rt::export!(thiscall, rw_005e4690(this: u32) -> () {
    unsafe {
        let object = this as *mut u8;
        let incoming_height = f32::from_bits(lf_checker_rt::xmm_word(1, 0));
        let incoming_secondary = f32::from_bits(lf_checker_rt::xmm_word(2, 0));

        let old_height_ptr = object.add(0x28) as *mut f32;
        let old_height = old_height_ptr.read_unaligned();
        let new_height = add32(old_height, incoming_height);
        let height_limit = (object.add(0x0c) as *const f32).read_unaligned();

        let did_flush = !jbe32(new_height, height_limit) && old_height != 0.0;
        let mut result_register = incoming_height;
        let stored_height = if did_flush {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
            let refreshed_limit = (object.add(0x0c) as *const f32).read_unaligned();
            let cap = lf_checker_rt::global::<f32>(0x00FE_8BF4).read_unaligned();
            if !jbe32(refreshed_limit, cap) {
                let recomputed_height = add32(incoming_height, old_height);
                result_register = recomputed_height;
                recomputed_height
            } else if !jbe32(incoming_height, old_height) {
                incoming_height
            } else {
                old_height
            }
        } else {
            new_height
        };
        old_height_ptr.write_unaligned(stored_height);

        let secondary_limit_ptr = object.add(0x24) as *mut f32;
        let secondary_limit = secondary_limit_ptr.read_unaligned();
        if !jbe32(incoming_secondary, secondary_limit) && !jbe32(result_register, 0.0) {
            secondary_limit_ptr.write_unaligned(incoming_secondary);
        }

        let index_ptr = object.add(0x238) as *mut u32;
        let index = index_ptr.read_unaligned();
        (object.add(0x0b8) as *mut u32).add(index as usize).write_unaligned(0);
        (object.add(0x138) as *mut u32).add(index as usize).write_unaligned(0);
        (object.add(0x1b8) as *mut f32)
            .add(index as usize)
            .write_unaligned(result_register);
        let carried_value = (object.add(0xb0) as *const u32).read_unaligned();
        (object.add(0x30) as *mut u32).add(index as usize).write_unaligned(carried_value);
        index_ptr.write_unaligned(index.wrapping_add(1));
        (object.add(0xb0) as *mut u32).write_unaligned(0);
    }
});

#[inline(never)]
fn add32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn jbe32(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan() || left <= right
}
