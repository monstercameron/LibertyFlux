// original: 0x0089B3B0 rage::audEnvelopeSound::vf7 (symbols)

/// Start an envelope sound from a parameter block.
///
/// `this` is an `audEnvelopeSound`, `arg1`/`arg2` are opaque words passed
/// through to the base start helper, and `arg3` points at a parameter
/// block. The base helper (callee 0, `thiscall` with the three words)
/// runs first; a zero answer returns 0. Otherwise a voice is allocated
/// through the allocator helper (callee 1, `thiscall` on the global
/// allocator with `0xA4` and the category byte at `+0x40`); a null voice
/// returns 0. The voice is initialised (callee 2), the first 24 bytes of
/// the parameter block are copied as three 8-byte words to voice bytes
/// `+0x78..+0x8F`, and the voice's slot index `(voice - row_base) / stride`
/// (unsigned divide; `row_base` from the global table by category,
/// `stride` global, both at `G_TABLE_BASE`/`G_STRIDE`) is stored at
/// `+0xF7`. The dword at descriptor word `+0x2D` (descriptor pointer from
/// `+0x94`) selects an attach path: `-1` or `0` skips it, anything else
/// runs the attach helper (callee 3, `thiscall` on the global registry
/// with that word, `this`, `arg2`, `arg3`), the link helper (callee 4,
/// `thiscall` on `this` with `0` and the attach answer) and the gate
/// helper (callee 5, `thiscall` on `this` with `0`), whose zero answer
/// returns 0. Then descriptor words/bytes feed `this` fields (`+0xF0`,
/// `+0xF2`, `+0xF4`, `+0xE8`, `+0xEC`), five descriptor dwords (at
/// `+0x19`, `+0x1D`, `+0x25`, `+0x21`, `+0x29`, in that order) each go
/// through virtual slot `+0x10` of `this` with the answers stored at
/// `+0xD4..+0xE4`, and three descriptor dwords (`+0xD`, `+0x11`, `+0x15`)
/// go to the part helper (callee 7, `thiscall` on the voice at `+0`,
/// `+0x28`, `+0x50`). Returns 1 when voice bytes `+0x26`, `+0x76` and
/// `+0x4E` are all non-zero, else 0. All comparisons on helper answers
/// are zero/non-zero tests, and the slot divide cannot overflow (zero
/// high half, non-zero divisor), so no signedness applies. The original
/// spills the descriptor pointer into its incoming third-argument slot;
/// the rewrite keeps it in a local (the stack check is off for that
/// reason; the spilled value is observed through the descriptor reads
/// and the call arguments instead).
///
/// Original: 0x0089B3B0 (thiscall, three stack words, returns `al`).
lf_checker_rt::export!(thiscall, rw_0089B3B0(this: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const CAT_INDEX: u32 = 0x40;
        const DESCRIPTOR: u32 = 0x94;
        const SLOT_OUT: u32 = 0xF7;
        const CAT_STRIDE: u32 = 0x6F40;
        const TABLE_ROW: u32 = 0x6F14;
        const ALLOC_SIZE: u32 = 0xA4;
        const G_ALLOCATOR: u32 = 0x115D8A0;
        const G_REGISTRY: u32 = 0x115DC18;
        const G_STRIDE: u32 = 0x115D968;
        const G_TABLE_BASE: u32 = 0x115D988;
        const VTABLE_SLOT: u32 = 0x10;
        const CALLEE_BASE: u32 = 0;
        const CALLEE_ALLOC: u32 = 1;
        const CALLEE_INIT: u32 = 2;
        const CALLEE_ATTACH: u32 = 3;
        const CALLEE_LINK: u32 = 4;
        const CALLEE_GATE: u32 = 5;
        const CALLEE_PART: u32 = 7;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let base_ok: u32 = lf_checker_rt::callee_thiscall!(CALLEE_BASE, u32, this, arg1, arg2, arg3);
        if (base_ok & 0xFF) == 0 {
            return 0;
        }
        let cat = rd8(this + CAT_INDEX) as u32;
        let voice: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_ALLOC,
            u32,
            lf_checker_rt::relocated(G_ALLOCATOR),
            ALLOC_SIZE,
            cat
        );
        if voice == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CALLEE_INIT, u32, voice);
        wr64(voice + 0x78, rd64(arg3));
        wr64(voice + 0x80, rd64(arg3 + 8));
        wr64(voice + 0x88, rd64(arg3 + 0x10));
        let table_base = (lf_checker_rt::global::<u32>(G_TABLE_BASE)).read_unaligned();
        let row = rd32(table_base.wrapping_add(cat.wrapping_mul(CAT_STRIDE)).wrapping_add(TABLE_ROW));
        let stride = (lf_checker_rt::global::<u32>(G_STRIDE)).read_unaligned();
        let slot = voice.wrapping_sub(row) / stride;
        ((this + SLOT_OUT) as *mut u8).write(slot as u8);
        let desc = rd32(this + DESCRIPTOR);
        let sel = rd32(desc + 0x2D);
        if sel != 0xFFFF_FFFF && sel != 0 {
            let attached: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_ATTACH,
                u32,
                lf_checker_rt::relocated(G_REGISTRY),
                sel,
                this,
                arg2,
                arg3
            );
            lf_checker_rt::callee_thiscall!(CALLEE_LINK, u32, this, 0, attached);
            let gate: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GATE, u32, this, 0);
            if gate == 0 {
                return 0;
            }
        }
        ((this + 0xF0) as *mut u16).write_unaligned(rd16(desc));
        ((this + 0xF2) as *mut u16).write_unaligned(rd16(desc + 2));
        ((this + 0xF4) as *mut u8).write(rd8(desc + 4));
        wr32(this + 0xE8, rd32(desc + 5));
        wr32(this + 0xEC, rd32(desc + 9));
        // Five virtual calls through slot +0x10, in descriptor order.
        let vtable = rd32(this);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + VTABLE_SLOT) as usize);
        wr32(this + 0xD4, hook(this, rd32(desc + 0x19)));
        wr32(this + 0xD8, hook(this, rd32(desc + 0x1D)));
        wr32(this + 0xDC, hook(this, rd32(desc + 0x25)));
        wr32(this + 0xE0, hook(this, rd32(desc + 0x21)));
        wr32(this + 0xE4, hook(this, rd32(desc + 0x29)));
        lf_checker_rt::callee_thiscall!(CALLEE_PART, u32, voice, rd32(desc + 0xD));
        lf_checker_rt::callee_thiscall!(CALLEE_PART, u32, voice + 0x28, rd32(desc + 0x11));
        lf_checker_rt::callee_thiscall!(CALLEE_PART, u32, voice + 0x50, rd32(desc + 0x15));
        if rd8(voice + 0x26) == 0 || rd8(voice + 0x76) == 0 || rd8(voice + 0x4E) == 0 {
            0
        } else {
            1
        }
    }
});
