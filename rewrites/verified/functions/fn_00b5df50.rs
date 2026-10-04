// original: 0x00b5df50 audio_scene_mix
/// Mix one audio scene pass over an array of emitter records.
///
/// Walks `n` records of 0x60 bytes at `arr`, probing each against the
/// category table, accumulating a proximity flag, and dispatching per-mode
/// mix calls whose float results are summed into the returned total.
/// `a1` is the listener handle (nullable), `a2` a target entity handle,
/// `gain` the master level, `a6` an extra mix parameter and `a7` an optional
/// table (three floats, may be null). Returns the mixed total in ST0.
/// Note: the flag word the original pushes carries three unwritten upper
/// bytes; the contract's `stack_fill` defines them as zero.
export!(thiscall, rw_b5df50(this: *mut u8, a1: u32, a2: u32, arr: *mut u8, n: i32, gain: f32, a6: f32, a7: u32) -> f32 {
    unsafe {
        if n <= 0 {
            return 0.0;
        }
        let key = *((this.add(0x18)) as *mut u32);
        let mut prev: u32 = 0xFFFFFFFF;
        let mut sub: u32 = 0;
        let mut acc: f32 = 0.0;
        let mut i: i32 = 0;
        'outer: while i < n {
            let elem = arr.add((i as usize) * 0x60);
            let p0 = *(elem as *mut u32);
            if p0 == 0 {
                i += 1;
                continue 'outer;
            }
            if *(((p0 as *mut u8).add(8)) as *mut u16) == 0xFFFF {
                i += 1;
                continue 'outer;
            }
            let h = callee_cdecl!(1, u32, p0);
            let c: u32 = if h == 0 {
                0
            } else {
                if h == a1 {
                    return 0.0;
                }
                if (*(((h as *mut u8).add(0x28)) as *mut u32) & 0x3c0) == 0xc0 {
                    h
                } else {
                    0
                }
            };
            // Note: the listener is the first stack arg (a1): the original
            // reads it with one word pushed, so [esp+0x40] addresses it.
            let l: u32 = if a1 != 0
                && (*(((a1 as *mut u8).add(0x28)) as *mut u32) & 0x3c0) == 0xc0
            {
                if *(((a1 as *mut u8).add(0x219)) as *mut u8) == 0 && c != 0 {
                    let cx = *(((a1 as *mut u8).add(0x224)) as *mut u32);
                    let r = callee_thiscall!(2, u32, cx, c);
                    if (r & 0xFF) != 0 {
                        i += 1;
                        continue 'outer;
                    }
                }
                a1
            } else {
                0
            };
            let mut flag: u32 = 0;
            if gain > 0.0 {
                let gv = *global::<u32>(0x018b8968);
                let vt = *(gv as *mut u32) as *mut u32;
                let slot = *(vt.add(0x14 / 4)) as usize;
                let probe: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot);
                let mut j: i32 = 0;
                while j < n {
                    if j != i {
                        let ptr = arr.add(0x18 + (j as usize) * 0x60);
                        let w = *(((ptr as *mut u8).add(0x30)) as *mut u32);
                        let mut hit = false;
                        let mut p = probe(gv, w);
                        if *(((p as *mut u8).add(0x20)) as *mut u16) == 0x24 {
                            hit = true;
                        } else {
                            p = probe(gv, w);
                            if *(((p as *mut u8).add(0x20)) as *mut u16) == 0x25 {
                                hit = true;
                            } else {
                                p = probe(gv, w);
                                if *(((p as *mut u8).add(0x20)) as *mut u16) == 0x26 {
                                    hit = true;
                                } else {
                                    p = probe(gv, w);
                                    if *(((p as *mut u8).add(0x20)) as *mut u16) == 0x27 {
                                        hit = true;
                                    }
                                }
                            }
                        }
                        if hit {
                            let da = *((ptr.sub(8)) as *mut f32)
                                - *((elem.add(0x10)) as *mut f32);
                            let db = *((ptr.sub(4)) as *mut f32)
                                - *((elem.add(0x14)) as *mut f32);
                            let dc = *((ptr.add(0)) as *mut f32)
                                - *((elem.add(0x18)) as *mut f32);
                            // Same association as the original: (b*b + a*a) + c*c.
                            let d2 = (db * db + da * da) + dc * dc;
                            if 1.0f32 > d2 {
                                flag = 1;
                            }
                        }
                    }
                    j += 1;
                }
            }
            if h != 0
                && (*(((h as *mut u8).add(0x28)) as *mut u32) & 0x3c0) == 0xc0
            {
                // When a7 is null the original skips the id4 call, the
                // [S+4] check and the doubling entirely (je past them).
                let mut v0 = gain;
                if a7 != 0 {
                    v0 = *((a7 as *mut u8) as *mut f32) * gain;
                    let s = callee_cdecl!(4, u32, key);
                    if *(((s as *mut u8).add(4)) as *mut u32) == 3 && i == 0 {
                        v0 = v0 * 2.0;
                    }
                }
                let f1 = callee_thiscall!(5, f32, this as u32, a1, a2,
                    elem as u32, v0.to_bits());
                // acc += f1 (the add reads the id5 result and acc, both
                // addressed with one word pushed).
                acc = f1 + acc;
                // a3 here is gain (read with one word pushed), not a6.
                let _r = callee_thiscall!(6, u32, this as u32, a1, a2,
                    elem as u32, gain.to_bits(), flag);
            } else {
                let _r = callee_thiscall!(6, u32, this as u32, a1, a2,
                    elem as u32, gain.to_bits(), flag);
                if h != 0 {
                    let m = ((*(((h as *mut u8).add(0x28)) as *mut u32) >> 6) & 0xf);
                    if m == 2 {
                        let mut v0b = gain;
                        if a7 != 0 {
                            v0b = *(((a7 as *mut u8).add(4)) as *mut f32) * gain;
                        }
                        let f = callee_thiscall!(7, f32, this as u32, a1, a2,
                            elem as u32, v0b.to_bits(), a6.to_bits());
                        acc = f + acc;
                    } else if m == 4 {
                        let s2 = callee_cdecl!(4, u32, key);
                        if ((*(((s2 as *mut u8).add(0x20)) as *mut u32) >> 5) & 1) != 0 {
                            let w52 = *(((elem.add(0x52)) as *mut u16)) as u32;
                            let _x = callee_thiscall!(8, u32, h,
                                (elem.add(0x10)) as u32, (elem.add(0x20)) as u32, w52);
                        }
                        let mut v0c = gain;
                        if a7 != 0 {
                            v0c = *(((a7 as *mut u8).add(8)) as *mut f32) * gain;
                        }
                        let f = callee_thiscall!(9, f32, this as u32, a1, a2,
                            elem as u32, v0c.to_bits());
                        acc = f + acc;
                        if *(((h as *mut u8).add(0x25c)) as *mut u32) != 0 {
                            let f2 = callee_thiscall!(10, f32, this as u32, a1, a2,
                                elem as u32, gain.to_bits());
                            acc = f2 + acc;
                        }
                    } else {
                        let w52 = *(((elem.add(0x52)) as *mut u16)) as u32;
                        let _x = callee_thiscall!(8, u32, h,
                            (elem.add(0x10)) as u32, (elem.add(0x20)) as u32, w52);
                        acc = acc + gain;
                    }
                }
            }
            if l != 0 && *(((l as *mut u8).add(0x219)) as *mut u8) != 0 {
                let g8 = *(((l as *mut u8).add(0x6c)) as *mut u32);
                if g8 == 0 || *(((g8 as *mut u8).add(0xe)) as *mut u8) == 0 {
                    let _t = callee_thiscall!(11, u32, this as u32, a1, a2,
                        elem as u32, acc.to_bits());
                }
            }
            let _t = callee_thiscall!(12, u32, this as u32, a1, a2,
                elem as u32, acc.to_bits());
            let eid = *((elem.add(0x48)) as *mut u32);
            if eid != prev && sub < 2 {
                let _u = callee_thiscall!(13, u32, this as u32, a1, a2,
                    elem as u32, acc.to_bits());
                prev = eid;
                sub += 1;
            }
            i += 1;
        }
        acc
    }
});