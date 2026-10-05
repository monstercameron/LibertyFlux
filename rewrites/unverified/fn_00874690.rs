// original: 0x00874690 crmt_block_init_and_zero

/// Initialise a 0x5c-byte block holder: clear `[this]`/`[this+4]`, run the setup hook from data-table slot 0xe731c4 on `(this, this+8)`, and when the tag at `[this+6]` is still zero set it to 0x17 and allocate the block through the thread manager; stamp count 0x17 at `[this+4]` and zero the block. Returns `this`. (True size 103 bytes; the batch lists 96, which omits the loop-back and epilogue.)
///
/// Original: 0x00874690 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00874690(this: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const ALLOC_SLOT: u32 = 8;
    const SETUP_TABLE: u32 = 0x00e731c4;
    const INIT_TAG: u16 = 0x17;
    const BLOCK: u32 = 0x5c;
    unsafe {
        ((this + 4) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(0);
        let tgt = lf_checker_rt::global::<u32>(SETUP_TABLE).read_unaligned();
        let setup: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let _: u32 = setup(this, this + 8);
        let tag = ((this + 6) as *const u16).read_unaligned();
        if tag == 0 {
            ((this + 6) as *mut u16).write_unaligned(INIT_TAG);
            let tls0 = lf_checker_rt::tls_slot(0);
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
                .read_unaligned();
            let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let fresh = alloc(manager, BLOCK, 0x10, 0);
            (this as *mut u32).write_unaligned(fresh);
        }
        ((this + 4) as *mut u16).write_unaligned(INIT_TAG);
        let base = (this as *const u32).read_unaligned();
        let mut off = 0u32;
        while off < BLOCK {
            ((base + off) as *mut u32).write_unaligned(0);
            off += 4;
        }
        this
    }
});
