// original: 0x00d76950 CRenderPhaseMirrorReflection::vf8
//! Mirror-reflection phase render.
//!
//! `this` is the render-phase object. After three gate checks (two global
//! flags and the phase slot), probes a capability, then walks four regions
//! of query/handle/release blocks: a float setup, a fetched-and-scaled
//! float block plus a toggle, four mixed blocks (two stamp their blocks
//! inline against a global counter), a player-ped virtual call pair, a
//! selector, and a final gated toggle. Null query answers release null and
//! continue; the function returns nothing meaningful (see contract note),
//! so the rewrite returns 0.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const F_QA: u32 = 1;
const F_QB: u32 = 2;
const F_QC: u32 = 3;
const F_QD: u32 = 4;
const F_PROBE: u32 = 5;
const F_APPLY6: u32 = 6;
const F_RELEASE: u32 = 7;
const F_FETCH4: u32 = 8;
const F_FEED4: u32 = 9;
const F_TOGGLE: u32 = 10;
const F_PAIR: u32 = 11;
const F_PLAYER: u32 = 12;
const F_SELECT: u32 = 14;
const F_COMMIT: u32 = 15;

// Stamp one queried block. All three immediates are loader-relocated image
// pointers. Inlined here so this file holds exactly one function.
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

unsafe fn call_vslot30(handle: u32, arg: u32) -> u32 {
    let vtable = *(handle as *const u32);
    let slot = *((vtable + 0x30) as *const u32);
    let method: extern "thiscall" fn(u32, u32) -> u32 =
        core::mem::transmute(slot as usize);
    method(handle, arg)
}

export!(thiscall, rw_00d76950(this: u32) -> u32 {
    unsafe {
        if *global::<u8>(0x0166DA20) == 0 {
            return 0;
        }
        if *global::<u8>(0x0166DA21) != 0 {
            return 0;
        }
        let slot = *((this + 0x938) as *const u32);
        if slot == 0xFFFFFFFF {
            return 0;
        }
        let probe = callee_thiscall!(F_PROBE, u32, this + 0xB0);
        // The staged float doubles as the high bytes of a later call
        // argument (see below); keep its exact bits.
        let f0bits = if (probe & 0xFF) != 0 {
            0u32
        } else {
            *global::<u32>(0x00FE88E8)
        };
        let q = callee_cdecl!(F_QA, u32, 0x18u32, 0u32);
        let h = if q != 0 {
            callee_thiscall!(F_APPLY6, u32, q, 0u32, 0u32, 1u32, f0bits, 1u32, 0u32)
        } else {
            0
        };
        callee_cdecl!(F_RELEASE, u32, h);
        if *global::<u8>(0x01056B50) != 0 {
            let f = callee_cdecl!(F_FETCH4, u32,);
            let v0 = *(f as *const f32);
            let v1 = *((f + 4) as *const f32);
            let v2 = *((f + 8) as *const f32);
            let v3 = *((f + 12) as *const f32) * *global::<f32>(0x00FE8D94);
            let q2 = callee_cdecl!(F_QB, u32, 0x20u32, 0u32);
            let h2 = if q2 != 0 {
                let mut blk = [v0, v1, v2, v3];
                callee_thiscall!(F_FEED4, u32, q2, 0u32, blk.as_mut_ptr() as u32)
            } else {
                0
            };
            callee_cdecl!(F_RELEASE, u32, h2);
            let q3 = callee_cdecl!(F_QB, u32, 0x0cu32, 0u32);
            let h3 = if q3 != 0 {
                callee_thiscall!(F_TOGGLE, u32, q3, 1u32)
            } else {
                0
            };
            callee_cdecl!(F_RELEASE, u32, h3);
        }
        let q4 = callee_cdecl!(F_QC, u32, 0x0cu32, 0u32);
        if q4 != 0 {
            stamp_block(q4, 0x009CB6A0);
        }
        callee_cdecl!(F_RELEASE, u32, if q4 != 0 { q4 } else { 0 });
        let q5 = callee_cdecl!(F_QC, u32, 0x0cu32, 0u32);
        if q5 != 0 {
            stamp_block(q5, 0x00C106F0);
        }
        callee_cdecl!(F_RELEASE, u32, if q5 != 0 { q5 } else { 0 });
        let q6 = callee_cdecl!(F_QC, u32, 0x10u32, 0u32);
        let h6 = if q6 != 0 {
            callee_thiscall!(F_PAIR, u32, q6, 2u32, 0u32)
        } else {
            0
        };
        callee_cdecl!(F_RELEASE, u32, h6);
        let q7 = callee_cdecl!(F_QC, u32, 0x10u32, 0u32);
        let h7 = if q7 != 0 {
            callee_thiscall!(F_PAIR, u32, q7, 8u32, 1u32)
        } else {
            0
        };
        callee_cdecl!(F_RELEASE, u32, h7);
        let mut ped = 0u32;
        let mut flag = 0u32;
        let found = callee_cdecl!(F_PLAYER, u32, 0u32);
        if found != 0 {
            flag = (((found + 0x24) as *const u32).read() >> 5) & 1;
            call_vslot30(found, 1);
            ped = found;
        }
        let sel = if *global::<u8>(0x0166DA22) != 0 || *global::<u8>(0x01797695) != 0 {
            1u32
        } else {
            2u32
        };
        callee_cdecl!(F_SELECT, u32, slot, 0x307u32, sel);
        if ped != 0 {
            // The original pushes a dword it earlier filled with the staged
            // float and has just overwritten byte-wise with the flag bit;
            // rebuild that exact dword from its two deterministic parts.
            call_vslot30(ped, (f0bits & 0xFFFFFF00) | flag);
        }
        callee_cdecl!(F_COMMIT, u32, slot);
        if *global::<u8>(0x01056B50) != 0 {
            let q8 = callee_cdecl!(F_QD, u32, 0x0cu32, 0u32);
            let h8 = if q8 != 0 {
                callee_thiscall!(F_TOGGLE, u32, q8, 0u32)
            } else {
                0
            };
            callee_cdecl!(F_RELEASE, u32, h8);
        }
        0
    }
});
