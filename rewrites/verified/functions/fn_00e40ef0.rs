// original: 0x00e40ef0 subsystem_reinit_tail
// subsystem re-init with tail handoff.
// Selects two tuning integers from globals under gate answers, converts
// them to float, and hands a five-word frame block (tag 0xff000000, zero,
// the two floats, zero) to the configure helper through two overlapping
// pointers; then runs four member passes, two shared helpers, an optional
// gated pass, and tail-hands-off to the shared routine. The frame pointers
// are verified by content snapshot (addresses skipped). Returns the shared
// routine's answer.
export!(thiscall, rw_00e40ef0(this_obj: u32) -> u32 {
    unsafe {
        const G_A0: u32 = 0x0105_C880;
        const G_A1: u32 = 0x0105_C87C;
        const G_B0: u32 = 0x0105_C884;
        const G_B1: u32 = 0x0105_C888;
        const GATE_OFF: u32 = 0x3cf;
        const TAG: u32 = 0xFF00_0000;
        let gate0: u32 = callee_cdecl!(1, u32,);
        let a = if (gate0 & 0xFF) != 0 {
            *global::<u32>(G_A1)
        } else {
            *global::<u32>(G_A0)
        };
        let gate1: u32 = callee_cdecl!(2, u32,);
        let b = if (gate1 & 0xFF) != 0 {
            *global::<u32>(G_B1)
        } else {
            *global::<u32>(G_B0)
        };
        let fa = (a as i32) as f32;
        let fb = (b as i32) as f32;
        let mut block = [TAG, 0u32, fa.to_bits(), fb.to_bits(), 0u32];
        let first = block.as_mut_ptr();
        let second = block.as_mut_ptr().add(1);
        callee_cdecl!(3, u32, second as u32, first as u32);
        callee_thiscall!(4, u32, this_obj);
        callee_thiscall!(5, u32, this_obj);
        callee_thiscall!(6, u32, this_obj);
        callee_thiscall!(7, u32, this_obj);
        callee_cdecl!(8, u32,);
        callee_cdecl!(9, u32,);
        if *((this_obj.wrapping_add(GATE_OFF)) as *const u8) != 0 {
            callee_thiscall!(10, u32, this_obj);
        }
        callee_cdecl!(8, u32,);
        callee_cdecl!(9, u32,)
    }
});
