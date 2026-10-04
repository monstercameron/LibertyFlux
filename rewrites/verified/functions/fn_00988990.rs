// original: 0x00988990 audCutsceneAudioEntity slot refresh (proposed)
//! Cutscene audio entity slot refresh: copies the global slot name into a
//! local buffer, and when it differs from the stored name for the current
//! index it rebuilds that slot through the audio helpers and stores the new
//! name. Then ensures the shared slot object exists and asks it for the
//! current slot handle.
//!
//! Returns 1 when the current slot is empty, 0 when no handle is available,
//! else the final slot query's answer.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// original: 0x00988990 audCutsceneAudioEntity slot refresh (proposed)
export!(thiscall, rw_00988990(this: u32, arg0: u32, arg1: u32) -> u32 {
    let mut buf = [0u8; 68];
    let b = buf.as_mut_ptr().wrapping_add(4);
    let src = unsafe { *global::<u32>(0x1038b0c) };
    let mut i = 0usize;
    loop {
        let ch = unsafe { *((src.wrapping_add(i as u32)) as *const u8) };
        unsafe { *b.add(i) = ch };
        i += 1;
        if ch == 0 {
            break;
        }
    }
    let mut len = 0u32;
    while unsafe { *b.add(len as usize) } != 0 {
        len += 1;
    }
    callee_cdecl!(1, u32, b as u32, arg0, 0x3fu32.wrapping_sub(len));
    callee_cdecl!(2, u32, b as u32);
    let idx = unsafe { *(this.wrapping_add(0xa0) as *const u8) } as u32;
    let base = this
        .wrapping_add(idx.wrapping_mul(64))
        .wrapping_add(0x14);
    let mut o = 0u32;
    let ord: i32 = loop {
        let a0 = unsafe { *((base.wrapping_add(o)) as *const u8) };
        let c0 = unsafe { *b.add(o as usize) };
        if a0 != c0 {
            break if (a0 as u32) < (c0 as u32) { -1 } else { 1 };
        }
        if a0 == 0 {
            break 0;
        }
        let a1 = unsafe { *((base.wrapping_add(o).wrapping_add(1)) as *const u8) };
        let c1 = unsafe { *b.add(o as usize + 1) };
        if a1 != c1 {
            break if (a1 as u32) < (c1 as u32) { -1 } else { 1 };
        }
        o += 2;
        if a1 == 0 {
            break 0;
        }
    };
    if ord != 0 {
        let slot = unsafe { *(this.wrapping_add(idx.wrapping_mul(4)).wrapping_add(8) as *const u32) };
        if slot != 0 {
            callee_thiscall!(3, u32, slot, 0u32);
        }
        let mut obj = [0u32; 10];
        callee_thiscall!(4, u32, obj.as_mut_ptr() as u32);
        obj[9] = arg1;
        let g = unsafe { *global::<u32>(0x1030c0c) };
        callee_cdecl!(5, u32, b.wrapping_sub(4) as u32, g);
        let dest = this.wrapping_add(idx.wrapping_add(2).wrapping_mul(4));
        callee_thiscall!(
            6, u32, this, b as u32, dest, obj.as_mut_ptr() as u32, 0xffffffffu32, 0u32, 0u32
        );
        let mut k = 0u32;
        loop {
            let ch = unsafe { *b.add(k as usize) };
            unsafe { *((base.wrapping_add(k)) as *mut u8) = ch };
            k += 1;
            if ch == 0 {
                break;
            }
        }
    }
    let idx2 = unsafe { *(this.wrapping_add(0xa0) as *const u8) } as u32;
    let slot2 = unsafe { *(this.wrapping_add(idx2.wrapping_mul(4)).wrapping_add(8) as *const u32) };
    if slot2 == 0 {
        return 1;
    }
    let mut shared = unsafe { *(this.wrapping_add(0x10) as *const u32) };
    if shared == 0 {
        let desc = [relocated(0x988670), relocated(0x9885c0), 0u32, 8u32];
        shared = callee_cdecl!(7, u32, desc.as_ptr() as u32);
        unsafe { *(this.wrapping_add(0x10) as *mut u32) = shared };
    }
    if shared == 0 {
        return 0;
    }
    if unsafe { *(shared.wrapping_add(0x28) as *const u8) } != 1 {
        return 0;
    }
    let idx3 = unsafe { *(this.wrapping_add(0xa0) as *const u8) } as u32;
    let word = unsafe {
        *(shared.wrapping_add(idx3.wrapping_mul(4)).wrapping_add(0x20) as *const u32)
    };
    let handle = unsafe {
        *(this.wrapping_add(idx3.wrapping_mul(4)).wrapping_add(8) as *const u32)
    };
    callee_thiscall!(8, u32, handle, word, 1u32)
});
