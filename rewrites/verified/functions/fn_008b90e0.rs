// original: 0x008B90E0 FE_ACCEPT

/// Register the frontend input actions and reset the frontend state.
///
/// Clears the frontend globals, runs the pad/keyboard setup queries, copies
/// the selected-device record into the frontend device slot, then registers
/// the 32 named frontend actions (accept, cancel, tabs, buttons, pad start,
/// the input-field keys and the mouse actions) by name, storing each
/// returned handle in its slot. Finally records whether the keyboard is
/// present and refreshes the device list. Returns the refresh result.
export!(cdecl, rw_008B90E0() -> u32 {
    unsafe {
        const UNBOUND: u32 = 0xFFFF_FFA6;
        *global::<u32>(0x0116_0C0C) = UNBOUND;
        *global::<u32>(0x0116_0C10) = UNBOUND;
        *global::<u32>(0x0116_0C14) = UNBOUND;
        *global::<u32>(0x0116_0C18) = UNBOUND;
        *global::<u32>(0x0116_0C1C) = UNBOUND;
        *global::<u32>(0x0116_0C20) = UNBOUND;
        *global::<u8>(0x0116_0C31) = 0;
        *global::<u8>(0x0116_0C32) = 0;
        callee_cdecl!(1, u32, 0);
        *global::<u8>(0x0116_0C34) = 0;
        *global::<u32>(0x0116_09C8) = 0;
        *global::<u32>(0x0116_09E8) = 0;
        *global::<u8>(0x0116_09D7) = 0;
        callee_cdecl!(2, u32,);
        callee_cdecl!(3, u32,);
        // Device select: the query writes three words through the stub
        // into `query_out` (its out-param); the fourth word is a fixed
        // handler address the original stores between the two half-copies.
        // The assembled record is copied into the frontend device slot.
        let mut query_out = [0u32; 4];
        callee_thiscall!(
            4,
            u32,
            query_out.as_mut_ptr() as u32,
            0,
            relocated(0x008C_0620),
            0,
            0
        );
        query_out[3] = relocated(0x0043_0260);
        let device = *global::<u32>(0x0118_F4A8) as *mut u32;
        core::ptr::copy_nonoverlapping(query_out.as_ptr(), device, 4);
        *global::<u8>(0x0116_09F6) = 0;
        // Action registrations: (name, handle slot).
        const ACTIONS: [(u32, u32); 32] = [
            (0x00E7_DBA0, 0x0116_0B88), // FE_ACCEPT
            (0x00E7_DBAC, 0x0116_0B8C), // FE_CANCEL
            (0x00E7_DBB8, 0x0116_0B90), // FE_TAB
            (0x00E7_DBC0, 0x0116_0B94), // FE_SPACE
            (0x00E7_DBCC, 0x0116_0B98), // FE_BUTRB
            (0x00E7_DBD8, 0x0116_0B9C), // FE_BUTLB
            (0x00E7_DBE4, 0x0116_0BA0), // FE_BUTX
            (0x00E7_DBEC, 0x0116_0BA4), // FE_BUTY
            (0x00E7_DBF4, 0x0116_0BA8), // PAD_START
            (0x00E7_DC00, 0x0116_0BAC), // INPUT_F_DELETE
            (0x00E7_DC10, 0x0116_0BB0), // INPUT_F_ENTER
            (0x00E7_DC20, 0x0116_0BB4), // INPUT_F_ESC
            (0x00E7_DC2C, 0x0116_0BB8), // INPUT_F_REFRESH
            (0x00E7_DC3C, 0x0116_0BBC), // INPUT_F_BACK
            (0x00E7_DC4C, 0x0116_0BC0), // INPUT_F_MODEL
            (0x00E7_DC5C, 0x0116_0BC4), // INPUT_F_PLAYERS
            (0x00E7_DC6C, 0x0116_0BC8), // INPUT_F_NEWGAME
            (0x00E7_DC7C, 0x0116_0BCC), // INPUT_F_OPTIONS
            (0x00E7_DC8C, 0x0116_0BD0), // INPUT_F_LOCK
            (0x00E7_DC9C, 0x0116_0BD4), // INPUT_F_CHAT_SC
            (0x00E7_DCAC, 0x0116_0BD8), // INPUT_F_RESU_SC
            (0x00E7_DCBC, 0x0116_0BDC), // INPUT_F_CHAT_U
            (0x00E7_DCCC, 0x0116_0BE0), // INPUT_F_CHAT_T
            (0x00E7_DCDC, 0x0116_0BE4), // INPUT_F_CHAT_I
            (0x00E7_DCEC, 0x0116_0BE8), // INPUT_F_VEHICLE
            (0x00E7_DCFC, 0x0116_0BEC), // FE_SAVE
            (0x00E7_DD04, 0x0116_0BF0), // FE_PREVIEW
            (0x00E7_DD10, 0x0116_0BF4), // FE_CUT_VE
            (0x00E7_DD1C, 0x0116_0BF8), // FE_COPY_VE
            (0x00E7_DD28, 0x0116_0BFC), // FE_PASTE_VE
            (0x00E7_DD34, 0x0116_0C00), // MO_U
            (0x00E7_DD3C, 0x0116_0C04), // MO_DELETE
        ];
        for &(name, slot) in ACTIONS.iter() {
            *global::<u32>(slot) = callee_cdecl!(5, u32, relocated(name), 0);
        }
        let keyboard_present = *global::<u8>(0x018B_7A79) != 0;
        *global::<u32>(0x0116_0ED4) = keyboard_present as u32;
        let devices = callee_cdecl!(6, u32,);
        callee_cdecl!(7, u32, devices)
    }
});
