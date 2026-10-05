// original: 0x009E5090 ped_task_state_snapshot (proposed)

/// Copy a ped's task-related state into this object's snapshot buffer.
///
/// `this` is the snapshot object, `arg0` the ped (or null, which answers
/// immediately, as does a ped whose sub-object at `+0x228` is null).
///
/// Three lookup callees fill the header: the first answers a two-word
/// descriptor copied to `+0x0`/`+0x4`, the next two answer words stored at
/// `+0x8` and `+0x10`. Then a field block is copied verbatim from the
/// sub-object (words, bytes and dwords at fixed offsets, with gaps left
/// untouched), two writable-global dwords land at `+0x30`/`+0x34`, and four
/// words (one plain, two float bit-copies, one plain) arrive from the object
/// at `arg0 + 0x20`. A virtual float getter (slot `+0xfc`) supplies `+0x50`,
/// a dword and a byte follow at `+0x54`/`+0x58`, and a ten-entry table is
/// built from every twelfth dword at `arg0 + 0x2d4`, each paired with the
/// low 16 bits of a per-entry callee answer. Bit 15 of the dword at
/// `arg0 + 0x264` becomes the byte at `+0x98`. When the dword at
/// `arg0 + 0xe48` is not -1, a seven-word registration call runs first;
/// either way a lookup result lands at `+0xa4`, two marker words of -1 at
/// `+0x9c`/`+0xa0`, and eleven pairs of callee low bytes at `+0xa8`/`+0xb3`.
///
/// Nothing is returned (the function is void; the early exits even leave the
/// incoming `eax` in place, which no caller can rely on).
///
/// Original: 0x009E5090 (thiscall, this + one stack word).
lf_checker_rt::export!(thiscall, rw_009E5090(this: u32, arg0: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x228;
        const CTX_OFF: u32 = 0x20;
        const VT_SLOT: u32 = 0xFC;
        const GLOB_A: u32 = 0x0103B6EC;
        const GLOB_B: u32 = 0x0103B6F0;
        const FLAG_OFF: u32 = 0x264;
        const FLAG_BIT: u32 = 15;
        const SEL_OFF: u32 = 0xE48;
        const SEL_AUX: u32 = 0xE4C;
        const SEL_ABSENT: u32 = 0xFFFF_FFFF;
        const DESC_CALLEE: u32 = 1;
        const WORD1_CALLEE: u32 = 2;
        const WORD2_CALLEE: u32 = 3;
        const FLOAT_CALLEE: u32 = 4;
        const ENTRY_CALLEE: u32 = 5;
        const REG_CALLEE: u32 = 6;
        const LOOKUP_CALLEE: u32 = 7;
        const EXTRA_CALLEE: u32 = 8;
        const PAIR0_CALLEE: u32 = 9;
        const PAIR1_CALLEE: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        if arg0 == 0 {
            return 0;
        }
        let sub = rd32(arg0.wrapping_add(SUB_OFF));
        if sub == 0 {
            return 0;
        }
        let desc: u32 = lf_checker_rt::callee_thiscall!(DESC_CALLEE, u32, sub);
        wr32(this, rd32(desc));
        wr32(this.wrapping_add(4), rd32(desc.wrapping_add(4)));
        let w1: u32 = lf_checker_rt::callee_thiscall!(WORD1_CALLEE, u32, sub);
        wr32(this.wrapping_add(8), w1);
        let w2: u32 = lf_checker_rt::callee_thiscall!(WORD2_CALLEE, u32, sub);
        wr32(this.wrapping_add(0x10), w2);
        wr16(this.wrapping_add(0x0C), rd16(sub.wrapping_add(0x540)));
        wr8(this.wrapping_add(0x14), rd8(sub.wrapping_add(0x550)));
        wr32(this.wrapping_add(0x18), rd32(sub.wrapping_add(0x4C0)));
        wr32(this.wrapping_add(0x1C), rd32(sub.wrapping_add(0x4C4)));
        wr8(this.wrapping_add(0x20), rd8(sub.wrapping_add(0x552)));
        wr8(this.wrapping_add(0x21), rd8(sub.wrapping_add(0x553)));
        wr8(this.wrapping_add(0x22), rd8(sub.wrapping_add(0x554)));
        wr16(this.wrapping_add(0x24), rd16(sub.wrapping_add(0x556)));
        wr16(this.wrapping_add(0x26), rd16(sub.wrapping_add(0x558)));
        wr8(this.wrapping_add(0x28), rd8(sub.wrapping_add(0x55A)));
        wr8(this.wrapping_add(0x29), rd8(sub.wrapping_add(0x55B)));
        wr8(this.wrapping_add(0x2A), rd8(sub.wrapping_add(0x55C)));
        wr8(this.wrapping_add(0x2B), rd8(sub.wrapping_add(0x55D)));
        wr8(this.wrapping_add(0x2C), rd8(sub.wrapping_add(0x564)));
        wr16(this.wrapping_add(0x2E), rd16(sub.wrapping_add(0x566)));
        wr32(
            this.wrapping_add(0x30),
            rd32(lf_checker_rt::relocated(GLOB_A)),
        );
        wr32(
            this.wrapping_add(0x34),
            rd32(lf_checker_rt::relocated(GLOB_B)),
        );
        let ctx = rd32(arg0.wrapping_add(CTX_OFF));
        wr32(this.wrapping_add(0x40), rd32(ctx.wrapping_add(0x30)));
        wr32(this.wrapping_add(0x44), rd32(ctx.wrapping_add(0x34)));
        wr32(this.wrapping_add(0x48), rd32(ctx.wrapping_add(0x38)));
        wr32(this.wrapping_add(0x4C), rd32(ctx.wrapping_add(0x3C)));
        let table = rd32(arg0);
        let target = rd32(table.wrapping_add(VT_SLOT));
        let get_float: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(target as usize) };
        let fv = get_float(arg0);
        wr32(this.wrapping_add(0x50), fv.to_bits());
        wr32(this.wrapping_add(0x54), rd32(arg0.wrapping_add(0xB84)));
        wr8(this.wrapping_add(0x58), rd8(arg0.wrapping_add(0x2B0)));
        let gate = arg0.wrapping_add(0x2B0);
        let mut src = arg0.wrapping_add(0x2D4);
        let mut dst = this.wrapping_add(0x5C);
        let mut wdst = this.wrapping_add(0x84);
        let mut k = 0u32;
        while k < 10 {
            wr32(dst, rd32(src));
            let ans: u32 = lf_checker_rt::callee_thiscall!(ENTRY_CALLEE, u32, gate, k);
            wr16(wdst, (ans & 0xFFFF) as u16);
            src = src.wrapping_add(0x0C);
            dst = dst.wrapping_add(4);
            wdst = wdst.wrapping_add(2);
            k += 1;
        }
        let flags = rd32(arg0.wrapping_add(FLAG_OFF));
        wr8(this.wrapping_add(0x98), ((flags >> FLAG_BIT) & 1) as u8);
        let sel = rd32(arg0.wrapping_add(SEL_OFF));
        if sel == SEL_ABSENT {
            let found: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, arg0);
            wr32(this.wrapping_add(0xA4), found);
        } else {
            let aux = rd32(arg0.wrapping_add(SEL_AUX));
            lf_checker_rt::callee_cdecl!(REG_CALLEE, u32, arg0, 0u32, sel, aux, SEL_ABSENT, 0u32, 0u32);
            let found: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, arg0);
            wr32(this.wrapping_add(0xA4), found);
            lf_checker_rt::callee_cdecl!(EXTRA_CALLEE, u32, arg0, 0u32, 0u32);
        }
        wr32(this.wrapping_add(0x9C), SEL_ABSENT);
        wr32(this.wrapping_add(0xA0), SEL_ABSENT);
        let mut j = 0u32;
        while j < 11 {
            let b0: u32 = lf_checker_rt::callee_cdecl!(PAIR0_CALLEE, u32, arg0, j);
            wr8(this.wrapping_add(j).wrapping_add(0xA8), (b0 & 0xFF) as u8);
            let b1: u32 = lf_checker_rt::callee_cdecl!(PAIR1_CALLEE, u32, arg0, j);
            wr8(this.wrapping_add(j).wrapping_add(0xB3), (b1 & 0xFF) as u8);
            j += 1;
        }
        0
    }
});
