// original: 0x00963350 reset_or_advance_pair
/// Reset or advance a pair of table-driven channels.
///
/// A nonzero low argument byte resets both channels: each channel's flag
/// byte is set from the stored index, the stored index is refreshed (10
/// and 5), the live handles are latched and their targets zeroed, and both
/// ready flags become 2. It returns the second handle. A zero byte with the
/// first ready flag set, or with the force flag set, clears both channels'
/// targets and ready flags and tail-calls the shared follower with
/// argument 1, returning its answer. Otherwise each channel advances: the
/// divisor steps the residue from zero until its flag slot reads 2 (both
/// divisors are nonzero in every trial), helper 1 runs over the selected
/// table entry and the cap, the new slot is marked 2, the old slot 1, the
/// entry and residue are latched, the ready flag is cleared and the target
/// zeroed. Returns the second handle.
///
/// Note: the tail paths overwrite the low byte of the incoming argument
/// slot before jumping, which is irreproducible from Rust, so the contract
/// disables the stack check; nothing else touches the stack above the frame.
export!(cdecl, rw_00963350(arg0: u32) -> u32 {
    unsafe {
        if (arg0 as u8) != 0 {
            let i0 = *(relocated(0x120F298) as *const u8);
            *(relocated(0x120F290) as *mut u32) = 0;
            *((relocated(0x11F6FF0).wrapping_add(i0 as u32)) as *mut u8) = 1;
            let i1 = *(relocated(0x11FA010) as *const u8);
            *(relocated(0x120F298) as *mut u8) = 0xA;
            *((relocated(0x11F7018).wrapping_add(i1 as u32)) as *mut u8) = 1;
            let h1 = *(relocated(0x11F6FA4) as *const u32);
            *(relocated(0x120F294) as *mut u32) = h1;
            *(h1 as *mut u8) = 0;
            let h2 = *(relocated(0x11F7014) as *const u32);
            *(relocated(0x11FA008) as *mut u32) = 0;
            *(relocated(0x11FA010) as *mut u8) = 5;
            *(relocated(0x11FA00C) as *mut u32) = h2;
            *(h2 as *mut u8) = 0;
            *(relocated(0x11F6FFA) as *mut u8) = 2;
            *(relocated(0x11F701D) as *mut u8) = 2;
            return h2;
        }
        if *(relocated(0x11F6FFA) as *const u8) == 1
            || *(relocated(0x11F7073) as *const u8) != 0
        {
            let h1 = *(relocated(0x11F6FA4) as *const u32);
            *(relocated(0x11F6FFA) as *mut u8) = 0;
            *(h1 as *mut u8) = 0;
            let h2 = *(relocated(0x11F7014) as *const u32);
            *(relocated(0x11F701D) as *mut u8) = 0;
            *(h2 as *mut u8) = 0;
            return callee_cdecl!(2, u32, 1);
        }
        // First channel.
        let bl = *(relocated(0x11F6FFB) as *const u8);
        let mut dl = 0u32;
        if *(relocated(0x11F6FF0) as *const u8) != 2 {
            let ecx = bl as u32;
            let mut eax = 0u32;
            loop {
                eax = eax.wrapping_add(1);
                dl = eax % ecx;
                eax = dl;
                if *((relocated(0x11F6FF0).wrapping_add(eax)) as *const u8) == 2 {
                    break;
                }
            }
        }
        let edi = dl;
        let index = (edi.wrapping_add(1)) % (bl as u32);
        let h1 = *(relocated(0x11F6FA4) as *const u32);
        let entry = *((relocated(0x11F6F7C).wrapping_add(index.wrapping_mul(4)))
            as *const u32);
        callee_cdecl!(1, u32, entry, h1, 0x895440);
        let latched = *((relocated(0x11F6F7C).wrapping_add(index.wrapping_mul(4)))
            as *const u32);
        *((relocated(0x11F6FF0).wrapping_add(index)) as *mut u8) = 2;
        *(relocated(0x120F294) as *mut u32) = latched;
        let h1b = *(relocated(0x11F6FA4) as *const u32);
        *((relocated(0x11F6FF0).wrapping_add(edi)) as *mut u8) = 1;
        *(relocated(0x120F298) as *mut u8) = index as u8;
        *(relocated(0x11F6FFA) as *mut u8) = 0;
        *(h1b as *mut u8) = 0;
        // Second channel.
        let bl2 = *(relocated(0x11F6FFD) as *const u8);
        let mut dl2 = 0u32;
        if *(relocated(0x11F7018) as *const u8) != 2 {
            let ecx = bl2 as u32;
            let mut eax = 0u32;
            loop {
                eax = eax.wrapping_add(1);
                dl2 = eax % ecx;
                eax = dl2;
                if *((relocated(0x11F7018).wrapping_add(eax)) as *const u8) == 2 {
                    break;
                }
            }
        }
        let edi2 = dl2;
        let index2 = (edi2.wrapping_add(1)) % (bl2 as u32);
        let h2 = *(relocated(0x11F7014) as *const u32);
        let entry2 = *((relocated(0x11F7000).wrapping_add(index2.wrapping_mul(4)))
            as *const u32);
        callee_cdecl!(1, u32, entry2, h2, 0x895440);
        let latched2 = *((relocated(0x11F7000).wrapping_add(index2.wrapping_mul(4)))
            as *const u32);
        *((relocated(0x11F7018).wrapping_add(index2)) as *mut u8) = 2;
        *((relocated(0x11F7018).wrapping_add(edi2)) as *mut u8) = 1;
        *(relocated(0x11FA00C) as *mut u32) = latched2;
        let h2b = *(relocated(0x11F7014) as *const u32);
        *(relocated(0x11FA010) as *mut u8) = index2 as u8;
        *(relocated(0x11F701D) as *mut u8) = 0;
        *(h2b as *mut u8) = 0;
        h2b
    }
});
