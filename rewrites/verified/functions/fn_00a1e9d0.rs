// original: 0x00a1e9d0 cam_indirect_sample_route (proposed)

/// Samples a value through the object's virtual slot and routes it.
///
/// `a0` points at a source record, `a1`/`a3` are flag words (only their
/// low bytes matter), `a2` is a float limit, and `a4`/`a5` are
/// out-pointers. When the ready byte at `a0 + READY_OFF` is set, the
/// cached float at `this + CACHE_OFF` is written to `a4` directly.
/// Otherwise the sampler runs through the virtual slot at offset
/// `VT_SLOT` of the object at `a0 + OBJ_OFF`, yielding `f`: `f` is cached
/// to `+CACHE_OFF`, the word at `a0 + TAG_OFF` is copied to `+TAG_CACHE_OFF`,
/// `f` goes to `a4`, and the tag copy goes to `a5`. Then, unless the hold
/// bit in the flag at `+FLAG_OFF` is set: an `a1` of zero, a set global
/// switch with the limit check passing, or a nonzero `a3` each select the
/// load route (`a4`/`a5` reloaded from `+LOAD0_OFF`/`+LOAD1_OFF`); the
/// remaining case selects the store route (`+LOAD0_OFF`/`+LOAD1_OFF`
/// saved from `a4`/`a5`). A NaN limit takes the store-gated path. Returns
/// nothing. (The original spills the sampled float over the caller's `a0`
/// slot, which a Rust rewrite cannot address; the stack check is off and
/// the value is still observed through the cache and the out-pointer.)
///
/// Original: 0x00a1e9d0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00a1e9d0(
    this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32,
) -> u32 {
    unsafe {
        const C_SAMPLE: u32 = 1;
        const READY_OFF: u32 = 0x210;
        const OBJ_OFF: u32 = 0xa80;
        const TAG_OFF: u32 = 0xaa0;
        const CACHE_OFF: u32 = 0x2f0;
        const TAG_CACHE_OFF: u32 = 0x2f4;
        const FLAG_OFF: u32 = 0x38d;
        const LOAD0_OFF: u32 = 0x2e8;
        const LOAD1_OFF: u32 = 0x2ec;
        const VT_SLOT: u32 = 0x38;
        const HOLD_BIT: u8 = 4;
        const LIM: f32 = f32::from_bits(0x3f00_0000); // 0.5
        const SWITCH: u32 = 0x012d_d299;
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        unsafe fn wr32(x: u32, v: u32) {
            unsafe { (x as *mut u32).write_unaligned(v) }
        }
        if ((a0 + READY_OFF) as *const u8).read() == 0 {
            let obj = rd32(a0 + OBJ_OFF);
            let slot = rd32(rd32(obj) + VT_SLOT);
            let sample: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(slot as usize);
            let f = sample(obj);
            wr32(this + CACHE_OFF, f.to_bits());
            wr32(this + TAG_CACHE_OFF, rd32(a0 + TAG_OFF));
            wr32(a4, f.to_bits());
        } else {
            wr32(a4, rd32(this + CACHE_OFF));
        }
        wr32(a5, rd32(this + TAG_CACHE_OFF));
        if ((this + FLAG_OFF) as *const u8).read() & HOLD_BIT == 0 {
            // Load route: a1 clear; limit passed with the switch set;
            // otherwise (limit failed, or passed with the switch clear)
            // the a3 byte decides.
            let load = if a1 as u8 == 0 {
                true
            } else if LIM > f32::from_bits(a2) {
                lf_checker_rt::global::<u8>(SWITCH).read() != 0 || a3 as u8 != 0
            } else {
                a3 as u8 != 0
            };
            if load {
                wr32(a4, rd32(this + LOAD0_OFF));
                wr32(a5, rd32(this + LOAD1_OFF));
                return 0;
            }
        }
        wr32(this + LOAD0_OFF, rd32(a4));
        wr32(this + LOAD1_OFF, rd32(a5));
        0
    }
});
