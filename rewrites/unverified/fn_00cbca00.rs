// original: 0x00CBCA00 ped_move_blend_on_foot_initialize
/// Run the base setup helper, replace the object's first word with the
/// on-foot blend vtable pointer, run the derived setup helper, and return
/// the object pointer. Both helper calls are intercepted by the contract.
/// The method is thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00cbca00(this: *mut u8, argument: u32) -> u32 {
    const ON_FOOT_VTABLE: u32 = 0x00ED90C4;

    let this_address = this as u32;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this_address, argument);
    unsafe {
        this.cast::<u32>()
            .write_unaligned(lf_checker_rt::relocated(ON_FOOT_VTABLE));
    }
    let _ = lf_checker_rt::callee_thiscall!(2, u32, this_address);
    this_address
});
