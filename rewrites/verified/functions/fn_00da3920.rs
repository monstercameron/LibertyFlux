// original: 0x00DA3920 CPedAttractor::vf3

/// Builds a four-word position result from an attractor's center and spread.
/// An inactive object leaves the output untouched. The signed integer amount
/// is converted to `f32`, multiplied by the object's spread scale, and then
/// applied to the three spread coordinates with the original negative-one
/// factor. The center is added afterward, the fourth word is copied unchanged,
/// and nonzero amounts add two scripted per-object jitter samples to X and Y.
/// The routine uses two stack arguments and has no semantic return value.
lf_checker_rt::export!(thiscall, rw_00da3920(self_ptr: u32, amount: u32, output_ptr: u32) -> () {
    const ACTIVE_FLAG: u32 = 0x10;
    const SPREAD_SCALE: u32 = 0x40;
    const CENTER_X: u32 = 0x60;
    const CENTER_Y: u32 = 0x64;
    const CENTER_Z: u32 = 0x68;
    const SPREAD_X: u32 = 0x70;
    const SPREAD_Y: u32 = 0x74;
    const SPREAD_Z: u32 = 0x78;
    const FOURTH_WORD: u32 = 0x7c;
    const NEGATIVE_ONE: f32 = f32::from_bits(0xbf80_0000);

    if unsafe { (self_ptr.wrapping_add(ACTIVE_FLAG) as *const u32).read_unaligned() } == 0 {
        return;
    }

    let amount_as_float = (amount as i32) as f32;
    let scale = unsafe {
        f32::from_bits((self_ptr.wrapping_add(SPREAD_SCALE) as *const u32).read_unaligned())
    };
    let scale = core::hint::black_box(amount_as_float) * core::hint::black_box(scale);

    let spread_x = unsafe {
        f32::from_bits((self_ptr.wrapping_add(SPREAD_X) as *const u32).read_unaligned())
    };
    let spread_y = unsafe {
        f32::from_bits((self_ptr.wrapping_add(SPREAD_Y) as *const u32).read_unaligned())
    };
    let spread_z = unsafe {
        f32::from_bits((self_ptr.wrapping_add(SPREAD_Z) as *const u32).read_unaligned())
    };

    let scaled_x = core::hint::black_box(spread_x) * core::hint::black_box(scale);
    let scaled_y = core::hint::black_box(spread_y) * core::hint::black_box(scale);
    let scaled_z = core::hint::black_box(spread_z) * core::hint::black_box(scale);
    let scaled_x = core::hint::black_box(scaled_x) * core::hint::black_box(NEGATIVE_ONE);
    let scaled_y = core::hint::black_box(scaled_y) * core::hint::black_box(NEGATIVE_ONE);
    let scaled_z = core::hint::black_box(scaled_z) * core::hint::black_box(NEGATIVE_ONE);

    let center_x = unsafe {
        f32::from_bits((self_ptr.wrapping_add(CENTER_X) as *const u32).read_unaligned())
    };
    let center_y = unsafe {
        f32::from_bits((self_ptr.wrapping_add(CENTER_Y) as *const u32).read_unaligned())
    };
    let center_z = unsafe {
        f32::from_bits((self_ptr.wrapping_add(CENTER_Z) as *const u32).read_unaligned())
    };
    let position_x = core::hint::black_box(center_x) + core::hint::black_box(scaled_x);
    let position_y = core::hint::black_box(center_y) + core::hint::black_box(scaled_y);
    let position_z = core::hint::black_box(center_z) + core::hint::black_box(scaled_z);

    unsafe {
        (output_ptr as *mut f32).write_unaligned(position_x);
        (output_ptr.wrapping_add(4) as *mut f32).write_unaligned(position_y);
        (output_ptr.wrapping_add(8) as *mut f32).write_unaligned(position_z);
        (output_ptr.wrapping_add(12) as *mut u32)
            .write_unaligned((self_ptr.wrapping_add(FOURTH_WORD) as *const u32).read_unaligned());
    }

    if amount != 0 {
        let jitter_x = lf_checker_rt::callee_thiscall!(1, f32, self_ptr);
        let position_x = unsafe { (output_ptr as *const f32).read_unaligned() };
        let position_x = core::hint::black_box(jitter_x) + core::hint::black_box(position_x);
        unsafe { (output_ptr as *mut f32).write_unaligned(position_x) };

        let jitter_y = lf_checker_rt::callee_thiscall!(2, f32, self_ptr);
        let position_y = unsafe { (output_ptr.wrapping_add(4) as *const f32).read_unaligned() };
        let position_y = core::hint::black_box(jitter_y) + core::hint::black_box(position_y);
        unsafe { (output_ptr.wrapping_add(4) as *mut f32).write_unaligned(position_y) };
    }
});
