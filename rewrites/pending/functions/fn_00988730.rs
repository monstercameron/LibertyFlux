// original: 0x00988730 audCutsceneAudioEntity::vf3
//! Cutscene audio entity slot-3 update: decides whether the entity's sound
//! set is active and drives each entry's voice through the audio manager.
//!
//! `this` points at the entity; the trailing stack argument is ignored.
//! Returns 0 when there is no sound table, the table pointer when the table
//! is empty, else the entry count.

use lf_k2_rt::{callee_thiscall, export, global, relocated};

#[allow(non_snake_case)]
fn active_flag(this: u32) -> u8 {
    let sticky = unsafe { *(this.wrapping_add(0xa2) as *const u8) };
    let gate = unsafe { *global::<u8>(0x129576e) };
    if gate == 0 {
        let first = unsafe { *(this.wrapping_add(8) as *const u32) };
        if first != 0 {
            let kind = unsafe { *(first.wrapping_add(6) as *const u16) };
            if kind == 2 || sticky != 0 {
                return 1;
            }
        }
    }
    let second = unsafe { *(this.wrapping_add(0xc) as *const u32) };
    if second != 0 {
        let kind = unsafe { *(second.wrapping_add(6) as *const u16) };
        if kind == 2 || sticky != 0 {
            return 1;
        }
    }
    0
}

// original: 0x00988730 audCutsceneAudioEntity::vf3
export!(thiscall, rw_00988730(this: u32, _arg: u32) -> u32 {
    let cl = active_flag(this);
    let base = unsafe { *(this.wrapping_add(0x98) as *const u32) }.wrapping_add(0x1f4);
    let below = unsafe { *global::<u32>(0x11735b4) } < base;
    let quick_fail = unsafe { *global::<u32>(0x11f7060) } == 1
        || unsafe { *global::<u32>(0x12088b4) } != unsafe { *global::<u32>(0xf1c040) }
        || unsafe { *global::<u32>(0x1037720) } == 0x12;
    let mode = unsafe { *global::<u8>(0x11d7629) };
    let mgr = relocated(0x115d9a0);
    if !quick_fail && (cl != 0 || below || mode != 0) {
        unsafe { *global::<u8>(0x128465d) = 1 };
        if mode != 0 {
            let tbl = unsafe { *global::<u32>(0x1282fa4) };
            unsafe { *(this.wrapping_add(0x94) as *mut u32) = tbl };
        }
        let tbl = unsafe { *(this.wrapping_add(0x94) as *const u32) };
        let out: u32;
        if tbl == 0 {
            out = 0;
        } else if unsafe { *(tbl.wrapping_add(0xa) as *const u8) } == 0 {
            out = tbl;
        } else {
            let mut idx: u32 = 0;
            let mut off: u32 = 0;
            let mut n: u8;
            loop {
                let word = unsafe { *(tbl.wrapping_add(off).wrapping_add(0xb) as *const u32) };
                let h = callee_thiscall!(1, u32, mgr, word);
                if h != 0 {
                    let level = unsafe {
                        *(tbl.wrapping_add(off).wrapping_add(0xf) as *const i8)
                    } as f32;
                    callee_thiscall!(2, u32, h, level.to_bits());
                }
                idx += 1;
                off += 5;
                n = unsafe { *(tbl.wrapping_add(0xa) as *const u8) };
                if idx >= n as u32 {
                    break;
                }
            }
            out = n as u32;
        }
        if cl != 0 {
            unsafe { *(this.wrapping_add(0xa1) as *mut u16) = 1 };
        } else {
            unsafe { *(this.wrapping_add(0xa2) as *mut u8) = 0 };
        }
        out
    } else {
        let tbl = unsafe { *(this.wrapping_add(0x94) as *const u32) };
        let out: u32;
        if tbl == 0 {
            out = 0;
        } else if unsafe { *(tbl.wrapping_add(0xa) as *const u8) } == 0 {
            out = tbl;
        } else {
            let mut idx: u32 = 0;
            let mut off: u32 = 0;
            let mut n: u8;
            loop {
                let word = unsafe { *(tbl.wrapping_add(off).wrapping_add(0xb) as *const u32) };
                let h = callee_thiscall!(1, u32, mgr, word);
                if h != 0 {
                    callee_thiscall!(2, u32, h, 0u32);
                }
                idx += 1;
                off += 5;
                n = unsafe { *(tbl.wrapping_add(0xa) as *const u8) };
                if idx >= n as u32 {
                    break;
                }
            }
            out = n as u32;
        }
        unsafe { *(this.wrapping_add(0xa1) as *mut u8) = 0 };
        unsafe { *global::<u8>(0x128465d) = 0 };
        unsafe { *(this.wrapping_add(0xa2) as *mut u8) = 0 };
        out
    }
});
