// original: 0x00DA4D70 dword_vector_push_back (proposed)

/// Append one dword to a dword vector: while capacity remains, copy the
/// word through `value_ptr` into the slot at the head pointer and advance
/// the head, otherwise grow through the slow-path callee.
///
/// `this` holds the head pointer at `+0x04` and the end pointer at `+0x08`.
/// A null head with remaining capacity still advances the head past the
/// skipped store. The slow path forwards (head, value pointer, a frame
/// pointer, 1, 1) to the grow callee; the frame pointer's address cannot
/// be reproduced across frames, so the contract skips that argument and
/// compares the rest. The return value is whatever the taken path leaves
/// behind (the callee answer, the copied word, or the entry EAX), hence
/// uncompared. Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da4d70(this: u32, value_ptr: u32) -> u32 {
    unsafe {
        const GROW: u32 = 1;
        const HEAD_OFF: u32 = 0x04;
        const END_OFF: u32 = 0x08;

        let head = (this + HEAD_OFF) as *mut u32;
        let end = ((this + END_OFF) as *const u32).read();
        let cur = head.read();
        if cur != end {
            if cur != 0 {
                let v = (value_ptr as *const u32).read();
                (cur as *mut u32).write(v);
            }
            head.write(cur.wrapping_add(4));
        } else {
            // Own-frame pointer stand-in: the address is skipped by the
            // contract (it points into the original's frame there too).
            let probe: u32 = 0;
            let frame_ptr = (&probe as *const u32) as u32;
            lf_checker_rt::callee_thiscall!(GROW, u32, this, cur, value_ptr, frame_ptr, 1, 1);
        }
        0
    }
});
