// original: 0x00cb8410 ped_heading_observe
/// Observe a ped's heading through the float normaliser (1 call).
///
/// With a null `a0` returns 0 at once (thiscall, one stack argument).
/// Otherwise copies the heading at `[a0 + 0xAA0]` into `[this + 0x18]`,
/// passes it through the float callee, stores the answer back, then
/// clears bit `0x10` and sets bit `0x100` in the flag word at
/// `[this + 0x54]` and returns it. The callee is intercepted by the
/// checker and answers on the x87 register.
lf_checker_rt::export!(thiscall, rw_00cb8410(this: u32, a0: u32) -> u32 {
    unsafe {
        /// Heading offsets in the ped object and in this object.
        const PED_HEADING: u32 = 0xAA0;
        const HDG_OFF: u32 = 0x18;
        /// Flag word clear/set masks.
        const FLAG_OFF: u32 = 0x54;
        const FLAG_CLEAR: u32 = 0x10;
        const FLAG_SET: u32 = 0x100;
        /// Callee id of the float normaliser.
        const NORMALISE: u32 = 1;
        if a0 == 0 {
            return 0;
        }
        let h = ((a0 + PED_HEADING) as *const u32).read_unaligned();
        ((this + HDG_OFF) as *mut u32).write_unaligned(h);
        let v: f32 = lf_checker_rt::callee_cdecl!(NORMALISE, f32, h);
        ((this + HDG_OFF) as *mut u32).write_unaligned(v.to_bits());
        let f = ((this + FLAG_OFF) as *const u32).read_unaligned();
        let f2 = (f & !FLAG_CLEAR) | FLAG_SET;
        ((this + FLAG_OFF) as *mut u32).write_unaligned(f2);
        f2
    }
});
