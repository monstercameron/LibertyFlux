// original: 0x00bbb660 NativeImpl_CHANGE_CHAR_SIT_IDLE_ANIM
/// Change a sitting ped's idle animation through its task manager.
///
/// Does nothing and returns the gate answer when the start-up gate is set.
/// Otherwise resolves the ped, converts the two animation arguments through
/// their lookup helpers, fetches the sit-task slot from the ped's task
/// manager and, when the slot is filled, issues the change with the converted
/// values and the flags word. Returns the issue call's answer, or zero when
/// the slot was empty.
export!(cdecl, rw_00bbb660(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let gate: u32 = callee_cdecl!(1, u32,);
        if (gate & 0xFF) != 0 {
            return gate;
        }
        let pool = *global::<u32>(0x18B6F1C);
        let ped: u32 = callee_thiscall!(2, u32, pool, a0);
        let first: u32 = callee_cdecl!(3, u32, a1);
        let second: u32 = callee_cdecl!(4, u32, a2);
        let mgr = (*((ped + 0x224) as *const u32)).wrapping_add(0x44);
        let slot: u32 = callee_thiscall!(5, u32, mgr, 0xDD);
        if slot == 0 {
            return 0;
        }
        callee_thiscall!(6, u32, slot, first, second, a3)
    }
});
