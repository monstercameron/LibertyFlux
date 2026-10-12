// original: 0x00C55440 CEventLeaderEnteredCarAsDriver::CEventLeaderEnteredCarAsDriver_2
/// Store the driver-entered-event type tag, optionally release its owner in
/// the field at `this + 0x0c`, then return the base constructor's result. The
/// release helper receives the owner in ECX and the field address as its only
/// stack argument; the base constructor is an E9 tail call with `this` in ECX.
lf_checker_rt::export!(thiscall, rw_c55440(this: u32) -> u32 {
    unsafe {
        const TYPE_TAG_VA: u32 = 0x00ec_ae04;
        const OWNER_FIELD: u32 = 0x0c;

        let field = this.wrapping_add(OWNER_FIELD);
        let owner = (field as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_TAG_VA));
        if owner != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, owner, field);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
