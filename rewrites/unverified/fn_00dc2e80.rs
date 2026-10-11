// original: 0x00DC2E80 task_profile_from_owner

/// Build the task profile output from the owner's signed profile index.
///
/// The stack argument points to an owner record whose 16-bit index is at
/// offset 0x2e. That index selects a pointer from a global table; byte
/// 0xef of the selected record is clamped to the interval from zero to the
/// first read-only bound. The function subtracts that value from a second
/// read-only bound and stores five scaled floats at output offsets 0, 4, 8,
/// 0x10, and 0x18. The five scale factors are writable globals and are
/// supplied by the contract as varied input state.
lf_checker_rt::export!(thiscall, rw_00DC2E80(output: u32, owner: u32) -> u32 {
    unsafe {
        const OWNER_PROFILE_INDEX: u32 = 0x2e;
        const PROFILE_POINTER_TABLE: u32 = 0x0129_5cd8;
        const PROFILE_VALUE: u32 = 0xef;
        const CLAMP_MAX: u32 = 0x00fe_8ad8;
        const UPPER_BOUND: u32 = 0x00fe_8ae0;
        const SCALE_ADDRESSES: [u32; 5] = [
            0x017a_66c4, 0x017a_66c8, 0x017a_66cc, 0x017a_66d4, 0x017a_66dc,
        ];
        const OUTPUT_OFFSETS: [u32; 5] = [0x00, 0x04, 0x08, 0x10, 0x18];

        #[inline(always)]
        fn subtract(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn multiply(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let index = (owner.wrapping_add(OWNER_PROFILE_INDEX) as *const i16)
            .read_unaligned() as i32;
        let table_slot = PROFILE_POINTER_TABLE.wrapping_add((index as u32).wrapping_mul(4));
        let profile = (lf_checker_rt::relocated(table_slot) as *const u32).read_unaligned();
        let byte_value = (profile.wrapping_add(PROFILE_VALUE) as *const u8).read();
        let mut value = byte_value as f32;

        if 0.0f32 > value {
            value = 0.0;
        } else {
            let upper = (lf_checker_rt::global::<f32>(CLAMP_MAX)).read_unaligned();
            if value > upper {
                value = upper;
            }
        }
        let remainder = subtract(
            (lf_checker_rt::global::<f32>(UPPER_BOUND)).read_unaligned(),
            value,
        );
        for i in 0..OUTPUT_OFFSETS.len() {
            let scale = (lf_checker_rt::global::<f32>(SCALE_ADDRESSES[i])).read_unaligned();
            let scaled = multiply(remainder, scale);
            (output.wrapping_add(OUTPUT_OFFSETS[i]) as *mut f32).write_unaligned(scaled);
        }
    }
    0
});
