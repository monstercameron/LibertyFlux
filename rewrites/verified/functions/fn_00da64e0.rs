// original: 0x00DA64E0 CTaskComplexShockingEventWatch::vf6

/// Float query over the argument's nested object: follow `+0x224` then
/// `+0x54` from the caller argument; a null link, a virtual type check
/// (slot 0x0c) answering anything but 0x390, or a refused two-argument
/// validation call all select the default constant, otherwise the live
/// constant is returned. Both constants come from the read-only data and
/// are returned through the floating-point channel. Original: thiscall,
/// one stack word (the object, `this` is unused), callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da64e0(this: u32, obj: u32) -> f64 {
    unsafe {
        const TYPE_CHECK: u32 = 1;
        const VALIDATE: u32 = 2;
        const LIVE_SLOT: u32 = 0x00FE8D94;
        const DEFAULT_SLOT: u32 = 0x00FE8B40;
        const WANT_TYPE: u32 = 0x390;

        let _ = this;
        let live = (lf_checker_rt::relocated(LIVE_SLOT) as *const f32).read();
        let default = (lf_checker_rt::relocated(DEFAULT_SLOT) as *const f32).read();
        let mid = ((obj + 0x224) as *const u32).read();
        let inner = ((mid + 0x54) as *const u32).read();
        if inner == 0 {
            return default as f64;
        }
        let table = (inner as *const u32).read();
        let slot = ((table + 0x0C) as *const u32).read();
        let type_check: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if type_check(inner) != WANT_TYPE {
            return default as f64;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, inner, obj, 1);
        if (ok & 0xFF) == 0 {
            return default as f64;
        }
        live as f64
    }
});
