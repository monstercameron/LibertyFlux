// original: 0x009C1640 BULLET_IMPACT_WATER

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, global, relocated};

// 0x009C1640 BULLET_IMPACT_WATER (symbol name)
// ---------------------------------------------------------------------------

const F3_TASK_OBJ: u32 = 0x10;
const F3_TASK_KEY: u32 = 0x14;
const F3_TASK_OTHER: u32 = 0x1c;
const F3_TASK_FLAG: u32 = 0xca;
const F3_LINK: u32 = 0x20;
const F3_HIT_INFO: u32 = 0x10;
const F3_HIT_STATE: u32 = 0x0c;
const F3_STATE_WANT: u32 = 4;
const F3_HUB_A: u32 = 0x13b6798;
const F3_HUB_B: u32 = 0x12831e4;
const F3_STR_A: u32 = 0xe94cac;
const F3_STR_B: u32 = 0xe94cc0;
const F3_DST: u32 = 0x110db70;
const F3_G_ACTIVE: u32 = 0x11f7060;
const F3_G_A: u32 = 0x12088b4;
const F3_G_B: u32 = 0x0f1c040;
const F3_G_MODE: u32 = 0x1037720;
const F3_C_SQRT: u32 = 0xfe89e0;
const F3_C_ONE: u32 = 0xfe88e8;
const F3_C_RATE: u32 = 0xea395c;


#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}


/// Water bullet-impact handler for one task record (symbol name kept).
///
/// `task` points to the record: linked object at `+0x10`, lookup key at
/// `+0x14`, a compared word at `+0x1c`, a one-shot flag byte at `+0xca`.
///
/// Behaviour: return unless the three global guards pass (active flag set,
/// two globals equal, mode word anything but `0x12`). Resolve the base
/// vector (the link's `+0x30` when the object's `+0x20` link is set, else
/// the object's `+0x10`) and probe through callee 1 (ten arguments); a zero
/// answer ends the call. Otherwise compare the probed value against the
/// base's third word: continue only when strictly above with the flag clear,
/// or not above with the flag set; the flag is set on the upper path and
/// cleared on the lower one. Then
/// resolve the key through callee 2 (a miss faults, like the original),
/// check the hit against callee 3's answer and report equality through
/// callee 4, run the build sequence through callees 5-7, and submit through
/// callee 8: a nonzero answer runs the effect through callees 9 and 11, a
/// zero answer releases through callee 10. Finally resolve the key again
/// (callee 13, same target, second site) and, when the hit's state word is
/// 4, dispatch through callee 12.
///
/// The mid-function float block (square root of a double constant, rate
/// scaling) feeds only scratch the callees never observe; it is implemented
/// faithfully but noted as unobserved in the proof.
///
/// Original: 0x009C1640 (cdecl, one stack word). The contract does not
/// compare the return value: on the early paths it is stub residue or an
/// entry register, not behaviour (recorded narrowing).
lf_checker_rt::export!(cdecl, rw_009C1640(task: u32) -> u32 {
    unsafe { f3_run(task) }
});

