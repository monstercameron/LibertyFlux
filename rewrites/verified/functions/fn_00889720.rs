// original: 0x00889720 rage::audVoicePhysical::vf1

/// Initialise a physical voice from a parameter block: clear the state,
/// derive the pool slot index, and resolve the voice and output handles.
///
/// `this` points to the voice and `param` to the parameter block. The flags
/// at `+0x8c` are masked with 0xb0, the parameter pointer is stored at `+0x04`
/// and the state words (`+0x08`, `+0x18`, `+0x20`..`+0x34`, `+0x38`..`+0x7c`)
/// are cleared. The slot index at `+0x88` is the object address minus the pool
/// base (pool pointer global), divided by the slot stride with a magic
/// multiply (signed, rounded toward zero). The voice id word at `param+0x14`
/// decides the rest: 0xffff takes the zero path (null handles at `+0x10` and
/// `+0x14`, default rate at `+0x1c`, returns 0xffff). Otherwise the id is
/// sign-extended (values above 0x7fff go negative) and handed to the voice
/// lookup (callee 1, cdecl); its answer is stored at `+0x10` and, with
/// `param+0x08`, to the output lookup (callee 2, cdecl), whose answer lands at
/// `+0x14`. Bit 4 of the flags is then set to bit 0 of the byte at `+0x48` of
/// the voice object, the word at `+0x18` of the output object goes to `+0x0c`,
/// `+0x1c` takes the default rate, and the word is returned.
///
/// Original: 0x00889720 (thiscall, one stack word, 2 calls).
lf_checker_rt::export!(thiscall, rw_00889720(this: u32, param: u32) -> u32 {
    unsafe {
        const PARAM: u32 = 0x04;
        const HANDLE_A: u32 = 0x10;
        const OUT_WORD: u32 = 0x0c;
        const HANDLE_B: u32 = 0x14;
        const RATE: u32 = 0x1c;
        const SLOT_INDEX: u32 = 0x88;
        const FLAGS: u32 = 0x8c;
        const FLAGS_KEEP: u8 = 0xb0;
        const FLAG_FROM_VOICE: u8 = 0x10;
        const POOL_GLOBAL: u32 = 0x0115_a520;
        const SLOT_MAGIC: u32 = 0x67b2_3a55;
        const SLOT_SHIFT: u32 = 9;
        const VOICE_ID: u32 = 0x14;
        const NO_VOICE: u16 = 0xffff;
        const PARAM_AUX: u32 = 0x08;
        const VOICE_FLAG_BYTE: u32 = 0x48;
        const OUT_WORD_OFF: u32 = 0x18;
        const DEFAULT_RATE: u32 = 0x46ba_b800;
        const VOICE_LOOKUP: u32 = 1;
        const OUT_LOOKUP: u32 = 2;

        let flags = ((this + FLAGS) as *const u8).read() & FLAGS_KEEP;
        ((this + FLAGS) as *mut u8).write(flags);
        ((this + PARAM) as *mut u32).write_unaligned(param);
        ((this + 0x08) as *mut u32).write_unaligned(0);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        let pool = (lf_checker_rt::relocated(POOL_GLOBAL) as *const u32).read_unaligned();
        let base = (pool as *const u32).read_unaligned();
        let diff = this.wrapping_sub(base);
        // Signed divide of `diff` by the slot stride: high word of the
        // SIGNED magic product (the original's imul is signed, so a
        // negative diff must sign-extend before multiplying), shifted,
        // rounded toward zero.
        let high = (((SLOT_MAGIC as i32 as i64) * (diff as i32 as i64)) >> 32) as i32;
        let quot = high >> SLOT_SHIFT;
        let index = quot + ((quot >> 31) & 1);
        ((this + SLOT_INDEX) as *mut u32).write_unaligned(index as u32);
        let mut w = 0x20u32;
        while w <= 0x34 {
            ((this + w) as *mut u32).write_unaligned(0);
            w += 4;
        }
        let mut z = 0u32;
        while z < 18 {
            ((this + 0x38 + z * 4) as *mut u32).write_unaligned(0);
            z += 1;
        }
        let id_word = ((param + VOICE_ID) as *const u16).read_unaligned();
        if id_word == NO_VOICE {
            ((this + HANDLE_A) as *mut u32).write_unaligned(0);
            ((this + HANDLE_B) as *mut u32).write_unaligned(0);
            ((this + RATE) as *mut u32).write_unaligned(DEFAULT_RATE);
            return NO_VOICE as u32;
        }
        // The original zero-extends the word and then sign-extends it back,
        // so ids above 0x7fff reach the lookup as negative numbers.
        let voice_id = (id_word as i16) as i32 as u32;
        let voice: u32 = lf_checker_rt::callee_cdecl!(VOICE_LOOKUP, u32, voice_id);
        ((this + HANDLE_A) as *mut u32).write_unaligned(voice);
        let aux = ((param + PARAM_AUX) as *const u32).read_unaligned();
        let out: u32 = lf_checker_rt::callee_cdecl!(OUT_LOOKUP, u32, voice, aux);
        ((this + HANDLE_B) as *mut u32).write_unaligned(out);
        let src = ((voice + VOICE_FLAG_BYTE) as *const u8).read();
        let mut cl = src.wrapping_shl(4);
        cl ^= flags;
        cl &= FLAG_FROM_VOICE;
        ((this + FLAGS) as *mut u8).write(flags ^ cl);
        let word = ((out + OUT_WORD_OFF) as *const u16).read_unaligned() as u32;
        ((this + OUT_WORD) as *mut u32).write_unaligned(word);
        ((this + RATE) as *mut u32).write_unaligned(DEFAULT_RATE);
        word
    }
});
