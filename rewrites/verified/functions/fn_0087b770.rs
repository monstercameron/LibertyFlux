// original: 0x0087b770 rage::crmtNodeFrame::vf4
/// Tag-checked dispatch through the partner's table.
///
/// Takes the node pointer in ECX and the partner object on the stack. Reads
/// the tag word beside the node header and the partner's routine table.
/// When the tag is clear, the result comes from the table's release entry
/// called with the partner alone. Otherwise the tag travels as the call word
/// into the table's forward entry, called with the partner and the tag, and
/// its answer is the result. The forward entry cleans the word it receives,
/// matching the plain-return path's stack adjustment.
export!(thiscall, rw_0087b770(this: u32, partner: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0x1C;
        const FORWARD_SLOT: u32 = 0x28;
        const RELEASE_SLOT: u32 = 0x48;
        let tag = *((this.wrapping_add(TAG_OFF)) as *const u32);
        let table = *(partner as *const u32);
        if tag == 0 {
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((table.wrapping_add(RELEASE_SLOT)) as *const u32) as usize);
            return release(partner);
        }
        let forward: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((table.wrapping_add(FORWARD_SLOT)) as *const u32) as usize);
        forward(partner, tag)
    }
});
