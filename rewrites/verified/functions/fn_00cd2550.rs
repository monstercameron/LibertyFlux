// original: 0x00cd2550 melee_timer_or_anim_pick
/// Pick the melee timer value and validate it, returning it or 0x3a.
///
/// Unless the flag at `this+0xf0` is clear (then the value comes straight
/// from `[arg+0xb94]`), the subtask at `this+8` must answer 0x11d through
/// its slot at `+0xc` (a null subtask skips to the timer gate): the timer
/// gate needs a non-zero byte at `this+0x40`, refreshes `this+0x38` from
/// the global clock when `this+0x41` is set, and takes the stored value
/// when `[this+0x3c]+[this+0x38]` no longer exceeds the clock (signed).
/// Otherwise the value resolves through `[arg+0x2c4]`'s chain (helper,
/// then its word at `+0xdc`) or, when the chain is null, through the
/// indexed table at 0x01295cd8 by the signed word at `arg+0x2e` (slot's
/// word at `+0x11c`). A value above -1 runs the validator chain (thiscall
/// on the global at 0x016dd63c, two cdecl helpers with the global at
/// 0x010496e8) and returns the value unless the last answers zero;
/// otherwise 0x3a. Thiscall with one stack argument.
export!(thiscall, rw_00cd2550(this: u32, arg: u32) -> u32 {
    unsafe {
        const CLOCK_G: u32 = 0x011735b4;
        const MGR_G: u32 = 0x016dd63c;
        const W2_G: u32 = 0x010496e8;
        const TABLE_C: u32 = 0x01295cd8;
        const FALLBACK: u32 = 0x3a;
        let gated = (this.wrapping_add(0xf0) as *const u8).read() & 8 != 0;
        let val;
        if !gated {
            val = (arg.wrapping_add(0xb94) as *const u32).read_unaligned();
        } else {
            let sub = (this.wrapping_add(0x08) as *const u32).read_unaligned();
            let mut probe = true;
            if sub != 0 {
                let vt = (sub as *const u32).read_unaligned();
                let s = (vt.wrapping_add(0x0c) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(s as usize);
                probe = f(sub) == 0x11d;
            }
            if !probe {
                let p = (arg.wrapping_add(0x2c4) as *const u32).read_unaligned();
                let r = if p != 0 {
                    (p.wrapping_add(0x25c) as *const u32).read_unaligned()
                } else {
                    0
                };
                let h = if r != 0 {
                    (r.wrapping_add(0x18) as *const u32).read_unaligned()
                } else {
                    0
                };
                val = if h != 0 {
                    let t: u32 = callee_cdecl!(2, u32, h);
                    (t.wrapping_add(0xdc) as *const u32).read_unaligned()
                } else {
                    let idx =
                        (arg.wrapping_add(0x2e) as *const i16).read_unaligned() as i32;
                    let tab = global::<u32>(TABLE_C) as *const u32 as u32;
                    let slot = (tab.wrapping_add((idx as u32).wrapping_mul(4))
                        as *const u32)
                        .read_unaligned();
                    (slot.wrapping_add(0x11c) as *const u32).read_unaligned()
                };
            } else if (this.wrapping_add(0x40) as *const u8).read() == 0 {
                val = (arg.wrapping_add(0xb94) as *const u32).read_unaligned();
            } else {
                if (this.wrapping_add(0x41) as *const u8).read() != 0 {
                    let now = *global::<u32>(CLOCK_G);
                    (this.wrapping_add(0x38) as *mut u32).write_unaligned(now);
                    (this.wrapping_add(0x41) as *mut u8).write(0);
                }
                let end = (this.wrapping_add(0x3c) as *const u32)
                    .read_unaligned()
                    .wrapping_add(
                        (this.wrapping_add(0x38) as *const u32).read_unaligned(),
                    );
                if (end as i32) <= (*global::<u32>(CLOCK_G) as i32) {
                    val = (arg.wrapping_add(0xb94) as *const u32).read_unaligned();
                } else {
                    let p = (arg.wrapping_add(0x2c4) as *const u32).read_unaligned();
                let r = if p != 0 {
                    (p.wrapping_add(0x25c) as *const u32).read_unaligned()
                } else {
                    0
                };
                let h = if r != 0 {
                    (r.wrapping_add(0x18) as *const u32).read_unaligned()
                } else {
                    0
                };
                val = if h != 0 {
                    let t: u32 = callee_cdecl!(2, u32, h);
                    (t.wrapping_add(0xdc) as *const u32).read_unaligned()
                } else {
                    let idx =
                        (arg.wrapping_add(0x2e) as *const i16).read_unaligned() as i32;
                    let tab = global::<u32>(TABLE_C) as *const u32 as u32;
                    let slot = (tab.wrapping_add((idx as u32).wrapping_mul(4))
                        as *const u32)
                        .read_unaligned();
                    (slot.wrapping_add(0x11c) as *const u32).read_unaligned()
                };
                }
            }
        }
        if (val as i32) <= -1 {
            return FALLBACK;
        }
        let mgr = *global::<u32>(MGR_G);
        let a3: u32 = callee_thiscall!(3, u32, mgr, val);
        let a4: u32 = callee_cdecl!(4, u32, a3);
        let w2 = *global::<u32>(W2_G);
        let a5: u32 = callee_cdecl!(5, u32, a4, w2);
        if a5 & 0xff != 0 {
            val
        } else {
            FALLBACK
        }
    }
});
