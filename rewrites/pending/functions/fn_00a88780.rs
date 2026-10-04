// original: 0x00a88780 input_ui_state_init (proposed)

/// Initialise an input/UI state object to its power-on state.
///
/// `this` points to a ~0xA34-byte object. Six header words (at `+0x20`,
/// `+0x24`, `+0x40`, `+0x44`, `+0x48`, `+0x4C`) are cleared, four embedded
/// buffers are zeroed (0x80 bytes at `+0x3A4`, 0x80 at `+0x19A`, 0x100 at
/// `+0x700`, 0x200 at `+0x800`), scattered flag/counter fields are cleared
/// (word at `+0x198`, dwords at `+0x194`, `+0x424`, byte at `+0x21A`, the
/// `+0xA00`..`+0xA14` block), then the registrar callee is invoked as
/// `registrar(this, 0, HANDLER, &out)` where `HANDLER` is a fixed code
/// address and `out` a stack slot; its return value lands at `+0xA08` and
/// the out word at `+0xA0C`. Finally the tail block is set: `+0xA1C`,
/// `+0xA2C`, `+0xA30` to zero, `+0xA20`/`+0xA24` to -1, byte `+0xA28` to 7.
/// Returns `this`.
///
/// The original zeroes the four buffers through the C library fill routine
/// (four call sites); the rewrite stores the zeroes directly, which the
/// checker observes as identical heap writes. The original's callee is
/// answered by script on both sides.
///
/// Original: 0x00A88780 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00a88780(this: u32) -> u32 {
    unsafe {
        const HANDLER: u32 = 0x0073c7a0;
        const REGISTRAR: u32 = 1;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn zero_range(base: u32, off: u32, len: u32) {
            unsafe {
                let mut p = base.wrapping_add(off);
                let end = p.wrapping_add(len);
                while p != end {
                    (p as *mut u8).write(0);
                    p = p.wrapping_add(1);
                }
            }
        }

        wr32(this.wrapping_add(0x20), 0);
        wr32(this.wrapping_add(0x24), 0);
        wr32(this.wrapping_add(0x40), 0);
        wr32(this.wrapping_add(0x44), 0);
        wr32(this.wrapping_add(0x48), 0);
        wr32(this.wrapping_add(0x4c), 0);
        zero_range(this, 0x3a4, 0x80);
        (this.wrapping_add(0x198) as *mut u16).write_unaligned(0);
        wr32(this.wrapping_add(0x424), 0);
        wr32(this.wrapping_add(0x194), 0);
        zero_range(this, 0x19a, 0x80);
        (this.wrapping_add(0x21a) as *mut u8).write(0);
        zero_range(this, 0x700, 0x100);
        wr32(this.wrapping_add(0xa08), 0);
        wr32(this.wrapping_add(0xa0c), 0);
        wr32(this.wrapping_add(0xa10), 0);
        wr32(this.wrapping_add(0xa14), 0);
        zero_range(this, 0x800, 0x200);
        wr32(this.wrapping_add(0xa00), 0);
        (this.wrapping_add(0xa04) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0xa18) as *mut u8).write(0);
        let mut out: u32 = 0;
        let code = lf_checker_rt::callee_cdecl!(
            REGISTRAR,
            u32,
            this,
            0u32,
            lf_checker_rt::relocated(HANDLER),
            core::ptr::addr_of_mut!(out) as u32
        );
        wr32(this.wrapping_add(0xa08), code);
        wr32(this.wrapping_add(0xa0c), out);
        wr32(this.wrapping_add(0xa1c), 0);
        wr32(this.wrapping_add(0xa20), 0xffff_ffff);
        wr32(this.wrapping_add(0xa24), 0xffff_ffff);
        (this.wrapping_add(0xa28) as *mut u8).write(7);
        wr32(this.wrapping_add(0xa2c), 0);
        wr32(this.wrapping_add(0xa30), 0);
        this
    }
});
