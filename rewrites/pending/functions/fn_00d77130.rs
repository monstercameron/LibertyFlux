// original: 0x00d77130 CRenderPhaseWaterSurface::vf7
//! Water-surface phase setup.
//!
//! `this` is the render-phase object. Registers itself in a global slot,
//! folds a virtual query pair into a first handle, refreshes a cached count
//! through a flag-guarded global block, then takes a fast or slow path by
//! the count's sign: both query, apply per-member settings, feed a staged
//! float block, and converge on folding a second handle pair before
//! clearing the global slot. Null handles fault on the vtable load like
//! the original.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const H_Q1: u32 = 1;
const H_OPEN_A: u32 = 2;
const H_QF: u32 = 4;
const H_APPLY4: u32 = 5;
const H_RELEASE: u32 = 6;
const H_NOTIFY: u32 = 7;
const H_QF2: u32 = 8;
const H_FEED: u32 = 9;
const H_QS: u32 = 10;
const H_QS2: u32 = 11;
const H_APP1: u32 = 12;
const H_QS3: u32 = 13;
const H_QL: u32 = 14;
const H_OPEN_B: u32 = 15;
const H_MEASURE: u32 = 16;
const H_FINISH: u32 = 17;

// Fold one virtual query answer pair into the handle's flags. Inlined here
// so this file holds exactly one function.
unsafe fn fold_handle_answer(handle: u32) -> u32 {
    let vtable = *(handle as *const u32);
    let slot = *((vtable + 8) as *const u32);
    let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
    let first = (query(handle) as i32) % 16;
    let shift = (16 - first) % 16;
    let second = query(handle) as i32;
    let mixed = second.wrapping_add(shift) / 16;
    let field = ((handle + 4) as *const u32).read();
    let mask = (((mixed << 14) as u32) ^ field) & 0x01FFC000;
    ((handle + 4) as *mut u32).write(field ^ mask);
    mask
}

// Run the four-arg apply with the guarded member (0x18-query sites: a set
// flag selects the 0x890 member).
unsafe fn apply_guarded(this: u32, handle: u32) -> u32 {
    let flag = *global::<u32>(0x01550DF4);
    let member = if flag != 0 {
        *((this + 0x890) as *const u32)
    } else {
        *((this + 0x8A0) as *const u32)
    };
    callee_thiscall!(H_APPLY4, u32, handle, 0u32, member, 0u32, 0u32)
}

export!(thiscall, rw_00d77130(this: u32) -> u32 {
    unsafe {
        *global::<u32>(0x012FB1B8) = this;
        let found = callee_cdecl!(H_Q1, u32, 0x10u32, 1u32);
        let first = if found != 0 {
            callee_thiscall!(H_OPEN_A, u32, found, 0x10u32)
        } else {
            0
        };
        fold_handle_answer(first);
        if *global::<u8>(0x0103F4D1) == 1 {
            let cached = *global::<u32>(0x0179769C);
            let count = if (cached & 1) == 0 {
                *global::<u32>(0x0179769C) = cached | 1;
                let fresh = callee_cdecl!(H_MEASURE, u32,).wrapping_shl(2);
                *global::<u32>(0x01797698) = fresh;
                fresh
            } else {
                *global::<u32>(0x01797698)
            };
            *((this + 0x940) as *mut u32) = count;
            *global::<u8>(0x0103F4D1) = 0;
            *global::<u32>(0x0103F4CC) = 0xFFFFFFFF;
        }
        if *((this + 0x940) as *const i32) > 0 {
            let qf = callee_cdecl!(H_QF, u32, 0x18u32, 0u32);
            let hf = if qf != 0 { apply_guarded(this, qf) } else { 0 };
            callee_cdecl!(H_RELEASE, u32, hf);
            callee_cdecl!(H_NOTIFY, u32, relocated(0x00AD99C0));
            let qf2 = callee_cdecl!(H_QF2, u32, 0x2cu32, 0u32);
            let hf2 = if qf2 != 0 {
                let mut blk = [
                    0x3F800000u32, 0x3F800000, 0xFF000000, 0, 0, 0, 1,
                ];
                callee_thiscall!(H_FEED, u32, qf2, 0u32, blk.as_mut_ptr() as u32)
            } else {
                0
            };
            callee_cdecl!(H_RELEASE, u32, hf2);
            let left = ((this + 0x940) as *const u32).read();
            ((this + 0x940) as *mut u32).write(left.wrapping_sub(1));
        } else {
            let qs = callee_cdecl!(H_QS, u32, 0x18u32, 0u32);
            let hs = if qs != 0 { apply_guarded(this, qs) } else { 0 };
            callee_cdecl!(H_RELEASE, u32, hs);
            let qs2 = callee_cdecl!(H_QS2, u32, 0x0cu32, 0u32);
            let hs2 = if qs2 != 0 {
                // Note the swapped sense versus the 0x18-query sites: a set
                // flag selects the 0x8a0 member here.
                let flag = *global::<u32>(0x01550DF4);
                let member = if flag != 0 {
                    *((this + 0x8A0) as *const u32)
                } else {
                    *((this + 0x890) as *const u32)
                };
                callee_thiscall!(H_APP1, u32, qs2, member)
            } else {
                0
            };
            callee_cdecl!(H_RELEASE, u32, hs2);
            let qs3 = callee_cdecl!(H_QS3, u32, 0x2cu32, 0u32);
            let hs3 = if qs3 != 0 {
                let mut blk = [
                    0x3F800000u32, 0x3F800000, 0xFF000000, 0, 0, 0, 1,
                ];
                callee_thiscall!(H_FEED, u32, qs3, 0u32, blk.as_mut_ptr() as u32)
            } else {
                0
            };
            callee_cdecl!(H_RELEASE, u32, hs3);
        }
        let found2 = callee_cdecl!(H_QL, u32, 8u32, 0u32);
        let second = if found2 != 0 {
            callee_thiscall!(H_OPEN_B, u32, found2)
        } else {
            0
        };
        fold_handle_answer(second);
        let done = callee_cdecl!(H_FINISH, u32,);
        *global::<u32>(0x012FB1B8) = 0;
        done
    }
});
