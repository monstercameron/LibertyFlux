// original: 0x0097C340 audio_event_emit_full
/// Emit a fully-built event packet through the audio core when live.
///
/// Gathers the interface, builds the packet the callee sequence expects, runs
/// the lookup/start calls and finishes through the 8-argument submit; the
/// low byte of the start answer selects submit versus release. Exit eax is
/// entry garbage on the first gate path, so the return channel is unchecked.
/// thiscall(obj, arg0).
export!(thiscall, rw_s103_97c340(obj: *mut u8, arg0: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x11F7060) == 1 {
            return 0;
        }
        if *global::<u32>(0x12088B4) != *global::<u32>(0xF1C040) {
            return 0;
        }
        if *global::<u32>(0x1037720) == 0x12 {
            return 0;
        }
        let iface = *(((obj as usize) + 0x120) as *const u32);
        if (*(((iface as usize) + 0x28) as *const u32) & 0x800000) != 0 {
            return 0;
        }
        // Packet replicating the original's stack frame: 12 zero bytes (the
        // checker's defined fill) then the fields the callees observe.
        let mut pkt = [0u32; 24];
        let base = (pkt.as_mut_ptr().add(3) as usize) as u32;
        callee_thiscall!(1, u32, base);
        *(((base as usize) + 0x46) as *mut u8) &= 0xEF;
        *(((base as usize) + 0x18) as *mut u32) = *global::<u32>(0x1231314);
        *(((base as usize) + 0x20) as *mut u32) = *(((obj as usize) + 8) as *const u32);
        *(((base as usize) + 0x0C) as *mut u32) = iface.wrapping_add(0x780);
        let h = callee_thiscall!(2, u32, base);
        let code = callee_cdecl!(3, u32, h);
        let ok = callee_thiscall!(4, u32, obj as usize as u32, arg0, base, h, code, 0);
        if (ok as u8) != 0 {
            *(((base as usize) - 12) as *mut u32) = 0;
            *(((base as usize) - 8) as *mut u32) = 0xFFFFFFFF;
            *(((base as usize) - 4) as *mut u32) = 0x34;
            callee_cdecl!(5, u32, arg0, 0, 0, 1, base, base.wrapping_sub(12), iface, h);
        } else {
            callee_cdecl!(6, u32, h);
        }
        0
    }
});
