// original: 0x00A4F9E0 FireManagerCountRange

/// Counts selected manager entries whose helper-provided two-dimensional
/// position lies within the inclusive radius of the supplied point. The
/// manager has 256 flag bytes at offset zero and entries at offset 0x100 with
/// stride 0xE0; an entry is selected when bit 0 is set and bit 1 is clear.
/// The helper receives the entry in ECX and writes x then y as two f32 words
/// through its stack pointer argument. Point coordinates and radius are f32
/// bit patterns passed under thiscall. NaN comparisons do not count; equality
/// with the radius does count. The counter is returned in EAX.
lf_checker_rt::export!(thiscall, rw_fire_manager_count_range(this: u32, point: u32, radius_bits: u32) -> u32 {
    const FLAG_BYTES: u32 = 0x0000;
    const ENTRY_BASE: u32 = 0x0100;
    const ENTRY_STRIDE: u32 = 0x00E0;
    const ENTRY_COUNT: u32 = 256;
    const OUTPUT_X: usize = 0;
    const OUTPUT_Y: usize = 1;

    #[inline(never)]
    fn range_sub(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left - right
    }

    #[inline(never)]
    fn range_mul(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left * right
    }

    #[inline(never)]
    fn range_add(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left + right
    }

    #[inline(never)]
    fn range_sqrt(value: f32) -> f32 {
        core::hint::black_box(value).sqrt()
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        unsafe { (address as *const f32).read_unaligned() }
    }

    let point_x = unsafe { read_f32(point) };
    let point_y = unsafe { read_f32(point.wrapping_add(4)) };
    let radius = f32::from_bits(radius_bits);
    let mut count = 0u32;
    let mut index = 0u32;

    while index < ENTRY_COUNT {
        let flags = unsafe { read_u8(this.wrapping_add(FLAG_BYTES).wrapping_add(index)) };
        if flags & 1 != 0 && flags & 2 == 0 {
            let entry = this
                .wrapping_add(ENTRY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let mut position = core::mem::MaybeUninit::<[f32; 2]>::uninit();
            let output_pointer = position.as_mut_ptr() as *mut f32 as u32;
            let _ = lf_checker_rt::callee_thiscall!(1, u32, entry, output_pointer);
            let position = unsafe { position.assume_init() };
            let delta_y = range_sub(position[OUTPUT_Y], point_y);
            let delta_x = range_sub(position[OUTPUT_X], point_x);
            let y_squared = range_mul(delta_y, delta_y);
            let x_squared = range_mul(delta_x, delta_x);
            let distance_squared = range_add(y_squared, x_squared);
            let distance = range_sqrt(distance_squared);
            if radius >= distance {
                count = count.wrapping_add(1);
            }
        }
        index += 1;
    }

    count
});
