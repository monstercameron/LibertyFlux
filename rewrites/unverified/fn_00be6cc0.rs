// original: 0x00BE6CC0 CTaskSimpleTired::vf5
/// Abort-check for a tired task against ped `a0`.
///
/// When the task's handle at `this+0x18` is null, returns 1 at once. Else
/// looks the handle up through the ped's table (thiscall on `[a0+0x78]`
/// with argument 0); only when the lookup returns the same handle is the
/// handle faded with blend factor -4.0 (bits `FADE`). Returns 1 in AL; the
/// high bytes of eax keep call residue or caller leftovers, so only AL is
/// behaviour. Original is thiscall with three stack words (only the first
/// is read) and pops 12 bytes.
lf_checker_rt::export!(thiscall, rw_00be6cc0(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    const LOOKUP: u32 = 1;
    const FADE: u32 = 2;
    const HANDLE_OFF: u32 = 0x18;
    const TABLE_OFF: u32 = 0x78;
    const FADE_FACTOR: u32 = 0xC0800000;
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        let handle = rd32(this + HANDLE_OFF);
        if handle == 0 {
            return 1;
        }
        let table = rd32(a0 + TABLE_OFF);
        let found = lf_checker_rt::callee_thiscall!(LOOKUP, u32, table, 0);
        if found == handle {
            lf_checker_rt::callee_thiscall!(FADE, u32, handle, FADE_FACTOR);
        }
        1
    }
});
