// original: 0x009512c0 span_rate_accumulate
/// Accumulate rate-adjusted span corrections over key-ordered entries.
///
/// Takes a bound, two out-pointers and a tag. When the tag is -1 a
/// global default is used instead. A scripted table helper answers a
/// descriptor whose words at +0x28/+0x2c are an entry-pointer array and
/// a 16-bit count; both out-words are zeroed first. Every entry whose
/// tag byte at +1 is anything but exactly 100 takes part: with the entry key at +0x14
/// as base, a span to the next key (or, for the last entry, to a
/// global hi bound) is converted to float, divided by the float of a
/// scripted convert(scripted rate(tag)) answer, scaled, nudged by a
/// constant away from zero on the negative side and toward it otherwise
/// (NaN counts as non-negative), truncated back to int with
/// out-of-range and NaN yielding INT_MIN, and added to the first
/// out-word minus the span. Entries whose key is also below the bound
/// (unsigned) run a second kernel of the same shape into the second
/// out-word, over the span to the bound when the next key reaches it
/// and over the reloaded first span otherwise. All key/bound compares
/// are unsigned except the last-entry hi test, which is signed.
export!(cdecl, rw_009512c0(bound: u32, out0: u32, out1: u32, tag: u32) -> u32 {
    unsafe {
        const G_THIS: u32 = 0x11F6954;
        const G_DEFTAG: u32 = 0x11F6F34;
        const G_HI_LO: u32 = 0x11F7028;
        const G_HI_HI: u32 = 0x11F702C;
        const K_GLOB: u32 = 0xE758F8;
        const C_GLOB: u32 = 0xFE8830;
        const ID_LOOKUP: u32 = 0;
        const ID_RATE: u32 = 1;
        const ID_CONV: u32 = 2;
        // Tag dispatch: the below-100 branch and the fall-through past
        // the equality-only skip both reach the main path, so every tag
        // except exactly 100 takes part.
        const TAG_SKIP: u8 = 100;

        // Truncating float-to-int exactly like cvttss2si, whose
        // out-of-range and NaN result is INT_MIN (Rust's `as` saturates).
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }

        let scale = *(relocated(K_GLOB) as *const f32);
        let adjust = *(relocated(C_GLOB) as *const f32);
        // Shared kernel: q = num / float(conv); q *= scale; q -= adjust
        // when negative else q += adjust (NaN takes the add). The
        // unsigned-to-float matches the original's fixup-table path bit
        // for bit: both round the exact integer once.
        let kernel = |num: f32, conv: u32| -> f32 {
            let den = (conv as f64) as f32;
            let mut q = num / den;
            q *= scale;
            if q < 0.0 {
                q -= adjust;
            } else {
                q += adjust;
            }
            q
        };

        let lookup_arg = if tag == 0xFFFFFFFF {
            *(global::<u32>(G_DEFTAG))
        } else {
            tag
        };
        let desc: u32 = callee_thiscall!(
            ID_LOOKUP,
            u32,
            *(global::<u32>(G_THIS)),
            lookup_arg
        );
        *(out1 as *mut u32) = 0;
        *(out0 as *mut u32) = 0;
        let array = *((desc.wrapping_add(0x28)) as *const u32);
        let count =
            core::ptr::read_unaligned((desc.wrapping_add(0x2c)) as *const u16) as u32;
        let end_of = |d: u32| -> u32 {
            let a = *((d.wrapping_add(0x28)) as *const u32);
            let n =
                core::ptr::read_unaligned((d.wrapping_add(0x2c)) as *const u16) as u32;
            a.wrapping_add(n.wrapping_mul(4))
        };
        let mut p = array;
        if p == end_of(desc) {
            return 0;
        }
        loop {
            let entry = *(p as *const u32);
            let etag = *((entry.wrapping_add(1)) as *const u8);
            if etag != TAG_SKIP {
                let key = *((entry.wrapping_add(0x14)) as *const u32);
                if p.wrapping_add(4) == end_of(desc) {
                    // Last entry: span to the global hi bound (signed test).
                    let hi = (*(global::<u32>(G_HI_HI)))
                        .wrapping_sub(*(global::<u32>(G_HI_LO)));
                    if (key as i32) < (hi as i32) {
                        let span = hi.wrapping_sub(key);
                        let num = (span as f64) as f32;
                        let rate: u32 = callee_cdecl!(ID_RATE, u32, etag as u32);
                        let conv: u32 = callee_cdecl!(ID_CONV, u32, rate);
                        let add = cvtt(kernel(num, conv)).wrapping_sub(span as i32) as u32;
                        let o1 = out1 as *mut u32;
                        *o1 = (*o1).wrapping_add(add);
                        if key < bound {
                            let span2 = bound.wrapping_sub(key);
                            let num2 = (span2 as f64) as f32;
                            let entry2 = *(p as *const u32);
                            let tag2 = *((entry2.wrapping_add(1)) as *const u8);
                            let rate2: u32 = callee_cdecl!(ID_RATE, u32, tag2 as u32);
                            let conv2: u32 = callee_cdecl!(ID_CONV, u32, rate2);
                            let add2 =
                                cvtt(kernel(num2, conv2)).wrapping_sub(span2 as i32) as u32;
                            let o0 = out0 as *mut u32;
                            *o0 = (*o0).wrapping_add(add2);
                        }
                    }
                } else {
                    // Interior entry: span to the next key.
                    let next_entry = *((p.wrapping_add(4)) as *const u32);
                    let next_key = *((next_entry.wrapping_add(0x14)) as *const u32);
                    let span = next_key.wrapping_sub(key);
                    let num = (span as i32) as f32;
                    let rate: u32 = callee_cdecl!(ID_RATE, u32, etag as u32);
                    let conv: u32 = callee_cdecl!(ID_CONV, u32, rate);
                    let add = cvtt(kernel(num, conv)).wrapping_sub(span as i32) as u32;
                    let o1 = out1 as *mut u32;
                    *o1 = (*o1).wrapping_add(add);
                    if key < bound {
                        let entry2 = *(p as *const u32);
                        let tag2 = *((entry2.wrapping_add(1)) as *const u8);
                        if next_key < bound {
                            let rate2: u32 = callee_cdecl!(ID_RATE, u32, tag2 as u32);
                            let conv2: u32 = callee_cdecl!(ID_CONV, u32, rate2);
                            let add2 =
                                cvtt(kernel(num, conv2)).wrapping_sub(span as i32) as u32;
                            let o0 = out0 as *mut u32;
                            *o0 = (*o0).wrapping_add(add2);
                        } else {
                            let span2 = bound.wrapping_sub(key);
                            let rate2: u32 = callee_cdecl!(ID_RATE, u32, tag2 as u32);
                            let conv2: u32 = callee_cdecl!(ID_CONV, u32, rate2);
                            let num2 = (span2 as i32) as f32;
                            let q2 = kernel(num2, conv2);
                            let add2 = cvtt(q2).wrapping_sub(span2 as i32) as u32;
                            let o0 = out0 as *mut u32;
                            *o0 = (*o0).wrapping_add(add2);
                        }
                    }
                }
            }
            p = p.wrapping_add(4);
            if p == end_of(desc) {
                break;
            }
        }
        0
    }
});
