// original: 0x00DA3870 CPedAttractor::vf4

/// Samples a two-dimensional attractor offset into the caller's output word.
/// The first stack argument selects the pair at object offsets `+0x70/+0x74`
/// when nonzero and `+0x80/+0x84` when zero; the second is the output pointer.
/// An inactive object leaves the output untouched. The sampled noise value is
/// written first, then a second per-object sample is added for a nonzero
/// selector. It consumes both stack arguments using the thiscall convention.
/// The original helper has no semantic return value; the contract ignores
/// its residual EAX value while checking memory, calls and x87 state.
lf_checker_rt::export!(thiscall, rw_00da3870(self_ptr: u32, selector: u32, output_ptr: u32) -> u32 {
    const ACTIVE_FLAG: u32 = 0x10;
    const PRIMARY_X: u32 = 0x70;
    const PRIMARY_Y: u32 = 0x74;
    const ALTERNATE_X: u32 = 0x80;
    const ALTERNATE_Y: u32 = 0x84;
    const NOISE_AXIS_COUNT: f32 = 0.0;

    if unsafe { (self_ptr.wrapping_add(ACTIVE_FLAG) as *const u32).read_unaligned() } == 0 {
        return output_ptr;
    }

    let (noise_x, noise_y) = if selector == 0 {
        unsafe {
            (
                f32::from_bits((self_ptr.wrapping_add(ALTERNATE_X) as *const u32).read_unaligned()),
                f32::from_bits((self_ptr.wrapping_add(ALTERNATE_Y) as *const u32).read_unaligned()),
            )
        }
    } else {
        unsafe {
            (
                f32::from_bits((self_ptr.wrapping_add(PRIMARY_X) as *const u32).read_unaligned()),
                f32::from_bits((self_ptr.wrapping_add(PRIMARY_Y) as *const u32).read_unaligned()),
            )
        }
    };

    let sampled = lf_checker_rt::callee_cdecl!(
        1,
        f32,
        noise_x.to_bits(),
        noise_y.to_bits(),
        NOISE_AXIS_COUNT.to_bits(),
        NOISE_AXIS_COUNT.to_bits()
    );
    unsafe { (output_ptr as *mut f32).write_unaligned(sampled) };

    if selector != 0 {
        let additional = lf_checker_rt::callee_thiscall!(2, f32, self_ptr);
        let current = unsafe { (output_ptr as *const f32).read_unaligned() };
        let sum = core::hint::black_box(additional) + core::hint::black_box(current);
        unsafe { (output_ptr as *mut f32).write_unaligned(sum) };
    }

    output_ptr
});
