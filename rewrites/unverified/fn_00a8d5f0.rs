// original: 0x00a8d5f0 pool_emit_positioned (proposed)

/// Emit one positioned record through the sink callee.
///
/// `this` is the pool. Returns the incoming register (fixed by the proof)
/// when the ready byte at +0x38 is clear. Otherwise marks the pool
/// emitting (+0xE0), asks the source slot for the record pointer, asks
/// the position slot for the depth float, and calls the sink with the
/// record block (three record words, a zero word, the depth bits), the
/// sink id, `this`, 0x2C and 0xD. Returns the sink answer. The proof
/// plants both slots; the source slot's out-words are unobserved. The
/// original realigns its stack, which only shifts frame addresses the
/// proof skips.
///
/// Original: 0x00A8D5F0 (thiscall, no stack words, vtable plus
/// register-indirect callees, x87 float return).
lf_checker_rt::export!(thiscall, rw_00a8d5f0(this: u32) -> u32 {
    unsafe {
        const CALLEE_SOURCE: u32 = 1;
        const CALLEE_DEPTH: u32 = 2;
        const CALLEE_SINK: u32 = 3;
        const SOURCE_SLOT: u32 = 0x54;
        const DEPTH_SLOT: u32 = 0x58;
        const READY: u32 = 0x38;
        const EMITTING: u32 = 0xe0;
        const SINK_ID: u32 = 0xa8fa00;
        const SINK_A: u32 = 0x2c;
        const SINK_B: u32 = 0xd;
        const INCOMING_EAX: u32 = 0x12345678;
        let ready = ((this + READY) as *const u8).read_unaligned();
        if ready == 0 {
            return INCOMING_EAX;
        }
        ((this + EMITTING) as *mut u8).write_unaligned(1);
        let vtable = ((this) as *const u32).read_unaligned();
        let out_addr = {
            let target = ((vtable + SOURCE_SLOT) as *const u32)
                .read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let mut out = [0u32; 4];
            let p = core::ptr::addr_of_mut!(out) as u32;
            f(this, p)
        };
        let target = ((vtable + DEPTH_SLOT) as *const u32).read_unaligned();
        let g: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(target as usize);
        let depth = g(this);
        let w0 = (out_addr as *const u32).read_unaligned();
        let w1 = ((out_addr + 4) as *const u32).read_unaligned();
        let w2 = ((out_addr + 8) as *const u32).read_unaligned();
        let block = [w0, w1, w2, 0, depth.to_bits()];
        let block_addr = core::ptr::addr_of!(block) as u32;
        lf_checker_rt::callee_cdecl!(
            CALLEE_SINK,
            u32,
            block_addr,
            lf_checker_rt::relocated(SINK_ID),
            this,
            SINK_A,
            SINK_B
        )
    }
});
