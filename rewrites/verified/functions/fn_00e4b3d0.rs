// original: 0x00e4b3d0 RP_EXPORTHI

/// High-detail export record constructor: chain the base initializer, stamp
/// the vtable, clear the record, then fill three text slots.
///
/// `this` is the new record, `a0`/`a1` are forwarded to the base initializer
/// (callee 0) unchanged. Thiscall with two stack words (callee pops 8).
///
/// Behaviour: run callee 0 on (`a0`, `a1`), write the vtable pointer to
/// `+0x0`, run the member initializer (callee 1) on `this+0x1f8`, then clear
/// the record: dwords at `+0x1e0/+0x1e4/+0x1f0/+0x208w/+0x229b/+0x429b/
/// +0x430/+0x434w/+0x438/+0x43cw/+0x440/+0x444/+0x448/+0x44c` to zero and
/// `+0x42c` to all-ones. Then fill three text slots at `+0x438`, `+0x440`
/// and `+0x448`: each gets a default string via callee 2, then a lookup
/// (callee 3) whose handle is resolved (callee 4) to a C string; a non-null
/// string replaces the slot content with its length-prefixed form (length
/// from a byte scan for the terminator), a null one keeps the default.
/// Returns `this`.
///
/// Edge cases: a null lookup result skips the slot refill; an empty string
/// still refills with length 0.
lf_checker_rt::export!(thiscall, rw_00e4b3d0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00F1_84DC;
        const MEMBER_OFF: u32 = 0x1F8;
        const LOOKUP_THIS: u32 = 0x0116_BFF0;
        const SLOT0: u32 = 0x438;
        const SLOT1: u32 = 0x440;
        const SLOT2: u32 = 0x448;
        const DEFAULT0: u32 = 0x00F1_7AD0;
        const KEY0: u32 = 0x00F1_7AD4;
        const DEFAULT1: u32 = 0x00F1_7AE0;
        const KEY1: u32 = 0x00F1_7AE4;
        const DEFAULT2: u32 = 0x00F1_7AF4;
        const KEY2: u32 = 0x00F1_7AF8;
        const BASE_INIT: u32 = 0;
        const MEMBER_INIT: u32 = 1;
        const SLOT_FILL: u32 = 2;
        const LOOKUP: u32 = 3;
        const RESOLVE: u32 = 4;

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
        #[inline(always)]
        unsafe fn strlen(s: u32) -> u32 {
            unsafe {
                let mut p = s;
                while (p as *const u8).read() != 0 {
                    p = p.wrapping_add(1);
                }
                p.wrapping_sub(s)
            }
        }
        #[inline(always)]
        unsafe fn fill_slot(this: u32, off: u32, default: u32, key: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    SLOT_FILL,
                    u32,
                    this + off,
                    lf_checker_rt::relocated(default),
                    1
                );
                let h = lf_checker_rt::callee_thiscall!(
                    LOOKUP,
                    u32,
                    lf_checker_rt::relocated(LOOKUP_THIS),
                    lf_checker_rt::relocated(key)
                );
                let s = lf_checker_rt::callee_cdecl!(RESOLVE, u32, h, 0, 0);
                if s != 0 {
                    let n = strlen(s);
                    lf_checker_rt::callee_thiscall!(SLOT_FILL, u32, this + off, s, n);
                }
            }
        }

        lf_checker_rt::callee_thiscall!(BASE_INIT, u32, this, a0, a1);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_INIT, u32, this + MEMBER_OFF);
        wr32(this + 0x43C, 0);
        wr32(this + 0x438, 0);
        wr32(this + 0x440, 0);
        wr32(this + 0x444, 0);
        wr32(this + 0x448, 0);
        wr32(this + 0x44C, 0);
        wr32(this + 0x1E0, 0);
        wr8(this + 0x429, 0);
        wr32(this + 0x1E4, 0);
        wr16(this + 0x208, 0);
        wr16(this + 0x434, 0);
        wr32(this + 0x430, 0);
        wr32(this + 0x1F0, 0);
        wr32(this + 0x42C, 0xFFFF_FFFF);
        wr8(this + 0x229, 0);
        wr16(this + 0x43C, 0);
        fill_slot(this, SLOT0, DEFAULT0, KEY0);
        fill_slot(this, SLOT1, DEFAULT1, KEY1);
        fill_slot(this, SLOT2, DEFAULT2, KEY2);
        this
    }
});
