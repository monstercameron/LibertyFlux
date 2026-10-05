// original: 0x008A9D40 aud_manager_init (proposed)

/// One-time initialization of the audio manager: allocate the work
/// buffer, build the free-entry list, and register with two globals.
///
/// If the initialized byte at `this+0x3230` is already set, returns 1 at
/// once. Otherwise helper 1 (callee id 1, cdecl, one word: the old
/// pointer at `this+0x28A0`) releases the previous buffer, helper 2
/// (id 2, cdecl, one word: 100, computed as 0x19 times 4 with an
/// overflow check that cannot fire) allocates the new one, and helper 3
/// (id 3, cdecl, three words: buffer, 0, 100, called twice) fills it
/// with zeros; the low bit of its first word is then set. The header at
/// `this+0x28A4` becomes 0x03200019 (count 0x19 in the low word),
/// `this+0xFA0` is zeroed, `this+0xFA4` gets the end marker 0xFFFF, the
/// free head at `this+0x320C` becomes 0, entries `this[0]`..`this[999]`
/// are chained with values 1..999 (words spaced 4 apart) terminated by
/// 0xFFFF at `this+0xF9C`, and 0x258 dwords at `this+0x28A8` are zeroed.
/// Finally the initialized byte is set, `this` is published to the
/// global at file address 0x115F848, the counter at 0x115F85C is copied
/// to 0x115F84C and incremented, and helpers 4 and 5 (ids 4 and 5,
/// cdecl, one word each: the global at 0x115F860) run. Returns 1.
///
/// Original: 0x008A9D40 (thiscall, `this` in ECX, no stack words,
/// plain `ret`, boolean in AL).
lf_checker_rt::export!(thiscall, rw_008A9D40(this: u32) -> u32 {
    unsafe {
        const G_SLOT: u32 = 0x115F848;
        const G_COPY: u32 = 0x115F84C;
        const G_COUNT: u32 = 0x115F85C;
        const G_ARG: u32 = 0x115F860;
        const INIT: u32 = 0x3230;
        const BUF: u32 = 0x28A0;
        const HDR: u32 = 0x28A4;
        const ZERO0: u32 = 0xFA0;
        const MARK: u32 = 0xFA4;
        const FREE_HEAD: u32 = 0x320C;
        const LIST_END: u32 = 0xF9C;
        const CLEAR_BASE: u32 = 0x28A8;
        const CLEAR_WORDS: u32 = 0x258;
        const ENTRIES: u32 = 0x3E7;
        const HDR_VAL: u32 = 0x0320_0019;
        const ALLOC_N: u32 = 0x19;
        const ALLOC_W: u32 = 4;
        const END: u16 = 0xFFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if unsafe { ((this + INIT) as *const u8).read() } != 0 {
            return 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, rd32(this + BUF));
        // 0x19 * 4 with overflow folded in (cannot overflow: 100 fits).
        let (prod, ov) = ALLOC_N.overflowing_mul(ALLOC_W);
        let size = prod | (if ov { 0xFFFF_FFFF } else { 0 });
        (this.wrapping_add(HDR) as *mut u32).write_unaligned(HDR_VAL);
        let buf: u32 = lf_checker_rt::callee_cdecl!(2, u32, size);
        let count = (rd32(this + HDR) & 0xFFFF).wrapping_mul(4);
        (this.wrapping_add(BUF) as *mut u32).write_unaligned(buf);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, buf, 0, count);
        let count2 = (rd32(this + HDR) & 0xFFFF).wrapping_mul(4);
        let buf2 = rd32(this + BUF);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, buf2, 0, count2);
        let b0 = rd32(buf2);
        (buf2 as *mut u32).write_unaligned(b0 | 1);
        (this.wrapping_add(ZERO0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(MARK) as *mut u16).write_unaligned(END);
        (this.wrapping_add(FREE_HEAD) as *mut u32).write_unaligned(0);
        let mut i: u32 = 0;
        while i < ENTRIES {
            (this.wrapping_add(i.wrapping_mul(4)) as *mut u16)
                .write_unaligned((i + 1) as u16);
            i += 1;
        }
        (this.wrapping_add(LIST_END) as *mut u16).write_unaligned(END);
        let mut c = CLEAR_BASE;
        let end = CLEAR_BASE.wrapping_add(CLEAR_WORDS.wrapping_mul(4));
        while c < end {
            (this.wrapping_add(c) as *mut u32).write_unaligned(0);
            c += 4;
        }
        ((this + INIT) as *mut u8).write(1);
        let garg = lf_checker_rt::global::<u32>(G_ARG).read();
        lf_checker_rt::global::<u32>(G_SLOT).write(this);
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, garg);
        let n = lf_checker_rt::global::<u32>(G_COUNT).read();
        let garg2 = lf_checker_rt::global::<u32>(G_ARG).read();
        lf_checker_rt::global::<u32>(G_COPY).write(n);
        lf_checker_rt::global::<u32>(G_COUNT).write(n.wrapping_add(1));
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32, garg2);
        1
    }
});
