// original: 0x00b42f80 adopt_peer_tag_counted
/// Adopt a peer's tag word and count the adoption.
///
/// When the peer object is active and its tag differs from ours, masks our
/// flag word down, copies the tag, clears our spare slot and bumps the
/// global adoption counter; the counter rolling exactly onto 0x2000
/// reports 0, every other path reports 1 (low byte defined).
export!(thiscall, rw_b42f80(peer: u32, obj: u32) -> u32 {
    unsafe {
        if (peer as *const u32).byte_add(0xc18).read() == 0 {
            return 1;
        }
        let ours = (obj as *const u16).byte_add(0xa).read();
        let tag = (peer as *const u16).byte_add(0xc42).read();
        if ours == tag {
            return 1;
        }
        let flags = obj as *mut u32;
        flags.write(flags.read() & 0xffe530ff);
        (obj as *mut u16).byte_add(0xa).write(tag);
        (obj as *mut u32).byte_add(0xc).write(0);
        let counter = global::<u32>(0x17A3374);
        let n = counter.read().wrapping_add(1);
        counter.write(n);
        if n == 0x2000 { 0 } else { 1 }
    }
});
