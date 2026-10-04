// original: 0x005e8110 DELETE_HTML_SCRIPT_OBJECT
/// Script native handler `DELETE_HTML_SCRIPT_OBJECT`.
///
/// Releases an HTML script object slot: resolves the slot through the object pool, marks it free in the slot bitmap, and updates the pool low-water mark and live count.
lf_rn94_rt::export!(cdecl, rw_fn_005e8110(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let index = *(args as *const u32);
        let base = *global::<i32>(0x110E8E4);
        let bitmap = *global::<u32>(0x110E8E8);
        let stride = *global::<i32>(0x110E8F0);
        // An object whose slot bit is already set resolves to slot 0;
        // otherwise its address is rebuilt from base + stride * index.
        let start = if *((bitmap + index) as *const u8) & 0x80 == 0 {
            stride.wrapping_mul(index as i32).wrapping_add(base)
        } else {
            0
        };
        // Normalise back to a slot number, mark it used, and fold it into
        // the low-water mark while dropping the live-object count by one.
        let slot = start.wrapping_sub(base) / stride;
        *((bitmap + slot as u32) as *mut u8) |= 0x80;
        let mark = global::<u32>(0x110E8F4);
        // Signed comparison: the original folds with `cmovl`.
        if slot < (*mark as i32) {
            *mark = slot as u32;
        }
        let live = global::<u32>(0x110E8F8);
        *live = (*live).wrapping_sub(1);
        slot as u32
    }
});
