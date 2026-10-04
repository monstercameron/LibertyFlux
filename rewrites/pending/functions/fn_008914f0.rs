// original: 0x008914f0 aud_slot_call_forward
/// Forwards the argument through a table-selected callee.
///
/// Resolves the target from the link byte at 5 (0xff faults on a null
/// dereference, exactly like the original) and the table index byte, reads
/// the slot index from the target, and calls the function-pointer table entry
/// with the target and the argument. Returns the callee's answer.
export!(thiscall, rw_008914f0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let b = *(this.add(5));
        let target = if b == 0xff {
            0
        } else {
            let stride = *global::<u32>(0x115d964);
            let table = *global::<u32>(0x115d988);
            let idx = *(this.add(0x40)) as u32;
            let entry = *((table
                .wrapping_add(idx.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32);
            stride.wrapping_mul(b as u32).wrapping_add(entry)
        };
        let slot = *((target.wrapping_add(0x3b)) as *const u8) as u32;
        let base = relocated(0x115d834);
        let callee = *((base.wrapping_add(slot.wrapping_mul(4))) as *const u32);
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee as usize);
        f(target, arg)
    }
});
