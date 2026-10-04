// original: 0x005dd1c0 CTaskSimpleAssessInjuredPed::vf17
/// Poll an assess task against a ped: refresh the cached reading on first
/// touch, run the injury scan when armed, clear the ped's stunned flag when
/// the scan observes it. Returns nonzero while waiting, zero when done.
export!(thiscall, rw_005dd1c0(this_ptr: *mut u8, ped: u32) -> u32 {
    unsafe {
        if *((this_ptr as *const u8).add(0x1c) as *const u32) == 0 {
            return 1;
        }
        if *this_ptr.add(0x14) & 1 != 0 {
            let vt = *(ped as *const u32);
            let tgt = *(((vt as *const u8).add(0xfc)) as *const u32);
            let sample: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(tgt as usize);
            let v = sample(ped);
            *((this_ptr as *mut u8).add(0x24) as *mut f32) = v;
            *this_ptr.add(0x14) &= 0xfe;
        }
        if *this_ptr.add(0x14) & 2 != 0 {
            return 1;
        }
        if *((this_ptr as *const u8).add(0x20) as *const u32) == 0 {
            callee_thiscall!(2, u32, this_ptr as u32, ped);
            return 0;
        }
        if *((this_ptr as *const u8).add(0x18) as *const u32) != 0x12a {
            return 0;
        }
        callee_thiscall!(3, u32, ped, relocated(0xF91B64), 0x3E4CCCCD, 0, 0);
        let target = *((this_ptr as *const u8).add(0x1c) as *const u32);
        if target != 0 && *((target as *const u8).add(0x211) as *const u8) != 0 {
            if *this_ptr.add(0x30) == 0 {
                return 0;
            }
            let r = callee_thiscall!(4, u32, (this_ptr as u32).wrapping_add(0x28));
            if r & 0xFF == 0 {
                return 0;
            }
        }
        let crew = *((this_ptr as *const u8).add(0x1c) as *const u32);
        let seen = callee_thiscall!(5, u32, crew);
        if seen & 0xFF != 0 {
            *((crew as *mut u8).add(0x270) as *mut u32) |= 0x2000;
        }
        let st = *((this_ptr as *const u8).add(0x20) as *const u32);
        let flags = *((st as *const u8).add(4) as *const u32);
        if (flags >> 5) & 1 != 0 {
            *((st as *mut u8).add(4) as *mut u32) = flags & 0xFFFFFFDF;
        }
        0
    }
});
