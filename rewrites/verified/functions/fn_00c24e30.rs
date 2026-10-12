// original: 0x00C24E30 CCamInterp::update
/// Resets the interpolation value, advances the interpolation state, and
/// combines compatible camera modes using the frame delta. The method
/// reports success in its low-byte return value.
const CALL_TEARDOWN: u32 = 1;
const CALL_RATE: u32 = 2;
const CALL_BLEND: u32 = 3;
const CALL_MODE_A: u32 = 4;
const CALL_MODE_B: u32 = 5;
const ONE_FILE_VA: u32 = 0x00FE_88E8;
const DELTA_FILE_VA: u32 = 0x0103_C0EC;
const STATE_OFFSET: usize = 0x148;
const VALUE_OFFSET: usize = 0x74;

#[inline(always)]
fn read_word(base: *const u8, offset: usize) -> u32 {
    // SAFETY: the checker supplies the same valid receiver memory to both sides.
    unsafe { base.add(offset).cast::<u32>().read_unaligned() }
}

#[inline(always)]
fn write_word(base: *mut u8, offset: usize, value: u32) {
    // SAFETY: the receiver owns the state fields written by the original method.
    unsafe { base.add(offset).cast::<u32>().write_unaligned(value) };
}

#[inline(always)]
fn read_float(base: *const u8, offset: usize) -> f32 {
    // SAFETY: the checker supplies the same valid receiver memory to both sides.
    unsafe { base.add(offset).cast::<f32>().read_unaligned() }
}

#[inline(always)]
fn read_image_float(file_va: u32) -> f32 {
    // SAFETY: the file address is a read-only floating-point operand used by this method.
    unsafe { lf_checker_rt::global::<f32>(file_va).read_unaligned() }
}

#[inline(always)]
fn call_blend(this: *mut u8, value: f32) {
    let _ = lf_checker_rt::callee_thiscall!(CALL_BLEND, u32, this as u32, value.to_bits());
}

lf_checker_rt::export!(thiscall, fn_00c24e30(this: *mut u8) -> u8 {
    write_word(this, VALUE_OFFSET, 0);
    let state = read_word(this, STATE_OFFSET) as i32;

    if state < 0 {
        if state == -1 {
            let _ = lf_checker_rt::callee_thiscall!(CALL_TEARDOWN, u32, this as u32);
            write_word(this, STATE_OFFSET, u32::MAX - 1);
        }
        return 1;
    }

    let rate = lf_checker_rt::callee_thiscall!(CALL_RATE, f32, this as u32);
    let one = read_image_float(ONE_FILE_VA);
    if rate < 0.0 {
        call_blend(this, 0.0);
    } else if rate < one {
        call_blend(this, rate);
    } else {
        call_blend(this, one);
        write_word(this, STATE_OFFSET, u32::MAX);
    }

    let object_a = read_word(this, 0x14C);
    let object_b = read_word(this, 0x150);
    let mode_a = lf_checker_rt::callee_thiscall!(CALL_MODE_A, u32, object_a);
    let mode_b = lf_checker_rt::callee_thiscall!(CALL_MODE_B, u32, object_b);
    let modes_match = (mode_a == 1 && mode_b == 9) || (mode_a == 9 && mode_b == 1);

    if modes_match {
        let previous = read_float(this.cast_const(), VALUE_OFFSET);
        let delta = read_image_float(DELTA_FILE_VA);
        let previous = core::hint::black_box(previous);
        let delta = core::hint::black_box(delta);
        let advanced = previous + delta;
        let next = if advanced < 0.0 {
            0.0
        } else {
            let upper_bound = read_image_float(ONE_FILE_VA);
            if advanced > upper_bound { upper_bound } else { advanced }
        };
        write_word(this, VALUE_OFFSET, next.to_bits());
    }

    1
});
