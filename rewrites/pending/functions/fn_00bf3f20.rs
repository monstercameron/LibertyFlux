// original: 0x00bf3f20 bind_slot
/// Tag the slot (0x25), resolve the group through the object's hook, and
/// forward the indexed entry to fetch_block_a.
export!(thiscall, rw_bf3f20(this: *mut u8, obj: *const u8, a1: u32, a2: u32) -> u32 {
    unsafe {
        let vtable = *(obj as *const u32);
        let target = *((vtable as *const u8).add(0xA0) as *const u32);
        let get: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let inner = get(obj as u32);
        let grp = *((inner as *const u8).add(0x64) as *const u32);
        *this.add(1) = a1 as u8;
        *this = 0x25;
        *this.add(2) = a2 as u8;
        let tab = if (a2 as u8) != 0 {
            *((grp as *const u8).add(0x168) as *const u32)
        } else {
            *((grp as *const u8).add(0x160) as *const u32)
        };
        let n = a1
            .wrapping_shl(6)
            .wrapping_add(*((tab as *const u8).add(0x14) as *const u32));
        callee_thiscall!(2, u32, this as u32, n)
    }
});
