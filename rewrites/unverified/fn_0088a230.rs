// original: 0x0088A230 stream_resource_slot_allocate (proposed)

/// Allocate the first free chunk slot of a resource group.
///
/// `this` points to the resource manager with table base at `+0xe8`,
/// `group` selects a group (0x6f40 bytes each). When the group's free count
/// at `+0x6f0f` is 0 the result is 0xff (with the pinned entry eax above the
/// low byte, as the original leaves it). Otherwise bytes `0..0xc0` of the
/// group are scanned for the first with bit 0 clear; none free yields 0xff.
/// The winner is marked 3, its bit lifted in the two flag words at
/// `winner * 32 + group + 0x54c8`/`0x54cc`/`0x54d0`, its state block (at
/// `winner * 0x70 + 0xc0` into the group) is reset through the block
/// initializer (patched callee; its address bytes are snapshotted), the free
/// count decremented, and the winner index returned. The callee-skipped
/// null-pointer branch never fires on mapped heaps.
///
/// Original: 0x0088A230 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_0088a230(this: u32, group: u32) -> u32 {
    unsafe {
        const ENTRY_EAX_HI: u32 = 0x5a5a_5a00;
        const BLOCK_INIT: u32 = 1;
        const TABLE_BASE: u32 = 0xe8;
        const FREE_OFF: u32 = 0x6f0f;
        const GROUP_STRIDE: u32 = 0x6f40;
        const SCAN_LEN: u32 = 0xc0;
        const TAKEN_MARK: u8 = 3;
        const FLAG_OR: u32 = 0x1fff_ffff;

        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let gbase = group.wrapping_mul(GROUP_STRIDE).wrapping_add(base);
        if ((gbase.wrapping_add(FREE_OFF)) as *const u8).read() == 0 {
            return ENTRY_EAX_HI | 0xff;
        }
        let mut winner = SCAN_LEN;
        let mut i = 0u32;
        while i < SCAN_LEN {
            if ((gbase.wrapping_add(i)) as *const u8).read() & 1 == 0 {
                winner = i;
                break;
            }
            i = i.wrapping_add(1);
        }
        if winner == SCAN_LEN {
            return 0xff;
        }
        ((gbase.wrapping_add(winner)) as *mut u8).write(TAKEN_MARK);
        let fbase = winner.wrapping_mul(32).wrapping_add(gbase);
        let w = ((fbase.wrapping_add(0x54cc)) as *const u32).read_unaligned();
        ((fbase.wrapping_add(0x54cc)) as *mut u32).write_unaligned(w & 0xffff_fffe);
        let w = ((fbase.wrapping_add(0x54c8)) as *const u32).read_unaligned();
        ((fbase.wrapping_add(0x54c8)) as *mut u32).write_unaligned(w | FLAG_OR);
        let w = ((fbase.wrapping_add(0x54d0)) as *const u32).read_unaligned();
        ((fbase.wrapping_add(0x54d0)) as *mut u32).write_unaligned(w & 0xffff_fffc);
        let block = winner
            .wrapping_mul(0x70)
            .wrapping_add(0xc0)
            .wrapping_add(gbase);
        if block != 0 {
            lf_checker_rt::callee_thiscall!(BLOCK_INIT, u32, block);
        }
        let free = (gbase.wrapping_add(FREE_OFF)) as *mut u8;
        free.write(free.read().wrapping_sub(1));
        winner
    }
});
