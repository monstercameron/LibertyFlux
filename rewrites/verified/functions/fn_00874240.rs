// original: 0x00874240 test_state_tag_or_flag
/// Tests the state of a motion-node sub-object: when the low two bits of the
/// tag word are set the node is decided by tag alone (decided only for tag
/// 3); otherwise the answer is bit 0 of the flag byte in the linked record,
/// or false when there is no linked record.
export!(thiscall, rw_00874240(node: u32) -> u32 {
    unsafe {
        let tag = (node as *const u32).read() & 3;
        if tag != 0 {
            (tag == 3) as u32
        } else {
            let linked = ((node + 0x18) as *const u32).read();
            if linked == 0 {
                0
            } else {
                (((linked + 6) as *const u8).read() & 1) as u32
            }
        }
    }
});
