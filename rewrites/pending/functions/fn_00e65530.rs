// original: 0x00e65530 DAMAGE_RATTLE_AND_SQUEEK
/// Resolve 6 audio events to their runtime handles and cache them.
///
/// Each event name (`DAMAGE_RATTLE_AND_SQUEEK`, `DAMAGE_EXTRA_LOOP_2`, `DAMAGE_EXTRA_LOOP_3`, `DAMAGE_EXTRA_LOOP_4`, `DAMAGE_EXTRA_LOOP_5`, `DAMAGE_EXTRA_LOOP_7`) is looked up through the shared
/// name-lookup routine and its handle stored in the matching slot.
/// Returns the last handle.
export!(cdecl, rw_00e65530() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8FD14), 0);
        *global::<u32>(0x012841B8) = r0;
        *global::<u32>(0x012841BC) = relocated(0x00E8FD64);
        let r1 = callee_cdecl!(1, u32, relocated(0x00E8FD88), 0);
        *global::<u32>(0x012841C0) = r1;
        *global::<u32>(0x012841C4) = relocated(0x00E8FD9C);
        let r2 = callee_cdecl!(1, u32, relocated(0x00E8FDB8), 0);
        *global::<u32>(0x012841C8) = r2;
        *global::<u32>(0x012841CC) = relocated(0x00E8FDCC);
        let r3 = callee_cdecl!(1, u32, relocated(0x00E8FE00), 0);
        *global::<u32>(0x012841D0) = r3;
        *global::<u32>(0x012841D4) = relocated(0x00E8FE14);
        let r4 = callee_cdecl!(1, u32, relocated(0x00E8FE30), 0);
        *global::<u32>(0x012841D8) = r4;
        *global::<u32>(0x012841DC) = relocated(0x00E8FE5C);
        let r5 = callee_cdecl!(1, u32, relocated(0x00E8FE78), 0);
        *global::<u32>(0x012841E0) = r5;
        *global::<u32>(0x012841E4) = relocated(0x00E8FE8C);
        r5
    }
});
