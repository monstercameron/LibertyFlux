// original: 0x009C1940 task_drive_move_blend (proposed)

use lf_checker_rt::{callee_cdecl, callee_thiscall, global};

const OBJ_KIND: u32 = 0x28;
const OBJ_OPTS: u32 = 0x26c;
const OPTS_BIT: u8 = 4;
const OBJ_LINK: u32 = 0xb30;
// ---------------------------------------------------------------------------
// 0x009C1940 task_drive_move_blend (proposed)
// ---------------------------------------------------------------------------

const F4_TASK_OBJ: u32 = 0x10;
const F4_TASK_OTHER: u32 = 0x1c;
const F4_TIMER: u32 = 0x170;
const F4_GUARD: u32 = 0x16c;
const F4_TARGET: u32 = 0x190;
const F4_LINK: u32 = 0x20;
const F4_PARAM: u32 = 0x180;
const F4_VT_SLOT_APPLY: u32 = 8;
const F4_K_BLEND: u32 = 0xfe8a24;
const F4_DFLT_VEC: u32 = 0x1b4b320;
const F4_KIND_MASK: u32 = 0x3c0;
const F4_KIND_WANT: u32 = 0x0c0;


#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}


/// Blended move update of one task record.
///
/// `task` points to the record: current position floats at `+0/+4/+8`, a
/// linked object at `+0x10`, an optional other object at `+0x1c`. The linked
/// object carries a timer at `+0x170`, a guard word at `+0x16c`, a target
/// position at `+0x190/+0x194/+0x198`, a link at `+0x20` and a parameter
/// block at `+0x180`.
///
/// Behaviour: when the timer is strictly above zero and the guard is either
/// null or differs from the other object, blend the current position a
/// constant factor towards the target (`p + (t - p) * k`, in that operand
/// order), run the position through callee 1, then submit the move through
/// callee 2 (seven arguments). When callee 2 reports nonzero, apply through
/// the object's virtual slot `+8`, then always finish through callee 4 with
/// the parameter block. Otherwise (timer spent, or guard equal to the other
/// object) submit a default move built from a global vector through callee 6
/// (same target as callee 2, second call site); when that reports nonzero
/// and the other object passes a kind/flag/link check, compare callee 5's
/// answer against the link, then apply and finish as above unless the
/// answers match.
///
/// Original: 0x009C1940 (cdecl, one stack word). Returns the last callee
/// answer observed (0 when a submit reports zero).
lf_checker_rt::export!(cdecl, rw_009C1940(task: u32) -> u32 {
    unsafe { f4_run(task) }
});

/// Shared body of the function.
unsafe fn f4_run(task: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        // Scratch mirror of the original's frame words the callees observe:
        // one buffer so the snapshot offsets hold by construction: the
        // blended position, zeros where the original leaves them, and the
        // default vector on the fallback path.
        let mut buf = [0u32; 8];
        let mut vec = [0u32; 3];

        let obj = rd32(task + F4_TASK_OBJ);
        let timer = rdf(obj + F4_TIMER);
        // `comiss`/`jbe`: the main path needs strictly greater (a NaN or a
        // signed zero takes the fallback on both sides).
        let main = timer > 0.0
            && ({
                let g = rd32(obj + F4_GUARD);
                g == 0 || g != rd32(task + F4_TASK_OTHER)
            });
        if main {
            let k = f32::from_bits(global::<u32>(F4_K_BLEND).read_unaligned());
            for i in 0..3u32 {
                let c = rdf(task + i * 4);
                let t = rdf(obj + F4_TARGET + i * 4);
                let b = add(c, mul(sub(t, c), k));
                buf[i as usize] = b.to_bits();
            }
            // Callee 1 takes a pointer four words past the position; the
            // contract snapshots the eight words ending at it.
            let anchor = (&mut buf[4] as *mut u32) as u32;
            let _: u32 = callee_thiscall!(1, u32, anchor);
            // Callee 2 takes the position itself and the pad.
            let pp = (&mut buf[0] as *mut u32) as u32;
            let r: u32 = callee_cdecl!(
                2, u32, task, pp, rd32(task + F4_TASK_OTHER), anchor, 0xae, 0xffff_ffff, 4
            );
            if r != 0 {
                let vt: u32 = rd32(obj);
                let apply: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + F4_VT_SLOT_APPLY) as usize);
                let zero3 = [0u32; 3];
                apply(obj, (&zero3[0] as *const u32) as u32, 0, 0);
            }
            return callee_cdecl!(4, u32, task, 1, obj.wrapping_add(F4_PARAM), 0);
        }

        // Fallback: default move from the global vector.
        for i in 0..3u32 {
            vec[i as usize] = global::<u32>(F4_DFLT_VEC + i * 4).read_unaligned();
        }
        let link = rd32(obj + F4_LINK);
        let base = if link != 0 {
            link.wrapping_add(0x30)
        } else {
            obj.wrapping_add(0x10)
        };
        let zero2 = [0u32; 2];
        let r: u32 = callee_cdecl!(
            6, u32, task, base, rd32(task + F4_TASK_OTHER),
            (&zero2[0] as *const u32) as u32, 0xae, 0xffff_ffff, 4
        );
        if r == 0 {
            return 0;
        }
        let other = rd32(task + F4_TASK_OTHER);
        if other != 0
            && rd32(other + OBJ_KIND) & F4_KIND_MASK == F4_KIND_WANT
            && rd8(other + OBJ_OPTS) & OPTS_BIT != 0
            && rd32(other + OBJ_LINK) != 0
        {
            // The original passes the scratch word (zero: callee 6 leaves
            // it alone) and finishes early when the answers match.
            let probe: u32 = callee_cdecl!(5, u32, zero2[0]);
            if probe == rd32(other + OBJ_LINK) {
                return probe;
            }
        }
        let vt: u32 = rd32(obj);
        let apply: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt + F4_VT_SLOT_APPLY) as usize);
        apply(obj, (&vec[0] as *const u32) as u32, 0, 0);
        callee_cdecl!(
            7, u32, task, 1,
            (&vec[0] as *const u32) as u32, 0
        )
    }
}
