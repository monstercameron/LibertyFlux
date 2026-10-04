// original: 0x00d76c10 CRenderPhaseWaterReflection::vf8
//! Water-reflection phase render.
//!
//! `this` is the render-phase object. After gate checks (a global flag, a
//! virtual probe through a global holder with two field tests, and the
//! phase slot), probes a capability, then walks query/handle/release
//! blocks: a float setup with a global argument, two inline stamps, a gated
//! region fetching and offsetting a float plus a toggle, another stamp, a
//! global apply, a const selector with a short stamp, a commit, and a final
//! gated toggle. Returns nothing meaningful (see contract note).

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const G_QA: u32 = 1;
const G_QB: u32 = 2;
const G_QC: u32 = 3;
const G_QD: u32 = 4;
const G_PROBE: u32 = 5;
const G_APPLY6: u32 = 6;
const G_RELEASE: u32 = 7;
const G_FETCHF: u32 = 8;
const G_FEED4: u32 = 9;
const G_TOGGLE: u32 = 10;
const G_APP1: u32 = 11;
const G_SELECT: u32 = 12;
const G_COMMIT: u32 = 13;

// Stamp one queried block (full form with trailer word). All immediates are
// loader-relocated image pointers. Inlined here so this file holds exactly
// one function.
unsafe fn stamp_block(block: u32, trailer_file_va: u32) {
    let field = ((block + 4) as *const u32).read();
    (block as *mut u32).write(relocated(0x00E7E048));
    let counter = global::<u32>(0x010327A0).read();
    let mix = (field ^ counter) & 0x3FFF;
    ((block + 4) as *mut u32).write(field ^ mix);
    global::<u32>(0x010327A0).write(counter.wrapping_add(1));
    (block as *mut u32).write(relocated(0x00E7E080));
    ((block + 8) as *mut u32).write(relocated(trailer_file_va));
}

// The short stamp (header + counter mix, no trailer word).
unsafe fn stamp_block_single(block: u32) {
    let field = ((block + 4) as *const u32).read();
    (block as *mut u32).write(relocated(0x00E7E048));
    let counter = global::<u32>(0x010327A0).read();
    let mix = (field ^ counter) & 0x3FFF;
    ((block + 4) as *mut u32).write(field ^ mix);
    global::<u32>(0x010327A0).write(counter.wrapping_add(1));
    (block as *mut u32).write(relocated(0x00EA76C4));
}

export!(thiscall, rw_00d76c10(this: u32) -> u32 {
    unsafe {
        if *global::<u8>(0x0166DA20) == 1 {
            return 0;
        }
        if *global::<u8>(0x011609F6) != 0 {
            let holder = *global::<u32>(0x0166DA10);
            let vtable = *(holder as *const u32);
            let slot = *((vtable + 0x2C) as *const u32);
            let probe: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let ans = probe(holder);
            if ans != 0
                && *((ans + 0x30) as *const u32) != 0
                && *((ans + 0x38) as *const i32) > 0
            {
                return 0;
            }
        }
        let slot = *((this + 0x938) as *const u32);
        if slot == 0xFFFFFFFF {
            return 0;
        }
        let probe = callee_thiscall!(G_PROBE, u32, this + 0xB0);
        let fbits = if (probe & 0xFF) != 0 {
            0u32
        } else {
            *global::<u32>(0x00FE88E8)
        };
        let q = callee_cdecl!(G_QA, u32, 0x18u32, 0u32);
        let h = if q != 0 {
            callee_thiscall!(G_APPLY6, u32, q, 0u32, 0u32, 1u32, fbits, 1u32,
                *global::<u32>(0x01056B44))
        } else {
            0
        };
        callee_cdecl!(G_RELEASE, u32, h);
        let q2 = callee_cdecl!(G_QC, u32, 0x0cu32, 0u32);
        if q2 != 0 {
            stamp_block(q2, 0x009CB6A0);
        }
        callee_cdecl!(G_RELEASE, u32, if q2 != 0 { q2 } else { 0 });
        let q3 = callee_cdecl!(G_QC, u32, 0x0cu32, 0u32);
        if q3 != 0 {
            stamp_block(q3, 0x00C107A0);
        }
        callee_cdecl!(G_RELEASE, u32, if q3 != 0 { q3 } else { 0 });
        if *global::<u8>(0x01056B48) != 0 {
            let fetched: f32 = callee_cdecl!(G_FETCHF, f32,);
            let diff = *global::<f32>(0x01056B4C) - fetched;
            let q4 = callee_cdecl!(G_QB, u32, 0x20u32, 0u32);
            let h4 = if q4 != 0 {
                let mut blk = [0.0f32, 0.0, 1.0, diff];
                callee_thiscall!(G_FEED4, u32, q4, 0u32, blk.as_mut_ptr() as u32)
            } else {
                0
            };
            callee_cdecl!(G_RELEASE, u32, h4);
            let q5 = callee_cdecl!(G_QB, u32, 0x0cu32, 0u32);
            let h5 = if q5 != 0 {
                callee_thiscall!(G_TOGGLE, u32, q5, 1u32)
            } else {
                0
            };
            callee_cdecl!(G_RELEASE, u32, h5);
        }
        let q6 = callee_cdecl!(G_QC, u32, 0x0cu32, 0u32);
        if q6 != 0 {
            stamp_block(q6, 0x00C106F0);
        }
        callee_cdecl!(G_RELEASE, u32, if q6 != 0 { q6 } else { 0 });
        let q7 = callee_cdecl!(G_QC, u32, 0x0cu32, 0u32);
        let h7 = if q7 != 0 {
            callee_thiscall!(G_APP1, u32, q7, *global::<u32>(0x01797690))
        } else {
            0
        };
        callee_cdecl!(G_RELEASE, u32, h7);
        callee_cdecl!(G_SELECT, u32, slot, 0x90du32, 2u32);
        let q8 = callee_cdecl!(G_QC, u32, 8u32, 0u32);
        if q8 != 0 {
            stamp_block_single(q8);
        }
        callee_cdecl!(G_RELEASE, u32, if q8 != 0 { q8 } else { 0 });
        callee_cdecl!(G_COMMIT, u32, slot);
        if *global::<u8>(0x01056B48) != 0 {
            let q9 = callee_cdecl!(G_QD, u32, 0x0cu32, 0u32);
            let h9 = if q9 != 0 {
                callee_thiscall!(G_TOGGLE, u32, q9, 0u32)
            } else {
                0
            };
            callee_cdecl!(G_RELEASE, u32, h9);
        }
        0
    }
});
