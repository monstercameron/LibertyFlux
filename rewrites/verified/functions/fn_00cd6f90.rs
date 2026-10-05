// original: 0x00CD6F90 CTaskSimpleGetUp::vf17

/// returns its answer in `al`).
export!(thiscall, rw_00cd6f90(task: u32, sub: u32) -> u32 {
    unsafe {
        const OWNER_MARK: u32 = 0xbe0;
        const OWNER_TRY: u32 = 0x298;
        const TRY_STAMP: u8 = 0x32;
        const STATE: u32 = 0x14;
        const OWNER_FLAG: u32 = 0x29c;
        const ADJ_TASK_BYTE: u32 = 0x20;
        const ADJ_SUB_BYTE: u32 = 0xf4;
        const ADJ_SUB_BITS: u8 = 0x60;
        const DIRECT_CODE_A: u32 = 0x61;
        const DIRECT_CODE_B: u32 = 0x62;
        const EIGHT_BITS: u32 = 0x4100_0000;
        const NEG_ONE_BITS: u32 = 0xbf00_0000;
        const SUB_REC: u32 = 0x20;
        const TASK_21: u32 = 0x21;
        const PROBE_FLAG_WANT: u32 = 4;
        const DRIVER_OBJ: u32 = 0x224;
        const DRIVER_SLOT: u32 = 0x1c;
        const RESOLVE_OFF: u32 = 0x20;
        const KIND_SLOT: u32 = 0x04;
        const KIND_WANT: u32 = 9;
        const FETCH_ARG: u32 = 0x20;
        const FETCH_WORD: u32 = 0x0c;
        const TAIL_MARK: u32 = 0x270;
        const SUB_WORD: u32 = 0x10;
        const SUB_STATE: u32 = 0x18;
        const SUB_STATE_PLACING: u32 = 5;
        const SUB_STATE_NEXT: u32 = 6;
        const SUB_PARAM: u32 = 0x1c;
        const SUB_PARAM_RESET: u32 = 0x66;
        const RISE_PTR: u32 = 0x10;
        const RISE_BYTE: u32 = 0x22;
        const RISE_REF: u32 = 0x24;
        const HEIGHT_OFF: u32 = 0x4c;
        const DONE_MARK: u32 = 0x26c;
        const SIGN_FLIP: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn or32(a: u32, bits: u32) {
            unsafe {
                let v = rd32(a) | bits;
                (a as *mut u32).write_unaligned(v);
            }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        or32(sub.wrapping_add(OWNER_MARK), 2);
        (sub.wrapping_add(OWNER_TRY) as *mut u8).write(TRY_STAMP);

        match rd32(task.wrapping_add(STATE)) {
            0 => {
                or32(sub.wrapping_add(OWNER_FLAG), 1);
                let _: u32 = callee_thiscall!(1, u32, sub, 1);
                let mut probe = [0u32; 16];
                let probe_arg = (&mut probe[3] as *mut u32) as u32;
                let _: u32 = callee_thiscall!(2, u32, probe_arg);
                if rd8(task.wrapping_add(ADJ_TASK_BYTE)) != 0
                    && rd8(sub.wrapping_add(ADJ_SUB_BYTE)) & ADJ_SUB_BITS != 0
                {
                    let p0 = (&probe[0] as *const u32) as u32;
                    let p2 = (&probe[2] as *const u32) as u32;
                    let p3 = (&probe[3] as *const u32) as u32;
                    let _: u32 = callee_thiscall!(3, u32, task, sub, p0, p2, p3);
                }
                // Probe floats past the stub's sixteen words are unwritten
                // frame slots: the defined stack fill (0.0).
                let f30 = f32::from_bits(probe[15]);
                if f30 != 0.0 || 0.0f32 != 0.0 || 0.0f32 != 0.0 {
                    let code = probe[2];
                    if code == DIRECT_CODE_A || code == DIRECT_CODE_B {
                        (task.wrapping_add(STATE) as *mut u32).write_unaligned(3);
                    } else {
                        let rec = rd32(sub.wrapping_add(SUB_REC));
                        let mut frame = [0u32; 16];
                        frame[..12].copy_from_slice(&probe[3..15]);
                        frame[12] = rd32(rec.wrapping_add(0x30));
                        frame[13] = rd32(rec.wrapping_add(0x34));
                        frame[14] = rd32(rec.wrapping_add(0x38));
                        frame[15] = rd32(rec.wrapping_add(0x3c));
                        let fptr = (&frame[0] as *const u32) as u32;
                        let _: u32 = callee_thiscall!(
                            6, u32, task, sub, probe[0], code, EIGHT_BITS, fptr, 1
                        );
                        if probe[0] == PROBE_FLAG_WANT {
                            (task.wrapping_add(TASK_21) as *mut u8).write(1);
                        }
                        (task.wrapping_add(STATE) as *mut u32).write_unaligned(1);
                    }
                } else {
                    let pose: u32 = callee_thiscall!(4, u32, task, sub, EIGHT_BITS);
                    let st = rd32(task.wrapping_add(SUB_STATE));
                    let _: u32 = callee_thiscall!(5, u32, task, sub, st, pose);
                    (task.wrapping_add(STATE) as *mut u32).write_unaligned(2);
                }
                if rd32(task.wrapping_add(STATE)) == 3 {
                    return 0;
                }
                let driver = rd32(sub.wrapping_add(DRIVER_OBJ));
                if driver == 0 {
                    return 0;
                }
                let slot = rd32(rd32(driver).wrapping_add(DRIVER_SLOT));
                let drv: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let target: u32 = callee_thiscall!(8, u32, drv(driver).wrapping_add(RESOLVE_OFF));
                if target == 0 {
                    return 0;
                }
                let kslot = rd32(rd32(target).wrapping_add(KIND_SLOT));
                let kind: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(kslot as usize);
                if kind(target) != KIND_WANT {
                    return 0;
                }
                let fetched: u32 = callee_cdecl!(10, u32, rd32(target.wrapping_add(FETCH_ARG)));
                if fetched == 0 {
                    return 0;
                }
                if rd32(fetched.wrapping_add(FETCH_WORD)) != 1 {
                    return 0;
                }
                or32(sub.wrapping_add(TAIL_MARK), 4);
                0
            }
            1 => {
                or32(sub.wrapping_add(OWNER_FLAG), 1);
                if rd32(task.wrapping_add(SUB_WORD)) != 0 {
                    return 0;
                }
                if rd32(task.wrapping_add(SUB_STATE)) == SUB_STATE_PLACING
                    && rd8(task.wrapping_add(TASK_21)) != 0
                {
                    let param = rd32(task.wrapping_add(SUB_PARAM));
                    (task.wrapping_add(SUB_STATE) as *mut u32).write_unaligned(SUB_STATE_NEXT);
                    let placed: u32 = callee_cdecl!(11, u32, 6, param);
                    if placed == 0 {
                        (task.wrapping_add(SUB_PARAM) as *mut u32)
                            .write_unaligned(SUB_PARAM_RESET);
                    }
                }
                let pose: u32 = callee_thiscall!(4, u32, task, sub, EIGHT_BITS);
                let st = rd32(task.wrapping_add(SUB_STATE));
                let _: u32 = callee_thiscall!(5, u32, task, sub, st, pose);
                (task.wrapping_add(STATE) as *mut u32).write_unaligned(2);
                0
            }
            2 => {
                let rise = rd32(task.wrapping_add(RISE_PTR));
                if rise == 0 {
                    (task.wrapping_add(STATE) as *mut u32).write_unaligned(3);
                    return 0;
                }
                let mut slow = false;
                if rd8(task.wrapping_add(RISE_BYTE)) == 0 {
                    slow = true;
                } else {
                    let h = rdf(rise.wrapping_add(HEIGHT_OFF));
                    let r = rdf(task.wrapping_add(RISE_REF));
                    if !(h >= r) {
                        slow = true;
                    }
                }
                if !slow {
                    let _: u32 = callee_thiscall!(12, u32, rise, NEG_ONE_BITS);
                    (task.wrapping_add(STATE) as *mut u32).write_unaligned(3);
                    return 0;
                }
                let c75 = f32::from_bits(*global::<u32>(0xFE888C));
                if !(c75 > rdf(rise.wrapping_add(HEIGHT_OFF))) {
                    let ready: u32 = callee_thiscall!(13, u32, sub);
                    if ready & 0xff == 0 {
                        return 0;
                    }
                    let c25 = f32::from_bits(*global::<u32>(0xFE87E4));
                    if !(rdf(rise.wrapping_add(HEIGHT_OFF)) > c25) {
                        return 0;
                    }
                    let owner: u32 = callee_thiscall!(14, u32, sub);
                    let m1: u32 = callee_thiscall!(15, u32, owner);
                    let v1: f32 = callee_cdecl!(16, f32, m1);
                    let m2: u32 = callee_thiscall!(17, u32, owner);
                    let v2: f32 = callee_cdecl!(16, f32, m2);
                    let n1 = f32::from_bits(v2.to_bits() ^ SIGN_FLIP);
                    let norm = core::hint::black_box(add(mul(n1, n1), mul(v1, v1))).sqrt();
                    if !(norm > c25) {
                        return 0;
                    }
                    (task.wrapping_add(STATE) as *mut u32).write_unaligned(3);
                    return 0;
                }
                or32(sub.wrapping_add(OWNER_FLAG), 1);
                0
            }
            3 => {
                or32(sub.wrapping_add(DONE_MARK), 1);
                1
            }
            _ => 0,
        }
    }
});
