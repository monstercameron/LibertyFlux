// original: 0x0094bca0 handle_deref_slot
/// Resolve a handle through the engine lookup and return the pointer field
/// of its 32-byte pool entry, or 0xFFFFFFFF for an invalid handle.
export!(thiscall, rw_0094bca0(obj: *const u8, handle: u32) -> u32 {
    unsafe {
        let idx = callee_cdecl!(1, i32, handle, 9);
        if idx < 0 {
            return 0xFFFF_FFFF;
        }
        *(obj.add((idx as u32).wrapping_mul(32).wrapping_add(4) as usize) as *const u32)
    }
});
