// original: 0x00a4f590 vehicle_find_nearest_slot (proposed)

/// Find the active slot nearest to the point: its address, or zero.
///
/// Scans the 256 flag bytes at `this`, keeping the slot with the smallest
/// squared distance from the point argument (two floats). A slot is skipped
/// when its flag is even; when the first flag word is set and its flag has
/// bit 1; when the second flag word is clear and its link is a live kind
/// with state 2 (a null link or any other kind still proceeds); and when
/// the third flag word is clear and the row's byte has bit 4. Survivors go
/// to the position callee with a scratch word, and the best distance (kept
/// strictly: ties and NaN never replace) starts at the global float. The
/// scratch word is callee output: skipped, never snapped. Thiscall, five
/// stack words (second unread), one callee, best slot or zero in eax.
lf_checker_rt::export!(thiscall, rw_00a4f590(this: u32, pt: u32, _u: u32, f1: u32, f2: u32, f3: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        const G_VA: u32 = 0x00fe8d18;
        const ROW_STRIDE: u32 = 0xe0;
        const SLOTS_BASE: u32 = 0x100;
        const LINKS_BASE: u32 = 0x148;
        const ROWFLAG_OFF: u32 = 0x1dc;
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let mut best = rdf(lf_checker_rt::relocated(G_VA));
        let mut bestslot: u32 = 0;
        let mut i: u32 = 0;
        while i < 0x100 {
            let flag = rd8(this.wrapping_add(i));
            if flag & 1 != 0 {
                let mut skip = false;
                if (f1 & 0xff) != 0 && (flag & 2 != 0) {
                    skip = true;
                }
                if !skip && (f2 & 0xff) == 0 {
                    let q = rd32(this.wrapping_add(LINKS_BASE).wrapping_add(i.wrapping_mul(ROW_STRIDE)));
                    if q != 0 {
                        let k = rd32(q.wrapping_add(0x28)) & 0x3c0;
                        if k == 0x80 && rd32(q.wrapping_add(0x1304)) == 2 {
                            skip = true;
                        }
                    }
                }
                if !skip && (f3 & 0xff) == 0 {
                    let b = rd8(this.wrapping_add(ROWFLAG_OFF).wrapping_add(i.wrapping_mul(ROW_STRIDE)));
                    if b & 0x10 != 0 {
                        skip = true;
                    }
                }
                if !skip {
                    let slotaddr = this.wrapping_add(SLOTS_BASE).wrapping_add(i.wrapping_mul(ROW_STRIDE));
                    let mut scratch = [0u32; 2];
                    let frame = (&mut scratch as *mut u32) as u32;
                    let p: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, slotaddr, frame);
                    let dx = sub(rdf(p), rdf(pt));
                    let dy = sub(rdf(p.wrapping_add(4)), rdf(pt.wrapping_add(4)));
                    let d2 = add(mul(dy, dy), mul(dx, dx));
                    if best > d2 {
                        best = d2;
                        bestslot = slotaddr;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        bestslot
    }
});
