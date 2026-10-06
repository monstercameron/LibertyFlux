// original: 0x008e5c80 menu_music_state_machine
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated};

#[inline]
unsafe fn rd8(base: u32, off: u32) -> u8 {
    unsafe { (base.wrapping_add(off) as *const u8).read() }
}
#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { (base.wrapping_add(off) as *const u32).read() }
}
#[inline]
unsafe fn rdf(base: u32, off: u32) -> f32 {
    unsafe { (base.wrapping_add(off) as *const f32).read() }
}
#[inline]
unsafe fn wr8(base: u32, off: u32, v: u8) {
    unsafe { (base.wrapping_add(off) as *mut u8).write(v) }
}
#[inline]
unsafe fn wr32(base: u32, off: u32, v: u32) {
    unsafe { (base.wrapping_add(off) as *mut u32).write(v) }
}
#[inline]
unsafe fn wrf(base: u32, off: u32, v: f32) {
    unsafe { (base.wrapping_add(off) as *mut f32).write(v) }
}
#[inline]
unsafe fn gf32(va: u32) -> f32 {
    unsafe { global::<f32>(va).read() }
}
#[inline]
unsafe fn gu32(va: u32) -> u32 {
    unsafe { global::<u32>(va).read() }
}
#[inline]
unsafe fn gu8(va: u32) -> u8 {
    unsafe { global::<u8>(va).read() }
}

/// Dual music/ambience state machine advance.
///
/// State A at +0x204 (cases: 0 acquire gated on the engine check, 1 acquire,
/// 2 create the voice handle at +0x21C and start it, 3 run the fade ramp on
/// +0x220 through the float helper, 4 run the ramp on +0x224 and stop) runs
/// first. Then a gate section scales a global float by 1000, truncates it
/// toward zero to a 64-bit integer (out of range or NaN yields the indefinite
/// value, whose low dword is stored at +0x214 on exit), and, unless the
/// +0x43D flag is set, polls a data-table hook plus several flag bytes to
/// decide that flag. State B at +0x200 (cases: 0 enable gated on the same
/// flag, 1 acquire, 2 create the handle at +0x7C, 3 poll two input sightings
/// and two modals to pick a level for +0x7C, 4 stop) runs last. The
/// stack-cookie check runs on every exit and its answer is the returned EAX.
const C_ENG_STR0: u32 = 0x008E_4480;
const C_ENG_STR1: u32 = 0x008E_4050;
const C_FMT_A: u32 = 0x00E8_28F4;
const C_FMT_B: u32 = 0x00E8_2908;
const C_MODAL_OBJ: u32 = 0x018E_51E8;
const C_RAMP_UP: u32 = 0x00E8_3170;
const C_ONE: u32 = 0x00FE_88E8;
const C_HALF_PI: u32 = 0x00FE_8978;
const C_THOUSAND: u32 = 0x00FE_8C58;
const C_MUTE_LEVEL: u32 = 0xC190_0000;
const G_CREATE_ARG: u32 = 0x0103_0C0C;
const G_RATE: u32 = 0x0103_234C;
const G_CMP_A: u32 = 0x0105_B48F;
const G_FADE_IN: u32 = 0x0115_DBE8;
const G_ARM_BYTE: u32 = 0x0116_09F6;
const G_POLL_GATE: u32 = 0x0116_0D80;
const G_FLAG_A: u32 = 0x0117_3590;
const G_FLAG_B: u32 = 0x0117_3591;
const G_FADE_RATE: u32 = 0x0117_35BC;
const G_POLL_OBJ: u32 = 0x0118_F4A8;
const G_HOOK_ARG: u32 = 0x017A_CCD8;
const G_CMP_B: u32 = 0x017E_D8D1;

const ID_ENTER: u32 = 1;
const ID_ENG_OK: u32 = 2;
const ID_ACQUIRE: u32 = 3;
const ID_VOICE_STOP: u32 = 4;
const ID_CREATE_FMT: u32 = 5;
const ID_ZERO_REQ: u32 = 6;
const ID_CREATE_FILL: u32 = 7;
const ID_CREATE_A: u32 = 8;
const ID_CREATE_B: u32 = 9;
const ID_START_A: u32 = 10;
const ID_START_B: u32 = 11;
const ID_RUN: u32 = 12;
const ID_CURVE: u32 = 13;
const ID_SHAPE: u32 = 14;
const ID_LEVEL_A: u32 = 15;
const ID_LEVEL_B: u32 = 16;
const ID_RESTART: u32 = 17;
const ID_HOOK: u32 = 18;
const ID_ENABLE: u32 = 19;
const ID_SIGHT_A: u32 = 20;
const ID_SIGHT_B: u32 = 21;
const ID_MODAL_A: u32 = 22;
const ID_MODAL_B: u32 = 23;
const ID_MAY_START: u32 = 24;
const ID_BEGIN: u32 = 25;
const ID_COOKIE: u32 = 26;

