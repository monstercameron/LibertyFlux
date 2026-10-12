// original: 0x00A17550 vec3_cross
/// Writes B cross A to the destination's xyz words using the original SSE
/// operand order. The fourth word comes from an unwritten aligned-frame word;
/// this proof constrains that word to zero and lists the narrowing.
lf_checker_rt::export!(thiscall, rw_00A17550(_this: u32, _unused: u32, destination: u32, source: u32) -> u8 {
    unsafe {
        #[inline(always)] fn mul(left: f32, right: f32) -> f32 { core::hint::black_box(left) * core::hint::black_box(right) }
        #[inline(always)] fn sub(left: f32, right: f32) -> f32 { core::hint::black_box(left) - core::hint::black_box(right) }
        #[inline(always)] unsafe fn load(address: u32) -> f32 { unsafe { f32::from_bits((address as *const u32).read_unaligned()) } }
        #[inline(always)] unsafe fn store(address: u32, value: f32) { unsafe { (address as *mut u32).write_unaligned(value.to_bits()); } }
        let ax = load(destination); let ay = load(destination.wrapping_add(4)); let az = load(destination.wrapping_add(8));
        let bx = load(source); let by = load(source.wrapping_add(4)); let bz = load(source.wrapping_add(8));
        let x = sub(mul(by, az), mul(bz, ay));
        let y = sub(mul(ax, bz), mul(bx, az));
        let z = sub(mul(bx, ay), mul(ax, by));
        store(destination, x); store(destination.wrapping_add(4), y); store(destination.wrapping_add(8), z);
        store(destination.wrapping_add(12), 0.0);
        1
    }
});
