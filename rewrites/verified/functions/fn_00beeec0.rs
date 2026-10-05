// original: 0x00beeec0 ambient_jet_update (symbols: AMBIENT_JET)

/// Update the ambient-jet slot for one task: create the jet object on first
/// use, ensure its helpers exist, run the audio tail, and rebind it.
///
/// `this` is the task; the slot index is the byte at `+0xe` selecting one
/// lane of each global table (object table `0x169e84c`, jet table
/// `0x169e7a0`, helper tables `0x169e7e8`/`0x169e800`, stride-80 parameter
/// table `0x169e870`, result table `0x169e8b8`).
///
/// When the object slot is empty, a factory callee builds the object, stores
/// it, and (for a nonzero object) configures it with constant arguments,
/// wakes the jet's mover when its state word is zero, and links object and
/// jet through three binder callees. Slots below index 2 then ensure the
/// first helper through a creator callee addressed at a fixed manager object
/// (a scratch struct built by a constructor callee, a name constant, the
/// table address, and constant flags), use it through an apply callee when all
/// three of helper, object and jet exist, and ping the second helper when it
/// exists. Slots at index 2 and above do the same for the first helper, then
/// for the second helper, and when both exist run the audio tail: a setup
/// callee on the parameter row (name constant, scratch struct, three float
/// constants passed as pre-stored outgoing words, which is why it pops five
/// words though only two are pushed at the call), the mover wake-up, the
/// mover state stored into the result row at `80*index`, a measure callee
/// returning on the x87 stack whose result feeds two level callees, and the
/// apply callee. The function ends by waking the jet's mover again and
/// rebinding the object when both exist.
///
/// The result-row index is `80*index` held in a local, not the incoming
/// `edi` register: the word read sits above the post-call stack pointer
/// because the setup callee pops five words.
///
/// Original: thiscall, no stack words, no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00beeec0(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x0e;
        const JET_STATE: u32 = 0x20;
        const JET_MOVER: u32 = 0x10;
        const ROW_STRIDE: u32 = 80;
        const T_JET: u32 = 0x169e7a0;
        const T_HELP0: u32 = 0x169e7e8;
        const T_HELP1: u32 = 0x169e800;
        const T_OBJ: u32 = 0x169e84c;
        const T_PARAM: u32 = 0x169e870;
        const T_RESULT: u32 = 0x169e8b8;
        const MANAGER: u32 = 0x1231800;
        const NAME_HELP0_LO: u32 = 0xebae98;
        const NAME_HELP0_HI: u32 = 0xebaeac;
        const NAME_HELP1: u32 = 0xebaeb8;
        const NAME_AUDIO: u32 = 0xebaecc;
        const CFG_RATE: u32 = 0xfa0;
        const CFG_GAIN: u32 = 0x3f000000; // 0.5
        const CFG_RANGE: u32 = 0x42200000; // 40.0
        const AU_F0: u32 = 0xc1c00000; // -24.0
        const AU_F1: u32 = 0x42200000; // 40.0
        const AU_F2: u32 = 0x42b40000; // 90.0
        const FULL: u32 = 0x3f800000; // 1.0
        const C_FACTORY: u32 = 1;
        const C_CONFIG: u32 = 2;
        const C_WAKE: u32 = 3;
        const C_LINK_MOVER: u32 = 4;
        const C_BIND_STATE: u32 = 5;
        const C_BIND_JET: u32 = 6;
        const C_ACTIVATE: u32 = 7;
        const C_CTOR: u32 = 8;
        const C_CREATE: u32 = 9;
        const C_APPLY: u32 = 10;
        const C_PING: u32 = 11;
        const C_AUDIO_SETUP: u32 = 12;
        const C_MEASURE: u32 = 13;
        const C_LEVEL_A: u32 = 14;
        const C_LEVEL_B: u32 = 15;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn glob(a: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(a)) }
        }
        #[inline(always)]
        unsafe fn set_glob(a: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u32).write_unaligned(v) }
        }

        /// Wake the jet's mover when its state word is zero, then link it.
        unsafe fn wake_and_link(jet: u32) {
            unsafe {
                if rd32(jet + JET_STATE) == 0 {
                    lf_checker_rt::callee_thiscall!(C_WAKE, u32, jet);
                    lf_checker_rt::callee_thiscall!(C_LINK_MOVER, u32, jet + JET_MOVER, rd32(jet + JET_STATE));
                }
            }
        }

        let esi = rd8(this + SLOT) as u32;
        let g_obj = T_OBJ + esi * 4;
        let g_jet = T_JET + esi * 4;
        if glob(g_obj) == 0 {
            let h = lf_checker_rt::callee_thiscall!(C_FACTORY, u32, this);
            set_glob(g_obj, h);
            if h != 0 {
                lf_checker_rt::callee_thiscall!(C_CONFIG, u32, h, 0, CFG_RANGE, 0, CFG_RATE, CFG_GAIN);
                let jet = glob(g_jet);
                wake_and_link(jet);
                lf_checker_rt::callee_thiscall!(C_BIND_STATE, u32, glob(g_obj), rd32(jet + JET_STATE));
                lf_checker_rt::callee_thiscall!(C_BIND_JET, u32, glob(g_obj), glob(g_jet));
                lf_checker_rt::callee_thiscall!(C_ACTIVATE, u32, glob(g_obj));
            }
        }
        if esi < 2 {
            let g_h = T_HELP0 + esi * 4;
            if glob(g_h) == 0 {
                let h = glob(g_obj);
                if h != 0 {
                    let tmp = [0u32; 16];
                    lf_checker_rt::callee_thiscall!(C_CTOR, u32, tmp.as_ptr() as u32);
                    let arg = [0u32; 8];
                    lf_checker_rt::callee_thiscall!(
                        C_CREATE, u32, lf_checker_rt::relocated(MANAGER),
                        lf_checker_rt::relocated(NAME_HELP0_LO),
                        lf_checker_rt::relocated(g_h), arg.as_ptr() as u32, 0xffffffff, 0, 0
                    );
                }
            }
            if glob(g_h) != 0 && glob(g_obj) != 0 && glob(g_jet) != 0 {
                let a = glob(g_jet);
                let d = rd32(a + JET_STATE);
                let p = if d != 0 { d + 0x30 } else { a + JET_MOVER };
                lf_checker_rt::callee_thiscall!(C_APPLY, u32, glob(g_h), p);
            }
            let f = glob(T_HELP1 + esi * 4);
            if f != 0 {
                lf_checker_rt::callee_thiscall!(C_PING, u32, f, 0);
            }
        } else {
            let g_h = T_HELP0 + esi * 4;
            let tmp = [0u32; 16];
            lf_checker_rt::callee_thiscall!(C_CTOR, u32, tmp.as_ptr() as u32);
            if glob(g_h) == 0 {
                let h = glob(g_obj);
                if h != 0 {
                    let arg = [0u32; 8];
                    lf_checker_rt::callee_thiscall!(
                        C_CREATE, u32, lf_checker_rt::relocated(MANAGER),
                        lf_checker_rt::relocated(NAME_HELP0_HI),
                        lf_checker_rt::relocated(g_h), arg.as_ptr() as u32, 0xffffffff, 0, 0
                    );
                }
            }
            if glob(g_h) != 0 && glob(g_obj) != 0 && glob(g_jet) != 0 {
                let a = glob(g_jet);
                let d = rd32(a + JET_STATE);
                let p = if d != 0 { d + 0x30 } else { a + JET_MOVER };
                lf_checker_rt::callee_thiscall!(C_APPLY, u32, glob(g_h), p);
            }
            let g_h1 = T_HELP1 + esi * 4;
            if glob(g_h1) == 0 {
                let h = glob(g_obj);
                if h != 0 {
                    let arg = [0u32; 8];
                    lf_checker_rt::callee_thiscall!(
                        C_CREATE, u32, lf_checker_rt::relocated(MANAGER),
                        lf_checker_rt::relocated(NAME_HELP1),
                        lf_checker_rt::relocated(g_h1), arg.as_ptr() as u32, 0xffffffff, 0, 0
                    );
                }
            }
            if glob(g_h1) != 0 && glob(g_jet) != 0 {
                let row = esi * ROW_STRIDE;
                let frame = [0u32, FULL, 0u32, 0u32];
                lf_checker_rt::callee_thiscall!(
                    C_AUDIO_SETUP, u32, lf_checker_rt::relocated(T_PARAM + row),
                    lf_checker_rt::relocated(NAME_AUDIO), frame.as_ptr() as u32, AU_F0, AU_F1, AU_F2
                );
                let b = glob(g_jet);
                wake_and_link(b);
                set_glob(T_RESULT + row, rd32(glob(g_jet) + JET_STATE));
                let r: f32 = lf_checker_rt::callee_thiscall!(
                    C_MEASURE, f32, lf_checker_rt::relocated(T_PARAM + row), 0
                );
                let f = glob(g_h1);
                lf_checker_rt::callee_thiscall!(C_LEVEL_A, u32, f, r.to_bits());
                lf_checker_rt::callee_thiscall!(C_LEVEL_B, u32, f, r.to_bits());
                let a = glob(g_jet);
                let d = rd32(a + JET_STATE);
                let p = if d != 0 { d + 0x30 } else { a + JET_MOVER };
                lf_checker_rt::callee_thiscall!(C_APPLY, u32, f, p);
            }
        }
        if glob(g_obj) != 0 {
            let jet = glob(g_jet);
            if jet != 0 {
                wake_and_link(jet);
                lf_checker_rt::callee_thiscall!(C_BIND_STATE, u32, glob(g_obj), rd32(jet + JET_STATE));
            }
        }
        0
    }
});
