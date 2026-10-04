// original: 0x00873b40 crmt_ctor_set_vtable
// Tiny constructor: clear the flags word, publish the vtable, return the
// object. (thiscall/0)
export!(thiscall, rw_00873b40(this_ptr: u32) -> u32 {
    unsafe {
        let base = this_ptr as *mut u32;
        base.add(1).write(0);
        base.write(relocated(0x00FE8060));
    }
    this_ptr
});
