// original: 0x008A5E20 rage::audForLoopSound::vf7 (symbols)

/// Validate loop-sound parameters, snapshot them into the object, and
/// resolve the bank index for the loop slot.
///
/// `this` is a loop-sound object, `a0`/`a1` are opaque words passed to the
/// helpers, and `a2` points at a 24-byte parameter block. Helper 1 (callee
/// id 1, thiscall on `this` with `a0`, `a1`, `a2`) must answer with a
/// nonzero low byte or the function returns 0.
///
/// On success the 24 bytes at `a2` are copied to `this+0xB0`, then fields
/// are gathered from the descriptor at `this+0x94`: words at `+0`/`+4`
/// go to `this+0xC8`/`+0xE0`, words at `+0x14`/`+0xC` go to
/// `this+0xE4`/`+0xDC`, and four values pass through virtual slot `+0x10`
/// of `this` (callee id 2, thiscall, one word each: descriptor words at
/// `+8`, `+0x18`, `+0x10`, `+0x1C`) into `this+0xD0`, `+0xD4`, `+0xD8`,
/// `+0xCC`. A zero answer from the last of the four returns 0.
///
/// Helper 2 (callee id 3, thiscall on the global audio manager at
/// file address 0x115DC18, four words: `this+0xC8`, `this`, `a1`, `a2`)
/// resolves a raw entry pointer. A null answer selects index 0xFF;
/// otherwise the index is the low byte of
/// `(answer - table[IDX1]) / STRIDE`, an unsigned 32-bit divide with a
/// zero high word, where `IDX1` is the byte at `this+0x40` selecting one
/// of 256 banks (each `BANK_STRIDE` = 0x6F40 bytes) in the table at global
/// `AUD_TABLE` (0x115D988), the slot is at bank offset `BANK_ENTRY` =
/// 0x6F10, and the stride comes from global `AUD_STRIDE` (0x115D964).
/// The index is stored at `this+0x48`; 0xFF returns 0, as does a null
/// `table[IDX1] + STRIDE * index`. Otherwise the function returns 1.
///
/// Original: 0x008A5E20 (thiscall, `this` in ECX, three stack words,
/// callee pops 12, boolean in AL).
lf_checker_rt::export!(thiscall, rw_008A5E20(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const AUD_STRIDE: u32 = 0x115D964;
        const AUD_TABLE: u32 = 0x115D988;
        const AUD_MGR: u32 = 0x115DC18;
        const DESC: u32 = 0x94;
        const IDX1: u32 = 0x40;
        const IDX2: u32 = 0x48;
        const BANK_STRIDE: u32 = 0x6F40;
        const BANK_ENTRY: u32 = 0x6F10;
        const VF_SLOT: u32 = 0x10;
        const NONE: u8 = 0xFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn table_entry(idx1: u8) -> u32 {
            unsafe {
                let stride = idx1 as u32;
                let _ = stride;
                let base = lf_checker_rt::global::<u32>(AUD_TABLE).read();
                let bank = (idx1 as u32).wrapping_mul(BANK_STRIDE);
                rd32(base.wrapping_add(bank).wrapping_add(BANK_ENTRY))
            }
        }
        #[inline(always)]
        unsafe fn vf(this: u32, word: u32) -> u32 {
            unsafe {
                let vtable = rd32(this);
                let slot = rd32(vtable.wrapping_add(VF_SLOT));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(this, word)
            }
        }

        let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1, a2);
        if r1 & 0xFF == 0 {
            return 0;
        }
        // 24-byte parameter snapshot, three 8-byte moves.
        for k in 0..3u32 {
            let w = (a2.wrapping_add(k * 8) as *const u64).read_unaligned();
            (this.wrapping_add(0xB0).wrapping_add(k * 8) as *mut u64)
                .write_unaligned(w);
        }
        let desc = rd32(this + DESC);
        wr32(this + 0xC8, rd32(desc));
        wr32(this + 0xE0, rd32(desc + 4));
        wr32(this + 0xD0, vf(this, rd32(desc + 8)));
        wr32(this + 0xE4, rd32(desc + 0x14));
        wr32(this + 0xD4, vf(this, rd32(desc + 0x18)));
        wr32(this + 0xDC, rd32(desc + 0xC));
        wr32(this + 0xD8, vf(this, rd32(desc + 0x10)));
        let last = vf(this, rd32(desc + 0x1C));
        wr32(this + 0xCC, last);
        if last == 0 {
            return 0;
        }
        let mgr = lf_checker_rt::relocated(AUD_MGR);
        let r2: u32 = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            mgr,
            rd32(this + 0xC8),
            this,
            a1,
            a2
        );
        let idx: u8 = if r2 == 0 {
            NONE
        } else {
            let entry = table_entry(rd8(this + IDX1));
            let stride = lf_checker_rt::global::<u32>(AUD_STRIDE).read();
            (r2.wrapping_sub(entry) / stride) as u8
        };
        ((this + IDX2) as *mut u8).write(idx);
        if idx == NONE {
            return 0;
        }
        let entry = table_entry(rd8(this + IDX1));
        let stride = lf_checker_rt::global::<u32>(AUD_STRIDE).read();
        if entry.wrapping_add(stride.wrapping_mul(idx as u32)) == 0 {
            return 0;
        }
        1
    }
});
