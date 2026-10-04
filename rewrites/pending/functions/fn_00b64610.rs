// original: 0x00b64610 NativeImpl_FORCE_CHAR_TO_DROP_WEAPON_2
// thiscall/2. Validates the given object chain; resolves through the shared
// step (passing the caller's mode word and a validated flag);
// attaches the inner record when the
// resolved entry is fresh. Returns the resolved entry, or null.
export!(thiscall, rw_rs11f2(this: *mut u8, obj: u32, mode: u32) -> u32 {
    unsafe {
        let mut validated = 1u32;
        if obj != 0 {
            let inner = *((obj + 0x6c) as *const u32);
            if inner != 0 && *((inner + 0xe) as *const u8) != 0 {
                if *this.add(0x109) == 0 {
                    return 0;
                }
                if *((inner + 0x804) as *const u32) != 0 {
                    return 0;
                }
                validated = 0;
            }
        }
        let resolve: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let out = resolve(
            this as u32,
            (this as u32).wrapping_add(0x14),
            validated,  // v2 port: leftover high bytes dropped; contract skips this arg
            mode,
        );
        if out != 0 && *((out + 0x6c) as *const u32) == 0 && obj != 0 {
            let inner = *((obj + 0x6c) as *const u32);
            if inner != 0 {
                let attach: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(2) as usize);
                attach(inner, out);
            }
        }
        out
    }
});
