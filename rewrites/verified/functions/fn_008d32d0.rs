// original: 0x008D32D0 state_block_init
/// Initializes a state block: zeroes its fields, sets flag bits and seed
/// constants, and fills two float slots from queries.
///
/// `this` is the state object. All words in the block are reset to zero
/// except a flag word updated by masked read-modify-write steps, two seed
/// constants, a float slot that samples uninitialized stack (defined as 0
/// by the contract's `stack_fill`), and two float slots filled from the
/// query callee with selectors 7 and 8. A finalizer call completes the
/// setup. Returns the finalizer's answer (left in EAX by the original).
lf_checker_rt::export!(thiscall, rb109_fn3(this: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_QUERY: u32 = 1; // float query (cdecl/1: selector, f32 ST0 result)
    const CAL_FIN: u32 = 2; // finalizer (thiscall/0, answer kept in EAX)

    const OFF_FLAGS: u32 = 0x3D0; // flag word, masked in four steps
    const SEED_A: u32 = 0x6300_0000; // seed constant at +0x3DC
    const SEED_B: u32 = 0x36A0; // seed constant at +0x3FC
    const NEG_BIT: u32 = 0x8000_0000; // sign word at +0x424

    #[inline(always)]
    unsafe fn store(base: u32, off: u32, v: u32) {
        *((base.wrapping_add(off)) as *mut u32) = v;
    }

    unsafe {
        store(this, 0x3AC, 0);
        store(this, 0x3B0, 0);
        let f0 = *((this.wrapping_add(OFF_FLAGS)) as *const u32);
        store(this, OFF_FLAGS, (f0 & 0xFFFF_FFF0) | 0x10);
        store(this, 0x3A4, 0);
        store(this, 0x3A8, 0);
        store(this, 0x3C0, 0);
        store(this, 0x3C4, 0);
        *((this.wrapping_add(0x3BC)) as *mut u8) = 0;
        store(this, 0x3CC, 0);
        store(this, 0x3C8, 0);
        store(this, 0x3D8, 0);
        store(this, 0x3DC, SEED_A);
        store(this, 0x3EC, 0);
        store(this, 0x3F0, 0);
        store(this, 0x3F4, 0);
        store(this, 0x3F8, 0);
        store(this, 0x3FC, SEED_B);
        store(this, 0x400, 0);
        store(this, 0x404, 0);
        store(this, 0x408, 0);
        // Uninitialized-stack float slot: the contract defines the fill
        // as 0, so both sides observe 0.0 here.
        store(this, 0x40C, 0);
        store(this, 0x418, 0);
        let f1 = *((this.wrapping_add(OFF_FLAGS)) as *const u32);
        store(this, OFF_FLAGS, (f1 & 0xFFFE_77BF) | 0x0002_0000);
        store(this, 0x410, 0);
        *((this.wrapping_add(0x414)) as *mut u16) = 0;
        *((this.wrapping_add(0x417)) as *mut u8) = 0;
        let f7: f32 = lf_checker_rt::callee_cdecl!(CAL_QUERY, f32, 7);
        store(this, 0x3B4, f7.to_bits());
        let f2 = *((this.wrapping_add(OFF_FLAGS)) as *const u32);
        store(this, OFF_FLAGS, (f2 & 0xFFFF_AFFF) | 0x2000);
        store(this, 0x3B8, 0);
        let f8: f32 = lf_checker_rt::callee_cdecl!(CAL_QUERY, f32, 8);
        store(this, 0x3E0, f8.to_bits());
        let f3 = *((this.wrapping_add(OFF_FLAGS)) as *const u32);
        store(this, OFF_FLAGS, f3 & 0xFFFF_F87F);
        store(this, 0x41C, 0);
        store(this, 0x420, 0);
        store(this, 0x424, NEG_BIT);
        store(this, 0x428, 0);
        store(this, 0x42C, 0);
        let r = lf_checker_rt::callee_thiscall!(CAL_FIN, u32, this);
        store(this, 0x430, 0);
        store(this, 0x434, 0);
        store(this, 0x438, 0);
        store(this, 0x43C, 0);
        store(this, 0x440, 0);
        store(this, 0x444, 0);
        store(this, 0x448, 0);
        r
    }
});

