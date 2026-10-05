// original: 0x00a45520 CVehicle::vf101
/// Reset the vehicle's detachable state and return its kind bits.
///
/// When the link at `this+0x6C` is null or its byte at `+0xE` is clear,
/// calls the slot-0xF0 virtual (callee 1, thiscall, one stack word:
/// 0x447A0000) through a planted vtable; when linked and the kind bits
/// `[this+0x28]&0x7C00` equal 0xC00, returns 0xC00 at once. Otherwise runs
/// the reset chain: two sub-resets on `this+0x10D0` (callees 3 and 4,
/// thiscall, no stack arguments), the base reset (callee 5), the slot-0x120
/// virtual (callee 2, thiscall, no stack arguments), zeroes the four dwords
/// at `this+0x1AC..0x1B8`, calls the registrar (callee 6, thiscall, ECX
/// 0x013BABA0, stack words `this` and 0) and the trailer reset on
/// `this+0x210` (callee 7), clears the byte at `[this+0x34]+4+0xCC` when
/// that chain is non-null, and when the kind bits equal 0xC00 clears bit
/// 0x10000 of `this+0x118` (thiscall, no stack arguments). Returns the kind
/// bits in all cases.
export!(thiscall, rw_00a45520(this: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x6c;
        const LINK_FLAG: u32 = 0xe;
        const KIND_OFF: u32 = 0x28;
        const KIND_MASK: u32 = 0x7c00;
        const KIND_SKIP: u32 = 0xc00;
        const VT_RESET: u32 = 0xf0;
        const VT_NOTIFY: u32 = 0x120;
        const SUB_OFF: u32 = 0x10d0;
        const TRAIL_OFF: u32 = 0x210;
        const CLEAR_OFF: u32 = 0x1ac;
        const REG_OBJ: u32 = 0x013baba0;
        const OUTER_OFF: u32 = 0x34;
        const INNER_FLAG: u32 = 0xcc;
        const MODE_OFF: u32 = 0x118;
        const MODE_BIT: u32 = 0x10000;
        const PUSHED_FLOAT: u32 = 0x447a0000;
        let link = (this.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let linked = link != 0 && ((link.wrapping_add(LINK_FLAG)) as *const u8).read() != 0;
        if !linked {
            let vt = (this as *const u32).read_unaligned();
            let faddr = (vt.wrapping_add(VT_RESET) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(faddr as usize);
            f(this, PUSHED_FLOAT);
        } else if (this.wrapping_add(KIND_OFF) as *const u32).read_unaligned() & KIND_MASK == KIND_SKIP {
            return KIND_SKIP;
        }
        let _: u32 = callee_thiscall!(3, u32, this.wrapping_add(SUB_OFF));
        let _: u32 = callee_thiscall!(4, u32, this.wrapping_add(SUB_OFF));
        let _: u32 = callee_thiscall!(5, u32, this);
        let vt = (this as *const u32).read_unaligned();
        let faddr = (vt.wrapping_add(VT_NOTIFY) as *const u32).read_unaligned();
        let g: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(faddr as usize);
        g(this);
        ((this.wrapping_add(CLEAR_OFF)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(CLEAR_OFF + 4)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(CLEAR_OFF + 8)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(CLEAR_OFF + 12)) as *mut u32).write_unaligned(0);
        let _: u32 = callee_thiscall!(6, u32, relocated(REG_OBJ), this, 0);
        let _: u32 = callee_thiscall!(7, u32, this.wrapping_add(TRAIL_OFF));
        let o = (this.wrapping_add(OUTER_OFF) as *const u32).read_unaligned();
        let inner = (o.wrapping_add(4) as *const u32).read_unaligned();
        if inner != 0 {
            ((inner.wrapping_add(INNER_FLAG)) as *mut u8).write(0);
        }
        let masked = (this.wrapping_add(KIND_OFF) as *const u32).read_unaligned() & KIND_MASK;
        if masked == KIND_SKIP {
            let w = (this.wrapping_add(MODE_OFF) as *const u32).read_unaligned();
            ((this.wrapping_add(MODE_OFF)) as *mut u32).write_unaligned(w & !MODE_BIT);
        }
        masked
    }
});
