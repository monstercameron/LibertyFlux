// original: 0x00b55560 index_of_link_in_helper_chain
/// Find the position of a link node inside the helper's own chain.
///
/// Takes the anchor object from the helper, follows the +0x0c chain from the
/// argument's +0x08 link until a node pointing at the anchor, then counts how
/// far into the anchor's +0x10 chain (from its +0x1c head) that node sits.
/// Returns -1 when the anchor is null, the link is null, the link is the
/// anchor itself, or the chain never reaches the anchor; 0 when the anchor's
/// chain is empty or never contains the node.
export!(stdcall, rw_00b55560(arg0: u32) -> u32 {
    unsafe {
        let anchor: u32 = callee_cdecl!(1, u32,);
        if anchor == 0 {
            return 0xffff_ffff;
        }
        let mut cursor = ((arg0 + 8) as *const u32).read();
        if cursor == 0 || cursor == anchor {
            return 0xffff_ffff;
        }
        loop {
            let next = ((cursor + 0x0c) as *const u32).read();
            if next == anchor {
                break;
            }
            cursor = next;
            if cursor == 0 {
                return 0xffff_ffff;
            }
        }
        let mut entry = ((anchor + 0x1c) as *const u32).read();
        let mut index: u32 = 0;
        if entry == 0 {
            return 0;
        }
        loop {
            if entry == cursor {
                return index;
            }
            entry = ((entry + 0x10) as *const u32).read();
            index = index.wrapping_add(1);
            if entry == 0 {
                return 0;
            }
        }
    }
});
