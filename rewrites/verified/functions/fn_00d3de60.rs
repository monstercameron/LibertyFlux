// original: 0x00D3DE60 CAnimRequestHelper::vf0
/// Performs the base update, optionally notifies the request manager when the
/// low bit of `notify` is set, and returns the object pointer. The helper is a
/// thiscall method with one 32-bit stack argument. Both outgoing calls are
/// observed through checker stand-ins.
lf_checker_rt::export!(thiscall, rw_d3de60(this: u32, notify: u32) -> u32 {
    const BASE_UPDATE: u32 = 1;
    const REQUEST_NOTIFY: u32 = 2;
    let _ = lf_checker_rt::callee_thiscall!(BASE_UPDATE, u32, this);
    if notify & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(REQUEST_NOTIFY, u32, this);
    }
    this
});
