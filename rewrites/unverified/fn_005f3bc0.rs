// original: 0x005F3BC0 fragInstNMGta::vf2

/// Link two member slots to a peer and notify the observer, if any.
///
/// Calls the link helper twice with the peer as `this`, passing `this + 4`
/// then `this + 0x6c`. When the observer at `this + 0x64` is non-null, calls
/// the notify helper with the observer as `this` and the peer on the stack
/// between the two link calls. Returns the second link call's result.
/// The original is thiscall with one stack argument (the peer).
lf_checker_rt::export!(thiscall, rw_005f3bc0(this: u32, peer: u32) -> u32 {
    unsafe {
        const FIRST_OFFSET: u32 = 0x04;
        const OBSERVER_OFFSET: u32 = 0x64;
        const SECOND_OFFSET: u32 = 0x6c;
        const LINK_CALLEE: u32 = 1;
        const NOTIFY_CALLEE: u32 = 2;
        let first =
            (this.wrapping_add(FIRST_OFFSET) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(LINK_CALLEE, u32, peer, first);
        let observer = (this.wrapping_add(OBSERVER_OFFSET) as *const u32)
            .read_unaligned();
        if observer != 0 {
            lf_checker_rt::callee_thiscall!(NOTIFY_CALLEE, u32, observer, peer);
        }
        let second =
            (this.wrapping_add(SECOND_OFFSET) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(LINK_CALLEE, u32, peer, second)
    }
});
