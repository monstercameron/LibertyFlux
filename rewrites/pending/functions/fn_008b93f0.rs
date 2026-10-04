// original: 0x008B93F0 fe_input_state_init

/// Initialize the frontend input state.
///
/// Raises the frontend-active flags, probes the input devices (skipping the
/// keyboard/mouse setup when no keyboard answers), registers the cursor and
/// map devices with the frontend group when the frontend context exists,
/// then writes the default dead-zone, repeat-rate and timing values and
/// clears the transient state. Returns the timing-setup result.
export!(cdecl, rw_008B93F0() -> u32 {
    unsafe {
        *global::<u8>(0x0116_0B85) = 1;
        *global::<u8>(0x0116_0B84) = 0;
        *global::<u8>(0x0118_F4BC) = 1;
        *global::<u8>(0x0116_0C28) = 0;
        if callee_cdecl!(1, u32,) & 0xFF == 0 {
            if callee_cdecl!(2, u32,) & 0xFF != 0 {
                *global::<u8>(0x0116_0C28) = 1;
                callee_cdecl!(3, u32,);
            }
            callee_cdecl!(4, u32,);
            callee_cdecl!(5, u32,);
        }
        let context = callee_cdecl!(6, u32, relocated(0x00E7_DEAC));
        if context != 0xFFFF_FFFF {
            callee_cdecl!(7, u32,);
            callee_cdecl!(8, u32, context);
            // Cursor/map device registrations: (group slot, name).
            const DEVICES: [(u32, u32); 8] = [
                (0x0116_1818, 0x00E7_DEB8), // mousecursor
                (0x0116_181C, 0x00E7_DEC4), // replay_pointer
                (0x0116_1820, 0x00E7_DED4), // replay_pointer
                (0x0116_1510, 0x00E7_DEE4), // controller
                (0x0116_1800, 0x00E7_DEF0), // map1
                (0x0116_1804, 0x00E7_DEF8), // map2
                (0x0116_1808, 0x00E7_DF00), // map3
                (0x0116_180C, 0x00E7_DF08), // map4
            ];
            for &(group, name) in DEVICES.iter() {
                callee_thiscall!(9, u32, relocated(group), relocated(name));
            }
            callee_cdecl!(10, u32,);
        }
        *global::<u32>(0x0116_1500) = 0x434B_7EB8;
        *global::<u32>(0x0116_1504) = 0x440A_0148;
        *global::<u32>(0x0116_1824) = 0x434B_7EB8;
        *global::<u32>(0x0117_6760) = 0xFFFF_FFFF;
        *global::<u32>(0x0116_09D0) = 0x45CC_6000;
        *global::<u32>(0x0116_09CC) = 0x45CC_6000;
        *global::<u32>(0x0116_1828) = 0x440A_0148;
        *global::<u8>(0x0116_09D4) = 1;
        *global::<u32>(0x0116_09D8) = 0;
        *global::<u32>(0x0116_0C14) = 0xFFFF_FFA6;
        *global::<u32>(0x0116_09EC) = 0;
        *global::<u32>(0x0116_09DC) = 0;
        *global::<u8>(0x0116_09D5) = 0;
        *global::<u8>(0x0116_09D6) = 0;
        let result = callee_cdecl!(11, u32, 0xFFFF_FFFF, 0);
        *global::<u8>(0x0116_0B87) = 0;
        *global::<u8>(0x0116_09F6) = 1;
        *global::<u8>(0x0116_0C2A) = 0;
        *global::<u8>(0x0116_0C2B) = 0;
        result
    }
});
