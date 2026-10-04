// original: 0x008AC0D0 audio_row_notify
/// Refresh the base state, notify the consumer of the selected row
/// pointer, then tail-forward to the shared finish routine.
export!(thiscall, rw_008AC0D0(obj: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32);
        let step = *(obj.add(0x2C) as *const u32);
        let row = step.wrapping_add(5);
        let tripled = row.wrapping_add(row.wrapping_mul(2));
        let ptr = (obj as u32).wrapping_add(tripled.wrapping_mul(8));
        callee_stdcall!(2, u32, ptr);
        callee_thiscall!(3, u32, obj as u32)
    }
});
