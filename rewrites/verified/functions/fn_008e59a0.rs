// original: 0x008e59a0 gps_slot_advance
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
#[inline]
unsafe fn rd8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}

#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read() }
}

#[inline]
unsafe fn wr8(base: u32, off: u32, v: u8) {
    unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
}

#[inline]
unsafe fn wr16(base: u32, off: u32, v: u16) {
    unsafe { ((base.wrapping_add(off)) as *mut u16).write(v) }
}

#[inline]
unsafe fn wr32(base: u32, off: u32, v: u32) {
    unsafe { ((base.wrapping_add(off)) as *mut u32).write(v) }
}
/// Advance a GPS-prompt slot machine by one step.
///
/// The three gate words must read enabled/idle or nothing happens. Otherwise
/// it queries the engine audio object for a level, marks the zero-level flag
/// when that level compares equal to +0.0, transforms the level through the
/// float helper, and forwards it to the live handle at +0x34C if there is
/// one. It then tries to advance the slot cursor at +0x348 through a table
/// of fourteen 12-byte rows: the next row must be valid and the state flags
/// must read idle. On success it formats the row's two table-driven strings
/// into a frame buffer, builds the request struct, resolves the prompt name
/// hash, and issues the start request; on any failure it tears the handle
/// down, clears the state, and reloads the default at +0x344. The stack-cookie
/// check runs on every exit and its answer is the returned EAX.
const FN1_ENG: u32 = 0x0116_5880;
const FN1_SINK: u32 = 0x0115_D9A0;
const FN1_FMT: u32 = 0x00E8_2F54;
const FN1_NAME: u32 = 0x00E8_2F5C;
const FN1_TAB0: u32 = 0x0103_2FE8;
const FN1_TAB1: u32 = 0x0103_2FD0;
const FN1_WORD: u32 = 0x0117_6878;
const FN1_DFLT: u32 = 0x0117_35B4;

#[inline]
fn fn1_cookie() -> u32 {
    // Stack-cookie check: the original passes cookie^ESP in ECX, which a
    // rewrite cannot reproduce (different frame), so the contract excludes
    // ECX from the comparison and only the scripted answer matters.
    callee_thiscall!(10, u32, 0)
}

export!(thiscall, rw_008E59A0(this: u32, _dead: u32) -> u32 {
    if unsafe { rd8(this, 0x340) } == 0 {
        return fn1_cookie();
    }
    if unsafe { rd32(this, 0x234) } != 0 {
        return fn1_cookie();
    }
    if unsafe { rd32(this, 0x200) } != 0 {
        return fn1_cookie();
    }
    let mut level: u32 = 0;
    let _q = callee_thiscall!(
        1, u32, relocated(FN1_ENG),
        core::ptr::addr_of_mut!(level) as u32, 0
    );
    // ucomiss against +0.0 with the jp-taken-iff-unequal idiom: the flag is
    // set only on ordered equality (-0.0 counts, NaN does not).
    if f32::from_bits(level) == 0.0 {
        unsafe { wr8(this, 0x3FA, 1) };
    }
    let f = callee_cdecl!(2, f32, level);
    let handle = unsafe { rd32(this, 0x34C) };
    if handle != 0 {
        let _d = callee_thiscall!(3, u32, handle, f.to_bits());
    }
    let cur = unsafe { rd32(this, 0x348) };
    if cur != 0xFFFF_FFFF && unsafe { rd32(this, 0x34C) } != 0 && unsafe { rd8(this, 0x3FA) } == 0 {
        return fn1_cookie();
    }
    let next = cur.wrapping_add(1);
    let mut take = false;
    if next < 0xE {
        let row0 = unsafe { rd32(this, 0x350 + next.wrapping_mul(12)) };
        if row0 != 0xFFFF_FFFF
            && unsafe { rd8(this, 0x3F9) } == 0
            && unsafe { rd8(this, 0x3FA) } == 0
            && (unsafe { rd8(this, 0x3F8) } == 0
                || unsafe {
                    rd8(
                        this,
                        0x358u32
                            .wrapping_add(cur.wrapping_mul(3).wrapping_mul(4)),
                    )
                } == 0)
        {
            take = true;
        }
    }
    if take {
        unsafe { wr32(this, 0x348, next) };
        let idx0 = unsafe { rd32(this, 0x350 + next.wrapping_mul(12)) };
        let t1 = unsafe { global::<u32>(FN1_TAB0.wrapping_add(idx0.wrapping_mul(4))).read() };
        let b400 = unsafe { rd8(this, 0x400) } as u32;
        let t2 = unsafe { global::<u32>(FN1_TAB1 + b400 * 4).read() };
        let mut sbuf = [0u32; 16];
        let _s = callee_cdecl!(
            4, u32,
            core::ptr::addr_of_mut!(sbuf) as u32,
            relocated(FN1_FMT), t2, t1
        );
        // Request struct at the original's field offsets (the contract
        // snapshots the first eight words).
        let mut s = [0u32; 24];
        let sptr = core::ptr::addr_of_mut!(s) as u32;
        let _b = callee_thiscall!(5, u32, sptr);
        let h = callee_cdecl!(6, u32, relocated(FN1_NAME), 0);
        let ans = callee_thiscall!(7, u32, relocated(FN1_SINK), h);
        unsafe { ((sptr + 0x00) as *mut u32).write(f.to_bits()) };
        unsafe { ((sptr + 0x08) as *mut u32).write(0xBB8) };
        unsafe { ((sptr + 0x18) as *mut u32).write(rd32(this, 0x3FC)) };
        unsafe { ((sptr + 0x1C) as *mut u32).write(ans) };
        unsafe {
            ((sptr + 0x28) as *mut u32).write(rd32(this, 0x354 + next.wrapping_mul(12)))
        };
        let w = unsafe { global::<u16>(FN1_WORD).read() } as i16 as i32 as u32;
        unsafe { ((sptr + 0x40) as *mut u32).write(w) };
        unsafe {
            let bp = (sptr + 0x46) as *mut u8;
            bp.write(bp.read() | 0x10)
        };
        let _big = callee_thiscall!(
            8, u32, this,
            core::ptr::addr_of_mut!(sbuf) as u32,
            this.wrapping_add(0x34C), sptr, 0xFFFF_FFFF, 0, 0
        );
    } else {
        let h = unsafe { rd32(this, 0x34C) };
        unsafe { wr8(this, 0x340, 0) };
        unsafe { wr32(this, 0x348, 0xFFFF_FFFF) };
        if h != 0 {
            let _e = callee_thiscall!(9, u32, h, 0);
        }
        unsafe { wr16(this, 0x3F8, 0) };
        unsafe { wr8(this, 0x3FA, 0) };
        let d = unsafe { global::<u32>(FN1_DFLT).read() };
        unsafe { wr32(this, 0x344, d) };
    }
    fn1_cookie()
});
