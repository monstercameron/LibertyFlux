// original: 0x00d76ef0 CRenderPhaseCloudGeneration::vf7
//! Cloud-generation phase setup.
//!
//! Opens two resource handles through queries, folds a virtual query answer
//! pair into each handle's flag word (the `% 16` dance shared below),
//! derives a tuning float from a six-word global parameter block, submits
//! the block, and returns the second handle's mask. Null query answers make
//! the original fault on the vtable load; the rewrite faults identically.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const E_QUERY: u32 = 1;
const E_OPEN_A: u32 = 2;
const E_OPEN_B: u32 = 5;
const E_TUNE: u32 = 4;

// Fold one virtual query answer pair into the handle's flags. Inlined here
// so this file holds exactly one function.
unsafe fn fold_handle_answer(handle: u32) -> u32 {
    // A null handle faults reading its vtable pointer, exactly like the
    // original's unconditional load (fault parity, not a branch).
    let vtable = *(handle as *const u32);
    let slot = *((vtable + 8) as *const u32);
    let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
    // Both `% 16` remainders use the truncating (C-like) semantics the
    // original's and/jns sequences implement; the divisor is a const 16.
    let first = (query(handle) as i32) % 16;
    let shift = (16 - first) % 16;
    let second = query(handle) as i32;
    let mixed = second.wrapping_add(shift) / 16;
    let field = ((handle + 4) as *const u32).read();
    let mask = (((mixed << 14) as u32) ^ field) & 0x01FFC000;
    ((handle + 4) as *mut u32).write(field ^ mask);
    mask
}

export!(thiscall, rw_00d76ef0(_this: u32) -> u32 {
    unsafe {
        let found = callee_cdecl!(E_QUERY, u32, 0x10u32, 1u32);
        let first = if found != 0 {
            callee_thiscall!(E_OPEN_A, u32, found, 0x1eu32)
        } else {
            0
        };
        fold_handle_answer(first);
        let g40 = *global::<u32>(0x01295840);
        let g44 = *global::<u32>(0x01295844);
        let g48 = *global::<u32>(0x01295848);
        let g4c = *global::<u32>(0x0129584C);
        let g50 = *global::<u32>(0x01295850);
        let g54 = *global::<u32>(0x01295854);
        // ecx = low16 * 15, then scaled by 4 into the index (wrapping).
        let index = g4c.wrapping_add((g48 & 0xFFFF).wrapping_mul(15).wrapping_mul(4));
        let mut selector = g48;
        if g54 != 0xFFFFFFFF {
            selector = g54;
        }
        let gain = *global::<f32>(0x00FE8724);
        let base = (g50 as i32 as f32) * gain;
        let tuned = ((index as i32 as f32) + base) * gain;
        let mut w40 = g40;
        let mut w44 = g44;
        let mut w_sel = selector;
        let mut w_tuned = tuned;
        callee_cdecl!(
            E_TUNE, u32,
            // A relocated image pointer (the loader rebases the immediate).
            relocated(0x00DBBDF0),
            &mut w_tuned as *mut f32 as u32,
            &mut w_sel as *mut u32 as u32,
            &mut w44 as *mut u32 as u32,
            &mut w40 as *mut u32 as u32
        );
        let found2 = callee_cdecl!(E_QUERY, u32, 8u32, 0u32);
        let second = if found2 != 0 {
            callee_thiscall!(E_OPEN_B, u32, found2)
        } else {
            0
        };
        fold_handle_answer(second)
    }
});
