// original: 0x0086e890 TIMERC
/// Script native `TIMERC` (hash 0x1BF55D6F).
///
/// Reads the script timer value with no engine call: it follows a
/// global pointer to the timer block and copies the dword at offset
/// 0x24 into the return slot, also leaving it in EAX.
export!(cdecl, rw_0086e890(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let base = *global::<u32>(0x1BB54DC);
        let v = *((base.wrapping_add(0x24)) as *const u32);
        *slot = v;
        v
    }
});
