// original: 0x00609340 rage::ioValue::vf0

/// Deleting destructor for a script value: reset the header, release the
/// attached payload when one is present, and free the object itself when
/// the caller asked for it.
///
/// `this` in ECX, one stack argument holding the MSVC deleting-destructor
/// flag. The header always gets `VTAB`. The payload pointer at `PAYLOAD`
/// (+0x0c) is released first when non-null, through the allocator reached
/// via TLS slot 0 (`ALLOC` at +0x08 on the thread state, release entry at
/// `FREE_SLOT` +0x0c on its table, checker callee 1). Then, only when bit
/// 0 of the flag is set, the object itself is released through the same
/// entry. Returns `this` unchanged. Thiscall.
lf_checker_rt::export!(thiscall, rw_00609340(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTAB: u32 = 0x00E837F0;
        const PAYLOAD: u32 = 0x0c;
        const ALLOC: u32 = 0x08;
        const FREE_SLOT: u32 = 0x0c;

        (this as *mut u32).write(lf_checker_rt::relocated(VTAB));
        let state = lf_checker_rt::tls_slot(0);
        let alloc = ((state + ALLOC) as *const u32).read();
        let vtab = (alloc as *const u32).read();
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((vtab + FREE_SLOT) as *const u32).read()) as usize);
        let payload = ((this + PAYLOAD) as *const u32).read();
        if payload != 0 {
            let _: u32 = free(alloc, payload);
        }
        if flag & 1 != 0 {
            let _: u32 = free(alloc, this);
        }
        this
    }
});
