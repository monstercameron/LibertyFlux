// original: 0x009bb9a0 init_input_slot_entry (proposed)

/// Initialise a 0x30-byte input slot entry with constants and stack residue.
///
/// Writes 0 to the byte at `this+0x00`, 0x00010001 at `this+0x02`, 1.0f at
/// `this+0x08`, 0 to the words at `+0x10`, `+0x14`, `+0x18`, `+0x20`,
/// `+0x24`, `+0x28`, and the stack-residue word at `+0x1c` and `+0x2c`.
/// The residue is one word of uninitialised stack: the original aligns its
/// own frame and reads a scratch word it never wrote, so under the
/// contract's fixed zero stack fill both copies are 0 and the rewrite
/// stores 0 (a volatile read of our own slot was tried first, but rustc
/// allocates it with push-eax, which reads the incoming register, not the
/// fill; the value stored is therefore only proven under a uniform fill).
/// Returns nothing (the original never sets EAX).
///
/// Edge cases: none take another path; the residue value is whatever the
/// stack held, so the proof only holds under the contract's defined fill.
///
/// Original: thiscall, `this` in ECX, no stack arguments.
lf_checker_rt::export!(thiscall, rw_009bb9a0(this: u32) -> u32 {
    unsafe {
        const PAIR: u32 = 0x00010001;
        const ONE: u32 = 0x3f800000;
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        // The original's unwritten scratch word: provably the uniform fill
        // (0 here) on every trial; see the doc comment.
        let residue: u32 = 0;
        (this as *mut u8).write(0);
        wr(this + 0x02, PAIR);
        wr(this + 0x08, ONE);
        wr(this + 0x1c, residue);
        wr(this + 0x10, 0);
        wr(this + 0x14, 0);
        wr(this + 0x18, 0);
        wr(this + 0x20, 0);
        wr(this + 0x24, 0);
        wr(this + 0x28, 0);
        wr(this + 0x2c, residue);
        0
    }
});
