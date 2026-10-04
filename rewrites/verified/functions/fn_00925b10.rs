// original: 0x00925B10 input_slot_copy
/// Copy two shared blocks into the slot selected by global `0x01174790`.
///
/// Copies 0x880 words-units from `0x0119F100` to slot `index*0x880` past
/// `0x0119FA00`, then 0x1000 units from `0x011A0B10` to slot `index*0x1000`
/// past `0x0119D0F0`, via the shared three-argument block copier.
export!(cdecl, rw_00925B10() -> u32 {
    unsafe {
        let index = *global::<u32>(0x1174790);
        let dst1 = index.wrapping_mul(0x880).wrapping_add(relocated(0x119FA00));
        callee_cdecl!(1, u32, dst1, relocated(0x119F100), 0x880);
        let dst2 = index.wrapping_shl(12).wrapping_add(relocated(0x119D0F0));
        callee_cdecl!(1, u32, dst2, relocated(0x11A0B10), 0x1000);
        0
    }
});
