// original: 0x009817c0 audAmbientAudioEntity::vf1
/// Reset an ambient-audio entity's slot table, then adopt one script-defined
/// ambient bank and register the entity's voice classes.
///
/// Behaviour: clears the flag byte of each of 245 fixed-stride slots plus
/// four counter words; asks the audio bank manager for the named bank and,
/// when one is defined with a nonzero entry count, allocates the entity's
/// entry array and walks the bank entries; registers eight voice classes;
/// runs the entity's sub-initialiser; clears two state words and chains to
/// the shared entity initialiser (tail call).
///
/// Counts above 8 entries per trial are not covered: the checker's call log
/// holds 256 entries and a trial with more calls corrupts the worker (the
/// log-full branch uses `ja` where the design needs `jae`, so the 257th
/// call's entry overwrites the stub table). Counts 0, 1, 2, 3, 5 and 8 plus
/// the null-bank and zero-count paths are covered.
export!(thiscall, rw_rb52_9817c0(this: *mut u8) -> u32 {
    unsafe {
        // 245 slots, 0x14 bytes apart: clear each slot's flag byte.
        const SLOT_COUNT: u32 = 0xF5;
        const SLOT_STRIDE: usize = 0x14;
        let mut slot = this.add(0x5C0C);
        let mut left = SLOT_COUNT;
        while left != 0 {
            *slot = 0;
            slot = slot.add(SLOT_STRIDE);
            left -= 1;
        }
        *(this.add(0x5BF8) as *mut u32) = 0;
        *(this.add(0x5BF0) as *mut u32) = 0;
        *(this.add(0x5BF4) as *mut u32) = 0;
        *(this.add(0x6F24) as *mut u32) = 0;

        let manager = relocated(0x115D9A0);
        let bank: u32 = callee_thiscall!(1, u32, manager, relocated(0xE8D8D8));
        if bank != 0 {
            let count = *((bank as *const u8).add(0xA));
            if count != 0 {
                // The original re-tests the count after storing the capacity
                // below; it cannot change in between (dead branch), so one
                // test covers it.
                let n = count as u32;
                *(this.add(0x6F24) as *mut u32) = n;
                let entries = this.add(0x6F28);
                if *(entries.add(6) as *const u16) == 0 {
                    *(entries.add(6) as *mut u16) = n as u16;
                    let items: u32 = callee_thiscall!(2, u32, entries as u32, n);
                    *(entries as *mut u32) = items;
                }
                *(entries.add(4) as *mut u16) = n as u16;
                // Walk the bank entries; each step is 0x360 bytes into the
                // entity's item array and 4 bytes into the bank list.
                let mut cursor = (bank as *const u8).add(0xB);
                let mut step = 0u32;
                let mut done = 0u32;
                while done < n {
                    let resolved: u32 =
                        callee_thiscall!(3, u32, manager, *(cursor as *const u32));
                    let base = *(entries as *const u32);
                    callee_thiscall!(4, u32, base.wrapping_add(step), resolved);
                    cursor = cursor.add(4);
                    step = step.wrapping_add(0x360);
                    done += 1;
                }
            }
        }

        // Register the eight voice classes for this entity kind.
        callee_thiscall!(5, u32, relocated(0x1238810), relocated(0xE8D8EC));
        callee_thiscall!(5, u32, relocated(0x1231788), relocated(0xE8D908));
        callee_thiscall!(5, u32, relocated(0x12317B0), relocated(0xE8D920));
        callee_thiscall!(5, u32, relocated(0x1238760), relocated(0xE8D938));
        callee_thiscall!(5, u32, relocated(0x1238788), relocated(0xE8D94C));
        callee_thiscall!(5, u32, relocated(0x12387B0), relocated(0xE8D960));
        callee_thiscall!(5, u32, relocated(0x12387D8), relocated(0xE8D974));
        callee_thiscall!(5, u32, relocated(0x12317D8), relocated(0xE8D988));

        // Sub-initialiser, then clear state and chain onward (tail call).
        callee_thiscall!(6, u32, this as u32);
        *(this.add(0x6F38) as *mut u32) = 0;
        *(this.add(0x6F20) as *mut u32) = 0;
        callee_thiscall!(7, u32, this as u32)
    }
});
