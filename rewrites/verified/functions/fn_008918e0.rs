// original: 0x008918e0 aud_guarded_slot_forward
/// Forwards the argument through guarded virtual slots, then runs the tail.
///
/// Resolves the target from the byte at 4 (0xff returns immediately) and the
/// table index byte. When the link at 0x74 is non-null and the target's flag
/// byte lacks bit 5, its virtual slot 1 handles the argument and the answer
/// feeds the vector-apply callee. When the link at 0x78 is non-null and the
/// flag byte has bit 1 but not bit 3, its virtual slot 4 handles this object
/// and the argument. Always finishes with the tail callee on the target.
export!(thiscall, rw_008918e0(this: *mut u8, arg: u32) -> () {
    unsafe {
        let b = *(this.add(4));
        if b == 0xff {
            return;
        }
        let stride = *global::<u32>(0x115d968);
        let table = *global::<u32>(0x115d988);
        let idx = *(this.add(0x40)) as u32;
        let entry = *((table
            .wrapping_add(idx.wrapping_mul(0x6f40))
            .wrapping_add(0x6f14)) as *const u32);
        let target = stride.wrapping_mul(b as u32).wrapping_add(entry);
        let obj74 = *(this.add(0x74) as *const u32);
        if obj74 != 0 && *((target + 0xe8) as *const u8) & 0x20 == 0 {
            let vtable = *(obj74 as *const u32);
            let slot = *((vtable.wrapping_add(4)) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let answer = f(obj74, arg);
            let _: u32 = callee_thiscall!(3, u32, this as u32, answer);
        }
        let obj78 = *(this.add(0x78) as *const u32);
        if obj78 != 0 {
            let flags = *((target + 0xe8) as *const u8);
            if flags & 2 != 0 && flags & 8 == 0 {
                let vtable = *(obj78 as *const u32);
                let slot = *((vtable.wrapping_add(0x10)) as *const u32);
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let _ = f(obj78, this as u32, arg);
            }
        }
        let _: u32 = callee_thiscall!(4, u32, target);
    }
});
