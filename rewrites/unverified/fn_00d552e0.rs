// original: 0x00d552e0 ccam_wasted_fetch_pose

/// Fetch the wasted camera's pose into `out`, by peer mode.
///
/// `this` points to the object and `out` to 16 bytes. The peer pointer at
/// `PEER` selects the mode from bits 6..9 of its word at `MODE`: mode 3 runs
/// the fetch routine with (`out`, 0x4b3, 1) and returns its answer; mode 2
/// copies the source quad (from the link at `LINK`, or from the peer itself
/// when the link is null) into `out`, then adds the indexed table float at
/// `TABLE` (indexed by the signed word at `INDEX`) to the third lane and
/// returns the table entry; any other mode returns the mode itself.
///
/// Original: 0x00d552e0 (thiscall, one stack argument, one conditional call).
lf_checker_rt::export!(thiscall, rw_00d552e0(this: u32, out: u32) -> u32 {
    unsafe {
        /// Slot holding the peer pointer.
        const PEER: u32 = 0x144;
        /// Peer word carrying the mode in bits 6..9.
        const MODE: u32 = 0x28;
        /// Peer link to the source quad (null means the peer itself).
        const LINK: u32 = 0x20;
        /// Peer word holding the table index (signed).
        const INDEX: u32 = 0x2e;
        /// Global table of float-block pointers.
        const TABLE: u32 = 0x01295cd8;
        /// Float slot within a table block.
        const FSLOT: u32 = 0x38;
        /// Mode that delegates to the fetch routine.
        const MODE_FETCH: u32 = 3;
        /// Mode that copies from the source quad.
        const MODE_COPY: u32 = 2;
        /// Mode tag passed to the fetch routine.
        const FETCH_TAG: u32 = 0x4b3;
        /// Pose fetch routine (intercepted; thiscall, three stack arguments).
        const FETCH: u32 = 1;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let peer = ((this + PEER) as *const u32).read_unaligned();
        let mode = (((peer + MODE) as *const u32).read_unaligned() >> 6) & 0xf;
        if mode == MODE_FETCH {
            return lf_checker_rt::callee_thiscall!(FETCH, u32, peer, out, FETCH_TAG, 1);
        }
        if mode != MODE_COPY {
            return mode;
        }
        let link = ((peer + LINK) as *const u32).read_unaligned();
        let src = if link == 0 {
            peer.wrapping_add(0x10)
        } else {
            link.wrapping_add(0x30)
        };
        let v0 = (src as *const u32).read_unaligned();
        let v1 = f32::from_bits(((src.wrapping_add(4)) as *const u32).read_unaligned());
        let v2 = f32::from_bits(((src.wrapping_add(8)) as *const u32).read_unaligned());
        let v3 = ((src.wrapping_add(0xc)) as *const u32).read_unaligned();
        (out as *mut u32).write_unaligned(v0);
        ((out.wrapping_add(4)) as *mut u32).write_unaligned(v1.to_bits());
        ((out.wrapping_add(8)) as *mut u32).write_unaligned(v2.to_bits());
        ((out.wrapping_add(0xc)) as *mut u32).write_unaligned(v3);
        let peer_again = ((this + PEER) as *const u32).read_unaligned();
        let idx = ((peer_again.wrapping_add(INDEX)) as *const i16).read_unaligned() as i32;
        let entry = (lf_checker_rt::global::<u32>(TABLE).wrapping_add(idx as usize)
            as *const u32).read_unaligned();
        let t = f32::from_bits(((entry.wrapping_add(FSLOT)) as *const u32).read_unaligned());
        ((out.wrapping_add(8)) as *mut u32).write_unaligned(add(t, v2).to_bits());
        entry
    }
});
