// original: 0x008BF590 ui_element_init_gated (proposed)

/// Initialise an input-ui element from two word pairs, unless gated off.
///
/// When the global gate byte is zero the function does nothing and returns
/// the incoming eax untouched (the contract pins it to zero). Otherwise it
/// writes the payload word at +0x08, the kind tag 0 at +0x14, zero at +0x04,
/// the two words from the first pointer at +0x18/+0x1c, the two words from
/// the second pointer at +0x20/+0x24 and the two flag top-bytes at
/// +0x28/+0x2c, then runs the shared core initialiser through ecx and returns
/// its answer (thiscall, five stack words; only the low byte of each flag
/// word is read).
lf_checker_rt::export!(thiscall, rw_008BF590(
    this: u32,
    payload: u32,
    pair1: u32,
    pair2: u32,
    flag_a: u32,
    flag_b: u32,
) -> u32 {
    unsafe {
        /// Global gate byte: zero means do nothing.
        const GATE: u32 = 0x011609F7;
        /// Payload word, kind tag (0 here) and zeroed link word.
        const PAYLOAD: u32 = 0x08;
        const KIND: u32 = 0x14;
        const KIND_GATED: u32 = 0;
        const LINK: u32 = 0x04;
        /// Rectangle words copied from the two pointers.
        const RECT_A: u32 = 0x18;
        const RECT_B: u32 = 0x1C;
        const RECT_C: u32 = 0x20;
        const RECT_D: u32 = 0x24;
        /// Flag words holding one byte each in the top byte.
        const FLAG_A: u32 = 0x28;
        const FLAG_B: u32 = 0x2C;
        /// Id of the core-initialiser callee in the contract.
        const CORE_INIT: u32 = 1;
        if (lf_checker_rt::relocated(GATE) as *const u8).read() == 0 {
            // Incoming eax passes through; the contract pins it to zero.
            return 0;
        }
        let rd32 = |a: u32| (a as *const u32).read_unaligned();
        let wr32 = |off: u32, v: u32| ((this + off) as *mut u32).write_unaligned(v);
        wr32(PAYLOAD, payload);
        wr32(KIND, KIND_GATED);
        wr32(LINK, 0);
        wr32(RECT_A, rd32(pair1));
        wr32(RECT_B, rd32(pair1 + 4));
        wr32(RECT_C, rd32(pair2));
        wr32(RECT_D, rd32(pair2 + 4));
        wr32(FLAG_A, (flag_a & 0xFF) << 24);
        wr32(FLAG_B, (flag_b & 0xFF) << 24);
        lf_checker_rt::callee_thiscall!(CORE_INIT, u32, this)
    }
});
