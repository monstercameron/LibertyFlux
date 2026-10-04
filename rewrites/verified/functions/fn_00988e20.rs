// original: 0x00988e20 E2_APC
//! Resolves one audio entity reference: stores the requested id, looks up
//! its voice handle (hashing the well-known id directly, asking the weapon
//! table otherwise) and records the handle, then refreshes the entity if it
//! is tagged and clears the shared voice table.
//!
//! Returns 0.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[allow(non_snake_case)]
fn attach_voice(this: u32, h: u32) {
    let r = callee_thiscall!(3, u32, relocated(0x115d9a0), h);
    unsafe { *(this.wrapping_add(0x10) as *mut u32) = r };
    unsafe { *(this.wrapping_add(0x14) as *mut u32) = h };
}

// original: 0x00988e20 E2_APC
export!(thiscall, rw_00988e20(this: u32, arg: u32) -> u32 {
    unsafe { *(this.wrapping_add(8) as *mut u32) = arg };
    unsafe { *(this.wrapping_add(0xc) as *mut u32) = 0 };
    let ready = (unsafe { *global::<u32>(0x11d6fd4) } as i32) >= 2;
    if ready && arg == 0x28 {
        let h = callee_cdecl!(1, u32, relocated(0xe8e720), 0u32);
        attach_voice(this, h);
    } else if arg == 0 {
        unsafe { *(this.wrapping_add(0x10) as *mut u32) = 0 };
    } else {
        let info = callee_cdecl!(2, u32, arg);
        if info != 0 {
            let inner = unsafe { *(info.wrapping_add(0x24) as *const u32) };
            if inner != 0 {
                attach_voice(this, inner);
            }
        }
    }
    if unsafe { *(this.wrapping_add(4) as *const u16) } == 0xffff {
        callee_thiscall!(4, u32, this);
    }
    let table = relocated(0x1283148);
    let mut i = 0u32;
    while i < 34 {
        unsafe { *((table.wrapping_add(i.wrapping_mul(4))) as *mut u32) = 0 };
        i += 1;
    }
    0
});
