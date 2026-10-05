// original: 0x00cce200 CTaskSimpleDead::vf5
/// Ensure the dead task's ped helper is shut down, returning true.
///
/// Reads the inner object at `ped+0x6c` (`ped` is the first of three stack
/// arguments; the other two and ECX are unused). Unless the inner object
/// exists and its flag byte at `+0xe` is already set, the shutdown helper
/// (thiscall on the ped, one zero argument) runs. Returns 1 in AL.
export!(thiscall, rw_00cce200(_this: u32, ped: u32, _u1: u32, _u2: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x6c;
        const FLAG_OFF: u32 = 0x0e;
        let inner = (ped.wrapping_add(INNER_OFF) as *const u32).read_unaligned();
        let need_call =
            inner == 0 || (inner.wrapping_add(FLAG_OFF) as *const u8).read() == 0;
        if need_call {
            let _: u32 = callee_thiscall!(1, u32, ped, 0u32);
        }
        1
    }
});
