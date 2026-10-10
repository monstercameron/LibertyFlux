// original: 0x00898260 input_release_object_0xd4

/// Releases the object held at +0xD4 of `this`, when there is one, by calling its
/// release callee with the object in ECX and a pointer to the field at +0xA0 as the one stack
/// argument (the callee cleans up). The object pointer itself is not cleared here.
/// Thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00898260(this: u32) -> () {
    unsafe {
        const OBJECT_FIELD: u32 = 0xD4;
        const LINK_FIELD: u32 = 0xA0;
        let object = (this.wrapping_add(OBJECT_FIELD) as *const u32).read_unaligned();
        if object != 0 {
            lf_checker_rt::callee_thiscall!(3, (), object, this.wrapping_add(LINK_FIELD));
        }
    }
});
