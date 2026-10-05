// original: 0x00883750 stream_slot_free (proposed)
/// Free a slot and its two auxiliary objects, clearing every allocation bit.
///
/// Each non-null auxiliary pointer (at `obj+0x08` and `obj+0x0c`) is
/// converted to an index relative to the auxiliary base (global at file
/// address `0x1154064`, divided by 32 through an arithmetic shift right by 2
/// and a logical shift right by 3) and its bit cleared in the auxiliary
/// bitmap (global at `0x1154098`), each step guarded by the auxiliary lock
/// (intercepted callee 1, thiscall: scratch in `ecx`, lock address as the
/// stack argument; released by intercepted callee 2). Then the slot itself
/// is converted to an index relative to the pool base (global at
/// `0x1154088`, signed division by `SLOT_BYTES` = 24) and its bit cleared in
/// the pool bitmap (global at `0x1154090`) under the pool lock. The scratch
/// area is two zero words, matching the checker's zero stack fill.
///
/// Original: cdecl, one stack argument, no return value.
lf_checker_rt::export!(cdecl, rw_00883750(obj: u32) -> u32 {
    unsafe {
        const AUX_LOCK: u32 = 0x0115_40a0;
        const AUX_BASE: u32 = 0x0115_4064;
        const AUX_BITMAP: u32 = 0x0115_4098;
        const POOL_LOCK: u32 = 0x0115_4068;
        const POOL_BASE: u32 = 0x0115_4088;
        const POOL_BITMAP: u32 = 0x0115_4090;
        const SLOT_BYTES: i32 = 24;
        const LOCK_CALLEE: u32 = 1;
        const UNLOCK_CALLEE: u32 = 2;
        let mut scratch = [0u32; 2];
        let aux_base =
            (lf_checker_rt::global::<u32>(AUX_BASE) as *const u32).read_unaligned();
        let aux_bitmap =
            (lf_checker_rt::global::<u32>(AUX_BITMAP) as *const u32).read_unaligned();
        for off in [8u32, 12u32] {
            let aux = ((obj + off) as *const u32).read_unaligned();
            if aux != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    LOCK_CALLEE,
                    u32,
                    scratch.as_mut_ptr() as u32,
                    lf_checker_rt::relocated(AUX_LOCK)
                );
                let index = (aux.wrapping_sub(aux_base) as i32 >> 2) as u32 >> 3;
                let word = aux_bitmap.wrapping_add((index >> 5).wrapping_mul(4));
                let bit = 1u32 << (index & 31);
                let slot = word as *mut u32;
                slot.write_unaligned(slot.read_unaligned() & !bit);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    UNLOCK_CALLEE,
                    u32,
                    scratch.as_mut_ptr() as u32
                );
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_CALLEE,
            u32,
            scratch.as_mut_ptr() as u32,
            lf_checker_rt::relocated(POOL_LOCK)
        );
        let base = (lf_checker_rt::global::<u32>(POOL_BASE) as *const u32).read_unaligned();
        let bitmap =
            (lf_checker_rt::global::<u32>(POOL_BITMAP) as *const u32).read_unaligned();
        let index = (obj.wrapping_sub(base) as i32).wrapping_div(SLOT_BYTES);
        let word = bitmap.wrapping_add(((index >> 5) as u32).wrapping_mul(4));
        let bit = 1u32 << ((index & 31) as u32);
        let slot = word as *mut u32;
        slot.write_unaligned(slot.read_unaligned() & !bit);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
        0
    }
});
