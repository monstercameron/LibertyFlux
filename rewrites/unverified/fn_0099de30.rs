// original: 0x0099de30 voice_slot_check_positive
/// Ensures the voice slot, then tests a callee answer for positivity.
///
/// When the slot at offset 0x9c is empty it is acquired through callees 1
/// and 2. Callee 3 is then invoked with the slot value and the argument, and
/// the function returns whether that answer is signed-positive.
export!(thiscall, rw_0099de30(this: u32, a0: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x1288780;
        let mut v = *((this.wrapping_add(0x9c)) as *const u32);
        if v == 0 {
            let r1 = callee_thiscall!(1, u32, this, 1);
            v = callee_thiscall!(2, u32, relocated(MANAGER), r1);
        }
        ((callee_cdecl!(3, u32, v, a0) as i32) > 0) as u32
    }
});
