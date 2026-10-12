// original: 0x00699B80 rage::crAnimChannelQuantizeFloat::try_build_from_samples
//! Forwards a sample pointer, count, stride, and scale to the quantizer while
//! passing three output-field addresses from the object; returns its status.

const QUANTIZER_CALLEE: u32 = 1;

lf_checker_rt::export!(thiscall, rw_fn_00699b80(
    object: *mut u8,
    samples: *const u32,
    count: u32,
    stride: u32,
    scale: f32,
) -> u32 {
    let base = object as u32;
    let output_min = base.wrapping_add(8);
    let output_range = base.wrapping_add(0x14);
    let output_bits = base.wrapping_add(0x18);

    lf_checker_rt::callee_fastcall!(
        QUANTIZER_CALLEE,
        u32,
        samples as u32,
        count,
        scale.to_bits(),
        output_min,
        output_range,
        output_bits,
    )
});
