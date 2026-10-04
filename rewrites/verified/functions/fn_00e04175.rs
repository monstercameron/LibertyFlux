// original: 0x00e04175 decode_and_call_slot
/// Decodes the stored function pointer and invokes it, normalizing to 0/1.
///
/// Decodes the slot at 0x17AC28C through the pointer-decoding import
/// (stubbed by the checker); returns 0 when decoding yields null or the
/// call returns 0, else 1.
export!(cdecl, rw_00e04175(arg: u32) -> u32 {
    unsafe {
        let slot = *global::<u32>(0x17AC28C);
        let target = callee_stdcall!(1, u32, slot);
        if target == 0 {
            return 0;
        }
        let f: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if f(arg) == 0 {
            0
        } else {
            1
        }
    }
});
