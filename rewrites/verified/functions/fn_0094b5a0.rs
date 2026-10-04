// original: 0x0094b5a0 NativeImpl_HAS_SCRIPT_LOADED
/// Map a script name to its pool slot: hash the name through the engine
/// helper, then scan the script pool for a live entry with that id.
/// Returns the slot index, or 0xFFFFFFFF when the pool is empty or holds
/// no match.
export!(cdecl, rw_0094b5a0(name: u32) -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, name);
        let desc = *global::<u32>(0x0167F594) as *const u32;
        let count = *(desc.add(2)) as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let flags = *(desc.add(1)) as *const u8;
        let base = *desc;
        let stride = *(desc.add(3));
        for i in 0..(count as u32) {
            if *flags.add(i as usize) & 0x80 != 0 {
                continue;
            }
            let entry = base.wrapping_add(stride.wrapping_mul(i));
            if entry == 0 {
                continue;
            }
            if *((entry.wrapping_add(8)) as *const u32) == id {
                return i;
            }
        }
        0xFFFF_FFFF
    }
});
