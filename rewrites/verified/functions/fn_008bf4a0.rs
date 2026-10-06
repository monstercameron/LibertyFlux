// original: 0x008BF4A0 ui_element_init_type3 (proposed)

/// Initialise a type-3 input-ui element.
///
/// Writes the payload word at +0x08, the two colour/flag bytes (each shifted
/// into the top byte of its word) at +0x28 and +0x2c, the kind tag 3 at
/// +0x14 and zero at +0x04, then runs the shared core initialiser (active
/// byte at +0, zero dword at +0x0c, zero byte at +0x10) through the object
/// pointer in ecx. Returns the core initialiser's answer (thiscall, three
/// stack words; only the low byte of each flag word is read).
lf_checker_rt::export!(thiscall, rw_008BF4A0(this: u32, payload: u32, flag_a: u32, flag_b: u32) -> u32 {
    unsafe {
        /// Payload word.
        const PAYLOAD: u32 = 0x08;
        /// Flag words holding one byte each in the top byte.
        const FLAG_A: u32 = 0x28;
        const FLAG_B: u32 = 0x2C;
        /// Kind tag: this constructor writes 3.
        const KIND: u32 = 0x14;
        const KIND_TYPE3: u32 = 3;
        /// Zeroed link/state word.
        const LINK: u32 = 0x04;
        /// Id of the core-initialiser callee in the contract.
        const CORE_INIT: u32 = 1;
        ((this + PAYLOAD) as *mut u32).write_unaligned(payload);
        ((this + FLAG_A) as *mut u32).write_unaligned((flag_a & 0xFF) << 24);
        ((this + KIND) as *mut u32).write_unaligned(KIND_TYPE3);
        ((this + LINK) as *mut u32).write_unaligned(0);
        ((this + FLAG_B) as *mut u32).write_unaligned((flag_b & 0xFF) << 24);
        lf_checker_rt::callee_thiscall!(CORE_INIT, u32, this)
    }
});
