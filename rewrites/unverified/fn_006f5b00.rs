// original: 0x006F5B00 input_event_accept (proposed)

/// Accept one input event `a1` into `this`, returning 1, or reject it with 0.
///
/// Rejects when the disabled flag (bit 3 at `+0x88`) is set, when `a1`
/// exceeds 0x394, or when the slot lookup callee returns null. Otherwise
/// applies the event through the slot callee (passing `a1`, the sequence
/// word at `+0x80`, a related counter, and two flag words derived from `a2`
/// and `a3`), copies a slice selected by the slot header's kind bit through
/// the copy callee, and links the slot into one of two lists depending on
/// whether the state word at `+0x20` was zero (setting it to 1 in that case).
/// When the kind bit is set, the sequence word is also noted through the
/// notify callee and mirrored to `+0x86`. A non-null `a3` receives the
/// sequence word, the sequence word is incremented, and the emit callee is
/// called. `a0` is passed through to the copy callee as its middle argument.
/// Thiscall with four stack words; the low byte of the incoming arg1 slot is
/// reused as a local, so the proof runs with the stack check off.
lf_checker_rt::export!(thiscall, rw_006F5B00(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const ID_LOOKUP: u32 = 1;
        const ID_APPLY: u32 = 2;
        const ID_COPY: u32 = 3;
        const ID_LISTADD: u32 = 4;
        const ID_NOTE: u32 = 5;
        const ID_EMIT: u32 = 6;
        const MAX_EVENT: u32 = 0x394;
        const DISABLED_FLAG: u8 = 0x08;
        const SEQ: u32 = 0x80;
        const STATE: u32 = 0x20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// Kind bit carried in bit 5 of a slot header's second byte.
        #[inline(always)]
        fn kind_bit(header_byte: u8) -> u32 {
            ((header_byte >> 5) & 1) as u32
        }

        if rd8(this + 0x88) & DISABLED_FLAG != 0 {
            return 0;
        }
        if a1 > MAX_EVENT {
            return 0;
        }
        let fresh = (rd32(this + STATE) == 0) as u32;
        let slot: u32 = lf_checker_rt::callee_thiscall!(ID_LOOKUP, u32, this, a1, a2);
        if slot == 0 {
            return 0;
        }
        let seq = rd16(this + SEQ);
        let counter = if fresh != 0 { seq.wrapping_sub(1) as u32 } else { rd16(this + 0x86) as u32 };
        let flag_a = (a3 != 0) as u32;
        let flag_b = fresh | a2;
        lf_checker_rt::callee_thiscall!(ID_APPLY, u32, slot, a1, seq as u32, counter, flag_b, flag_a);
        let header = rd32(slot);
        let dst = header.wrapping_add((kind_bit(rd8(header.wrapping_add(1))) | 2).wrapping_mul(2));
        lf_checker_rt::callee_cdecl!(ID_COPY, u32, dst, a0, a1);
        let mut link_out = [0u32; 2];
        if fresh != 0 {
            lf_checker_rt::callee_thiscall!(
                ID_LISTADD, u32, this.wrapping_add(0x44), link_out.as_mut_ptr() as u32, 0, slot
            );
            wr32(this + STATE, 1);
        } else {
            lf_checker_rt::callee_thiscall!(
                ID_LISTADD, u32, this.wrapping_add(0x50), link_out.as_mut_ptr() as u32, 0, slot
            );
        }
        // The original reuses one stack slot for the notify input and the
        // emit frame: on the notify path the emit snapshot still sees `seq`.
        let mut shared_slot = [0u32; 2];
        let header2 = rd32(slot);
        if kind_bit(rd8(header2.wrapping_add(1))) != 0 {
            let mut note_out = [0u32; 2];
            shared_slot[0] = seq as u32;
            lf_checker_rt::callee_thiscall!(
                ID_NOTE, u32, this.wrapping_add(0x5c), note_out.as_mut_ptr() as u32,
                shared_slot.as_mut_ptr() as u32, slot
            );
            wr16(this + 0x86, seq);
        }
        if a3 != 0 {
            wr16(a3, seq);
        }
        wr16(this + SEQ, seq.wrapping_add(1));
        lf_checker_rt::callee_thiscall!(
            ID_EMIT, u32, this.wrapping_add(0x5c), shared_slot.as_mut_ptr() as u32,
            this.wrapping_add(SEQ), a3
        );
        1
    }
});
