// original: 0x00D4B420 CTaskComplexGun::CTaskComplexGun

#![allow(unsafe_code)]

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

/// Constructor of the gun-task object.
///
/// `this` is the fresh object. The eight stack arguments are: a selector
/// byte (low byte; upper bytes unread), an optional attachment object (null
/// or a pointer, kept at `+0x14` and, when non-null, given the base-class
/// constructor's follow-up call with `&this+0x14`), an optional 16-byte
/// vector copied to `+0x20` (when null the words are zeroed except `+0x2c`,
/// which keeps whatever an uninitialised stack slot held -- zero under the
/// checker's defined fill), two floats stored at `+0x30` and `+0x40`, a word
/// stored at `+0x74`, and two words stored at `+0x3a` and `+0x34`.
///
/// After the base constructor and the vtable store the body writes fixed
/// defaults: flag words OR-ed with `0xffff`, four floats copied from the
/// game's data, and several zero fields. Finally the selector byte,
/// sign-extended minus one, picks one of seven layouts for the mode words at
/// `+0x6c`/`+0x70` (plus adjustments to `+0x30`/`+0x34`/`+0x3a`/`+0x40`/
/// `+0x74`); any other selector keeps the defaults. Returns `this`.
///
/// Original: 0x00D4B420 (thiscall, eight stack words).
lf_checker_rt::export!(thiscall, rw_00D4B420(this: u32, sel: u32, obj: u32, vec: u32, f1: u32, w74: u32, w3a: u32, w34: u32, f2: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EE4AD4;
        const NEG_ONE_HALF: u32 = 0xBF80_0000;
        const G84: u32 = 0x01050B44;
        const G88: u32 = 0x01050B40;
        const G8C: u32 = 0x01050B50;
        const G90: u32 = 0x01050B4C;
        const BASE_CTOR: u32 = 0;
        const ATTACH_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
        }
        #[inline(always)]
        unsafe fn and32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) & v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
        }
        // Mode-word tail shared by the switch cases: store the kind byte,
        // then force bit 0x400 and clear the neighbouring flag bits.
        #[inline(always)]
        unsafe fn mode_tail(this: u32, kind: u8) {
            unsafe {
                wr8(this + 0x70, kind);
                let v = rd32(this + 0x70);
                wr32(this + 0x70, (v & 0xFFFF_04FF) | 0x400);
            }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        wr16(this + 0x34, (w34 & 0xFFFF) as u16);
        wr32(this + 0x30, f1);
        wr32(this + 0x14, obj);
        wr16(this + 0x3a, (w3a & 0xFFFF) as u16);
        wr8(this + 0x73, (sel & 0xFF) as u8);
        wr32(this + 0x40, f2);
        wr32(this + 0x74, w74);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr8(this + 0x72, 5);
        wr32(this + 0x98, 0);
        wr32(this + 0xa8, 0);
        if obj != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(ATTACH_CALLEE, u32, obj, this + 0x14);
        }
        // The original copies an uninitialised stack slot here; under the
        // contract's defined fill that slot is zero.
        wr32(this + 0x20, 0);
        wr32(this + 0x24, 0);
        wr32(this + 0x28, 0);
        wr32(this + 0x2c, 0);
        if vec != 0 {
            wr32(this + 0x20, rd32(vec));
            wr32(this + 0x24, rd32(vec + 4));
            wr32(this + 0x28, rd32(vec + 8));
            wr32(this + 0x2c, rd32(vec + 12));
        }
        or32(this + 0x4c, 0xFFFF);
        or32(this + 0x50, 0xFFFF);
        or32(this + 0x54, 0xFFFF);
        or32(this + 0x58, 0xFFFF);
        wr16(this + 0x3c, 0xFFFF);
        wr16(this + 0x3e, 0xFFFF);
        wr32(this + 0x36, 0x000A_0005);
        wr32(this + 0x44, NEG_ONE_HALF);
        wr32(this + 0x48, NEG_ONE_HALF);
        wr16(this + 0x4e, 0xFFFF);
        wr16(this + 0x52, 0xFFFF);
        wr16(this + 0x56, 0xFFFF);
        wr16(this + 0x5a, 0xFFFF);
        wr32(this + 0x6c, 0x0301_0303);
        wr16(this + 0x70, 0x0402);
        wr32(this + 0x5c, 0);
        wr32(this + 0x64, 0);
        wr32(this + 0x60, 0);
        wr32(this + 0x78, 0);
        wr32(this + 0x68, 0);
        wr16(this + 0x7c, 0);
        wr32(this + 0x80, 0);
        wr32(this + 0x84, g32(G84));
        wr32(this + 0x88, g32(G88));
        wr32(this + 0x8c, g32(G8C));
        wr32(this + 0x90, g32(G90));
        wr32(this + 0x94, 1);
        wr32(this + 0xa0, 0);
        wr32(this + 0xa4, 0);
        and32(this + 0x9c, 0xFFFF_FFFE);
        // Selector dispatch: sign-extended byte minus one, unsigned compare.
        let sel_idx = (((sel & 0xFF) as u8) as i8) as i32 - 1;
        if (sel_idx as u32) > 6 {
            return this;
        }
        match sel_idx {
            // The per-case byte store to +0x6c is overwritten by the dword
            // store that follows it, so only the dword is written here.
            0 => {
                wr8(this + 0x6c, 1);
                wr32(this + 0x30, NEG_ONE_HALF);
                mode_tail(this, 2);
                wr16(this + 0x3a, 0xFFFF);
                wr16(this + 0x34, 0xFFFF);
                wr32(this + 0x40, NEG_ONE_HALF);
            }
            1 => {
                wr32(this + 0x6c, 0x0101_0101);
                mode_tail(this, 2);
                wr16(this + 0x3a, 1);
                wr16(this + 0x34, 0xFFFF);
                wr32(this + 0x40, NEG_ONE_HALF);
            }
            2 => {
                wr32(this + 0x6c, 0x0101_0103);
                mode_tail(this, 2);
                wr16(this + 0x3a, 1);
                wr16(this + 0x34, 1);
                wr32(this + 0x40, NEG_ONE_HALF);
            }
            3 => {
                wr32(this + 0x6c, 0x0101_0103);
                mode_tail(this, 2);
                wr16(this + 0x3a, 1);
                wr16(this + 0x34, 0xFFFF);
                wr32(this + 0x40, NEG_ONE_HALF);
            }
            4 => {
                wr32(this + 0x6c, 0x0301_0303);
                mode_tail(this, 2);
                or32(this + 0x74, 0x0014_0000);
                wr16(this + 0x3a, 0xFFFF);
                wr16(this + 0x34, 0xFFFF);
                wr32(this + 0x40, NEG_ONE_HALF);
            }
            5 => {
                wr32(this + 0x6c, 0x0303_0303);
                mode_tail(this, 4);
                wr16(this + 0x34, 1);
                wr16(this + 0x3a, 0xFFFF);
                wr32(this + 0x40, NEG_ONE_HALF);
            }
            _ => {
                wr32(this + 0x6c, 0x0303_0303);
                mode_tail(this, 2);
                wr16(this + 0x3a, 0xFFFF);
                wr16(this + 0x34, 0xFFFF);
                wr32(this + 0x40, NEG_ONE_HALF);
            }
        }
        this
    }
});
