// original: 0x009B08E0 audPoliceScanner::destructor

/// Replaces the scanner's active vtable, notifies the engine when the
/// 16-bit notification field at `+0x62` is nonzero, then installs the base
/// audio-entity vtable and tail-forwards to its destructor. The call is
/// `thiscall` with the object in ECX and no stack arguments. The notification
/// call is a scripted cdecl call with one word argument; the base destructor's
/// result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_009b08e0(this: u32) -> u32 {
    const VTABLE_SCANNER: u32 = 0x00E93C58;
    const VTABLE_AUDIO_ENTITY: u32 = 0x00E83134;
    const NOTIFY_FIELD: u32 = 0x5c;
    const NOTIFY_ENABLED: u32 = 0x62;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_SCANNER));
        let notify_enabled = ((this + NOTIFY_ENABLED) as *const u16).read_unaligned();
        if notify_enabled != 0 {
            let notification = ((this + NOTIFY_FIELD) as *const u32).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(1, u32, notification);
        }
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_AUDIO_ENTITY));
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
