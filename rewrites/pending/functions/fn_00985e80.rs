// original: 0x00985e80 NativeImpl_FIND_STATIC_EMITTER_INDEX
/// Find `key` in the static-emitter index table.
///
/// Scans the `count` entries at `this+0xd8` (stride 0xd0) for `key` and
/// returns its index, or -1 when absent or when the count at `this+0x8230`
/// is not positive.
export!(thiscall, rw_00985e80(this: u32, key: u32) -> i32 {
    unsafe {
        let n = *((this.wrapping_add(0x8230)) as *const i32);
        if n <= 0 {
            return -1;
        }
        let mut i = 0i32;
        loop {
            let e = *((this.wrapping_add(0xd8).wrapping_add((i as u32) * 0xd0))
                as *const u32);
            if e == key {
                return i;
            }
            i += 1;
            if i >= n {
                return -1;
            }
        }
    }
});
