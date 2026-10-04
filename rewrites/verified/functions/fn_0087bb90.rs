// original: 0x0087bb90 rage::crmtNodeMirror::~crmtNodeMirror__deleting
/// Forward this node's payload word to the peer object's slot.
///
/// Reads the payload word at offset 0x20, then invokes the peer's table
/// entry at slot 0x40 with the peer as receiver and the payload as the
/// argument. Returns that entry's answer.
///
/// Note: the merged symbol name claims a deleting destructor, but the body
/// is a single table forward with no flag test and no delete; the name is
/// kept here only because the schema asks for the merged name.
export!(thiscall, rw_0087bb90(this: u32, peer: u32) -> u32 {
    /// Table slot invoked on the peer.
    const SLOT_BYTES: u32 = 0x40;
    unsafe {
        let payload = ((this + 0x20) as *const u32).read();
        let table = (peer as *const u32).read();
        let target = ((table + SLOT_BYTES) as *const u32).read();
        let entry: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        entry(peer, payload)
    }
});
