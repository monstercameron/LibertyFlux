// original: 0x00616DC0 rage::SkyDomeProceduralControl::CloudStruct::vf0

/// Deleting destructor for the cloud parameters block: point the header
/// and the embedded layer at their virtual tables, then free the block
/// through the thread allocator when the caller asked for it.
///
/// `this` in ECX, one stack argument holding the MSVC deleting-destructor
/// flag. The header always gets `VTAB` and the word at `LAYER` (+0x60)
/// always gets `VTAB_LAYER`; only when bit 0 of the flag is set is the
/// block released through the allocator reached via TLS slot 0 (its
/// release entry is checker callee 1, called through the planted vtable
/// with the allocator as `this` and the block as its argument). Returns
/// `this` unchanged. Thiscall.
lf_checker_rt::export!(thiscall, rw_00616DC0(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTAB: u32 = 0x00FE1EB0;
        const LAYER: u32 = 0x60;
        const VTAB_LAYER: u32 = 0x00FE1EF4;
        const ALLOC: u32 = 0x08;
        const FREE_SLOT: u32 = 0x0c;

        (this as *mut u32).write(lf_checker_rt::relocated(VTAB));
        ((this + LAYER) as *mut u32).write(lf_checker_rt::relocated(VTAB_LAYER));
        if flag & 1 != 0 {
            let state = lf_checker_rt::tls_slot(0);
            let alloc = ((state + ALLOC) as *const u32).read();
            let vtab = (alloc as *const u32).read();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((((vtab + FREE_SLOT) as *const u32).read()) as usize);
            let _: u32 = free(alloc, this);
        }
        this
    }
});
