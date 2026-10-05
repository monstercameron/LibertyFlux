// original: 0x008A61F0 aud_populate_bank_entries (proposed)

/// Copy per-slot parameter blocks into a bank-table entry, resolving
/// three words per slot through virtual slot `+0x10`.
///
/// `this` is an audio object and `a0` points at the parameter list. The
/// byte at `a0+4` is the slot count: it is stored to `this+0xB4` and a
/// zero count returns at once. Otherwise the destination base is
/// `table[IDX1] + STRIDE * COUNT1 + 0x14`, where `IDX1` is the byte at
/// `this+0x40` selecting one of 256 banks (each `BANK_STRIDE` = 0x6F40
/// bytes) in the table at global `AUD_TABLE` (0x115D988), the slot sits
/// at bank offset `BANK_ENTRY` = 0x6F10, the stride comes from global
/// `AUD_STRIDE` (0x115D964), and `COUNT1` is the byte at `this+0xB5`.
///
/// Each slot reads 0x1D bytes from `a0+5` onward and writes 0x18 bytes at
/// the destination: the first byte goes (zero-extended) to `dst-4`; then
/// three words at source offsets `+0`, `+8`, `+0x10` each either resolve
/// through the virtual call (callee id 1, thiscall on `this`, one word)
/// into `dst-0x14`, `dst-0x10`, `dst-0xC` when nonzero, or copy a default
/// word from source offsets `-4`, `+4`, `+0xC` when zero; a zero answer
/// from the call stores `1.0f` instead. Flag bits 0, 1, 2 of the byte at
/// `dst` record, per word, whether the default path was taken (set) or a
/// nonzero answer arrived (cleared). A fourth word at source `+0x14` is
/// always resolved through the same call into `dst-8` without any test.
///
/// Original: 0x008A61F0 (thiscall, `this` in ECX, one stack word, callee
/// pops 4, no return value).
lf_checker_rt::export!(thiscall, rw_008A61F0(this: u32, a0: u32) -> u32 {
    unsafe {
        const AUD_STRIDE: u32 = 0x115D964;
        const AUD_TABLE: u32 = 0x115D988;
        const IDX1: u32 = 0x40;
        const COUNT_OUT: u32 = 0xB4;
        const COUNT1: u32 = 0xB5;
        const BANK_STRIDE: u32 = 0x6F40;
        const BANK_ENTRY: u32 = 0x6F10;
        const DST_BIAS: u32 = 0x14;
        const SRC_STRIDE: u32 = 0x1D;
        const DST_STRIDE: u32 = 0x18;
        const VF_SLOT: u32 = 0x10;
        const ONE_F: u32 = 0x3F80_0000;

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
        unsafe fn vf(this: u32, word: u32) -> u32 {
            unsafe {
                let vtable = rd32(this);
                let slot = rd32(vtable.wrapping_add(VF_SLOT));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(this, word)
            }
        }
        /// One conditional word: resolve nonzero sources through the
        /// virtual call, else copy the default; maintain one flag bit.
        #[inline(always)]
        unsafe fn cond_word(this: u32, dst_flag: u32, bit: u8, src: u32, dst: u32, def: u32) {
            unsafe {
                let w = rd32(src);
                if w == 0 {
                    wr32(dst, rd32(def));
                    (dst_flag as *mut u8).write(rd8(dst_flag) | bit);
                } else {
                    let ans = vf(this, w);
                    if ans == 0 {
                        wr32(dst, ONE_F);
                        (dst_flag as *mut u8).write(rd8(dst_flag) | bit);
                    } else {
                        wr32(dst, ans);
                        (dst_flag as *mut u8).write(rd8(dst_flag) & !bit);
                    }
                }
            }
        }

        let count = rd8(a0 + 4);
        ((this + COUNT_OUT) as *mut u8).write(count);
        if count == 0 {
            return 0;
        }
        let count1 = rd8(this + COUNT1);
        let idx1 = rd8(this + IDX1);
        let stride = lf_checker_rt::global::<u32>(AUD_STRIDE).read();
        let base = lf_checker_rt::global::<u32>(AUD_TABLE).read();
        let bank = (idx1 as u32).wrapping_mul(BANK_STRIDE);
        let entry = rd32(base.wrapping_add(bank).wrapping_add(BANK_ENTRY));
        let mut dst = entry
            .wrapping_add(stride.wrapping_mul(count1 as u32))
            .wrapping_add(DST_BIAS);
        let mut src = a0.wrapping_add(10);
        let mut i: u32 = 0;
        loop {
            wr32(dst.wrapping_sub(4), rd8(src.wrapping_sub(5)) as u32);
            cond_word(this, dst, 1, src, dst.wrapping_sub(0x14), src.wrapping_sub(4));
            cond_word(this, dst, 2, src.wrapping_add(8), dst.wrapping_sub(0x10), src.wrapping_add(4));
            cond_word(this, dst, 4, src.wrapping_add(0x10), dst.wrapping_sub(0xC), src.wrapping_add(0xC));
            wr32(dst.wrapping_sub(8), vf(this, rd32(src.wrapping_add(0x14))));
            i += 1;
            src = src.wrapping_add(SRC_STRIDE);
            dst = dst.wrapping_add(DST_STRIDE);
            if i >= count as u32 {
                break;
            }
        }
        0
    }
});
