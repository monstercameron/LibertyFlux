// original: 0x00695240 anim_registry_insert
/// Register an animation object in the shared id table.
///
/// Ids below 0x80 index the direct table: the first registration also runs
/// the table-setup helper and clears the table. Ids at or above 0x80 go
/// through the overflow helper, which supplies the slot. Returns the table
/// base, or the overflow slot.
export!(thiscall, rs80_695240(this: *const u8) -> u32 {
    unsafe {
        const DIRECT_MAX: u8 = 0x80;
        const TABLE_WORDS: usize = 14;
        let id = *((this).add(0));
        if id < DIRECT_MAX {
            if *global::<u16>(0x019F2384) == 0 {
                let _: u32 = callee_stdcall!(1, u32, this as u32);
                let table = *global::<u32>(0x019F2380) as *mut u32;
                for i in 0..TABLE_WORDS {
                    *table.add(i) = 0;
                }
            }
            let table = *global::<u32>(0x019F2380) as *mut u32;
            *table.add(id as usize) = this as u32;
            table as u32
        } else {
            let slot: u32 = callee_stdcall!(2, u32, this as u32);
            *(slot as *mut u32) = this as u32;
            slot
        }
    }
});
