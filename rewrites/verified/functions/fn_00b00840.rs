// original: 0x00b00840 notify_all_if_enabled
/// Call the slot-1 hook on every element, when enabled.
///
/// thiscall `(this)`: when the flag byte at `+8` is zero returns at once
/// (the original passes the incoming return register through, which the
/// contract does not compare). Otherwise reads the element count (16-bit
/// at `+4`, compared UNSIGNED against zero, then SIGNED in the loop) and
/// the element-pointer array at `+0`, and calls each element's vtable
/// slot at `+4` as thiscall `(element)` with no stack arguments. The
/// loop is a do-while: a nonzero count always runs the body at least
/// once, even past the signed range.
export!(thiscall, rw_00b00840(this: u32) -> u32 {
    unsafe {
        if ((this + 8) as *const u8).read() == 0 {
            return 0; // unchecked passthrough, see doc comment
        }
        let count = ((this + 4) as *const u16).read_unaligned() as u32;
        if count != 0 {
            let arr = (this as *const u32).read_unaligned();
            let mut i = 0u32;
            loop {
                let elem = ((arr + i.wrapping_mul(4)) as *const u32).read_unaligned();
                let vtbl = (elem as *const u32).read_unaligned();
                let tgt = ((vtbl + 4) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let _ = f(elem);
                i = i.wrapping_add(1);
                if !((i as i32) < (count as i32)) {
                    break;
                }
            }
        }
    }
    0 // unchecked: last hook answer or passthrough, see doc comment
});