#[inline]
fn cookie() -> u32 {
    // The original passes cookie^ESP in ECX, which a rewrite cannot
    // reproduce (different frame), so the contract excludes ECX and only
    // the scripted answer matters.
    callee_thiscall!(ID_COOKIE, u32, 0)
}

/// Truncate a float toward zero to 64 bits, as the gate's x87 convert with
/// the chop control word does: NaN and out-of-range magnitudes yield the
/// indefinite value, not saturation.
#[inline]
fn chop_to_i64(x: f32) -> u64 {
    const TWO63: f32 = 9.223372e18;
    if x.is_nan() || x >= TWO63 || x < -TWO63 {
        0x8000_0000_0000_0000
    } else {
        x as i64 as u64
    }
}

#[inline]
unsafe fn release_engine(obj: u32) {
    // Emulation of the tiny release helper (one byte store); it runs
    // unpatched on the original side, so no stub id is needed.
    unsafe { wr8(obj, 0x28, 2) };
}

unsafe fn acquire_slot(this: u32, state_off: u32, next: u32) {
    if unsafe { rd32(this, 0x218) } == 0 {
        let mut s = [0u32; 4];
        let sptr = core::ptr::addr_of_mut!(s) as u32;
        unsafe {
            ((sptr + 0) as *mut u32).write(relocated(C_ENG_STR0));
            ((sptr + 4) as *mut u32).write(relocated(C_ENG_STR1));
            ((sptr + 8) as *mut u32).write(0);
            ((sptr + 12) as *mut u8).write(5);
        }
        let obj = callee_cdecl!(ID_ACQUIRE, u32, sptr);
        unsafe { wr32(this, 0x218, obj) };
    }
    let obj = unsafe { rd32(this, 0x218) };
    if obj != 0 && unsafe { rd8(obj, 0x28) } == 1 {
        unsafe { wr32(this, state_off, next) };
    }
    let h = unsafe { rd32(this, 0x34C) };
    if h != 0 {
        let _d = callee_thiscall!(ID_VOICE_STOP, u32, h, 0);
    }
}

