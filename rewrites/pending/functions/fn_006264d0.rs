// original: 0x006264d0 vtable_release_publish_pair
// Release one object through its table, then publish it on another.
//
// Does nothing for a null first object (that path returns entry residue and
// is not exercised). Otherwise calls slot 0 of the first object's table,
// then slot 3 of the second object's table with the first object as the
// argument. Returns the second call's answer.
export!(fastcall, rw_006264d0(first: u32, second: u32) -> u32 {
    unsafe {
        if first == 0 {
            return 0;
        }
        type SlotFn = extern "thiscall" fn(u32, u32) -> u32;
        let table_a: u32 = *(first as *const u32);
        let release: SlotFn =
            core::mem::transmute((*(table_a as *const u32)) as usize);
        let _released: u32 = release(first, 0);
        let table_b: u32 = *(second as *const u32);
        let publish: SlotFn =
            core::mem::transmute((*((table_b.wrapping_add(0x0c)) as *const u32)) as usize);
        publish(second, first)
    }
});
