// original: 0x00B59D40 rage::phConstrainedCollider::vf9
/// Advances the pending counts in two shared object slots. For each live
/// object it decrements a nonzero 16-bit count, invokes virtual slot zero with
/// the object and a flag of one when the count reaches zero in mode 2 or 4,
/// and clears the global slot. The callback is reached through the object's
/// own table. The return register is incidental and is not part of this proof.
lf_checker_rt::export!(cdecl, rw_00B59D40() -> u32 {
    unsafe {
        const FIRST_OBJECT_VA: u32 = 0x0166_DA14;
        const SECOND_OBJECT_VA: u32 = 0x0166_DA18;
        const OBJECT_MODE: u32 = 0x08;
        const OBJECT_PENDING_COUNT: u32 = 0x0a;
        const READY_MODE_A: u8 = 2;
        const READY_MODE_B: u8 = 4;

        let advance_slot = |global_slot: *mut u32| {
            let object = global_slot.read();
            if object != 0 {
                let count_word = object.wrapping_add(OBJECT_PENDING_COUNT) as *mut u16;
                let count = count_word.read_unaligned();
                if count != 0 {
                    let remaining = count.wrapping_sub(1);
                    count_word.write_unaligned(remaining);
                    let mode = (object.wrapping_add(OBJECT_MODE) as *const u8).read();
                    if remaining == 0 && (mode == READY_MODE_A || mode == READY_MODE_B) {
                        let vtable = (object as *const u32).read_unaligned();
                        let callback = (vtable as *const u32).read_unaligned();
                        let invoke: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(callback as usize);
                        let _ = invoke(object, 1);
                    }
                }
            }
            global_slot.write(0);
        };

        advance_slot(lf_checker_rt::global::<u32>(FIRST_OBJECT_VA));
        advance_slot(lf_checker_rt::global::<u32>(SECOND_OBJECT_VA));
        0
    }
});
