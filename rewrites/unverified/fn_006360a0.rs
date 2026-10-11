// original: 0x006360a0 streaming_record_cleanup_helper
/// Resets a streaming record and forwards its nested cleanup request.
/// `this` is in ECX and the resource manager is in EDX. A null record
/// returns without calls. Otherwise the base cleanup runs, the record's
/// dispatch pointer is relocated into its first word, and a nonzero
/// count at `+0x10` is increased with wrapping arithmetic by the helper
/// result. The final thiscall receives the manager and the ECX value
/// preserved by the preceding helper path. The original return register
/// is unused by the contract.
lf_checker_rt::export!(fastcall, rw_006360a0(this: u32, manager: u32) -> u32 {
    const VTABLE_ADDRESS: u32 = 0x00fe_2594;
    const COUNT_OFFSET: u32 = 0x10;
    const FINALIZER_OFFSET: u32 = 0x18;

    if this == 0 {
        return 0;
    }

    let _base_cleanup = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_ADDRESS));
    }

    let count_address = this.wrapping_add(COUNT_OFFSET);
    let requested = unsafe { (count_address as *const u32).read_unaligned() };
    let forwarded_ecx = if requested == 0 {
        this
    } else {
        let delta = lf_checker_rt::callee_thiscall!(2, u32, manager, requested);
        unsafe { (count_address as *mut u32).write_unaligned(requested.wrapping_add(delta)); }
        manager
    };

    let _nested_cleanup = lf_checker_rt::callee_thiscall!(
        3,
        u32,
        this.wrapping_add(FINALIZER_OFFSET),
        manager,
        forwarded_ecx,
    );
    0
});