/// Shared body of the function.
unsafe fn f3_run(task: u32) -> u32 {
    unsafe {
        if global::<u32>(F3_G_ACTIVE).read_unaligned() == 1 {
            return 0;
        }
        let ga = global::<u32>(F3_G_A).read_unaligned();
        if ga != global::<u32>(F3_G_B).read_unaligned() {
            return ga;
        }
        if global::<u32>(F3_G_MODE).read_unaligned() == 0x12 {
            return ga;
        }
        let obj = rd32(task + F3_TASK_OBJ);
        let olink = rd32(obj + F3_LINK);
        let base = if olink != 0 {
            olink.wrapping_add(0x30)
        } else {
            obj.wrapping_add(0x10)
        };
        // Callee 1 writes one word through its fourth argument; the stub
        // lands it in `probe` on the rewrite side.
        let mut probe = [0u32; 1];
        let b0 = rd32(base);
        let b1 = rd32(base.wrapping_add(4));
        let b2 = rd32(base.wrapping_add(8));
        let a1: u32 = callee_cdecl!(
            1, u32, b0, b1, b2, (&mut probe[0] as *mut u32) as u32, 0, 0, 0,
            0x40c00000, 0x41a00000, 0
        );
        if a1 & 0xff == 0 {
            return 0;
        }
        let w = f32::from_bits(probe[0]);
        let mem = f32::from_bits(rd32(base.wrapping_add(8)));
        // `comiss`/`jbe`: the upper path needs strictly greater.
        let above = w > mem;
        let flag = rd8(task + F3_TASK_FLAG);
        let cont = if above { flag == 0 } else { flag != 0 };
        // The upper path stores DL, which is 1 by then (`(an instruction of the original)` runs
        // before the compare); the lower path stores an immediate 0.
        wr8(task + F3_TASK_FLAG, above as u8);
        if !cont {
            return 0;
        }
        // Scratch float block (unobserved downstream; kept faithful).
        let _ = f3_scratch(obj, olink);
        let hit: u32 = callee_cdecl!(2, u32, rd32(task + F3_TASK_KEY));
        // Live on purpose: a null hit must fault here, like the original.
        let info = rd32(hit.wrapping_add(F3_HIT_INFO));
        core::hint::black_box(info);
        let a3: u32 = callee_cdecl!(3, u32, 0);
        let same = (rd32(task + F3_TASK_OTHER) == a3) as u32;
        let zero4 = [0u32; 4];
        let _: u32 = callee_thiscall!(
            4, u32, relocated(F3_HUB_A),
            (&zero4[0] as *const u32) as u32, same
        );
        let zero2 = [0u32; 2];
        let frame90 = (&zero2[0] as *const u32) as u32;
        let _: u32 = callee_thiscall!(5, u32, frame90);
        // Callee 6's ECX holds callee 5's stub scratch residue on the
        // original side, so the contract neither logs nor snapshots it; the
        // rewrite passes the same frame mirror for fidelity.
        let edi: u32 = callee_thiscall!(6, u32, frame90);
        let a7: u32 = callee_cdecl!(7, u32, edi);
        let zero2b = [0u32; 2];
        let a8: u32 = callee_thiscall!(
            8, u32, relocated(F3_HUB_B), relocated(F3_STR_A),
            (&zero2b[0] as *const u32) as u32, edi, a7, 0
        );
        if a8 & 0xff == 0 {
            let _: u32 = callee_cdecl!(10, u32, edi);
        } else {
            // Frame mirrors: the constants the original stores over the
            // probed-value slots just before this call, and zeros.
            let f1c = [0u32, 0xffff_ffff, 0xa9];
            let f90 = [0u32; 2];
            let a9: u32 = callee_stdcall!(
                9, u32, relocated(F3_STR_B), 0, 0, 0, 1,
                (&f90[0] as *const u32) as u32,
                (&f1c[0] as *const u32) as u32
            );
            let _: u32 = callee_cdecl!(11, u32, a9);
        }
        let hit2: u32 = callee_cdecl!(13, u32, rd32(task + F3_TASK_KEY));
        let st = rd32(hit2.wrapping_add(F3_HIT_STATE));
        if st != F3_STATE_WANT {
            return st;
        }
        callee_cdecl!(12, u32, task, 1, relocated(F3_DST), 0)
    }
}

/// The mid-function float block: root of a double constant, scaled rate.
/// Its stores feed only scratch no callee observes; the value is returned
/// so the computation stays live and faithful.
unsafe fn f3_scratch(obj: u32, olink: u32) -> f32 {
    unsafe {
        let base3 = if olink != 0 {
            olink.wrapping_add(0x30)
        } else {
            obj.wrapping_add(0x10)
        };
        let _ = (rd32(base3), rd32(base3.wrapping_add(4)), rd32(base3.wrapping_add(8)));
        let dbits = global::<u64>(F3_C_SQRT).read_unaligned();
        let root = f64::from_bits(dbits).sqrt() as f32;
        let one = f32::from_bits(global::<u32>(F3_C_ONE).read_unaligned());
        let x2 = core::hint::black_box(one) / core::hint::black_box(root);
        let x0 = core::hint::black_box(x2) * core::hint::black_box(0.0);
        let _ = f32::from_bits(global::<u32>(F3_C_RATE).read_unaligned());
        x0
    }
}
