// original: 0x00C78800 phbound_lazy_dispatch (proposed)

/// Dispatch through one of two tail handlers according to adjacent global
/// availability flags. The first handler wins when its byte is zero; the
/// second runs only when the first is nonzero and the second is nonzero. With
/// both conditions false, the function returns without a handler call.
lf_checker_rt::export!(thiscall, rw_phbound_lazy_dispatch(this: u32) -> () {
    const PRIMARY_FLAG: u32 = 0x0104_b8fd;
    const SECONDARY_FLAG: u32 = 0x0104_b8fe;
    const PRIMARY_HANDLER: u32 = 1;
    const SECONDARY_HANDLER: u32 = 2;

    let primary = unsafe { lf_checker_rt::global::<u8>(PRIMARY_FLAG).read_volatile() };
    if primary == 0 {
        let _ = lf_checker_rt::callee_thiscall!(PRIMARY_HANDLER, u32, this);
        return;
    }

    let secondary = unsafe { lf_checker_rt::global::<u8>(SECONDARY_FLAG).read_volatile() };
    if secondary != 0 {
        let _ = lf_checker_rt::callee_thiscall!(SECONDARY_HANDLER, u32, this);
    }
});
