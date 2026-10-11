// original: 0x00c8f340 fn_00c8f340_compare_against_two

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Return three below a global float threshold and two otherwise. COMISS plus
/// SETA makes unordered input return two; the argument is transported as its
/// original stack word to retain every floating-point bit pattern.
const COMPARISON_THRESHOLD_SOURCE: u32 = 0x00FE8A24;

unsafe fn compare_against_two(input_bits: u32, wrong_inclusive: bool) -> u32 {
    let input = f32::from_bits(input_bits);
    let threshold = unsafe { read_global_f32(COMPARISON_THRESHOLD_SOURCE) };
    let greater = if wrong_inclusive { threshold >= input } else { threshold > input };
    2 + u32::from(greater)
}

lf_checker_rt::export!(cdecl, rw_00c8f340_compare_against_two(input_bits: u32) -> u32 {
    unsafe { compare_against_two(input_bits, false) }
});
