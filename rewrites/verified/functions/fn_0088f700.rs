// original: 0x0088F700 rage::audSound::audSound_2

/// Second constructor of a sound: install the sound table, run one of two
/// initialisation paths, and reset the voice slots.
///
/// `this` points to the sound. Its table pointer is set to the sound
/// table. When flag bit 4 at `+0x3a` is clear, the bit is set and the
/// entry for sub-id `[this+0x3b]` in the sound-init table global runs
/// (callee 1, called with the sound). When bits 4 and 1 are both set and
/// the sub-id byte at `+0x4` is not 0xff, a mixer cursor is derived from
/// the voice table (voice id `[this+0x40]`, the scalar and table pointer
/// globals) and handed to the cursor entry (callee 2), then the registry
/// entry (callee 3) runs with the registry global and the two id bytes.
/// Finally the sequence word at `+0xa4` is set to -1, and each of the two
/// voice slots at `+0xa8`/`+0xac`, when non-null, is zeroed through and
/// cleared.
///
/// Original: 0x0088F700 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088F700(this: u32) -> () {
    unsafe {
        const VTABLE_SOUND: u32 = 0x00e7_854c;
        const FLAG_BYTE: u32 = 0x3a;
        const INIT_DONE: u8 = 0x10;
        const NEEDS_INIT: u8 = 0x02;
        const SUB_ID: u32 = 0x3b;
        const VOICE_KIND: u32 = 0x04;
        const VOICE_ID: u32 = 0x40;
        const INIT_TABLE: u32 = 0x0115_d774;
        const SCALAR_GLOBAL: u32 = 0x0115_d968;
        const VOICE_TABLE: u32 = 0x0115_d988;
        const REGISTRY: u32 = 0x0115_d8a0;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f14;
        const NO_ID: u8 = 0xff;
        const SEQ: u32 = 0xa4;
        const SLOT_A: u32 = 0xa8;
        const SLOT_B: u32 = 0xac;
        const RUN_INIT: u32 = 1;
        const SET_CURSOR: u32 = 2;
        const REGISTER: u32 = 3;

        let al = ((this + FLAG_BYTE) as *const u8).read();
        (this as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_SOUND));
        if al & INIT_DONE == 0 {
            ((this + FLAG_BYTE) as *mut u8).write(al | INIT_DONE);
            let sub = ((this + SUB_ID) as *const u8).read() as u32;
            let tab = lf_checker_rt::relocated(INIT_TABLE);
            let target: extern "cdecl" fn(u32) -> u32 =
                core::mem::transmute(
                    ((tab.wrapping_add(sub.wrapping_mul(4))) as *const u32)
                        .read_unaligned() as usize,
                );
            target(this);
        } else if al & NEEDS_INIT != 0 {
            let kind = ((this + VOICE_KIND) as *const u8).read();
            if kind != NO_ID {
                let vid = ((this + VOICE_ID) as *const u8).read() as u32;
                let scalar =
                    (lf_checker_rt::relocated(SCALAR_GLOBAL) as *const u32)
                        .read_unaligned();
                let tab = (lf_checker_rt::relocated(VOICE_TABLE)
                    as *const u32)
                    .read_unaligned();
                let w = ((tab
                    .wrapping_add(vid.wrapping_mul(ROW_STRIDE))
                    .wrapping_add(ROW_BIAS)) as *const u32)
                    .read_unaligned();
                let cursor = scalar
                    .wrapping_mul(kind as u32)
                    .wrapping_add(w);
                lf_checker_rt::callee_thiscall!(SET_CURSOR, u32, cursor);
                lf_checker_rt::callee_thiscall!(
                    REGISTER,
                    u32,
                    lf_checker_rt::relocated(REGISTRY),
                    vid,
                    kind as u32
                );
            }
        }
        ((this + SEQ) as *mut u32).write_unaligned(0xffff_ffff);
        let a = ((this + SLOT_A) as *const u32).read_unaligned();
        if a != 0 {
            (a as *mut u8).write(0);
            ((this + SLOT_A) as *mut u32).write_unaligned(0);
        }
        let b = ((this + SLOT_B) as *const u32).read_unaligned();
        if b != 0 {
            (b as *mut u32).write_unaligned(0);
            ((this + SLOT_B) as *mut u32).write_unaligned(0);
        }
    }
});
