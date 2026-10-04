// original: 0x006925e0 audio_keyed_mix_pass
/// Keyed mix pass over one channel bank (original 0x6925e0).
///
/// Same shape as `rb48_fn2`, but the gather phase is a keyed search: slot 0
/// walks a candidate list from a persistent cursor until it finds the entry
/// whose 24-bit key matches the slot object (advancing past smaller keys,
/// giving up past larger ones), later slots reuse the candidate just below
/// the cursor, and only slots with an unflagged voice reach the mix loop.
///
/// Scratch: three alloca `n`-word buffers as in `rb48_fn2`; the third holds
/// only the search cursor (`4 <= n <= 16` by contract).
///
/// EAX on exit is the mix-loop value (16-rounded `n * 4`) when the last slot
/// mixed, the last virtual-call answer when it only gathered, else the value
/// of the gather phase's last step (bound, masked key or loaded word).
export!(thiscall, rb48_fn3(this_ptr: u32, n: u32, arg1: u32, arg2: u32, _u1: u32, _u2: u32, arg3: u32) -> u32 {
    unsafe {
        let count = n as usize;
        let mut gathered = [0u32; 16];
        let mut voices = [0u32; 16];
        let mut cursor = [0u32; 16];
        let obj_key = |obj: u32| {
            (((*((obj.wrapping_add(5)) as *const u8)) as u32) << 16)
                | (*((obj.wrapping_add(6)) as *const u16) as u32)
        };
        let vt = *(this_ptr as *const u32);
        let bank = *((vt.wrapping_add(0x0c)) as *const u32);
        let m = *((vt.wrapping_add(0x10)) as *const u16) as u32;
        if m == 0 {
            return m;
        }
        let mut ret = m;
        let mut i = 0u32;
        while i < m {
            let obj = *(((bank.wrapping_add(i.wrapping_mul(4))) as *const u32));
            let src = core::slice::from_raw_parts(arg3 as *const u32, count);
            gathered[..count].copy_from_slice(src);
            voices[..count].fill(0);
            let mut live = false;
            let mut inner_eax = 0u32;
            let mut k = 0usize;
            // Gather loop; slot 0 searches, later slots reuse the cursor.
            'gather: loop {
                if k >= count {
                    break;
                }
                let key = *(((arg1.wrapping_add((k as u32).wrapping_mul(4))) as *const u32));
                if k == 0 {
                    let bound = *((key.wrapping_add(0x10)) as *const u16) as u32;
                    if cursor[0] >= bound {
                        inner_eax = bound;
                        break 'gather;
                    }
                    let cands = *((key.wrapping_add(0x0c)) as *const u32);
                    let want = obj_key(obj);
                    // EAX at this point holds only the low word of the key;
                    // the give-up path stamps 1 over its low byte.
                    let want_lo = *((obj.wrapping_add(6)) as *const u16) as u32;
                    loop {
                        let cand = *(((cands.wrapping_add(cursor[0].wrapping_mul(4)))
                            as *const u32));
                        let got = obj_key(cand);
                        if got == want {
                            voices[0] = cand;
                            cursor[0] = cursor[0].wrapping_add(1);
                            break;
                        }
                        if got > want {
                            inner_eax = (want_lo & !0xFF) | 1;
                            break 'gather;
                        }
                        cursor[0] = cursor[0].wrapping_add(1);
                        if cursor[0] >= bound {
                            break;
                        }
                    }
                } else {
                    let arr = *((key.wrapping_add(0x0c)) as *const u32);
                    voices[k] = *(((arr.wrapping_add(cursor[0].wrapping_sub(1)
                        .wrapping_mul(4))) as *const u32));
                }
                let loaded = voices[k];
                inner_eax = loaded;
                if loaded == 0 {
                    gathered[k] = 0;
                } else if *((loaded.wrapping_add(4)) as *const u8) & 0x10 != 0 {
                    // Flagged: both words silenced (the zeroing falls through).
                    voices[k] = 0;
                    gathered[k] = 0;
                } else {
                    live = true;
                }
                k += 1;
            }
            if live {
                if arg2 != 0 {
                    let b5 = *((obj.wrapping_add(5)) as *const u8) as u32;
                    let w6 = *((obj.wrapping_add(6)) as *const u16) as u32;
                    let vtable = *(arg2 as *const u32);
                    let target = *(((vtable.wrapping_add(0x10))) as *const u32);
                    let observe: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    ret = observe(arg2, b5, w6, n, gathered.as_ptr() as u32);
                } else {
                    ret = inner_eax;
                }
                if voices[0] != 0 && n > 0 {
                    let mut k = 0usize;
                    while k < count {
                        let sample = f32::from_bits(gathered[k]);
                        if sample > 0.0 {
                            let entry = voices[k];
                            let dl = *((obj.wrapping_add(4)) as *const u8);
                            match dl & 0x0f {
                                1 => {
                                    callee_thiscall!(2, u32, obj, entry);
                                }
                                0 => {
                                    callee_thiscall!(3, u32, obj, entry);
                                }
                                2 if dl & 0x10 == 0 => {
                                    let add = *((entry.wrapping_add(0x10)) as *const f32);
                                    let cur = *((obj.wrapping_add(0x10)) as *const f32);
                                    *((obj.wrapping_add(0x10)) as *mut f32) = cur + add * sample;
                                }
                                _ => {}
                            }
                        }
                        k += 1;
                    }
                    ret = n.wrapping_mul(4).wrapping_add(15) & !15;
                }
            } else {
                ret = inner_eax;
            }
            i += 1;
        }
        ret
    }
});
