// original: 0x00a1e310 ped_task_reset_state (proposed)

/// Reset a ped task record to its initial state.
///
/// `this` points to the task record (about 0x398 bytes). The function
/// clears some ninety fields, sets a handful of sentinel words
/// (0xffffffff), three unit floats (1.0), one FLT_MAX bound and two copies
/// of one tuning constant, adjusts four flag bytes, and copies one global
/// byte (sign-extended), one table float chosen by that byte, and four
/// global words into their slots. It returns 1 in the low byte.
///
/// Three conditional behaviours break the straight line:
///
/// - When the link word at `this+0x110` is zero on entry, the allocator
///   callee (id 1, thiscall on `this`) fills it in, the link is brought up
///   through callee id 2 (thiscall on the link), four words of the linked
///   record are initialised, and the link is pointed back at `this+0x10`
///   and `this`. Bit 0 of the flag byte at `this+0x13c` is set. A nonzero
///   link skips all of this.
/// - The global byte selects a float from a read-only table (negative
///   indexes read below the table base, as the original does); the float
///   lands in the two slots at `this+0x31c` and `this+0x320`, and the low
///   two bits of the flag byte at `this+0x38d` become binary `100`.
/// - The finaliser callee (id 3, thiscall on `this`) runs, then the
///   notifier callee (id 4, thiscall on the object at `this+0x12c` with the
///   sign-extended global byte and a zero word). The function's upper
///   return bytes are the notifier's return with the low byte replaced by
///   1, reproduced exactly.
///
/// The code pointer stored at `this+0x1e4` is written as the same constant
/// word the original stores; nothing dereferences it during the call.
///
/// Original: 0x00a1e310 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1e310(this: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x110;
        const NOTIFY_OBJ: u32 = 0x12c;
        const ONE_BITS: u32 = 0x3f800000;
        const NEG_ONE: u32 = 0xffffffff;
        const BOUND_MAX: u32 = 0x7f7fffff;
        const TUNE: u32 = 0xc61c3c00;
        const HANDLER: u32 = 0xa1e970;
        const GLOBAL_BYTE: u32 = 0x103bff8;
        const FLOAT_TABLE: u32 = 0xe9b288;
        const GLOBAL_QUAD: u32 = 0x1b4b2a0;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        wr8(this + 0x38e, rd8(this + 0x38e) & 0xcf);
        wr32(this + 0x330, 0);
        wr32(this + 0x380, 0);
        wr32(this + 0x2a8, 0);
        wr32(this + 0x2a4, 0);
        wr32(this + 0x2a0, 0);
        wr32(this + 0x2b8, 0);
        wr32(this + 0x2b4, 0);
        wr32(this + 0x2b0, 0);
        wr8(this + 0x38e, rd8(this + 0x38e) & 0xf0);
        wr32(this + 0x2c4, ONE_BITS);
        wr32(this + 0x348, NEG_ONE);
        wr32(this + 0x394, NEG_ONE);
        wr32(this + 0x34c, NEG_ONE);
        wr32(this + 0x350, NEG_ONE);
        wr32(this + 0x388, 0);
        wr32(this + 0x298, 0);
        wr32(this + 0x294, 0);
        wr32(this + 0x290, 0);
        wr8(this + 0x392, 0);
        wr32(this + 0x358, NEG_ONE);
        wr32(this + 0x35c, NEG_ONE);
        wr32(this + 0x288, 0);
        wr32(this + 0x284, 0);
        wr32(this + 0x280, 0);
        wr8(this + 0x38d, rd8(this + 0x38d) & 0x1f);
        wr8(this + 0x38c, rd8(this + 0x38c) | 4);
        let linked = rd32(this + LINK);
        wr32(this + 0x368, NEG_ONE);
        wr32(this + 0x36c, 0);
        wr32(this + 0x2d8, 0);
        if linked == 0 {
            lf_checker_rt::callee_thiscall!(1, u32, this);
            let sub = rd32(this + LINK);
            lf_checker_rt::callee_thiscall!(2, u32, sub);
            wr8(sub + 0x27c, 0);
            wr32(sub + 0x270, 0);
            wr32(sub + 0x278, 0);
            wr32(sub + 0x280, NEG_ONE);
            wr32(rd32(this + LINK) + 0x270, this.wrapping_add(0x10));
            wr32(rd32(this + LINK) + 0x278, this);
            wr8(this + 0x13c, rd8(this + 0x13c) | 1);
        }

        let sel = rd8(lf_checker_rt::relocated(GLOBAL_BYTE)) as i8 as i32 as u32;
        wr8(this + 0x38c, rd8(this + 0x38c) | 0x0a);
        wr32(this + 0x360, sel);
        wr32(this + 0x384, 0);
        wr32(this + 0x2c8, 0);
        wr32(this + 0x304, 0);
        wr32(this + 0x308, 0);
        wr32(this + 0x30c, 0);
        wr32(this + 0x310, 0);
        let picked = rd32(
            lf_checker_rt::relocated(FLOAT_TABLE)
                .wrapping_add((sel as i32).wrapping_mul(4) as u32),
        );
        wr32(this + 0x31c, picked);
        wr32(this + 0x320, picked);
        wr8(this + 0x38d, (rd8(this + 0x38d) & 0xfc) | 4);
        wr32(this + 0x1f8, 0);
        wr32(this + 0x1f4, 0);
        wr32(this + 0x1f0, 0);
        let gbase = lf_checker_rt::relocated(GLOBAL_QUAD);
        let g0 = rd32(gbase);
        let g1 = rd32(gbase + 4);
        let g2 = rd32(gbase + 8);
        let g3 = rd32(gbase + 12);
        wr32(this + 0x230, g0);
        wr32(this + 0x234, g1);
        wr32(this + 0x238, g2);
        wr32(this + 0x23c, g3);
        wr32(this + 0x370, 0);
        wr32(this + 0x240, g0);
        wr32(this + 0x244, g1);
        wr32(this + 0x248, g2);
        wr32(this + 0x24c, g3);
        wr32(this + 0x374, 0);
        wr32(this + 0x250, g0);
        wr32(this + 0x254, g1);
        wr32(this + 0x258, g2);
        wr32(this + 0x25c, g3);
        wr32(this + 0x378, 0);
        wr32(this + 0x260, g0);
        wr32(this + 0x264, g1);
        wr32(this + 0x268, g2);
        wr32(this + 0x26c, g3);
        wr32(this + 0x37c, 0);
        wr8(this + 0x38c, rd8(this + 0x38c) & 0x7f);
        wr8(this + 0x38e, rd8(this + 0x38e) & 0x7f);
        wr16(this + 0x390, 0);
        wr8(this + 0x38f, 0);
        wr32(this + 0x300, 0);
        wr32(this + 0x334, BOUND_MAX);
        wr32(this + 0x314, TUNE);
        wr32(this + 0x318, TUNE);
        lf_checker_rt::callee_thiscall!(3, u32, this);
        let notify: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, rd32(this + NOTIFY_OBJ), rd32(this + 0x360), 0);
        wr32(this + 0x1e0, 0);
        wr32(this + 0x1e4, HANDLER);
        wr8(this + 0x38d, rd8(this + 0x38d) & 0xe7);
        wr32(this + 0x324, 0);
        wr32(this + 0x328, 0);
        wr32(this + 0x278, 0);
        wr32(this + 0x274, 0);
        wr32(this + 0x270, 0);
        wr32(this + 0x2d4, ONE_BITS);
        wr32(this + 0x2fc, 0);
        wr32(this + 0x2dc, 0);
        wr32(this + 0x2e0, 0);
        wr8(this + 0x38c, rd8(this + 0x38c) & 0xae);
        wr32(this + 0x364, 0);
        wr32(this + 0x208, 0);
        wr32(this + 0x204, 0);
        wr32(this + 0x200, 0);
        wr32(this + 0x2cc, ONE_BITS);
        wr32(this + 0x218, 0);
        wr32(this + 0x214, 0);
        wr32(this + 0x210, 0);
        wr32(this + 0x228, 0);
        wr32(this + 0x224, 0);
        wr32(this + 0x220, 0);
        wr8(this + 0x38c, rd8(this + 0x38c) & 0xdf);
        wr8(this + 0x38e, rd8(this + 0x38e) & 0xbf);
        wr32(this + 0x2c0, 0);
        wr32(this + 0x2e4, 0);
        wr32(this + 0x2e8, 0);
        wr32(this + 0x2ec, 0);
        wr32(this + 0x2f0, 0);
        wr32(this + 0x2f4, 0);
        wr32(this + 0x354, 0);
        wr32(this + 0x32c, ONE_BITS);
        wr32(this + 0x338, 0);
        wr32(this + 0x344, 0);
        wr32(this + 0x340, 0);
        wr32(this + 0x33c, 0);
        (notify & 0xffffff00) | 1
    }
});
