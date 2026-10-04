// original: 0x00892df0 audSound_update
/// Advance this sound one mixer tick: resample, schedule and commit.
///
/// On first tick initialises the resampling state from the format header
/// (converting byte rates through float, flooring, and asking the
/// sample-rate helper for the step), then dispatches on the scheduler
/// verdict: commit the next window slices, refill the stream buffer, or
/// park the sound when the verdict is negative. Steady ticks only commit
/// slices, and idle ticks rebuild them from scratch. Always releases the
/// sound lock and returns the unlock answer.
///
/// Note: the textured refill path reloads two call registers from the
/// entry-EBX spill slot; both hold the sound pointer here.
export!(thiscall, rw_00892df0(sound: u32) -> u32 {
    unsafe {
        const STEP: f32 = f32::from_bits(0x39ff00ff);
        const RATE: f32 = f32::from_bits(0x3a000000);
        fn fistp_low(x: f32) -> u32 {
            if x.is_nan() {
                return 0;
            }
            let t = (x.trunc()) as f64;
            if t < -9223372036854775808.0 || t >= 9223372036854775808.0 {
                0
            } else {
                (t as i64) as u32
            }
        }
        fn rake(h: f32) -> f32 {
            let bits = h.to_bits();
            let sign = f32::from_bits(bits & 0x80000000);
            let mag = f32::from_bits(0x4b000000 | (bits & 0x80000000));
            let m = if f32::from_bits(bits & 0x7fffffff) < 8388608.0 {
                mag
            } else {
                sign
            };
            let rounded = (h + m) - m;
            let corr = rounded - h;
            // CMPNLE: subtract only when strictly greater (or unordered).
            if !(corr <= sign) {
                rounded - 1.0
            } else {
                rounded
            }
        }
        fn at(s: u32, off: u32) -> u32 {
            unsafe { *((s + off) as *const u32) }
        }
        fn put(s: u32, off: u32, v: u32) {
            unsafe { *((s + off) as *mut u32) = v; }
        }
        let lock = at(sound, 0x38);
        callee_cdecl!(1, u32, lock);
        if at(sound, 0x4c) as u8 == 0 {
            // Idle tick: rebuild the slices from scratch.
            if at(sound, 0x98) == 0 {
                callee_thiscall!(5, u32, sound);
            }
            let seed = at(sound, 0x68);
            let first = if (seed as i32) >= 0 {
                callee_thiscall!(4, u32, sound, seed)
            } else {
                at(sound, 0x6c)
            };
            put(sound, at(sound, 0x58).wrapping_mul(4).wrapping_add(0x70), first);
            let mut d = 1u32;
            let bound = at(sound, 0x50);
            if bound > 1 {
                while d < bound {
                    let k = at(sound, 0x58);
                    let v = at(sound, k.wrapping_mul(4).wrapping_add(0x70));
                    let k2 = k.wrapping_add(d) & 1;
                    put(sound, k2.wrapping_mul(4).wrapping_add(0x70), v.wrapping_add(d));
                    d += 1;
                }
            }
            put(sound, 0x54, bound);
            put(sound, 0x58, (at(sound, 0x58).wrapping_add(bound)) & 1);
            *((sound + 0x47) as *mut u8) = 0;
        } else if *((sound + 0x4d) as *const u8) != 0 {
            // Steady tick: commit the next slices.
            if at(sound, 0x98) == 0 {
                callee_thiscall!(5, u32, sound);
            }
            let first: u32 = callee_thiscall!(4, u32, sound, at(sound, 0x68));
            put(sound, at(sound, 0x58).wrapping_mul(4).wrapping_add(0x70), first);
            let mut d = 1u32;
            let bound = at(sound, 0x50);
            if bound > 1 {
                while d < bound {
                    let k = at(sound, 0x58);
                    let v = at(sound, k.wrapping_mul(4).wrapping_add(0x70));
                    let k2 = k.wrapping_add(d) & 1;
                    put(sound, k2.wrapping_mul(4).wrapping_add(0x70), v.wrapping_add(d));
                    d += 1;
                }
            }
            put(sound, 0x54, bound);
            put(sound, 0x58, (at(sound, 0x58).wrapping_add(bound)) & 1);
            *((sound + 0x47) as *mut u8) = 0;
        } else {
            // First tick: initialise the resampling state.
            *((sound + 0x4d) as *mut u8) = 1;
            callee_thiscall!(2, u32, sound);
            let h = at(sound, 0x7c);
            let c0 = at(h, 0x2c);
            let e0 = at(h, 0xc);
            let n0 = at(h, 0x24);
            let b0 = at(h, 0x10);
            let span = b0.wrapping_add(n0.wrapping_mul(2));
            let width = span.wrapping_mul(8).wrapping_add(0x18);
            put(sound, 0x80, h.wrapping_add(c0));
            put(sound, 0x84, h.wrapping_add(c0).wrapping_add(e0));
            let need = e0.wrapping_sub(width);
            let approx = ((need as f64) as f32) * STEP;
            let count = fistp_low(rake(approx)).wrapping_sub(1);
            put(sound, 0x64, count);
            let total = width.wrapping_add(count.wrapping_mul(8));
            put(sound, 0x60, total);
            let arg = (((total as f64) as f32) * RATE) as f64;
            let bits = arg.to_bits();
            let got: f64 = callee_cdecl!(3, f64, bits as u32, (bits >> 32) as u32);
            let taps = fistp_low(got as f32);
            put(sound, 0x60, taps << 11);
            let seed = at(sound, 0x68);
            let edx = if (seed as i32) >= 0 {
                callee_thiscall!(4, u32, sound, seed)
            } else {
                at(sound, 0x6c)
            };
            if (edx as i32) == 0 {
                if at(sound, 0x98) == 0 {
                    callee_thiscall!(5, u32, sound);
                }
                let first: u32 = callee_thiscall!(4, u32, sound, at(sound, 0x68));
                put(sound, at(sound, 0x58).wrapping_mul(4).wrapping_add(0x70), first);
                let mut lim = at(h, 8);
                let cap = at(sound, 0x50);
                if cap < lim {
                    lim = cap;
                }
                put(sound, 0x54, lim);
                let mut d = 1u32;
                if lim > 1 {
                    while d < lim {
                        let k = at(sound, 0x58);
                        let v = at(sound, k.wrapping_mul(4).wrapping_add(0x70));
                        let k2 = k.wrapping_add(d) & 1;
                        put(sound, k2.wrapping_mul(4).wrapping_add(0x70), v.wrapping_add(d));
                        d += 1;
                    }
                }
                put(sound, 0x58, (at(sound, 0x58).wrapping_add(lim)) & 1);
                *((sound + 0x47) as *mut u8) = 0;
            } else if (edx as i32) > 0 {
                if at(sound, 0x98) == 0 {
                    // Refill the stream buffer.
                    let stride = at(h, 0xc);
                    let mut lim = at(h, 8).wrapping_sub(edx);
                    let cap = at(sound, 0x50);
                    if cap < lim {
                        lim = cap;
                    }
                    put(sound, 0x50, lim);
                    let prod = stride.wrapping_mul(lim);
                    let base = at(sound, 0x58)
                        .wrapping_mul(stride)
                        .wrapping_add(at(h, 0x2c))
                        .wrapping_add(h);
                    let curs = stride.wrapping_mul(edx).wrapping_add(at(h, 0x2c));
                    let end: u32 = callee_cdecl!(8, u32, at(sound, 0x30));
                    let mut dx = prod;
                    if curs.wrapping_add(dx) > end {
                        dx = end.wrapping_sub(curs);
                    }
                    if (dx as i32) <= 0 {
                        callee_thiscall!(10, u32, sound);
                    } else {
                        let snap_slot = taps;
                        let ok: u32 = callee_cdecl!(
                            9, u32, at(sound, 0x30),
                            &snap_slot as *const u32 as u32, 1, curs,
                            at(sound, 0x34), 0, 0
                        );
                        if ok == 0 {
                            callee_thiscall!(10, u32, sound);
                        }
                    }
                    let _ = base;
                } else {
                    // Textured refill.
                    let ebp0 = at(sound, 0xc).wrapping_sub(at(sound, 0x10));
                    let mut lim = at(h, 8).wrapping_sub(edx);
                    let cap = at(sound, 0x50);
                    if cap < lim {
                        lim = cap;
                    }
                    put(sound, 0x50, lim);
                    let bx0 = ebp0.wrapping_sub(0x800) >> 13;
                    let di0 = bx0 << 10;
                    let si0 = h.wrapping_add(0x800);
                    callee_thiscall!(6, u32, sound, si0, bx0, di0);
                    let ebp1 = ebp0 >> 1;
                    let si1 = si0.wrapping_add(ebp1);
                    callee_thiscall!(6, u32, sound, si1, bx0, di0);
                    let done: u32 = callee_thiscall!(
                        7, u32, at(sound, 0x94),
                        si0.wrapping_add(0x800),
                        si1.wrapping_add(0x800),
                        ebp1.wrapping_sub(0x800),
                        at(sound, 0x34)
                    );
                    if (done as u8) == 0 {
                        callee_thiscall!(10, u32, sound);
                    }
                }
            } else {
                // Park the sound.
                let cur = *((sound + 0x42) as *const u16);
                *((sound + 0x44) as *mut u16) = cur;
                put(sound, 0x1c, 0);
                put(sound, 0x40, 0xffffffff);
                put(sound, 0x18, 0);
                put(sound, 0x14, 0);
                put(sound, 0x78, 0);
                *((sound + 0x46) as *mut u16) = 0;
                *((sound + 0x4d) as *mut u8) = 0;
            }
        }
        if *((sound + 0x47) as *const u8) == 0 && at(sound, 0x98) == 0 {
            callee_cdecl!(11, u32, at(sound, 0x30));
        }
        callee_cdecl!(12, u32, lock)
    }
});
