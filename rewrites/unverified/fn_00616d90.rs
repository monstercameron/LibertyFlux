// original: 0x00616D90 rage::SkyDomeProceduralControl::TopStruct::vf0

/// Deleting destructor for the sky-dome top parameters block: point the
/// header at the block's virtual table, then free the block through the
/// thread allocator when the caller asked for it.
///
/// `this` in ECX, one stack argument holding the MSVC deleting-destructor
/// flag. The header always gets `VTAB`; only when bit 0 of the flag is
/// set is the block released through the allocator reached via TLS slot
/// 0 (its release entry is checker callee 1, called through the planted
/// vtable with the allocator as `this` and the block as its argument).
/// Returns `this` unchanged. Thiscall.
lf_checker_rt::export!(thiscall, rw_00616D90(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTAB: u32 = 0x00FE1EF4;
        const ALLOC: u32 = 0x08;
        const FREE_SLOT: u32 = 0x0c;

        (this as *mut u32).write(lf_checker_rt::relocated(VTAB));
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
