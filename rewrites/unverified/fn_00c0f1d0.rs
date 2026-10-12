// original: 0x00C0F1D0 UIMusicContainer::vf86
/// Store four sampled frame values in the record selected by the image's
/// index global. The first sample is reduced by the image's 0.0009765625
/// bias, the third is increased by 0.001953125, and the other two are copied
/// bit-for-bit. Then set the record's active byte and tail-call the frame
/// refresh method. The four samples return through x87 ST0; `this` is in ECX.
lf_checker_rt::export!(thiscall, rw_c0f1d0(this: u32) -> u32 {
    unsafe {
        const INDEX_GLOBAL_VA: u32 = 0x017a_65a8;
        const FIRST_BIAS_VA: u32 = 0x00fe_86ac;
        const THIRD_BIAS_VA: u32 = 0x00fe_86c8;
        const FRAME_OBJECT: u32 = 0x1f8;
        const FIRST_VALUE: u32 = 0xf0;
        const SECOND_VALUE: u32 = 0xf4;
        const THIRD_VALUE: u32 = 0xf8;
        const FOURTH_VALUE: u32 = 0xfc;
        const ACTIVE_BYTE: u32 = 0x130;

        let slot = lf_checker_rt::global::<u32>(INDEX_GLOBAL_VA).read_unaligned();
        let record_stride = slot.wrapping_shl(4).wrapping_add(slot).wrapping_mul(4);
        let record = this.wrapping_add(record_stride);
        let frame = ((this + FRAME_OBJECT) as *const u32).read_unaligned();

        let first_sample = lf_checker_rt::callee_thiscall!(1, f32, frame);
        let first_bias_bits = (lf_checker_rt::relocated(FIRST_BIAS_VA) as *const u32)
            .read_unaligned();
        let first_bias = f32::from_bits(first_bias_bits);
        let first_adjusted = core::hint::black_box(first_sample)
            - core::hint::black_box(first_bias);
        ((record + FIRST_VALUE) as *mut f32).write_unaligned(first_adjusted);

        let second_sample = lf_checker_rt::callee_thiscall!(2, f32, frame);
        ((record + SECOND_VALUE) as *mut f32).write_unaligned(second_sample);

        let third_sample = lf_checker_rt::callee_thiscall!(3, f32, frame);
        let third_bias_bits = (lf_checker_rt::relocated(THIRD_BIAS_VA) as *const u32)
            .read_unaligned();
        let third_bias = f32::from_bits(third_bias_bits);
        let third_adjusted = core::hint::black_box(third_sample)
            + core::hint::black_box(third_bias);
        ((record + THIRD_VALUE) as *mut f32).write_unaligned(third_adjusted);

        let fourth_sample = lf_checker_rt::callee_thiscall!(4, f32, frame);
        ((record + FOURTH_VALUE) as *mut f32).write_unaligned(fourth_sample);
        (record.wrapping_add(ACTIVE_BYTE) as *mut u8).write_unaligned(1);

        lf_checker_rt::callee_thiscall!(5, u32, this)
    }
});
