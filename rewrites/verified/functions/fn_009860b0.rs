// original: 0x009860B0 audEmitterAudioEntity::vf1

/// Emitter audio entity virtual slot 1: (re)allocates the shared buffer and
/// initialises the slot table, then runs the base initialiser.
///
/// When the byte at `+SKIP_OFF` of `this` is set, the whole allocation block
/// is skipped. Otherwise the old buffer word at `+BUFFER_OFF` is handed to
/// the free callee (id 1, cdecl), `ALLOC_ELEMS * ELEM_BYTES` bytes are
/// requested from the allocator (id 2, cdecl; the original folds a multiply
/// with an overflow check over two constants, which cannot overflow), the
/// count word at `+COUNT_OFF` is set to `COUNT_MAGIC` whose low word times 4
/// is the fill length for the fill callee (id 3, cdecl: buffer, 0, length),
/// the fresh buffer is stored back, three header words are cleared, and 0xff
/// table words are cleared at stride `TABLE_STRIDE` from `+TABLE_OFF`. Always
/// afterwards two flag bytes are cleared, the global at `GLOBAL_ZERO` is
/// cleared, and the base initialiser (id 4, thiscall on `this + 8`) runs with
/// its descriptor argument. No meaningful return value.
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_009860B0(this: u32) -> u32 {
    const SKIP_OFF: u32 = 0x4a552;
    const BUFFER_OFF: u32 = 0x38240;
    const COUNT_OFF: u32 = 0x38244;
    const ALLOC_ELEMS: u32 = 0x40;
    const ELEM_BYTES: u32 = 4;
    const COUNT_MAGIC: u32 = 0x08000040;
    const TABLE_OFF: u32 = 0x3a34c;
    const TABLE_STRIDE: u32 = 0x104;
    const TABLE_ITERS: u32 = 0xff;
    const GLOBAL_ZERO: u32 = 0x1238950;
    const DESCRIPTOR: u32 = 0xe8e0b0;
    const FREE_OLD: u32 = 1;
    const ALLOC_NEW: u32 = 2;
    const FILL_BUF: u32 = 3;
    const BASE_INIT: u32 = 4;
    unsafe {
        if ((this + SKIP_OFF) as *const u8).read() == 0 {
            let old = ((this + BUFFER_OFF) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(FREE_OLD, u32, old);
            let bytes = ALLOC_ELEMS.wrapping_mul(ELEM_BYTES);
            ((this + COUNT_OFF) as *mut u32).write_unaligned(COUNT_MAGIC);
            let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC_NEW, u32, bytes);
            let count =
                ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
            let _: u32 =
                lf_checker_rt::callee_cdecl!(FILL_BUF, u32, buf, 0, count << 2);
            ((this + BUFFER_OFF) as *mut u32).write_unaligned(buf);
            ((this + 0x3a248) as *mut u32).write_unaligned(0);
            ((this + 0x4a548) as *mut u32).write_unaligned(0);
            ((this + 0x4a54c) as *mut u32).write_unaligned(0);
            let mut p = this.wrapping_add(TABLE_OFF);
            let mut k = TABLE_ITERS;
            while k != 0 {
                (p as *mut u16).write_unaligned(0);
                p = p.wrapping_add(TABLE_STRIDE);
                k = k.wrapping_sub(1);
            }
        }
        ((this + 0x4a550) as *mut u8).write(0);
        *lf_checker_rt::global::<u32>(GLOBAL_ZERO) = 0;
        ((this + 0x4a551) as *mut u8).write(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            BASE_INIT,
            u32,
            this.wrapping_add(8),
            lf_checker_rt::relocated(DESCRIPTOR)
        );
    }
    0
});
