// original: 0x0062C5E0 rage::VerletWaterSurface::vf1

/// Push two surface parameter words, the first scaled by a float product.
///
/// Multiplies the floats at `+0x14` and `+0x10` of `this` (in that order)
/// and passes the product by address in a scratch slot (compared by snapshot,
/// not by address) to the wide callee (thiscall on `arg0 + 0x18`:
/// `arg0 + 0x14`, `arg1 + 4`, the product address, `4`, `1`, `2`), then sends
/// the word at `+0x18` with `arg1 + 8` to the short callee. Returns the last
/// callee's answer (thiscall, two arguments).
lf_checker_rt::export!(thiscall, rw_0062c5e0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x18;
        const ADDR_OFF: u32 = 0x14;
        unsafe fn rd(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        let fa = f32::from_bits(rd(this, 0x14));
        let fb = f32::from_bits(rd(this, 0x10));
        let prod =
            (core::hint::black_box(fa) * core::hint::black_box(fb)).to_bits();
        let esi = arg0.wrapping_add(ADDR_OFF);
        let state = rd(arg0, STATE);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            1, u32, state, esi, rd(arg1, 4), &prod as *const u32 as u32, 4, 1, 2);
        lf_checker_rt::callee_thiscall!(2, u32, state, esi, rd(arg1, 8), rd(this, 0x18))
    }
});
