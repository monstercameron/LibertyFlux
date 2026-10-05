// original: 0x00bea570 task_copy_params (proposed)

/// Copy a task-parameter block from `src` into `this`, refreshing presence flags.
///
/// `this` is the destination parameter block, `src` the source. When the
/// position pointer at `src+0x14` is non-null, three floats are copied from it
/// to `this+0x00` and flag bit 0 is set; otherwise bit 0 is cleared. Nine
/// dwords and three bytes are then copied field by field (see the copy table
/// below), and flag bits 1..=6 of `this+0x33` record whether six source
/// dwords are non-zero. Bit 7 of the flags is preserved.
///
/// Copy table (`this` <- `src`): `0x0c<-0x00`, `0x10<-0x04`, `0x14<-0x08`,
/// `0x18<-0x24`, `0x1c<-0x28`, `0x20<-0x2c`, `0x24<-0x34`, `0x28<-0x3c`,
/// `0x2c<-0x40`, bytes `0x30<-0x44`, `0x31<-0x45`, `0x32<-0x46`. Flag bits:
/// bit1 `[src+0x0c]!=0`, bit2 `[src+0x10]!=0`, bit3 `[src+0x18]!=0`, bit4
/// `[src+0x1c]!=0`, bit5 `[src+0x20]!=0`, bit6 `[src+0x30]!=0`.
///
/// The original builds the flag byte and `this+0x32` with per-bit
/// insert sequences; the end state equals a plain byte copy plus the flag
/// computation above. Returns the new flag byte, zero-extended.
///
/// Original: 0x00bea570 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bea570(this: u32, src: u32) -> u32 {
    unsafe {
        const POS_PTR: u32 = 0x14;
        const FLAGS: u32 = 0x33;
        const MODE_BYTE: u32 = 0x32;
        const SRC_MODE: u32 = 0x46;

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

        let pos = rd32(src + POS_PTR);
        let mut flags = rd8(this + FLAGS);
        if pos != 0 {
            wr32(this, rd32(pos));
            wr32(this + 4, rd32(pos + 4));
            wr32(this + 8, rd32(pos + 8));
            flags |= 1;
        } else {
            flags &= !1;
        }
        wr32(this + 0x0c, rd32(src));
        wr32(this + 0x10, rd32(src + 0x04));
        wr32(this + 0x14, rd32(src + 0x08));
        wr32(this + 0x18, rd32(src + 0x24));
        wr32(this + 0x1c, rd32(src + 0x28));
        wr32(this + 0x20, rd32(src + 0x2c));
        wr32(this + 0x24, rd32(src + 0x34));
        wr32(this + 0x28, rd32(src + 0x3c));
        wr32(this + 0x2c, rd32(src + 0x40));
        wr8(this + 0x30, rd8(src + 0x44));
        wr8(this + 0x31, rd8(src + 0x45));
        wr8(this + MODE_BYTE, rd8(src + SRC_MODE));
        if rd32(src + 0x0c) != 0 {
            flags |= 0x02;
        } else {
            flags &= !0x02;
        }
        if rd32(src + 0x10) != 0 {
            flags |= 0x04;
        } else {
            flags &= !0x04;
        }
        if rd32(src + 0x18) != 0 {
            flags |= 0x08;
        } else {
            flags &= !0x08;
        }
        if rd32(src + 0x1c) != 0 {
            flags |= 0x10;
        } else {
            flags &= !0x10;
        }
        if rd32(src + 0x20) != 0 {
            flags |= 0x20;
        } else {
            flags &= !0x20;
        }
        if rd32(src + 0x30) != 0 {
            flags |= 0x40;
        } else {
            flags &= !0x40;
        }
        wr8(this + FLAGS, flags);
        flags as u32
    }
});
