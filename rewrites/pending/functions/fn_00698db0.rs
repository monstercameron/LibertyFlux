// original: 0x00698db0 guarded_attach_call
/// Run the attach helper on the second argument when the first is non-null.
///
/// Returns nothing observable: EAX keeps the entry value on the skip path.
export!(cdecl, rw_00698db0(arg1: u32, arg2: u32) -> () {
    unsafe {
        if arg1 != 0 {
            
            callee_thiscall!(1, u32, arg1, arg2);
        }
    }
});
