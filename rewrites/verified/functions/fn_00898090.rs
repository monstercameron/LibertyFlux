// original: 0x00898090 input_set_object_0xd4

/// Replaces the object pointer held at +0xD4 of `this` by the pointer given as argument.
///
/// The old object, when non-null, is released by the first callee (its own `this` in ECX); the
/// slot is then cleared. The new object, when non-null, is handed to the second callee, and the
/// slot finally holds the new pointer. Thiscall with one stack argument, callee cleans up.
lf_checker_rt::export!(thiscall, rw_00898090(this: u32, value: u32) -> () {
    unsafe {
        const OBJECT_FIELD: u32 = 0xD4;
        let old = (this.wrapping_add(OBJECT_FIELD) as *const u32).read_unaligned();
        if old != 0 {
            lf_checker_rt::callee_thiscall!(1, (), old);
            (this.wrapping_add(OBJECT_FIELD) as *mut u32).write_unaligned(0);
        }
        if value != 0 {
            lf_checker_rt::callee_thiscall!(2, (), value);
        }
        (this.wrapping_add(OBJECT_FIELD) as *mut u32).write_unaligned(value);
    }
});
