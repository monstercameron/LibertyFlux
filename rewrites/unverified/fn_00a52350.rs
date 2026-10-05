// original: 0x00a52350 vehicle_box_from_center (proposed)

/// Build an axis box from a center and a half-size.
///
/// The center (three floats at +0x19a0) minus/plus the half-size (float at
/// +0x1a00) gives the box minimum (+0x60) and maximum (+0x70), component by
/// component. The final maximum component adds in the opposite operand order
/// (half-size plus center), which matters for NaN payloads and is kept.
/// Thiscall, no stack words, no result.
lf_checker_rt::export!(thiscall, rw_00a52350(this: u32) -> u32 {
    unsafe {
        const C0: u32 = 0x19a0;
        const C1: u32 = 0x19a4;
        const C2: u32 = 0x19a8;
        const HALF: u32 = 0x1a00;
        const MIN0: u32 = 0x60;
        const MIN1: u32 = 0x64;
        const MIN2: u32 = 0x68;
        const MAX0: u32 = 0x70;
        const MAX1: u32 = 0x74;
        const MAX2: u32 = 0x78;
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let h = rdf(this.wrapping_add(HALF));
        let cx = rdf(this.wrapping_add(C0));
        let cy = rdf(this.wrapping_add(C1));
        let cz = rdf(this.wrapping_add(C2));
        wrf(this.wrapping_add(MIN0), sub(cx, h));
        wrf(this.wrapping_add(MIN1), sub(cy, h));
        wrf(this.wrapping_add(MIN2), sub(cz, h));
        wrf(this.wrapping_add(MAX0), add(cx, h));
        wrf(this.wrapping_add(MAX1), add(cy, h));
        wrf(this.wrapping_add(MAX2), add(h, cz));
        0
    }
});
