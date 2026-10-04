// original: 0x00952510 shutdown_sequence_tail
/// Runs the shutdown call sequence, publishes the final state, and hands off.
///
/// Two flagged releases, one plain release and five object releases run in a
/// fixed order with a flag byte set between them; three global words are then
/// published and control passes to the final handler with its result
/// forwarded. Returns nothing meaningful.
export!(cdecl, rw_00952510() -> u32 {
    unsafe {
        const FLAG: u32 = 0x0115_DBFC;
        const FIRST_OBJ: u32 = 0x0117_6888;
        const SECOND_OBJ: u32 = 0x0116_2630;
        const THIRD_OBJ: u32 = 0x0123_1800;
        const FOURTH_OBJ: u32 = 0x0123_8898;
        const FIFTH_OBJ: u32 = 0x0123_89E0;
        const SIXTH_OBJ: u32 = 0x0128_4A60;
        const SEVENTH_OBJ: u32 = 0x0128_8780;
        const FINAL_WORD: u32 = 0x0117_6D3C;
        const ZERO_WORD: u32 = 0x011F_66B0;
        const ONE_WORD: u32 = 0x011F_66B4;
        const FINAL_MAGIC: u32 = 0xC2C8_0000;
        callee_thiscall!(1, u32, relocated(FIRST_OBJ), 0xFFFF_FFFF);
        callee_thiscall!(2, u32, relocated(SECOND_OBJ), 0);
        callee_cdecl!(3, u32,);
        *global::<u8>(FLAG) = 1;
        callee_thiscall!(4, u32, relocated(THIRD_OBJ));
        callee_thiscall!(5, u32, relocated(FOURTH_OBJ));
        callee_thiscall!(6, u32, relocated(FIFTH_OBJ));
        callee_thiscall!(7, u32, relocated(SIXTH_OBJ));
        callee_thiscall!(8, u32, relocated(SEVENTH_OBJ));
        *global::<u32>(FINAL_WORD) = FINAL_MAGIC;
        *global::<u32>(ZERO_WORD) = 0;
        *global::<u32>(ONE_WORD) = 1;
        callee_cdecl!(9, u32,)
    }
});
