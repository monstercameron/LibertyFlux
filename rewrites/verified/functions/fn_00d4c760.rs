// original: 0x00d4c760 gun_task_setup (proposed)

/// Set up a ped's gun task state and build the subtask for it.
///
/// `this` is the task object, `ped` a ped pointer; the second stack word is
/// never read. The ped's weapon manager is resolved first (a null manager
/// returns null); the current weapon info is cached at `+0x68`, the flag
/// word at `+0x5c` is seeded from `+0x74`, and the mode byte at `+0x72` is
/// set to 1. Flag bit 3 (`FLAG_ARMED`) is then folded in from two hints:
/// virtual slot `+0xc` of the child task at `+0x08` (when non-null)
/// answering `0x413`, or the armed check over the ped's `+0x78` word with
/// constant argument `0xd1`. The ped's ducking state is recorded at `+0x7c`
/// (first answer) and branched on (second answer).
///
/// When ducking with a non-1 mode answer, a light task is built: the pool
/// allocator is asked for a block (null returns null), the `+0x40` float
/// scaled by a read-only factor is converted with `cvttss2si` semantics and
/// stored at block `+0x14`, and the block is stamped with its virtual table
/// and zeroed tail. When ducking with mode 1 and both `+0x50`/`+0x52` id
/// halfwords present (not -1), or when not ducking with both `+0x4c`/`+0x4e`
/// present, the gun-task constructor runs on an allocated block and the
/// halfwords are stored sign-extended into the new object (a null block is
/// still written through, faulting exactly like the original). Otherwise
/// the cached weapon info must be valid (`+0x28` not -1; the ducked path
/// also requires bit 15 of `+0x20`) and the constructor's answer is
/// returned directly.
///
/// Original: 0x00D4C760 (thiscall, two stack words, the second unread).
lf_checker_rt::export!(thiscall, rw_00d4c760(this: u32, ped: u32, _unused: u32) -> u32 {
    unsafe {
        const C_MGR: u32 = 1;
        const C_WINFO: u32 = 2;
        const C_ARMED: u32 = 4;
        const C_DUCK: u32 = 5;
        const C_MODE: u32 = 6;
        const C_ALLOC: u32 = 7;
        const C_LIGHT_INIT: u32 = 8;
        const C_GUN_CTOR: u32 = 9;
        const T_CHILD: u32 = 0x08;
        const T_CTOR_ARG: u32 = 0x14;
        const T_PARAMS: u32 = 0x20;
        const T_SCALE: u32 = 0x40;
        const ID_A0: u32 = 0x4c;
        const ID_A1: u32 = 0x4e;
        const ID_B0: u32 = 0x50;
        const ID_B1: u32 = 0x52;
        const T_FLAGS: u32 = 0x5c;
        const T_WINFO: u32 = 0x68;
        const T_MODE: u32 = 0x72;
        const T_FLAG_SEED: u32 = 0x74;
        const T_DUCK: u32 = 0x7c;
        const P_ARMED: u32 = 0x78;
        const P_MODE: u32 = 0x224;
        const P_WMGR: u32 = 0x2b0;
        const MGR_WEAPON_ID: u32 = 0x18;
        const INFO_FALLBACK: u32 = 0x20;
        const INFO_VALID: u32 = 0x28;
        const FALLBACK_BIT: u32 = 15;
        const ARMED_ARG: u32 = 0xd1;
        const CHILD_ARMED_KIND: u32 = 0x413;
        const FLAG_ARMED: u32 = 8;
        const LIGHT_VTABLE: u32 = 0x00eb391c;
        const CTOR_NEG4: u32 = 0xc0800000; // -4.0f
        const G_POOL: u32 = 0x0167e2a0;
        const G_CTOR_FACTOR: u32 = 0x010550b4;
        const C_SCALE_FACTOR: u32 = 0x00fe8c58;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (core::hint::black_box(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// `cvttss2si` semantics: truncate toward zero; NaN, infinities and
        /// out-of-range values yield `i32::MIN`.
        #[inline(always)]
        fn cvttss2si(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        unsafe fn construct_gun(this: u32, block: u32) -> u32 {
            unsafe {
                let factor = lf_checker_rt::global::<u32>(G_CTOR_FACTOR).read_unaligned();
                lf_checker_rt::callee_thiscall!(
                    C_GUN_CTOR,
                    u32,
                    block,
                    rd32(this + T_CTOR_ARG),
                    this + T_PARAMS,
                    rd32(this + T_FLAGS),
                    rd32(this + T_SCALE),
                    factor,
                    CTOR_NEG4
                )
            }
        }

        let mgr = lf_checker_rt::callee_thiscall!(C_MGR, u32, ped + P_WMGR);
        if mgr == 0 {
            return 0;
        }
        let info = lf_checker_rt::callee_cdecl!(C_WINFO, u32, rd32(mgr + MGR_WEAPON_ID));
        wr32(this + T_WINFO, info);
        wr32(this + T_FLAGS, rd32(this + T_FLAG_SEED));
        ((this + T_MODE) as *mut u8).write(1);
        let child = rd32(this + T_CHILD);
        if child != 0 {
            let kind: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(child) + 0x0c) as usize);
            if kind(child) == CHILD_ARMED_KIND {
                wr32(this + T_FLAGS, rd32(this + T_FLAGS) | FLAG_ARMED);
            }
        }
        if rd32(this + T_FLAGS) & FLAG_ARMED == 0 {
            let armed = lf_checker_rt::callee_thiscall!(C_ARMED, u32, rd32(ped + P_ARMED), ARMED_ARG);
            if armed != 0 {
                wr32(this + T_FLAGS, rd32(this + T_FLAGS) | FLAG_ARMED);
            }
        }
        let duck = lf_checker_rt::callee_thiscall!(C_DUCK, u32, ped);
        ((this + T_DUCK) as *mut u8).write(duck as u8);
        let duck = lf_checker_rt::callee_thiscall!(C_DUCK, u32, ped);
        if (duck as u8) != 0 {
            let mode = lf_checker_rt::callee_thiscall!(C_MODE, u32, rd32(ped + P_MODE));
            if mode != 1 {
                ((this + T_MODE) as *mut u8).write(5);
                let pool = lf_checker_rt::global::<u32>(G_POOL).read_unaligned();
                let blk = lf_checker_rt::callee_thiscall!(C_ALLOC, u32, pool);
                if blk == 0 {
                    return 0;
                }
                let scale = f32::from_bits(rd32(this + T_SCALE));
                let factor = f32::from_bits(lf_checker_rt::global::<u32>(C_SCALE_FACTOR).read_unaligned());
                let scaled = cvttss2si(fmul(scale, factor));
                lf_checker_rt::callee_thiscall!(C_LIGHT_INIT, u32, blk);
                wr32(blk + 0x14, scaled as u32);
                wr32(blk, lf_checker_rt::relocated(LIGHT_VTABLE));
                ((blk + 0x18) as *mut u8).write(0);
                wr32(blk + 0x1c, 0);
                return blk;
            }
            if rd16(this + ID_B0) as i16 != -1 && rd16(this + ID_B1) as i16 != -1 {
                let blk = lf_checker_rt::callee_thiscall!(C_ALLOC, u32, lf_checker_rt::global::<u32>(G_POOL).read_unaligned());
                let obj = if blk == 0 { 0 } else { construct_gun(this, blk) };
                wr32(obj + 0x54, rd16(this + ID_B0) as i16 as i32 as u32);
                wr32(obj + 0x58, rd16(this + ID_B1) as i16 as i32 as u32);
                if rd16(this + ID_A0) as i16 != -1 && rd16(this + ID_A1) as i16 != -1 {
                    wr32(obj + 0x4c, rd16(this + ID_A0) as i16 as i32 as u32);
                    wr32(obj + 0x50, rd16(this + ID_A1) as i16 as i32 as u32);
                }
                return obj;
            }
        } else if rd16(this + ID_A0) as i16 != -1 && rd16(this + ID_A1) as i16 != -1 {
            let blk = lf_checker_rt::callee_thiscall!(C_ALLOC, u32, lf_checker_rt::global::<u32>(G_POOL).read_unaligned());
            let obj = if blk == 0 { 0 } else { construct_gun(this, blk) };
            wr32(obj + 0x4c, rd16(this + ID_A0) as i16 as i32 as u32);
            wr32(obj + 0x50, rd16(this + ID_A1) as i16 as i32 as u32);
            if rd16(this + ID_B0) as i16 != -1 && rd16(this + ID_B1) as i16 != -1 {
                wr32(obj + 0x54, rd16(this + ID_B0) as i16 as i32 as u32);
                wr32(obj + 0x58, rd16(this + ID_B1) as i16 as i32 as u32);
            }
            return obj;
        }
        // Fallback gate. The ducked path also demands bit 15 of the info
        // word (the original shifts, tests the bit and jumps onto a shared
        // conditional jump, so the flags carry the decision); the
        // non-ducked path only demands a valid info row.
        let info = rd32(this + T_WINFO);
        if rd32(info + INFO_VALID) == 0xffffffff {
            return 0;
        }
        if (duck as u8) != 0 && (rd32(info + INFO_FALLBACK) >> FALLBACK_BIT) & 1 == 0 {
            return 0;
        }
        let blk = lf_checker_rt::callee_thiscall!(C_ALLOC, u32, lf_checker_rt::global::<u32>(G_POOL).read_unaligned());
        if blk == 0 {
            return 0;
        }
        construct_gun(this, blk)
    }
});
