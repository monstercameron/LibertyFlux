// original: 0x00A22520 cam_follow_ped_spread_and_update_records

/// Recompute a three-component spread vector, then invoke the enabled record
/// helpers across four signed-count task groups.
///
/// `this` is the 32-bit task object. Its current vector is at `+0x430`, and
/// the record descriptor pointer is the sole stack argument. Descriptor
/// offsets `+0x30..+0x38` are subtracted from the current vector; the matrix
/// coefficients at `+0x00..+0x28` combine those deltas in the order retained
/// below. The function then calls the vector helper followed by enabled
/// per-record helpers in group order. Counts are signed; nonpositive counts
/// visit no records. The original uses the callee pops 4 bytes; its accumulator is not used by
/// the caller. This proof checks call order and arguments for scripted helper
/// answers and all heap effects produced by this function.
pub(crate) unsafe fn cam_follow_ped_spread_and_update_records(this: u32, descriptor: u32) {
    unsafe {
        const VECTOR: u32 = 0x430;
        const DELTA_FIELDS: [u32; 3] = [0x30, 0x34, 0x38];
        const MATRIX_LEFT: [u32; 3] = [0x04, 0x14, 0x24];
        const MATRIX_RIGHT: [u32; 3] = [0x00, 0x10, 0x20];
        const MATRIX_Z: [u32; 3] = [0x08, 0x18, 0x28];
        const COUNTS: [u32; 4] = [0x400, 0x404, 0x408, 0x40c];
        const FLAGS: [u32; 4] = [0x410, 0x411, 0x412, 0x413];
        const RECORD_BASES: [u32; 4] = [0x90, 0x140, 0x200, 0x2e0];
        const STRIDES: [u32; 4] = [0xb0, 0xc0, 0xe0, 0x120];

        #[inline(always)]
        unsafe fn read_i32(address: u32) -> i32 {
            unsafe { (address as *const i32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits((address as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
        }
        #[inline(always)]
        fn sub_ss(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul_ss(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn add_ss(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        let mut delta = [0.0f32; 3];
        let mut component = 0usize;
        while component < delta.len() {
            delta[component] = sub_ss(
                read_f32(this.wrapping_add(VECTOR + component as u32 * 4)),
                read_f32(descriptor.wrapping_add(DELTA_FIELDS[component])),
            );
            component += 1;
        }

        let first = add_ss(
            mul_ss(read_f32(descriptor + MATRIX_LEFT[0]), delta[1]),
            mul_ss(read_f32(descriptor + MATRIX_RIGHT[0]), delta[0]),
        );
        write_f32(this + VECTOR, add_ss(first, mul_ss(read_f32(descriptor + MATRIX_Z[0]), delta[2])));

        let second = add_ss(
            mul_ss(read_f32(descriptor + MATRIX_LEFT[1]), delta[1]),
            mul_ss(read_f32(descriptor + MATRIX_RIGHT[1]), delta[0]),
        );
        write_f32(this + VECTOR + 4, add_ss(second, mul_ss(read_f32(descriptor + MATRIX_Z[1]), delta[2])));

        let third = add_ss(
            mul_ss(read_f32(descriptor + MATRIX_LEFT[2]), delta[1]),
            mul_ss(delta[0], read_f32(descriptor + MATRIX_RIGHT[2])),
        );
        write_f32(this + VECTOR + 8, add_ss(third, mul_ss(read_f32(descriptor + MATRIX_Z[2]), delta[2])));

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);

        let mut group = 0usize;
        while group < COUNTS.len() {
            let mut index = 0i32;
            while index < read_i32(this.wrapping_add(COUNTS[group])) {
                let flag = this
                    .wrapping_add(FLAGS[group])
                    .wrapping_add(index as u32);
                if read_u8(flag) != 0 {
                    let record = this
                        .wrapping_add(RECORD_BASES[group])
                        .wrapping_add((index as u32).wrapping_mul(STRIDES[group]));
                    let _ = match group {
                        0 => lf_checker_rt::callee_thiscall!(2, u32, record, descriptor),
                        1 => lf_checker_rt::callee_thiscall!(3, u32, record, descriptor),
                        2 => lf_checker_rt::callee_thiscall!(4, u32, record, descriptor),
                        _ => lf_checker_rt::callee_thiscall!(5, u32, record, descriptor),
                    };
                }
                index = index.wrapping_add(1);
            }
            group += 1;
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00A22520(this: u32, descriptor: u32) -> () {
    unsafe { cam_follow_ped_spread_and_update_records(this, descriptor) }
});
