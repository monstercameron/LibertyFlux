// original: 0x00c069b0 stream_slot_build
/// Build a fully-configured slot from (`a0`, `a1`, `a2`).
///
/// Takes a slot from the grow helper (thiscall/1), names it (thiscall/1 with
/// `a0`), derives a float through the float helper (cdecl/1, x87 result) into
/// +0x38 and +0x44, configures two fields (thiscall/1 with `a1`, `a2`),
/// registers a default (cdecl/2) and finishes through the finaliser
/// (thiscall/0), returning the slot. Thiscall: three stack words, callee
/// cleans 0xc. The stack check is off: the original spills the x87 result
/// into its incoming `a0` slot, an address the rewrite cannot reproduce
/// (see `narrowed`).
lf_checker_rt::export!(thiscall, rw_00c069b0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const GROW: u32 = 1;
        const NAME: u32 = 2;
        const FLOATFN: u32 = 3;
        const CONF20: u32 = 4;
        const CONF24: u32 = 5;
        const REG: u32 = 6;
        const FIN: u32 = 7;
        const DEFAULTS: u32 = 0x00EBDDD8;
        let _ = this;
        let slot: u32 = lf_checker_rt::callee_thiscall!(GROW, u32, this, 0x10);
        let _: u32 = lf_checker_rt::callee_thiscall!(NAME, u32, slot, a0);
        let f: f32 = lf_checker_rt::callee_cdecl!(FLOATFN, f32, a0);
        wr32(slot + 0x38, f.to_bits());
        wr32(slot + 0x44, f.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(CONF20, u32, slot, a1);
        let _: u32 = lf_checker_rt::callee_thiscall!(CONF24, u32, slot, a2);
        let _: u32 = lf_checker_rt::callee_cdecl!(REG, u32, lf_checker_rt::relocated(DEFAULTS), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN, u32, slot);
        slot
    }
});
