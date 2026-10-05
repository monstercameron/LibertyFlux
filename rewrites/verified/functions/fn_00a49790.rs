// original: 0x00a49790 vehicle_find_subpart_forward
/// Forward to the subpart search with the object and key read from `this`.
///
/// Pushes the dword at `this+0xC`, takes the object from `this+8` and calls
/// the search routine (callee 1, thiscall, one stack word), returning its
/// answer unchanged (thiscall, no stack arguments).
export!(thiscall, rw_00a49790(this: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        const OBJ_OFF: u32 = 8;
        const KEY_OFF: u32 = 0xc;
        let obj = (this.wrapping_add(OBJ_OFF) as *const u32).read_unaligned();
        let key = (this.wrapping_add(KEY_OFF) as *const u32).read_unaligned();
        callee_thiscall!(CALLEE, u32, obj, key)
    }
});
