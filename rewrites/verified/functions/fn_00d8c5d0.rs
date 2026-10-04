// original: 0x00d8c5d0 audio_list_drain
/// Drains a doubly-linked audio list, disposing every node in order.
///
/// `list` points at the head slot. Each node carries its links at +0xC
/// (prev) and +0x10 (next). Nodes are unlinked front to back and each is
/// handed to the disposal helper together with the global audio owner.
export!(thiscall, rw_00d8c5d0(list: *mut u32) -> u32 {
    unsafe {
        let mut cur = *list;
        while cur != 0 {
            let node = cur as *mut u32;
            let next = *node.add(4);
            if *list == cur {
                *list = next;
            }
            let prev = *((cur + 0xC) as *const u32);
            if prev != 0 {
                *((prev + 0x10) as *mut u32) = next;
            }
            if next != 0 {
                *((next + 0xC) as *mut u32) = prev;
            }
            let owner = *global::<u32>(0x0179_D10C);
            callee_thiscall!(1, u32, owner, cur);
            cur = next;
        }
        0
    }
});
