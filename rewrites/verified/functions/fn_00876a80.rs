// original: 0x00876A80 rage::crmtRequestFrame::vf0

/// Call the class-specific request teardown helper. If the low bit of the
/// stack flag is set and `this` is non-null, get the manager through TLS slot
/// 0 and call its vtable release method at +0x0c with `this`. Return `this`.
/// The direct helper and virtual release are scripted separately by the
/// contract. This is a thiscall method with one stack word and callee cleanup.
lf_checker_rt::export!(thiscall, rw_00876a80(this: u32, flags: u32) -> u32 {
    const TEARDOWN: u32 = 1;
    const MANAGER_OFFSET: u32 = 8;
    const RELEASE_SLOT: u32 = 0x0c;
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
        if flags & 1 != 0 && this != 0 {
            let tls_base = lf_checker_rt::tls_slot(0);
            let manager = ((tls_base + MANAGER_OFFSET) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(RELEASE_SLOT as usize) as *const u32)
                .read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let _ = release(manager, this);
        }
    }
    this
});
