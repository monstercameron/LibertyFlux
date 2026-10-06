// original: 0x00655AA0 shaft_group_ctor (proposed)

/// Construct a shaft-group object: set the outer vtable slot, build six
/// identical inner slots through the shared inner constructor, then resolve
/// the group's live counter through thread-local state and three helpers.
///
/// `this` points to the group object (0x118 bytes) and `arg0` is stored at
/// `+0xFC` at the end. The inner constructor (callee 1, thiscall with no
/// stack arguments) runs six times with its object at `+0x08`, `+0x30`,
/// `+0x58`, `+0x80`, `+0xA8` and `+0xD0`; each slot's vtable word is written
/// after its call and the four words at slot `+0x18`/`+0x1C`/`+0x20`/`+0x24`
/// (equivalently `+0x20`..`+0x2C` for the first three slots) are zeroed.
/// The word at `+0xF8` is zeroed, then the counter at `+0x100` is resolved:
/// the word at `+4` of TLS slot 0 (the context) must be non-null and
/// the counter non-zero, the lookup helper (callee 2, called
/// with `this` set to the word the context points at and the counter address
/// as its argument) must not answer -1 (compared as an exact 32-bit value),
/// and the adjust helper (callee 3, called with the context and the counter
/// value) adds its answer to the counter with wraparound. When the adjusted
/// counter is non-zero the register helper (callee 4, called with the counter
/// as `this` and the re-read context as its argument) runs. Any earlier
/// failure zeroes the counter instead. The tail stores `arg0` at `+0xFC`,
/// sets the flag word at `+0x10C` to 1, zeroes the counter and the flag bytes
/// at `+0x10E`, `+0x111`, `+0x113` and the words at `+0x108` and `+0x114`,
/// and returns `this`.
///
/// Original: 0x00655AA0 (thiscall, one stack word, returns `this`).
lf_checker_rt::export!(thiscall, rw_00655AA0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const VTABLE_OUTER: u32 = 0x00FE267C;
        const VTABLE_INNER: u32 = 0x00FE355C;
        const INNER_CTOR: u32 = 1;
        const LOOKUP: u32 = 2;
        const ADJUST: u32 = 3;
        const REGISTER: u32 = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let edi = this;
        let inner = lf_checker_rt::relocated(VTABLE_INNER);
        wr32(edi + 0x04, 0);
        wr32(edi, lf_checker_rt::relocated(VTABLE_OUTER));
        lf_checker_rt::callee_thiscall!(INNER_CTOR, u32, edi + 0x08);
        wr32(edi + 0x08, inner);
        wr32(edi + 0x20, 0);
        wr32(edi + 0x24, 0);
        wr32(edi + 0x28, 0);
        wr32(edi + 0x2C, 0);
        lf_checker_rt::callee_thiscall!(INNER_CTOR, u32, edi + 0x30);
        wr32(edi + 0x30, inner);
        wr32(edi + 0x48, 0);
        wr32(edi + 0x4C, 0);
        wr32(edi + 0x50, 0);
        wr32(edi + 0x54, 0);
        lf_checker_rt::callee_thiscall!(INNER_CTOR, u32, edi + 0x58);
        wr32(edi + 0x58, inner);
        wr32(edi + 0x70, 0);
        wr32(edi + 0x74, 0);
        wr32(edi + 0x78, 0);
        wr32(edi + 0x7C, 0);
        lf_checker_rt::callee_thiscall!(INNER_CTOR, u32, edi + 0x80);
        wr32(edi + 0x80, inner);
        wr32(edi + 0x98, 0);
        wr32(edi + 0x9C, 0);
        wr32(edi + 0xA0, 0);
        wr32(edi + 0xA4, 0);
        lf_checker_rt::callee_thiscall!(INNER_CTOR, u32, edi + 0xA8);
        wr32(edi + 0xA8, inner);
        wr32(edi + 0xC0, 0);
        wr32(edi + 0xC4, 0);
        wr32(edi + 0xC8, 0);
        wr32(edi + 0xCC, 0);
        lf_checker_rt::callee_thiscall!(INNER_CTOR, u32, edi + 0xD0);
        wr32(edi + 0xD0, inner);
        wr32(edi + 0xEC, 0);
        wr32(edi + 0xE8, 0);
        wr32(edi + 0xF0, 0);
        wr32(edi + 0xF4, 0);
        wr32(edi + 0xF8, 0);
        let ctx = rd32(lf_checker_rt::tls_slot(0) + 4);
        let counter = edi + 0x100;
        if ctx != 0 && rd32(counter) != 0 {
            let found = lf_checker_rt::callee_thiscall!(LOOKUP, u32, rd32(ctx), counter);
            if found != NOT_FOUND {
                let delta = lf_checker_rt::callee_thiscall!(ADJUST, u32, ctx, rd32(counter));
                let sum = rd32(counter).wrapping_add(delta);
                wr32(counter, sum);
                if sum != 0 {
                    let ctx2 = rd32(lf_checker_rt::tls_slot(0) + 4);
                    lf_checker_rt::callee_thiscall!(REGISTER, u32, sum, ctx2);
                }
            } else {
                wr32(counter, 0);
            }
        } else {
            wr32(counter, 0);
        }
        wr32(edi + 0xFC, arg0);
        wr16(edi + 0x10C, 1);
        wr8(edi + 0x111, 0);
        wr8(edi + 0x10E, 0);
        wr32(counter, 0);
        wr32(edi + 0x108, 0);
        wr8(edi + 0x113, 0);
        wr32(edi + 0x114, 0);
        edi
    }
});
