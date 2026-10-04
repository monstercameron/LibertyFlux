// original: 0x00E3E7F0 refresh_ui_state_slot
//! Drive one UI state-slot refresh cycle.
//!
//! Straight-line driver over the intercepted state-slot calls: seeds the slot,
//! picks selector 7 or 2 from two global bytes, fetches two words through the
//! lookup call, republishes them twice, publishes the fixed -5/+5 range, takes
//! the notify call's answer (or 0xFF) as the intensity byte, resolves the slot
//! color word through the table call, publishes the intensity and a zero, and
//! finishes by publishing 1.0 or 0.0 per the gate call's answer. Returns the
//! last call's answer.
//!
//! Verified by checker v3: 1000/1000 trials, 2 branch shapes, honesty mutant
//! (high selector off by one) fails on the call log. Lane r-b148.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

export!(cdecl, rw_00e3e7f0() -> u32 {
    unsafe { refresh_state_slot(0x41) }
});

/// Shared body; `hi_sel` is the table selector used when the mode check passes.
unsafe fn refresh_state_slot(hi_sel: u32) -> u32 {
    const SEL_WORD: u32 = 0x0116C250;
    const NOTIFY_FLAG: u32 = 0x01161548;
    const MODE: u32 = 0x011D6FD4;
    unsafe {
        let mut dummy = 0u32;
        let mut vw = [0u32, 0u32];
        callee_cdecl!(1, u32, &mut dummy as *mut u32 as u32);
        callee_cdecl!(2, u32, 1);
        let selw = global::<u32>(SEL_WORD).read();
        let sel = if (selw as u8) != 0x6A && (selw >> 24) as u8 == 0 {
            7u32
        } else {
            2u32
        };
        callee_cdecl!(3, u32, sel);
        let vp = vw.as_mut_ptr() as u32;
        callee_cdecl!(4, u32, vp, 0x2C);
        callee_cdecl!(5, u32, 7, 0, vp, 0);
        let (v, w) = (vw[0], vw[1]);
        callee_cdecl!(6, u32, v, w);
        callee_cdecl!(2, u32, 1);
        callee_cdecl!(6, u32, v, w);
        callee_cdecl!(7, u32, (-5.0f32).to_bits(), 5.0f32.to_bits());
        let mut bl: u8 = 0xFF;
        if global::<u8>(NOTIFY_FLAG).read() != 0 {
            bl = callee_thiscall!(8, u32, relocated(NOTIFY_FLAG)) as u8;
        }
        // Zero-extended: the original pushes the byte with scratch-high bytes
        // and the contract defines the fill as 0 on both sides.
        let blw: u32 = bl as u32;
        let ok = callee_cdecl!(9, u32, 0);
        let tsel = if (ok as u8) != 0 && global::<u32>(MODE).read() != 2 {
            hi_sel
        } else {
            0x3B
        };
        let p10 = callee_cdecl!(10, u32, &blw as *const u32 as u32, tsel, blw);
        callee_cdecl!(11, u32, (p10 as *const u32).read());
        callee_cdecl!(12, u32, (bl as u32) << 24);
        callee_cdecl!(13, u32, 0);
        let gate = callee_cdecl!(14, u32,);
        if (gate as u8) != 0 {
            callee_cdecl!(15, u32, 1.0f32.to_bits())
        } else {
            callee_cdecl!(15, u32, 0)
        }
    }
}
