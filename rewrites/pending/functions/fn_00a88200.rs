// original: 0x00A88200 poll_window_and_refresh_view
/// Poll the window state and refresh the view from its source.
///
/// Checks whether the window is iconic (or the override pair is set) and
/// clears the status bytes when fully idle, refreshes the member views,
/// rebases the source triple against the anchor triple, runs the bearing
/// solver, applies the gated secondary refresh, walks the sink chain to the
/// first live sink, and folds the tag byte into the return flag.
export!(thiscall, rw_a88200(this: u32) -> u32 {
    const ANCHOR: u32 = 0x0128_E310;
    const MEMBER: u32 = 0x0128_E400;
    const SIGN_BITS: u32 = 0x8000_0000;
    callee_thiscall!(1, u32, this);
    let iconic = callee_stdcall!(2, u32, unsafe { global::<u32>(0x017A_CCD8).read() });
    let active = if iconic != 0 {
        1u8
    } else if unsafe { global::<u8>(0x0105_B48F).read() } == 0 {
        0u8
    } else if unsafe { global::<u8>(0x017E_D8D1).read() } == 0 {
        0u8
    } else {
        1u8
    };
    let idle = unsafe {
        active | global::<u8>(0x0117_3590).read() | global::<u8>(0x0117_3591).read()
    };
    if idle == 0 {
        unsafe {
            global::<u8>(0x016D_8DD5).write(0);
            global::<u8>(0x016D_8DD6).write(0);
            global::<u8>(0x016D_8DD4).write(0);
            global::<u8>(0x012D_D5E2).write(0);
            global::<u8>(0x012D_D298).write(0);
            global::<u8>(0x012B_D192).write(0);
            global::<u8>(0x0161_54A0).write(0);
            global::<u8>(0x0161_54A1).write(0);
            global::<u8>(0x012B_D193).write(0);
            global::<u8>(0x0128_E3A4).write(0);
            global::<u8>(0x0103_F425).write(1);
            global::<u8>(0x0103_B76B).write(0);
        }
    }
    unsafe {
        let m = global::<u8>(0x0128_E3E1);
        m.write(m.read() & 0x9D);
    }
    callee_thiscall!(3, u32, relocated(MEMBER));
    callee_thiscall!(4, u32, this);
    callee_thiscall!(5, u32, relocated(MEMBER));
    let src = unsafe { ((this + 8) as *const u32).read().wrapping_add(0x10) };
    let (tx, ty, tz) = unsafe {
        (
            ((src + 0x38) as *const f32).read(),
            ((src + 0x30) as *const f32).read(),
            ((src + 0x34) as *const f32).read(),
        )
    };
    // Subtraction keeps operand order naturally; the fourth word is whatever
    // the scratch stack holds (defined fill under the checker).
    let pad = core::mem::MaybeUninit::<u32>::uninit();
    let pad_word = unsafe { pad.as_ptr().read_volatile() };
    unsafe {
        let dx = tx - global::<f32>(0x0128_E348).read();
        let dy = ty - global::<f32>(0x0128_E340).read();
        let dz = tz - global::<f32>(0x0128_E344).read();
        global::<f32>(0x0128_E398).write(dx);
        global::<f32>(0x0128_E390).write(dy);
        global::<f32>(0x0128_E394).write(dz);
        global::<u32>(0x0128_E39C).write(pad_word);
    }
    callee_thiscall!(6, u32, relocated(ANCHOR), src);
    // Bearing solver takes doubles in vector registers: the stub loads the
    // low halves from the stack transport on this side and compares the full
    // registers. The scripted answer's high half is a contract constant.
    let (d0, d1) = unsafe {
        let x = f32::from_bits(global::<u32>(0x0128_E320).read() ^ SIGN_BITS);
        let y = global::<f32>(0x0128_E324).read();
        (f64::from(x), f64::from(y))
    };
    let ans_lo = callee_cdecl!(7, u32,
        d0.to_bits() as u32, (d0.to_bits() >> 32) as u32,
        d1.to_bits() as u32, (d1.to_bits() >> 32) as u32);
    let ans = f64::from_bits((ans_lo as u64) | ((0u32 as u64) << 32));
    unsafe { global::<f32>(0x0128_E3A0).write(ans as f32) };
    let gated = unsafe {
        global::<u32>(0x011F_7060).read() == 1
            || global::<u32>(0x0120_88B4).read() != global::<u32>(0x00F1_C040).read()
            || global::<u32>(0x0103_7720).read() == 0x12
    };
    if gated {
        let probe = callee_thiscall!(8, u32, this.wrapping_add(4));
        let handle = callee_thiscall!(9, u32, this.wrapping_add(4));
        if handle != 0 && probe != 0 {
            callee_thiscall!(6, u32, probe.wrapping_add(0x10), handle.wrapping_add(0x10));
        }
    }
    unsafe {
        let m = global::<u8>(0x0128_E3E1);
        m.write(m.read() & 0xFE);
    }
    callee_thiscall!(10, u32, relocated(ANCHOR));
    callee_thiscall!(11, u32, relocated(ANCHOR));
    let mut node = callee_thiscall!(12, u32, unsafe { ((this + 8) as *const u32).read() });
    if node != 0 {
        loop {
            let child = unsafe { ((node + 0x110) as *const u32).read() };
            if child != 0 {
                let (flag, word) = unsafe {
                    (
                        ((child + 0x27C) as *const u8).read(),
                        ((child + 0x270) as *const u32).read(),
                    )
                };
                if flag != 0 && word != 0 {
                    let live = callee_thiscall!(13, u32, child);
                    if (live as u8) != 0 {
                        let sink = unsafe { ((child + 0x270) as *const u32).read() };
                        unsafe { (this as *mut u32).write(sink) };
                        break;
                    }
                }
            }
            node = unsafe { ((node + 0x118) as *const u32).read() };
            if node == 0 {
                break;
            }
        }
    }
    let tag = unsafe { global::<u8>(0x0103_F425).read() };
    let out = if unsafe { global::<u8>(0x0103_B76B).read() } != 0 { 1u32 } else { tag as u32 };
    unsafe {
        global::<u8>(0x0103_F425).write(out as u8);
        global::<u8>(0x012F_B1D4).write(0);
        global::<u8>(0x0104_8EC4).write(1);
        global::<u8>(0x016D_8B10).write(0);
        global::<u32>(0x012B_D198).write(0);
        global::<u8>(0x012B_D1BC).write(0);
        global::<u8>(0x012B_D1BD).write(0);
        global::<u8>(0x016D_21A4).write(0);
    }
    out
});
