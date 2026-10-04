// original: 0x00b77390 task_owner_init (proposed)

/// Initialise a task-owner object: nine identical parameter blocks, a header,
/// a footer, and a per-process sequence byte.
///
/// `this` points to the object (at least 0x1b0 bytes). `arg0` is stored
/// through the store callee at `this + STASH_SLOT` (the callee writes its
/// second argument to the address given as its first; the caller never reads
/// the slot back, so the call is observable only in the call log).
///
/// Layout written:
/// - nine blocks of `BLOCK_STRIDE` bytes starting at `BLOCK_BASE`, each
///   holding `DEFAULT_PARAM` at `+0x04` and `1.0` at `+0x18` and `+0x20`,
///   with every other word zero and a trailing zero byte at `+0x24`;
/// - header words at `+0x00`..`+0x24` all zero;
/// - footer: `+0x190`/`+0x194`/`+0x19c` zero, `+0x1a0`/`+0x1a8` set to
///   `EMPTY_SLOT` (0xfe, "no slot"), `+0x1a4` set to `SLOT_COUNT` (9 slots),
///   byte `+0x1ad` zero;
/// - byte `+0x1ac` takes the low byte of the process-wide counter
///   `SEQ_COUNTER` before it is incremented (wrapping).
///
/// The original writes block 7's trailing byte and all of block 8 after
/// pushing the call arguments; the order is unobservable (disjoint memory)
/// and this rewrite fills every block first. Returns `this`.
///
/// Original: 0x00b77390 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b77390(this: u32, arg0: u32) -> u32 {
    unsafe {
        const BLOCK_BASE: u32 = 0x28;
        const BLOCK_STRIDE: u32 = 0x28;
        const BLOCK_COUNT: u32 = 9;
        const DEFAULT_PARAM: u32 = 0x46bab800;
        const ONE_BITS: u32 = 0x3f800000;
        const STASH_SLOT: u32 = 0x198;
        const EMPTY_SLOT: u32 = 0xfe;
        const SLOT_COUNT: u32 = 9;
        const SEQ_COUNTER: u32 = 0x0115d984;
        const STORE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        // Nine identical parameter blocks.
        let mut k = 0u32;
        while k < BLOCK_COUNT {
            let b = this.wrapping_add(BLOCK_BASE).wrapping_add(k.wrapping_mul(BLOCK_STRIDE));
            wr32(b.wrapping_add(0x00), 0);
            wr32(b.wrapping_add(0x04), DEFAULT_PARAM);
            wr32(b.wrapping_add(0x08), 0);
            wr32(b.wrapping_add(0x0c), 0);
            wr32(b.wrapping_add(0x10), 0);
            wr32(b.wrapping_add(0x14), 0);
            wr32(b.wrapping_add(0x18), ONE_BITS);
            wr32(b.wrapping_add(0x1c), 0);
            wr32(b.wrapping_add(0x20), ONE_BITS);
            wr8(b.wrapping_add(0x24), 0);
            k += 1;
        }

        // Stash arg0 through the store callee; its answer is ignored.
        let _: u32 =
            lf_checker_rt::callee_cdecl!(STORE_CALLEE, u32, this.wrapping_add(STASH_SLOT), arg0);

        // Header and footer.
        wr32(this, 0);
        let mut w = 0u32;
        while w < 9 {
            wr32(this.wrapping_add(4).wrapping_add(w.wrapping_mul(4)), 0);
            w += 1;
        }
        wr32(this.wrapping_add(0x190), 0);
        wr32(this.wrapping_add(0x194), 0);
        wr32(this.wrapping_add(0x19c), 0);
        wr32(this.wrapping_add(0x1a0), EMPTY_SLOT);
        wr32(this.wrapping_add(0x1a4), SLOT_COUNT);
        wr32(this.wrapping_add(0x1a8), EMPTY_SLOT);
        wr8(this.wrapping_add(0x1ad), 0);

        // Sequence byte from the process-wide counter, then bump it.
        let ctr = lf_checker_rt::global::<u32>(SEQ_COUNTER);
        let old = *ctr;
        *ctr = old.wrapping_add(1);
        wr8(this.wrapping_add(0x1ac), old as u8);

        this
    }
});
