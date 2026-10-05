// original: 0x009DE530 pool_obj_bind_sync (proposed)

/// Bind a fresh object to a key slot and synchronise it through gates.
///
/// Marks `this` busy, runs a registrar call, then initialises the key
/// object (slot size 0x20) twice: once with the extra argument, once
/// cleared. The key's signed word at `+0x2E` indexes the shared object
/// table; unless the target's word at `+0x52` is -1 a three-argument sync
/// call runs with that word sign-extended, a global tag and 0x20. Two
/// parameterless sync calls always run. When the target word is still not
/// -1 the key's vtable slot 0x2A method runs; when the key's link at
/// `+0x38` is non-null and that link's word at `+8` is 0xFFFF the slot
/// 0x2B method runs too. `this` is marked idle and the last link (or null)
/// is returned.
///
/// Original: 0x009DE530 (thiscall, two stack arguments, seven outgoing
/// calls: five direct plus two through the key's vtable).
lf_checker_rt::export!(thiscall, rw_009DE530(this: u32, key: u32, extra: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x01295CD8;
        const REGISTRAR_VA: u32 = 0x011737D0;
        const TAG_VA: u32 = 0x0104B888;
        const SLOT_SIZE: u32 = 0x20;
        const SKIP: u16 = 0xFFFF;
        const REGISTRAR: u32 = 1;
        const INIT: u32 = 2;
        const SYNC3: u32 = 3;
        const SYNC0: u32 = 4;
        const SYNC20: u32 = 5;

        (this as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            REGISTRAR, u32, lf_checker_rt::relocated(REGISTRAR_VA), 0u32
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, key, extra, SLOT_SIZE);
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, key, 0u32, SLOT_SIZE);
        let slot = ((key as *const u8).wrapping_add(0x2E) as *const i16).read_unaligned()
            as i32 as u32;
        let target = (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(slot as usize)
            as *const u32)
            .read_unaligned();
        let gate = ((target as *const u8).wrapping_add(0x52) as *const u16).read_unaligned();
        if gate != SKIP {
            let tag = (lf_checker_rt::global::<u32>(TAG_VA) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(
                SYNC3, u32, (gate as i16) as i32 as u32, tag, SLOT_SIZE
            );
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(SYNC0, u32, 0u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(SYNC20, u32, SLOT_SIZE);
        let gate2 = ((target as *const u8).wrapping_add(0x52) as *const u16).read_unaligned();
        let mut out = 0u32;
        if gate2 != SKIP {
            let vtable = (key as *const u32).read_unaligned();
            let stage: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                (((vtable as *const u32).wrapping_add(0x2A)).read_unaligned()) as usize,
            );
            stage(key);
            let link = ((key as *const u8).wrapping_add(0x38) as *const u32).read_unaligned();
            if link != 0 {
                out = link;
                let mark =
                    ((link as *const u8).wrapping_add(8) as *const u16).read_unaligned();
                if mark == SKIP {
                    let finish: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        (((vtable as *const u32).wrapping_add(0x2B)).read_unaligned())
                            as usize,
                    );
                    finish(key);
                    out = 0;
                }
            }
        }
        (this as *mut u8).write(0);
        out
    }
});
