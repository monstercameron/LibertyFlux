// original: 0x009e1360 audio_source_configure
/// Reconfigure an audio source object (thiscall).
///
/// Notifies the source's owner through its function table when the source
/// is active, drops any previous live registration, clears the source's id
/// pair, publishes the source position plus `edi` and the low byte of `argb`
/// to shared slots, and runs a scripted configure step whose two out-words
/// either install a fresh id pair (storing the second word and flagging the
/// source live) or, when the step reports nothing new and `edi` is null,
/// park the source. A source left with a live id pair is registered through
/// two scripted calls, and the child object records the outcome. Returns the
/// last value produced along the executed path.
export!(thiscall, rw_9e1360(obj: *mut u8, edi: u32, argb: u32) -> u32 {
    unsafe {
        let mut eax: u32 = 0;
        let al = (argb & 0xFF) as u8;
        if *(obj.add(0x3C) as *const u8) == 0 {
            let child = *(obj.add(0x34) as *const u32);
            if child != 0 {
                let w20 = *((child as usize as *const u32).add(8));
                let picked = if w20 != 0 {
                    w20.wrapping_add(0x30)
                } else {
                    (child as usize).wrapping_add(0x10) as u32
                };
                let vtable = *(obj as *const u32);
                let target = *((vtable as usize as *const u8).add(4) as *const u32);
                let notify: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                eax = notify(obj as u32, picked, 0);
            }
        }
        if *(obj.add(0x30) as *const u32) != 0
            && (*(obj.add(0x38) as *const u32) as i32) > 0
        {
            eax = callee_stdcall!(1, u32, obj as u32);
        }
        *(obj.add(0x38) as *mut u32) = 0;
        *(obj.add(0x30) as *mut u32) = 0;
        let f0 = *(obj.add(0x20) as *const u32);
        let f1 = *(obj.add(0x24) as *const u32);
        let f2 = *(obj.add(0x28) as *const u32);
        *(global::<u8>(0x103B10C)) = al;
        *(global::<u32>(0x103B104)) = 0x49742400;
        *(global::<u32>(0x12B4194)) = 0;
        *(global::<u32>(0x103B108)) = 0xFFFFFFFF;
        *(global::<u32>(0x12B41A0)) = edi;
        let frame = [f0, f1, f2, 0u32, 0x3DCCCCCDu32];
        eax = callee_cdecl!(
            2,
            u32,
            frame.as_ptr() as u32,
            relocated(0x9DFC80),
            obj as u32,
            0x180,
            4
        );
        if *global::<u32>(0x103B108) == 0xFFFFFFFF {
            *(global::<u8>(0x103B10C)) = 1;
        } else {
            // Dead while the configure callee is stubbed (it cannot change
            // the published slot); kept to mirror the original.
            *(obj.add(0x30) as *mut u32) = *global::<u32>(0x12B4194);
            *(obj.add(0x38) as *mut u32) = *global::<u32>(0x103B108);
        }
        let mut out0: u32 = 0;
        let mut out1: u32 = 0;
        let mut slot: u32 = 0;
        let answered =
            callee_thiscall!(3, u32, obj as u32, &mut out0 as *mut u32 as u32, &mut out1 as *mut u32 as u32, &mut slot as *mut u32 as u32);
        let cl = (answered & 0xFF) as u8;
        *(obj.add(0x46) as *mut u8) = 0;
        if cl != 0 {
            *(obj.add(0x3E) as *mut u16) = 0;
        }
        eax = out0;
        if (out0 as i32) > 0 {
            *(obj.add(0x38) as *mut u32) = out0;
            eax = out1;
            *(obj.add(0x30) as *mut u32) = out1;
            *(obj.add(0x46) as *mut u8) = 1;
        } else if cl != 0 && edi == 0 {
            *(obj.add(0x38) as *mut u32) = 0xFFFFFFFF;
            *(obj.add(0x30) as *mut u32) = 0;
        }
        if *(obj.add(0x30) as *const u32) != 0
            && (*(obj.add(0x38) as *const u32) as i32) > 0
        {
            eax = callee_stdcall!(4, u32, obj as u32);
            let child2 = *(obj.add(0x34) as *const u32);
            if child2 != 0 {
                *((child2 as usize as *mut u8).add(0x40)) =
                    (*(obj.add(0x38) as *const u32) & 0xFF) as u8;
                let id = *(obj.add(0x30) as *const u32);
                let reg = *global::<u32>(0x12FB214);
                eax = callee_thiscall!(5, u32, reg, id);
                *((child2 as usize as *mut u32).add(0x12)) = eax;
            }
        }
        eax
    }
});
