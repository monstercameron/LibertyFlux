// original: 0x00697e30 rage::crCreatureComponentMover::vf0

/// Destroy a mover component, freeing it when the flag asks.
///
/// `this` has its payload words (`+0x04`, `+0x08`, `+0x0c`) zeroed and its
/// vtable slot set to the base-component vtable. When bit 0 of `flag` is set,
/// the object is handed to the allocator's slot-3 release routine (reached
/// through TLS slot 0: allocator owner at `[slot0]`, allocator at `+8`,
/// routine at vtable `+0x0c`). Returns `this`.
///
/// Original: thiscall, one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00697e30(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTABLE_BASE: u32 = 0x00fe38ac;
        const FREE_SLOT: u32 = 0x0c;
        let p = this as *mut u32;
        p.byte_offset(0x0c).write_unaligned(0);
        p.byte_offset(0x04).write_unaligned(0);
        p.byte_offset(0x08).write_unaligned(0);
        p.write_unaligned(lf_checker_rt::relocated(VTABLE_BASE));
        if flag & 1 != 0 {
            let owner = lf_checker_rt::tls_slot(0);
            let alloc = (owner as *const u32).byte_offset(8).read_unaligned();
            let vtable = (alloc as *const u32).read_unaligned();
            let routine = (vtable as *const u32).byte_offset(FREE_SLOT as isize).read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(routine as usize);
            free(alloc, this);
        }
        this
    }
});
