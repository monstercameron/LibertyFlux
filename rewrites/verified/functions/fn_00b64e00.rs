// original: 0x00b64e00 linked_entry_or_fallback
// thiscall/0 leaf. Follows the link at +0x14 to its entry when present;
// otherwise falls back to the word at +0x20 unless a nonzero non-default tag
// sits at +0x0, in which case it reports null.
export!(thiscall, rw_rs11f13(this: *const u8) -> u32 {
    unsafe {
        let linked = *(this.add(0x14) as *const u32);
        if linked != 0 {
            return *((linked + 0x25c) as *const u32);
        }
        let tag = *(this as *const u32);
        if tag != 0 && tag != 0xa {
            return 0;
        }
        *(this.add(0x20) as *const u32)
    }
});
