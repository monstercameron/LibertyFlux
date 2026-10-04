// original: 0x00988cf0 MOLOTOV_FUSE_FIRE_LOOP_IN_AIR
//! Weapon-loop sound starter: unless the audio system is unavailable, picks
//! a loop descriptor by the requested code and flag, attaches it to the
//! given entity slot and records the resulting voice handles on it.
//!
//! Returns nothing.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// original: 0x00988cf0 MOLOTOV_FUSE_FIRE_LOOP_IN_AIR
export!(cdecl, rw_00988cf0(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    if unsafe { *global::<u32>(0x11f7060) } == 1 {
        return 0;
    }
    if unsafe { *global::<u32>(0x12088b4) } != unsafe { *global::<u32>(0xf1c040) } {
        return 0;
    }
    if unsafe { *global::<u32>(0x1037720) } == 0x12 {
        return 0;
    }
    let flag = (arg3 & 0xff) as u8;
    let code: u32 = match arg0 {
        5 => relocated(0xe8e4d0),
        6 => {
            if flag != 0 {
                relocated(0xe8e4f0)
            } else {
                relocated(0xe8e504)
            }
        }
        0x26 => {
            if flag != 0 {
                relocated(0xe8e520)
            } else {
                relocated(0xe8e53c)
            }
        }
        _ => return 0,
    };
    let mut obj = [0u32; 4];
    callee_thiscall!(1, u32, obj.as_mut_ptr() as u32);
    obj[3] = arg1.wrapping_add(0x264);
    callee_thiscall!(
        2, u32, relocated(0x12831e4), code, arg2, obj.as_mut_ptr() as u32, 0xffffffffu32, 0u32,
        0u32
    );
    let inner = unsafe { *(arg2 as *const u32) };
    if inner == 0 {
        return 0;
    }
    callee_thiscall!(3, u32, inner, u32::from(flag != 0));
    let mut tmp = [0u32; 4];
    tmp[1] = 0xffffffff;
    tmp[2] = 0xaa;
    let h = callee_cdecl!(
        4, u32, code, 0u32, 0u32, 1u32, 1u32, obj.as_mut_ptr() as u32,
        tmp.as_mut_ptr() as u32, arg1, 0xffffffffu32
    );
    let s = callee_cdecl!(5, u32, h);
    let t = callee_cdecl!(6, u32, s);
    let edx = unsafe { *(arg2 as *const u32) };
    unsafe { *(edx.wrapping_add(0xa4) as *mut u32) = s };
    unsafe { *(edx.wrapping_add(0xa8) as *mut u32) = t };
    unsafe { *(edx.wrapping_add(0xac) as *mut u32) = 0 };
    let ec = unsafe { *(arg2 as *const u32) };
    callee_thiscall!(7, u32, ec, 0u32, 0u32, 0u32)
});
