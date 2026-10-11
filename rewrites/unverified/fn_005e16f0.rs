// original: 0x005E16F0 compute_blend_weights
/// Computes two nonnegative blend measures from the two stack floats, then
/// writes four values into one 0xF0-byte row of the receiver's blend table.
/// The average is `(first + second) * HALF`; the difference is `second -
/// first`. Each is clamped to zero only when an ordered comparison says it is
/// negative, so NaNs pass through unchanged. If `SPECIAL_MODE` equals one,
/// TLS flag bits 1 or 3 choose the first global row index (otherwise the
/// second); the function writes `[HALF, difference, HALF, average]` at the
/// following row. Otherwise it writes the ordered expression
/// `[(average-difference*HALF)*HALF, difference, difference*HALF, average]`
/// at `NORMAL_ROW_FIELDS`; the first value is clamped to zero only when it is
/// negative, preserving signed zero and unordered NaN results.
/// Float operations preserve the original operand order and round at each
/// single-precision step. This is a thiscall method with two stack floats;
/// it returns no meaningful value and the callee removes both arguments.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_compute_blend_weights(
    this: *mut u8,
    first: f32,
    second: f32,
) {
    const HALF_BITS: u32 = 0x3F00_0000;
    const HALF: f32 = f32::from_bits(HALF_BITS);
    const SPECIAL_MODE: usize = 0x264;
    const TLS_FLAGS: usize = 0x8D0;
    const ROW_STRIDE: u32 = 0xF0;
    const SPECIAL_GLOBAL_INDEX_VA: u32 = 0x0117_4790;
    const NORMAL_GLOBAL_INDEX_VA: u32 = 0x0117_4794;
    const NORMAL_ROW_FIELDS: usize = 0xE0;
    const LOW_BLEND: usize = 0x00;
    const DIFFERENCE: usize = 0x04;
    const HALF_DIFFERENCE: usize = 0x08;
    const AVERAGE: usize = 0x0C;

    #[inline(never)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(never)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(never)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    let mut average = mul_f32(add_f32(first, second), HALF);
    if average < 0.0 {
        average = 0.0;
    }

    let mut difference = sub_f32(second, first);
    if difference < 0.0 {
        difference = 0.0;
    }

    let receiver_bytes = this as usize;

    let special_mode = unsafe { core::ptr::read_unaligned(this.add(SPECIAL_MODE)) };
    if special_mode == 1 {
        let thread_data = lf_checker_rt::tls_slot(0) as *const u8;
        let thread_flags = unsafe {
            core::ptr::read_unaligned(thread_data.add(TLS_FLAGS).cast::<u32>())
        };
        let index_va = if thread_flags & ((1 << 1) | (1 << 3)) != 0 {
            SPECIAL_GLOBAL_INDEX_VA
        } else {
            NORMAL_GLOBAL_INDEX_VA
        };
        let row_index = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(index_va))
        };
        let row = row_index.wrapping_add(1).wrapping_mul(ROW_STRIDE) as usize;
        unsafe {
            core::ptr::write_unaligned((receiver_bytes + row + LOW_BLEND) as *mut f32, HALF);
            core::ptr::write_unaligned((receiver_bytes + row + DIFFERENCE) as *mut f32, difference);
            core::ptr::write_unaligned((receiver_bytes + row + HALF_DIFFERENCE) as *mut f32, HALF);
            core::ptr::write_unaligned((receiver_bytes + row + AVERAGE) as *mut f32, average);
        }
        return;
    }

    let half_difference = mul_f32(difference, HALF);
    let adjusted = mul_f32(sub_f32(average, half_difference), HALF);
    let low_blend = if adjusted < 0.0 { 0.0 } else { adjusted };
    let thread_data = lf_checker_rt::tls_slot(0) as *const u8;
    let thread_flags = unsafe {
        core::ptr::read_unaligned(thread_data.add(TLS_FLAGS).cast::<u32>())
    };
    let index_va = if thread_flags & ((1 << 1) | (1 << 3)) != 0 {
        SPECIAL_GLOBAL_INDEX_VA
    } else {
        NORMAL_GLOBAL_INDEX_VA
    };
    let row_index = unsafe { core::ptr::read_unaligned(lf_checker_rt::global::<u32>(index_va)) };
    let row = row_index.wrapping_mul(ROW_STRIDE) as usize + NORMAL_ROW_FIELDS;
    unsafe {
        core::ptr::write_unaligned((receiver_bytes + row + LOW_BLEND) as *mut f32, low_blend);
        core::ptr::write_unaligned((receiver_bytes + row + DIFFERENCE) as *mut f32, difference);
        core::ptr::write_unaligned((receiver_bytes + row + HALF_DIFFERENCE) as *mut f32, half_difference);
        core::ptr::write_unaligned((receiver_bytes + row + AVERAGE) as *mut f32, average);
    }
}
