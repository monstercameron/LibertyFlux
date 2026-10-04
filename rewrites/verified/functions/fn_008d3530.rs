// original: 0x008D3530 SetRefField598
/// Replaces the reference in slot 0x598, releasing the old value and
/// acquiring the new one through the two bookkeeping calls. Each call is
/// skipped when its value is null; the slot always takes the new value.
export!(thiscall, rw_008D3530(this: u32, new_val: u32) -> () {
    unsafe {
        let slot = (this + 0x598) as *mut u32;
        let old = *slot;
        if old != 0 {
            callee_thiscall!(0, u32, old, slot as u32);
        }
        *slot = new_val;
        if new_val != 0 {
            callee_thiscall!(1, u32, new_val, slot as u32);
        }
    }
});
