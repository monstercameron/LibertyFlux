// original: 0x00873f70 crmt_zeroing_ctor
// Zeroing constructor: clear every field, publish the vtable, return the
// object. (thiscall/0)
export!(thiscall, rw_00873f70(this_ptr: u32) -> u32 {
    unsafe {
        let base = this_ptr as *mut u32;
        base.add(1).write(0);
        base.write(relocated(0x00FE8084));
        for i in 2..9usize {
            base.add(i).write(0);
        }
    }
    this_ptr
});
