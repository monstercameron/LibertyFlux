// original: 0x00D4E1C0 CTaskSimpleDuck::vf17

// Update handler (vf17): times out the duck, then either finishes or sustains it.
///
/// When the word at `+0x18` is non-zero and the global tick minus the word at
/// `+0x14` reaches it (unsigned), the byte at `+0x29` is set. A set `+0x29`
/// jumps to the finishing path. Otherwise the argument is queried through its
/// vtable slot at `+0xfc` for a float (intercepted callee 4); below 1.0 also
/// takes the finishing path. (The original parks that float in its own
/// argument slot as scratch; the stack check is off for that reason.)
/// Otherwise the argument goes in ECX with (1, -1) to intercepted callee 1;
/// a set byte at `+0x2a` then yields 0. With `+0x2a` clear and `+0x18`
/// non-zero, a tick past `+0x14` + `+0x18` (unsigned, reachable only through
/// wraparound) runs intercepted callee 2 with (argument, 0, 0). A
/// non-positive word at `+0x1c` then yields 0; otherwise intercepted callee 3
/// supplies a subtrahend for `+0x18`, clamped at zero, the low 16 bits go
/// back to `+0x1c`, and the result is 0. The finishing path: with `+0x2a`
/// clear and bit 0 of `+0xc` clear, vtable slot `+0x14` of the object runs
/// with (argument, 1, 0) (intercepted callee 5), setting bit 2 of `+0xc` on a
/// non-zero answer; then the argument goes in ECX with (0, -1) to callee 1
/// and the result is 1.
///
/// Original: 0x00D4E1C0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4e1c0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const TICK_SLOT: u32 = 0x011735B4;
        const ONE_SLOT: u32 = 0x00FE88E8;
        const ANNOUNCE: u32 = 1;
        const SUSTAIN: u32 = 2;
        const ELAPSED: u32 = 3;
        const SAMPLE: u32 = 4;
        const FINISH: u32 = 5;
        const SAMPLE_SLOT: u32 = 0xfc;
        const FINISH_SLOT: u32 = 0x14;
        unsafe fn vcall0f(obj: u32, slot: u32) -> f32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let addr = ((vt + slot) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(addr as usize);
                f(obj)
            }
        }
        unsafe fn vcall3(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let addr = ((vt + slot) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(obj, a0, a1, a2)
            }
        }
        let tick = lf_checker_rt::global::<u32>(TICK_SLOT).read();
        let w18 = ((this + 0x18) as *const u32).read_unaligned();
        if w18 != 0 {
            let w14 = ((this + 0x14) as *const u32).read_unaligned();
            if tick.wrapping_sub(w14) >= w18 {
                ((this + 0x29) as *mut u8).write(1);
            }
        }
        let finishing: bool;
        if ((this + 0x29) as *const u8).read() != 0 {
            finishing = true;
        } else {
            let v = vcall0f(arg0, SAMPLE_SLOT);
            let one = f32::from_bits(lf_checker_rt::global::<u32>(ONE_SLOT).read());
            if one > v {
                finishing = true;
            } else {
                lf_checker_rt::callee_thiscall!(ANNOUNCE, u32, arg0, 1, 0xFFFFFFFF);
                if ((this + 0x2a) as *const u8).read() != 0 {
                    return 0;
                }
                let w18b = ((this + 0x18) as *const u32).read_unaligned();
                if w18b != 0 {
                    let w14b = ((this + 0x14) as *const u32).read_unaligned();
                    if tick > w14b.wrapping_add(w18b) {
                        lf_checker_rt::callee_thiscall!(SUSTAIN, u32, this, arg0, 0, 0);
                    }
                }
                if ((this + 0x1c) as *const i16).read_unaligned() <= 0 {
                    return 0;
                }
                let sub: u32 = lf_checker_rt::callee_cdecl!(ELAPSED, u32,);
                let left = w18b.wrapping_sub(sub);
                let clamped = if (left as i32) < 0 { 0 } else { left };
                ((this + 0x1c) as *mut u16).write_unaligned(clamped as u16);
                return 0;
            }
        }
        if finishing {
            if ((this + 0x2a) as *const u8).read() == 0
                && ((this + 0xc) as *const u8).read() & 1 == 0
            {
                if vcall3(this, FINISH_SLOT, arg0, 1, 0) as u8 != 0 {
                    let f = ((this + 0xc) as *const u32).read_unaligned();
                    ((this + 0xc) as *mut u32).write_unaligned(f | 2);
                }
            }
            lf_checker_rt::callee_thiscall!(ANNOUNCE, u32, arg0, 0, 0xFFFFFFFF);
            1
        } else {
            0
        }
    }
});
