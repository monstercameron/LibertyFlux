// original: 0x00907A80 blip_list_update (proposed)

/// Sweep the 1500-entry blip pointer table, refreshing each live drawable
/// entry, then emit one trailing draw item.
///
/// The hardware path is chosen once from thread state: the TLS slot whose
/// index the `TLS_INDEX` global names holds an object whose `READY` word is
/// non-zero on the hardware path (`hw`). The `TAG_MODE` byte picks the tag
/// (6 when the byte is 0, else 4) carried by the leading item.
///
/// On the hardware path a 16-byte item is allocated, constructed (the buffer
/// word is mixed with the low 14 bits of the buffer word xored against the
/// `HANDLE_CTR` global, which then increments) and submitted; a failed
/// allocation submits null instead. On the fallback path a two-word legacy
/// call is made instead.
///
/// Each table entry equal to the `COUNT` index, null, or whose kind word is
/// not `KIND_DRAW` is skipped. Otherwise a flag byte on the entry selects
/// whether the two mode bytes come from the entry itself or from the
/// `COUNT`-indexed entry, and two bit tests plus up to two query calls steer
/// around the refresh; every path that reaches this point finishes by
/// clearing the entry's kind word. The refresh reads three floats from the
/// entry, scales the third by one of two constants when `TAG_MODE` is set
/// (chosen by a second query call), derives a flag bit and a data word from
/// the selected object, runs the transform call over a two-word scratch
/// pair, folds the scratch word the transform saw into a color word (the
/// previous color, so with a silent transform the color is constant),
/// and either builds a
/// 28-byte item through the item constructor and submits the constructor's
/// answer (hardware path) or makes the five-word legacy call (fallback).
///
/// The epilogue mirrors the prologue: on the hardware path a 16-byte item is
/// allocated and constructed, then the trailing fixup runs; on the fallback
/// path a two-word legacy call is made. The fixup calls the item's virtual
/// slot twice and folds the answers into the buffer word with 16-byte
/// alignment arithmetic: the first answer is reduced **signed** modulo 16
/// (the original's and/branch sequence is the signed-remainder lowering, so
/// negative answers stay negative), padded up to a multiple of 16, added to
/// the second answer, divided by 16 with truncation toward zero (the
/// original's sign-extend/adjust/shift sequence), scaled by 2^14 and masked
/// into bits 22:14 of the buffer word. A null allocation faults reading
/// address 0, exactly as the original does.
///
/// Original: 0x00907A80 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00907A80() -> u32 {
    unsafe {
        const TLS_INDEX_G: u32 = 0x17aba14;
        const TAG_MODE_G: u32 = 0x11609f6;
        const COUNT_G: u32 = 0x1034494;
        const TABLE_G: u32 = 0x118f6f8;
        const HANDLE_CTR_G: u32 = 0x10327a0;
        const QUERY_THIS: u32 = 0x118d7f0;
        const SCALE_WHEN_SET: u32 = 0xfe89b8;
        const SCALE_WHEN_CLEAR: u32 = 0xfe8934;
        const VT_FIRST: u32 = 0xe7e048;
        const VT_FINAL: u32 = 0xe84c78;
        const ITEM_KIND: u32 = 0x59d8b0;
        const VT_SLOT: u32 = 8;
        const TLS_READY_OFF: u32 = 0x8cc;
        const ENTRY_KIND: u32 = 0x0c;
        const ENTRY_LOCAL: u32 = 0x08;
        const MODE_B0: u32 = 0x20;
        const MODE_B1: u32 = 0x21;
        const ENTRY_DATA: u32 = 0x54;
        const ENTRY_F0: u32 = 0x10;
        const KIND_DRAW: u32 = 2;
        const TABLE_LEN: i32 = 0x5dc;
        const COLOR_BITS: u32 = 0xffffff;
        const COLOR_BASE: u32 = 0x50000000;

        const CALLOC: u32 = 1;
        const CSUBMIT: u32 = 2;
        const CLEGACY2: u32 = 3;
        const CQUERY: u32 = 4;
        const CQUERY2: u32 = 5;
        const CXFORM: u32 = 6;
        const CCTOR: u32 = 7;
        const CLEGACY5: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        /// Build a 16-byte item: stamp the first table, mix the buffer word
        /// with the handle counter (low 14 bits of buffer xor counter), bump
        /// the counter, then stamp the final table, the kind and the tag.
        #[inline(always)]
        unsafe fn construct_item(obj: u32, tag: u32) {
            unsafe {
                let saved = rd32(obj + 4);
                wr32(obj, lf_checker_rt::relocated(VT_FIRST));
                let ctr = rd32(lf_checker_rt::relocated(HANDLE_CTR_G));
                let mix = (saved ^ ctr) & 0x3fff;
                wr32(obj + 4, saved ^ mix);
                wr32(
                    lf_checker_rt::relocated(HANDLE_CTR_G),
                    ctr.wrapping_add(1),
                );
                wr32(obj, lf_checker_rt::relocated(VT_FINAL));
                wr32(obj + 8, lf_checker_rt::relocated(ITEM_KIND));
                wr32(obj + 0xc, tag);
            }
        }

        /// Trailing alignment fixup. Reads the item's table pointer
        /// unconditionally, so a null item faults on address 0 exactly like
        /// the original.
        #[inline(always)]
        unsafe fn align_fixup(obj: u32) {
            unsafe {
                let load_hook = |o: u32| unsafe {
                    let slot: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(o).wrapping_add(VT_SLOT))
                            as usize);
                    slot
                };
                let first = load_hook(obj)(obj);
                // Signed remainder and division, as the original.
                let rem = (first as i32) % 16;
                let padded = 16i32 - rem;
                let pad = (padded % 16) as u32;
                let second = load_hook(obj)(obj);
                let total = second.wrapping_add(pad);
                let scaled = ((total as i32) / 16) as u32;
                let folded = scaled.wrapping_shl(14);
                let buf = rd32(obj + 4);
                let tmp = (buf ^ folded) & 0x01ffc000;
                wr32(obj + 4, buf ^ tmp);
            }
        }

        /// The Y check: re-read the entry, fall back to the COUNT-indexed
        /// entry when its local flag is clear, and test mode byte 1. A clear
        /// bit, or a set query answer, sends the entry straight to the
        /// kind-word clear (returns true).
        #[inline(always)]
        unsafe fn y_check(table: u32, idx: u32, count: u32) -> bool {
            unsafe {
                let mut e = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                if rd8(e.wrapping_add(ENTRY_LOCAL)) == 0 {
                    e = rd32(table.wrapping_add(count.wrapping_mul(4)));
                }
                if (rd8(e.wrapping_add(MODE_B1)) & 1) == 0 {
                    return true;
                }
                let q = lf_checker_rt::callee_cdecl!(CQUERY, u32,) & 0xff;
                q != 0
            }
        }

        /// Refresh one entry: scale the third float, run the transform, fold
        /// the color, then build-and-submit or the five-word legacy call.
        #[inline(always)]
        #[allow(clippy::too_many_arguments)]
        unsafe fn refresh(
            table: u32,
            count_g: u32,
            idx: u32,
            ent: u32,
            local: u8,
            tag_mode: u8,
            hw: bool,
            xscratch: &mut [u32; 2],
        ) {
            unsafe {
                let count2 = rd32(count_g);
                let e = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                let f0 = rdf(e.wrapping_add(ENTRY_F0));
                let f1 = rdf(e.wrapping_add(ENTRY_F0 + 4));
                let f2raw = rdf(e.wrapping_add(ENTRY_F0 + 8));
                // The original scales only the second slot of the pair: the
                // first keeps the raw value, the second takes raw*scale.
                let mut f2 = f2raw;
                if tag_mode != 0 {
                    let q = lf_checker_rt::callee_thiscall!(
                        CQUERY2,
                        u32,
                        lf_checker_rt::relocated(QUERY_THIS)
                    ) & 0xff;
                    let scale = if q != 0 {
                        rdf(lf_checker_rt::relocated(SCALE_WHEN_SET))
                    } else {
                        rdf(lf_checker_rt::relocated(SCALE_WHEN_CLEAR))
                    };
                    f2 = mul(f2, scale);
                }
                let sel_byte = |off: u32| unsafe {
                    if local != 0 {
                        rd8(ent.wrapping_add(off))
                    } else {
                        let other =
                            rd32(table.wrapping_add(count2.wrapping_mul(4)));
                        rd8(other.wrapping_add(off))
                    }
                };
                let flag1 = ((sel_byte(MODE_B0) >> 1) & 1) as u32;
                let data = if local != 0 {
                    rd32(ent.wrapping_add(ENTRY_DATA))
                } else {
                    let other =
                        rd32(table.wrapping_add(count2.wrapping_mul(4)));
                    rd32(other.wrapping_add(ENTRY_DATA))
                };
                xscratch[1] = flag1;
                lf_checker_rt::callee_cdecl!(
                    CXFORM,
                    u32,
                    xscratch.as_ptr() as u32,
                    data,
                    flag1,
                    0
                );
                // The color folds the scratch word the transform saw (the
                // previous color, 0 on the first pass), not the entry float:
                // the original reads it from the same frame slot it passed
                // by pointer.
                let color = (xscratch[0] & COLOR_BITS) | COLOR_BASE;
                xscratch[0] = color;
                if hw {
                    let obj = lf_checker_rt::callee_cdecl!(CALLOC, u32, 0x1c, 0);
                    if obj != 0 {
                        let pair0 = [f0.to_bits(), f1.to_bits()];
                        let pair1 = [f2raw.to_bits(), f2.to_bits()];
                        let built = lf_checker_rt::callee_thiscall!(
                            CCTOR,
                            u32,
                            obj,
                            pair0.as_ptr() as u32,
                            pair1.as_ptr() as u32,
                            color
                        );
                        lf_checker_rt::callee_cdecl!(CSUBMIT, u32, built);
                    } else {
                        lf_checker_rt::callee_cdecl!(CSUBMIT, u32, 0);
                    }
                } else {
                    let pair0 = [f0.to_bits(), f1.to_bits()];
                    let pair1 = [f2raw.to_bits(), f2.to_bits()];
                    let pair2 = [color, flag1];
                    lf_checker_rt::callee_cdecl!(
                        CLEGACY5,
                        u32,
                        pair0.as_ptr() as u32,
                        pair1.as_ptr() as u32,
                        0x28,
                        pair2.as_ptr() as u32,
                        0
                    );
                }
            }
        }

        let table = lf_checker_rt::relocated(TABLE_G);
        let count_g = lf_checker_rt::relocated(COUNT_G);
        let slot = rd32(lf_checker_rt::relocated(TLS_INDEX_G));
        let tls = lf_checker_rt::tls_slot(slot as usize);
        let hw = rd32(tls.wrapping_add(TLS_READY_OFF)) != 0;
        let tag_mode = rd8(lf_checker_rt::relocated(TAG_MODE_G));
        let lead_tag = if tag_mode == 0 { 6u32 } else { 4u32 };

        if hw {
            let obj = lf_checker_rt::callee_cdecl!(CALLOC, u32, 0x10, 0);
            if obj != 0 {
                construct_item(obj, lead_tag);
                lf_checker_rt::callee_cdecl!(CSUBMIT, u32, obj);
            } else {
                lf_checker_rt::callee_cdecl!(CSUBMIT, u32, 0);
            }
        } else {
            lf_checker_rt::callee_cdecl!(CLEGACY2, u32, 2, lead_tag);
        }

        // Transform scratch pair, reused across iterations exactly like the
        // original's frame slots: word 0 keeps the previous color (0 on the
        // first pass), word 1 is the fresh flag bit.
        let mut xscratch = [0u32, 0u32];
        let mut idx = 0u32;
        while (idx as i32) < TABLE_LEN {
            let count = rd32(count_g);
            if idx != count {
                let ent = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                if ent != 0 && rd32(ent.wrapping_add(ENTRY_KIND)) == KIND_DRAW {
                    let local = rd8(ent.wrapping_add(ENTRY_LOCAL));
                    // Mode bytes come from the entry itself when its local
                    // flag is set, else from the COUNT-indexed entry. The
                    // COUNT-indexed entry is only touched on that path.
                    let sel_byte = |off: u32| unsafe {
                        if local != 0 {
                            rd8(ent.wrapping_add(off))
                        } else {
                            let other =
                                rd32(table.wrapping_add(count.wrapping_mul(4)));
                            rd8(other.wrapping_add(off))
                        }
                    };
                    let bit_a = ((sel_byte(MODE_B0) >> 2) & 1) != 0;
                    let direct = !bit_a && (sel_byte(MODE_B1) & 1) == 0;
                    if !direct {
                        // Slow steer: retest bit A, then either query or the
                        // Y check; either may send the entry straight to the
                        // kind-word clear below.
                        let bit_a2 = ((sel_byte(MODE_B0) >> 2) & 1) != 0;
                        let mut skip_refresh = false;
                        if !bit_a2 {
                            skip_refresh = y_check(table, idx, count);
                        } else {
                            let q =
                                lf_checker_rt::callee_cdecl!(CQUERY, u32,)
                                    & 0xff;
                            if q == 0 {
                                skip_refresh = y_check(table, idx, count);
                            }
                        }
                        if !skip_refresh {
                            refresh(
                                table, count_g, idx, ent, local, tag_mode, hw,
                                &mut xscratch,
                            );
                        }
                    } else {
                        refresh(
                            table, count_g, idx, ent, local, tag_mode, hw,
                            &mut xscratch,
                        );
                    }
                    let again = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                    wr32(again.wrapping_add(ENTRY_KIND), 0);
                }
            }
            idx = idx.wrapping_add(1);
        }

        if hw {
            let obj = lf_checker_rt::callee_cdecl!(CALLOC, u32, 0x10, 0);
            if obj != 0 {
                construct_item(obj, 6);
            }
            align_fixup(obj);
        } else {
            lf_checker_rt::callee_cdecl!(CLEGACY2, u32, 2, 6);
        }
        0
    }
});