unsafe fn inner(this: u32, mutant: bool) -> u32 {
    let _e = callee_thiscall!(ID_ENTER, u32, this);
    match unsafe { rd32(this, 0x204) } {
        0 => {
            if (callee_thiscall!(ID_ENG_OK, u32, this) & 0xFF) != 0 {
                unsafe {
                    wrf(this, 0x220, 0.0);
                    wrf(this, 0x224, 0.0);
                    wr32(this, 0x204, 1);
                }
                unsafe { acquire_slot(this, 0x204, 2) };
                let h = unsafe { rd32(this, 0x228) };
                if h != 0 {
                    let _d = callee_thiscall!(ID_VOICE_STOP, u32, h, 0);
                    unsafe { wr32(this, 0x234, 0) };
                }
            }
        }
        1 => {
            unsafe { acquire_slot(this, 0x204, 2) };
            let h = unsafe { rd32(this, 0x228) };
            if h != 0 {
                let _d = callee_thiscall!(ID_VOICE_STOP, u32, h, 0);
                unsafe { wr32(this, 0x234, 0) };
            }
        }
        2 => {
            let slot = this.wrapping_add(0x21C);
            if unsafe { rd32(this, 0x21C) } == 0 {
                let mut anchor = [0u32; 4];
                let mut req = [0u32; 9];
                let mut out = [0u32; 4];
                let mut tail = [0u32; 4];
                let _f = callee_cdecl!(
                    ID_CREATE_FMT, u32,
                    core::ptr::addr_of_mut!(anchor) as u32, 0x40,
                    relocated(C_FMT_A), 6
                );
                let _z = callee_thiscall!(
                    ID_ZERO_REQ, u32,
                    core::ptr::addr_of_mut!(req) as u32
                );
                let g = unsafe { gu32(G_CREATE_ARG) };
                let _c = callee_cdecl!(
                    ID_CREATE_FILL, u32,
                    core::ptr::addr_of_mut!(out) as u32, g
                );
                let _h = callee_thiscall!(
                    ID_CREATE_A, u32, this,
                    core::ptr::addr_of_mut!(tail) as u32,
                    slot,
                    core::ptr::addr_of_mut!(req) as u32,
                    0xFFFF_FFFF, 0, 0
                );
            }
            let h = unsafe { (slot as *const u32).read() };
            if h != 0 {
                let eng = unsafe { rd32(this, 0x218) };
                let w = unsafe { rd32(eng, 0x20) };
                let ans = callee_thiscall!(ID_START_A, u32, h, w, 1);
                if ans == 1 {
                    let h2 = unsafe { (slot as *const u32).read() };
                    let _r = callee_thiscall!(ID_RUN, u32, h2);
                    unsafe {
                        wr8(this, 0x402, 0);
                        wr32(this, 0x204, 3);
                    }
                }
            }
            if (callee_thiscall!(ID_ENG_OK, u32, this) & 0xFF) == 0 {
                unsafe { wr32(this, 0x204, 4) };
            }
        }
        3 => {
            let rate = unsafe { gf32(G_FADE_RATE) } / unsafe { gf32(G_RATE) };
            let ramp = rate + unsafe { rdf(this, 0x220) };
            unsafe { wrf(this, 0x220, ramp) };
            let live = (callee_thiscall!(ID_ENG_OK, u32, this) & 0xFF) != 0;
            let h = unsafe { rd32(this, 0x21C) };
            if !live || h == 0 {
                unsafe { wr32(this, 0x204, 4) };
            } else {
                let scaled = ramp * unsafe { gf32(C_RAMP_UP) };
                let one = unsafe { gf32(C_ONE) };
                // Ordered greater-than picks 1.0, else the value (NaN
                // keeps the value, as the compare-and-jump-above does).
                let capped = if mutant {
                    if scaled > one { scaled } else { one }
                } else if scaled > one {
                    one
                } else {
                    scaled
                };
                let curved_in = capped * unsafe { gf32(C_HALF_PI) };
                let ans = callee_stdcall!(ID_CURVE, u32, curved_in.to_bits());
                let _s = callee_stdcall!(ID_SHAPE, u32, ans);
                let _l = callee_thiscall!(ID_LEVEL_A, u32, h);
            }
        }
        4 => {
            let rate = unsafe { gf32(G_FADE_RATE) } / unsafe { gf32(G_RATE) };
            let ramp = rate + unsafe { rdf(this, 0x224) };
            unsafe { wrf(this, 0x224, ramp) };
            let h = unsafe { rd32(this, 0x21C) };
            if h != 0 && unsafe { rd8(this, 0x402) } == 0 {
                let _r = callee_thiscall!(ID_RESTART, u32, h);
                unsafe { wr8(this, 0x402, 1) };
            }
            if unsafe { rd32(this, 0x21C) } == 0 {
                let eng = unsafe { rd32(this, 0x218) };
                if eng != 0 {
                    unsafe { release_engine(eng) };
                    unsafe { wr32(this, 0x218, 0) };
                }
                unsafe { wr32(this, 0x204, 0) };
            }
        }
        _ => {}
    }
    // Gate: scaled truncate whose low dword lands at +0x214, plus the
    // hook/flag poll that arms +0x43D.
    let ticks = chop_to_i64(unsafe { gf32(G_FADE_IN) } * unsafe { gf32(C_THOUSAND) }) as u32;
    if unsafe { rd8(this, 0x43D) } == 0 {
        let arg = unsafe { gu32(G_HOOK_ARG) };
        let a = callee_stdcall!(ID_HOOK, u32, arg);
        let b = if a == 0 {
            if unsafe { gu8(G_CMP_A) } == 0 {
                0
            } else if unsafe { gu8(G_CMP_B) } == 0 {
                0
            } else {
                1
            }
        } else {
            1
        } as u8;
        let f = b | unsafe { gu8(G_FLAG_A) } | unsafe { gu8(G_FLAG_B) };
        if f == 0 && unsafe { gu8(G_ARM_BYTE) } == 0 {
            unsafe { wr8(this, 0x43D, 1) };
        }
    }
    match unsafe { rd32(this, 0x200) } {
        0 => {
            if (callee_thiscall!(ID_ENABLE, u32, this) & 0xFF) != 0
                && unsafe { rd8(this, 0x43D) } != 0
            {
                unsafe { wr32(this, 0x200, 1) };
            }
        }
        1 => {
            unsafe { acquire_slot(this, 0x200, 2) };
            let h = unsafe { rd32(this, 0x228) };
            if h != 0 {
                let _d = callee_thiscall!(ID_VOICE_STOP, u32, h, 0);
                unsafe { wr32(this, 0x234, 0) };
            }
        }
        2 => {
            let slot = this.wrapping_add(0x7C);
            if unsafe { rd32(this, 0x7C) } == 0 {
                let mut req = [0u32; 9];
                let mut out = [0u32; 4];
                let _z = callee_thiscall!(
                    ID_ZERO_REQ, u32,
                    core::ptr::addr_of_mut!(req) as u32
                );
                let g = unsafe { gu32(G_CREATE_ARG) };
                let _c = callee_cdecl!(
                    ID_CREATE_FILL, u32,
                    core::ptr::addr_of_mut!(out) as u32, g
                );
                let _h = callee_thiscall!(
                    ID_CREATE_B, u32, this,
                    relocated(C_FMT_B),
                    slot,
                    core::ptr::addr_of_mut!(req) as u32,
                    0xFFFF_FFFF, 0, 0
                );
            }
            let h = unsafe { (slot as *const u32).read() };
            if h == 0 {
                unsafe { wr32(this, 0x200, 4) };
            } else {
                let eng = unsafe { rd32(this, 0x218) };
                let w = unsafe { rd32(eng, 0x20) };
                let ans = callee_thiscall!(ID_START_B, u32, h, w, 1);
                if ans == 1 {
                    let h2 = unsafe { (slot as *const u32).read() };
                    let _r = callee_thiscall!(ID_RUN, u32, h2);
                    unsafe {
                        wr8(this, 0x208, 1);
                        wr32(this, 0x200, 3);
                    }
                } else if ans == 2 {
                    unsafe { wr32(this, 0x200, 4) };
                }
            }
        }
        3 => {
            if unsafe { rd32(this, 0x7C) } == 0 {
                unsafe { wr32(this, 0x200, 4) };
            } else {
                // The level call below runs on the poll-failed paths too,
                // with zero bits; each failed poll just skips the rest.
                let mut bits = 0u32;
                if unsafe { gu32(G_POLL_GATE) } == 0 {
                    let obj = unsafe { gu32(G_POLL_OBJ) };
                    let sa = callee_thiscall!(ID_SIGHT_A, u32, obj, 0x66);
                    if (sa as i32) > 0 {
                        let obj2 = unsafe { gu32(G_POLL_OBJ) };
                        let sb = callee_thiscall!(ID_SIGHT_B, u32, obj2, 0x64);
                        if sb != 0 {
                            let modal = relocated(C_MODAL_OBJ);
                            if (callee_thiscall!(ID_MODAL_A, u32, modal) & 0xFF) != 0
                                || (callee_thiscall!(ID_MODAL_B, u32, modal) & 0xFF) != 0
                            {
                                bits = C_MUTE_LEVEL;
                            }
                        }
                    }
                }
                let h = unsafe { rd32(this, 0x7C) };
                let _l = callee_thiscall!(ID_LEVEL_B, u32, h, bits);
                if unsafe { rd32(this, 0x7C) } == 0 {
                    unsafe { wr32(this, 0x200, 4) };
                } else {
                    if (callee_thiscall!(ID_ENABLE, u32, this) & 0xFF) == 0 {
                        unsafe { wr32(this, 0x200, 4) };
                    }
                    if (callee_thiscall!(ID_MAY_START, u32, 0) & 0xFF) != 0 {
                        let _b = callee_thiscall!(ID_BEGIN, u32, this);
                    }
                }
            }
        }
        4 => {
            let h = unsafe { rd32(this, 0x7C) };
            if h != 0 {
                let _d = callee_thiscall!(ID_VOICE_STOP, u32, h, 0);
            }
            let eng = unsafe { rd32(this, 0x218) };
            if eng != 0 {
                unsafe { release_engine(eng) };
                unsafe { wr32(this, 0x218, 0) };
            }
            unsafe { wr32(this, 0x200, 0) };
        }
        _ => {}
    }
    unsafe { wr32(this, 0x214, ticks) };
    cookie()
}

export!(thiscall, rw_008E5C80(this: u32) -> u32 {
    unsafe { inner(this, false) }
});

export!(thiscall, mut_008E5C80(this: u32) -> u32 {
    unsafe { inner(this, true) }
});
