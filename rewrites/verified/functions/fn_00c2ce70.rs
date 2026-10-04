// original: 0x00c2ce70 store_arg_handle
/// Runs the base initialiser, stores the argument handle, returns it.
export!(thiscall, rw_00c2ce70(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *(this.add(8) as *mut u32) = arg;
        arg
    }
});
