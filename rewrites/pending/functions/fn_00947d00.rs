// original: 0x00947d00 obj_init_xor_id
/// Initialise an object header: install the interim vtable, mix a global
/// sequence number into the id slot, install the final vtable, and store
/// the three constructor arguments (a value, a pointed-to dword and a
/// pointed-to byte). Returns the object pointer.
export!(thiscall, rw_00947d00(obj: *mut u8, a: u32, p: *const u32, q: *const u8) -> u32 {
    unsafe {
        let slot = obj.add(4) as *mut u32;
        let old = *slot;
        *(obj as *mut u32) = relocated(0x00E7E048);
        let seq = global::<u32>(0x010327A0);
        *slot ^= (old ^ *seq) & 0x3FFF;
        *seq = (*seq).wrapping_add(1);
        *(obj.add(8) as *mut u32) = a;
        *(obj as *mut u32) = relocated(0x00E8909C);
        *(obj.add(0xC) as *mut u32) = *p;
        *obj.add(0x10) = *q;
        obj as u32
    }
});
