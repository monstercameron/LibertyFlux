// original: 0x00987040 audEmitter_range_check (proposed)

/// Emitter range test: reports whether a slot's point lies outside its
/// radius from a thread-selected anchor.
///
/// `this` points at the entity whose slots are `SLOT_STRIDE` bytes apart;
/// `index` (unsigned) selects the slot. Fast path: returns 1 when the slot's
/// byte at `+READY_OFF` is set while its dwords at `+LINK0_OFF` and
/// `+LINK1_OFF` are both zero. Otherwise reads the TLS slot number from the
/// global `TLS_SLOT_SRC`, takes the thread value from that fabricated slot,
/// follows its word at `+ANCHOR_OFF` as an anchor index into the anchor table
/// at `ANCHORS` (64 bytes per anchor), and compares: the slot's radius float
/// at `+RADIUS_OFF` times `RADIUS_MUL`, squared, against the squared distance
/// from the anchor to the slot's point (`+PX_OFF`/`+PY_OFF`/`+PZ_OFF`), summed
/// in the original's order (pinned against reassociating). Returns 1 when the
/// squared distance is strictly above the squared radius (ordered float
/// compare; NaN gives 0), else 0.
/// Original: thiscall, one stack word, callee pops 4, return in `al`.
lf_checker_rt::export!(thiscall, rw_00987040(this: u32, index: u32) -> u32 {
    const SLOT_STRIDE: u32 = 0xd0;
    const READY_OFF: u32 = 0xf2;
    const LINK0_OFF: u32 = 0xdc;
    const LINK1_OFF: u32 = 0xe8;
    const TLS_SLOT_SRC: u32 = 0x17aba14;
    const ANCHOR_OFF: u32 = 0x70;
    const ANCHORS: u32 = 0x115df20;
    const PX_OFF: u32 = 0xa2;
    const PY_OFF: u32 = 0xa6;
    const PZ_OFF: u32 = 0xaa;
    const RADIUS_OFF: u32 = 0xb2;
    const RADIUS_MUL: u32 = 0xe8e2f8;

    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn sub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
    }

    unsafe {
        let base = this.wrapping_add(index.wrapping_mul(SLOT_STRIDE));
        let ready = ((base + READY_OFF) as *const u8).read();
        let l0 = ((base + LINK0_OFF) as *const u32).read_unaligned();
        let l1 = ((base + LINK1_OFF) as *const u32).read_unaligned();
        if ready != 0 && l0 == 0 && l1 == 0 {
            return 1;
        }
        let slot = *lf_checker_rt::global::<u32>(TLS_SLOT_SRC);
        let thread = lf_checker_rt::tls_slot(slot as usize);
        let anchor = ((thread + ANCHOR_OFF) as *const u32).read_unaligned();
        let vec = lf_checker_rt::relocated(ANCHORS)
            .wrapping_add(anchor.wrapping_shl(6));
        let r = mul(
            rdf(base + RADIUS_OFF),
            rdf(lf_checker_rt::relocated(RADIUS_MUL)),
        );
        let r2 = mul(r, r);
        let dx = sub(rdf(base + PX_OFF), rdf(vec));
        let dy = sub(rdf(base + PY_OFF), rdf(vec + 4));
        let dz = sub(rdf(base + PZ_OFF), rdf(vec + 8));
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        (d2 > r2) as u32
    }
});
