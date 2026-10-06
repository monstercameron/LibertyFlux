// original: 0x008BF4E0 ui_element_init_type1 (proposed)

/// Initialise a type-1 input-ui element with a default unit rectangle.
///
/// Writes a zero position pair at +0x18/+0x1c, a unit size pair (1.0, 1.0) at
/// +0x20/+0x24, the two flag top-bytes at +0x28/+0x2c, the kind tag 1 at
/// +0x14, the payload word at +0x08 and zero at +0x04, then runs the shared
/// core initialiser through ecx. Afterwards the two flag bytes are compared
/// UNSIGNED: when the first is below the second the dword at +0x0c becomes
/// 255.0 and the byte at +0x10 becomes 0xff, otherwise both stay zero; a null
/// payload clears the active byte. Returns the core initialiser's answer
/// with its low byte replaced by the first flag byte (thiscall, three stack
/// words; only the low byte of each flag word is read).
lf_checker_rt::export!(thiscall, rw_008BF4E0(this: u32, payload: u32, flag_a: u32, flag_b: u32) -> u32 {
    unsafe {
        /// Position pair (zeroed) and size pair (unit).
        const POS_X: u32 = 0x18;
        const POS_Y: u32 = 0x1C;
        const SIZE_W: u32 = 0x20;
        const SIZE_H: u32 = 0x24;
        const ONE_BITS: u32 = 0x3F80_0000;
        /// Flag words holding one byte each in the top byte.
        const FLAG_A: u32 = 0x28;
        const FLAG_B: u32 = 0x2C;
        /// Kind tag: this constructor writes 1.
        const KIND: u32 = 0x14;
        const KIND_TYPE1: u32 = 1;
        /// Payload word and zeroed link word.
        const PAYLOAD: u32 = 0x08;
        const LINK: u32 = 0x04;
        /// Active byte at the object base.
        const ACTIVE: u32 = 0x00;
        /// State dword and flag byte set by the flag comparison.
        const STATE: u32 = 0x0C;
        const FLAG: u32 = 0x10;
        const STATE_BITS: u32 = 0x437F_0000; // 255.0
        /// Id of the core-initialiser callee in the contract.
        const CORE_INIT: u32 = 1;
        let wr32 = |off: u32, v: u32| ((this + off) as *mut u32).write_unaligned(v);
        wr32(POS_X, 0);
        wr32(POS_Y, 0);
        wr32(SIZE_W, ONE_BITS);
        wr32(SIZE_H, ONE_BITS);
        let a = (flag_a & 0xFF) as u8;
        let b = (flag_b & 0xFF) as u8;
        wr32(FLAG_A, (a as u32) << 24);
        wr32(KIND, KIND_TYPE1);
        wr32(PAYLOAD, payload);
        wr32(LINK, 0);
        wr32(FLAG_B, (b as u32) << 24);
        let answer = lf_checker_rt::callee_thiscall!(CORE_INIT, u32, this);
        if a < b {
            wr32(STATE, STATE_BITS);
            ((this + FLAG) as *mut u8).write(0xFF);
        } else {
            wr32(STATE, 0);
            ((this + FLAG) as *mut u8).write(0);
        }
        if payload == 0 {
            ((this + ACTIVE) as *mut u8).write(0);
        }
        (answer & 0xFFFF_FF00) | u32::from(a)
    }
});
