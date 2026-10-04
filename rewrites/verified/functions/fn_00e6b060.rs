// original: 0x00e6b060 scale_const_init_07 (proposed)

/// Derive one start-up tuning constant: `*DST = *NUM / *DEN`
/// in single precision.
///
/// Reads two 4-byte globals (numerator, then divisor), divides
/// them with the processor's scalar single-precision divide, and
/// stores the quotient into a third global. Takes no arguments
/// (cdecl with an empty stack list; the plain return pops nothing)
/// and leaves no meaningful return value: `eax` still holds its
/// entry value on return. Edge cases follow the hardware: a zero
/// divisor yields a signed infinity (NaN for 0/0), never a fault.
lf_checker_rt::export!(cdecl, rw_00e6b060() -> u32 {
    const NUM_ADDR: u32 = 0x010490cc;
    const DEN_ADDR: u32 = 0x010490d0;
    const DST_ADDR: u32 = 0x016d9efc;
    unsafe {
        let num = f32::from_bits(lf_checker_rt::global::<u32>(NUM_ADDR).read_unaligned());
        let den = f32::from_bits(lf_checker_rt::global::<u32>(DEN_ADDR).read_unaligned());
        let quot = core::hint::black_box(num) / core::hint::black_box(den);
        lf_checker_rt::global::<u32>(DST_ADDR).write_unaligned(quot.to_bits());
    }
    0
});
