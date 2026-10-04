// original: 0x009528d0 pool_slot_field34
/// Fetch the pool element selected by the global index and return the word
/// at offset 0x34 of that element.
export!(cdecl, rw_009528d0() -> u32 {
    unsafe {
        let index = *global::<u32>(0x011f6f34);
        let pool = *global::<u32>(0x011f6954);
        let elem: u32 = callee_thiscall!(1, u32, pool, index);
        *((elem + 0x34) as *const u32)
    }
});
