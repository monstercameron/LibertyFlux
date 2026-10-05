// original: 0x0088C950 rage::audVoiceDSoundAdpcm::vf8

/// Queue one ADPCM synthesis block on this voice and advance its block
/// cursor.
///
/// `this` points to the voice and `a0` selects the synthesis path. The
/// 14-word parameter block at `+0x14` is copied into the voice's current
/// slot (`+0xf4`, stride 64, four slots past the live window) and the
/// slot's accumulator is cleared. When `a0` is non-zero and synth bit 4
/// of the flags at `+0x8c` is set, the codec at `+0x10` resolves `a0` to
/// a table entry (callee 1), a rate is derived through the codec tables
/// and the rate helper (callee 2, called with `a0` and the word at `+0xc`),
/// the result is stored at `+0xcc`, a predictor triple is fetched from the
/// table at `+0xdc` into `+0x9e`/`+0xa0`, and the slot accumulator is set.
/// The tail always runs: the slot's remainder (source word minus
/// accumulator) is stored, and the cursor advances by one modulo the
/// divisor at `+0xfc`.
///
/// Original: 0x0088C950 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088C950(this: u32, a0: u32) -> () {
    unsafe {
        const SRC: u32 = 0x14;
        const CODEC: u32 = 0x10;
        const ARGVAL: u32 = 0x0c;
        const FLAGS: u32 = 0x8c;
        const SYNTH: u8 = 0x10;
        const IDX: u32 = 0xf4;
        const DIV: u32 = 0xfc;
        const CODEC_TAB: u32 = 0x7c;
        const STORE_CC: u32 = 0xcc;
        const LEA_BASE: u32 = 0xdc;
        const OUT_W: u32 = 0x9e;
        const OUT_B: u32 = 0xa0;
        const SLOT_ACC: u32 = 0x138;
        const SLOT_REM: u32 = 0x13c;
        const COPY_WORDS: u32 = 14;

        let idx = ((this + IDX) as *const u32).read_unaligned();
        let src = ((this + SRC) as *const u32).read_unaligned();
        let dst = this.wrapping_add((idx.wrapping_add(4)) << 6);
        let mut i = 0;
        while i < COPY_WORDS {
            let w = ((src.wrapping_add(i * 4)) as *const u32).read_unaligned();
            ((dst.wrapping_add(i * 4)) as *mut u32).write_unaligned(w);
            i += 1;
        }
        let slot = this.wrapping_add(idx << 6);
        ((slot.wrapping_add(SLOT_ACC)) as *mut u32).write_unaligned(0);
        if a0 != 0 && ((this + FLAGS) as *const u8).read() & SYNTH != 0 {
            let codec = ((this + CODEC) as *const u32).read_unaligned();
            let ans1 = lf_checker_rt::callee_thiscall!(1, u32, codec, a0);
            let val = ((this + ARGVAL) as *const u32).read_unaligned();
            let t1 = ((codec.wrapping_add(CODEC_TAB)) as *const u32)
                .read_unaligned();
            let t2 = (t1 as *const u32).read_unaligned();
            let w = ((t2.wrapping_add(ans1.wrapping_mul(8))) as *const u32)
                .read_unaligned();
            let edi = w.wrapping_mul(2) >> 2;
            let ans2 = lf_checker_rt::callee_cdecl!(2, u32, a0, val);
            ((this + STORE_CC) as *mut u32).write_unaligned(ans2);
            let edx0 = ans2.wrapping_mul(2) >> 2;
            let edx1 = (edx0 >> 11) + ((edx0 & 0x7ff != 0) as u32);
            let base = ((this + LEA_BASE) as *const u32).read_unaligned();
            let cp = (edx1 + edx1 * 2).wrapping_add(base);
            let ax = (cp as *const u16).read_unaligned();
            ((this + OUT_W) as *mut u16).write_unaligned(ax);
            let al = ((cp.wrapping_add(2)) as *const u8).read();
            ((this + OUT_B) as *mut u8).write(al);
            let acc = (edx1 << 11).wrapping_sub(edi);
            ((slot.wrapping_add(SLOT_ACC)) as *mut u32).write_unaligned(acc);
        }
        let idx2 = ((this + IDX) as *const u32).read_unaligned();
        let slot2 = this.wrapping_add(idx2 << 6);
        let src2 = ((this + SRC) as *const u32).read_unaligned();
        let v = (((src2.wrapping_add(0xc)) as *const u32).read_unaligned())
            .wrapping_sub(
                ((slot2.wrapping_add(SLOT_ACC)) as *const u32).read_unaligned(),
            );
        ((slot2.wrapping_add(SLOT_REM)) as *mut u32).write_unaligned(v);
        let div = ((this + DIV) as *const u32).read_unaligned();
        ((this + IDX) as *mut u32)
            .write_unaligned(idx2.wrapping_add(1) % div);
    }
});
