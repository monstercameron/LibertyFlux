// original: 0x0099f340 audio_config_init
/// Initialize the audio configuration tables and the global mix presets.
///
/// Behavior: publish six name pointers to the config table; resolve seven
/// entries by calling the table lookup per name and registering each
/// answer, storing every registration answer to its config slot; register
/// four group names and keep the last answer's low byte in a flag; copy two
/// runtime pointers into their slots and zero two counters; then twelve
/// times format an indexed name, look it up and register it into the
/// indexed table; register one final name into slot zero; finally read the
/// quality level and store the matching float preset block (level 1, level
/// 2, or the default), returning the level read.
export!(cdecl, rw_0099f340() -> u32 {
    unsafe {
        core::ptr::write_unaligned(global::<u32>(0x012843E8) as u32 as *mut u32, relocated(0x00E90966));
        core::ptr::write_unaligned(global::<u32>(0x012843EC) as u32 as *mut u32, relocated(0x00E90F94));
        core::ptr::write_unaligned(global::<u32>(0x012843F0) as u32 as *mut u32, relocated(0x00E90FA0));
        core::ptr::write_unaligned(global::<u32>(0x012843F4) as u32 as *mut u32, relocated(0x00E90FAC));
        core::ptr::write_unaligned(global::<u32>(0x012843F8) as u32 as *mut u32, relocated(0x00E90FB4));
        core::ptr::write_unaligned(global::<u32>(0x012843FC) as u32 as *mut u32, relocated(0x00E90FC0));
        let table = relocated(0x0115D9A0);
        let a: u32 = callee_cdecl!(1, u32, relocated(0x00E90FCC), 0);
        let mut prev: u32 = callee_thiscall!(2, u32, table, a);
        let names_slots: [(u32, u32); 6] = [
            (0x00E90FDC, 0x012843B0),
            (0x00E90FE4, 0x01284398),
            (0x00E90FF0, 0x0128439C),
            (0x00E91004, 0x012843A0),
            (0x00E91014, 0x012843AC),
            (0x00E91024, 0x012843A8),
        ];
        for (nm, slot) in names_slots {
            let b: u32 = callee_cdecl!(1, u32, relocated(nm), 0);
            core::ptr::write_unaligned(global::<u32>(slot) as u32 as *mut u32, prev);
            prev = callee_thiscall!(2, u32, table, b);
        }
        core::ptr::write_unaligned(global::<u32>(0x012843A4) as u32 as *mut u32, prev);
        let g1: u32 = callee_thiscall!(3, u32, relocated(0x01284468), relocated(0x00E9103C));
        let g2: u32 = callee_thiscall!(3, u32, relocated(0x0128453C), relocated(0x00E9105C));
        let g3: u32 = callee_thiscall!(3, u32, relocated(0x012844F0), relocated(0x00E91078));
        let g4: u32 = callee_thiscall!(3, u32, relocated(0x012844BC), relocated(0x00E9109C));
        let _ = (g1, g2, g3);
        core::ptr::write_unaligned(global::<u8>(0x01284382) as u32 as *mut u8, g4 as u8);
        core::ptr::write_unaligned(global::<u32>(0x01284390) as u32 as *mut u32, core::ptr::read_unaligned(global::<u32>(0x01284438) as u32 as *const u32));
        core::ptr::write_unaligned(global::<u32>(0x01284388) as u32 as *mut u32, 0);
        core::ptr::write_unaligned(global::<u32>(0x0128438C) as u32 as *mut u32, 0);
        core::ptr::write_unaligned(global::<u32>(0x01284394) as u32 as *mut u32, core::ptr::read_unaligned(global::<u32>(0x0128458C) as u32 as *const u32));
        for n in 1..13u32 {
            let mut local = 0u32;
            let _: u32 = callee_cdecl!(4, u32, &mut local as *mut u32 as u32,
                relocated(0x00E910C4), n);
            let mut dummy = 0u32;
            let b: u32 = callee_cdecl!(5, u32, &mut dummy as *mut u32 as u32, 0);
            let h: u32 = callee_thiscall!(2, u32, table, b);
            core::ptr::write_unaligned((global::<u32>(0x012843B4) as u32 + n.wrapping_mul(4)) as *mut u32, h);
        }
        let z: u32 = callee_cdecl!(1, u32, relocated(0x00E910E0), 0);
        let z2: u32 = callee_thiscall!(2, u32, table, z);
        core::ptr::write_unaligned(global::<u32>(0x012843B4) as u32 as *mut u32, z2);
        let sw = core::ptr::read_unaligned(global::<u32>(0x011D6FD0) as u32 as *const u32);
        if sw == 1 {
            core::ptr::write_unaligned(global::<u32>(0x01038D4C) as u32 as *mut u32, 0x40400000);
            core::ptr::write_unaligned(global::<u32>(0x01038D6C) as u32 as *mut u32, 0xC0800000);
            core::ptr::write_unaligned(global::<u32>(0x01038D74) as u32 as *mut u32, 0xC0800000);
            core::ptr::write_unaligned(global::<u32>(0x01038D7C) as u32 as *mut u32, 0xC0000000);
            core::ptr::write_unaligned(global::<u32>(0x01038D84) as u32 as *mut u32, 0xC0000000);
            core::ptr::write_unaligned(global::<u32>(0x01038D90) as u32 as *mut u32, 0x40900000);
            core::ptr::write_unaligned(global::<u32>(0x01038D58) as u32 as *mut u32, 0xC0800000);
        } else if sw == 2 {
            core::ptr::write_unaligned(global::<u32>(0x01038D4C) as u32 as *mut u32, 0x3F800000);
            core::ptr::write_unaligned(global::<u32>(0x01038D6C) as u32 as *mut u32, 0xC0C00000);
            core::ptr::write_unaligned(global::<u32>(0x01038D74) as u32 as *mut u32, 0xC0B00000);
            core::ptr::write_unaligned(global::<u32>(0x01038D7C) as u32 as *mut u32, 0xC0400000);
            core::ptr::write_unaligned(global::<u32>(0x01038D84) as u32 as *mut u32, 0xC0000000);
            core::ptr::write_unaligned(global::<u32>(0x01038D90) as u32 as *mut u32, 0x40400000);
            core::ptr::write_unaligned(global::<u32>(0x01038D58) as u32 as *mut u32, 0xC0000000);
        } else {
            core::ptr::write_unaligned(global::<u32>(0x01038D4C) as u32 as *mut u32, 0);
            core::ptr::write_unaligned(global::<u32>(0x01038D6C) as u32 as *mut u32, 0xC0E00000);
            core::ptr::write_unaligned(global::<u32>(0x01038D74) as u32 as *mut u32, 0xC0E00000);
            core::ptr::write_unaligned(global::<u32>(0x01038D7C) as u32 as *mut u32, 0xC0800000);
            core::ptr::write_unaligned(global::<u32>(0x01038D84) as u32 as *mut u32, 0xC0400000);
            core::ptr::write_unaligned(global::<u32>(0x01038D90) as u32 as *mut u32, 0x3FC00000);
            core::ptr::write_unaligned(global::<u32>(0x01038D58) as u32 as *mut u32, 0xC0800000);
        }
        sw
    }
});
