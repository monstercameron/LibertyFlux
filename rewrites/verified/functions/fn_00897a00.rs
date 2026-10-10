// original: 0x00897A00 input_release_slot_objects

/// Releases the objects held in the array at the start of `this`, the count being the word at +0x60.
/// For each non-null entry the object is passed to the free routine (cdecl, one argument); the entry is
/// read again, and if it is still non-null the object's method in virtual slot 0 is called with the
/// object as `this` and the value 1 as its one argument (callee cleans up). The entry is then cleared.
/// Afterwards nine words from +0x64 to +0x84 are cleared, and the tail routine is called with `this` + 0x88
/// as ECX; its return value is this function's return value. The count is read again on every pass.
lf_checker_rt::export!(thiscall, rw_00897a00(this: u32) -> u32 {
    unsafe {
        const COUNT_FIELD: u32 = 0x60;
        const TAIL_FIELD: u32 = 0x88;
        const ZEROED_FIELDS: [u32; 9] = [0x64, 0x68, 0x6C, 0x70, 0x74, 0x78, 0x7C, 0x80, 0x84];
        let mut i: u32 = 0;
        // The count is read again on every pass, as the original does.
        while i < (this.wrapping_add(COUNT_FIELD) as *const u32).read_unaligned() {
            let slot = this.wrapping_add(i.wrapping_mul(4));
            let object = (slot as *const u32).read_unaligned();
            if object != 0 {
                lf_checker_rt::callee_cdecl!(3, (), object);
                // The first call may have changed the slot, so it is read again.
                let object = (slot as *const u32).read_unaligned();
                if object != 0 {
                    let vtable = (object as *const u32).read_unaligned();
                    let method_bits = (vtable as *const u32).read_unaligned();
                    let method: extern "thiscall" fn(u32, u32) = core::mem::transmute(method_bits);
                    method(object, 1);
                }
                (slot as *mut u32).write_unaligned(0);
            }
            i = i.wrapping_add(1);
        }
        for field in ZEROED_FIELDS {
            (this.wrapping_add(field) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(4, u32, this.wrapping_add(TAIL_FIELD))
    }
});
