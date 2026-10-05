// original: 0x00c06900 stream_slot_append_or_forward
/// Append a slot built from (`a0`, `a2`, `a3`), or forward when full.
///
/// When `a0` equals the slot count at +4, forwards (`a0`, `a2`, `a3`) to the
/// slot-append helper (thiscall/3) and returns its answer. Otherwise grows
/// through the grow helper (thiscall/1) when count equals capacity at +6,
/// takes a fresh slot from the allocator helper (thiscall/1), initialises it
/// (thiscall/0), names it (thiscall/1 with `a0`), derives a float through the
/// float helper (cdecl/1, x87 result) into +0x38 and +0x44, configures two
/// fields (thiscall/1 with `a2`, `a3`), registers a default (cdecl/2) and
/// finishes through the finaliser (thiscall/0), returning the slot. `a1` is
/// never read. Thiscall: four stack words, callee cleans 0x10.
lf_checker_rt::export!(thiscall, rw_00c06900(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
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

        const APPEND: u32 = 1;
        const GROW: u32 = 2;
        const ALLOC: u32 = 3;
        const SLOT_FN: u32 = 4;
        const NAME: u32 = 5;
        const FLOATFN: u32 = 6;
        const CONF20: u32 = 7;
        const CONF24: u32 = 8;
        const REG: u32 = 9;
        const FIN: u32 = 10;
        const DEFAULTS: u32 = 0x00EBDDE8;
        let _ = a1;
        if a0 == rd16(this + 4) {
            return lf_checker_rt::callee_thiscall!(APPEND, u32, this, a0, a2, a3);
        }
        if rd16(this + 4) == rd16(this + 6) {
            let _: u32 = lf_checker_rt::callee_thiscall!(GROW, u32, this, 0x10);
            let c = rd16(this + 4);
            wr16(this + 4, c.wrapping_sub(1) as u16);
        }
        let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, this, a0);
        let _: u32 = lf_checker_rt::callee_thiscall!(SLOT_FN, u32, slot);
        let _: u32 = lf_checker_rt::callee_thiscall!(NAME, u32, slot, a0);
        let f: f32 = lf_checker_rt::callee_cdecl!(FLOATFN, f32, a0);
        wr32(slot + 0x38, f.to_bits());
        wr32(slot + 0x44, f.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(CONF20, u32, slot, a2);
        let _: u32 = lf_checker_rt::callee_thiscall!(CONF24, u32, slot, a3);
        let _: u32 = lf_checker_rt::callee_cdecl!(REG, u32, lf_checker_rt::relocated(DEFAULTS), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN, u32, slot);
        slot
    }
});
