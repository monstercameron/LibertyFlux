// original: 0x00952c00 pool_slot_plus_28
/// Fetch the pool element selected by the global index and return the
/// address 0x28 bytes past its start.
export!(cdecl, rw_00952c00() -> u32 {
    unsafe {
        let index = *global::<u32>(0x011f6f34);
        let pool = *global::<u32>(0x011f6954);
        let elem: u32 = callee_thiscall!(1, u32, pool, index);
        elem.wrapping_add(0x28)
    }
});
