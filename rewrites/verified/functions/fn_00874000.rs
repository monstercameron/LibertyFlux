// original: 0x00874000 crmt_chained_ctor
// Chained constructor: publish the base vtable, run the shared base
// initializer, then publish this object's vtable. (thiscall/0)
export!(thiscall, rw_00874000(this_ptr: u32) -> () {
    unsafe {
        (this_ptr as *mut u32).write(relocated(0x00FE8084));
    }
    callee_thiscall!(1, u32, this_ptr);
    unsafe {
        (this_ptr as *mut u32).write(relocated(0x00E86AFC));
    }
});
