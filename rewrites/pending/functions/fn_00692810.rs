// original: 0x00692810 audio_perkey_mix_pass
/// Per-key mix pass over one channel bank (original 0x692810).
///
/// Twin of `rb48_fn3`, but every slot runs its own keyed search with its own
/// persistent cursor (rather than slot 0 searching and later slots reusing
/// the cursor), and a slot whose cursor has run past its bound simply sits
/// this round out instead of ending the gather phase.
///
/// Scratch and contract (`4 <= n <= 16`) are as in `rb48_fn3`; the third
/// buffer holds one cursor per slot.
///
/// EAX on exit is the voice count when no slot ran, the gather phase's last
/// loaded word (low byte replaced by the live flag) when nothing was live,
/// else the mix loop's running value: the count, the buffer distance, or a
/// helper answer, depending on which path the last slot took.
export!(thiscall, rb48_fn4(this_ptr: u32, n: u32, arg1: u32, arg2: u32, _u1: u32, _u2: u32, arg3: u32) -> u32 {
    unsafe {
        let count = n as usize;
        let delta = n.wrapping_mul(4).wrapping_add(15) & !15;
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
            while k < count {
                let key = *(((arg1.wrapping_add((k as u32).wrapping_mul(4))) as *const u32));
                let bound = *((key.wrapping_add(0x10)) as *const u16) as u32;
                if cursor[k] < bound {
                    let cands = *((key.wrapping_add(0x0c)) as *const u32);
                    let want = obj_key(obj);
                    loop {
                        let cand = *(((cands.wrapping_add(cursor[k].wrapping_mul(4)))
                            as *const u32));
                        let got = obj_key(cand);
                        if got == want {
                            voices[k] = cand;
                            cursor[k] = cursor[k].wrapping_add(1);
                            break;
                        }
                        if got > want {
                            break;
                        }
                        cursor[k] = cursor[k].wrapping_add(1);
                        if cursor[k] >= bound {
                            break;
                        }
                    }
                }
                let loaded = voices[k];
                if loaded == 0 {
                    inner_eax = (!live) as u32;
                    gathered[k] = 0;
                } else if *((loaded.wrapping_add(4)) as *const u8) & 0x10 != 0 {
                    inner_eax = (loaded & !0xFF) | ((!live) as u32);
                    voices[k] = 0;
                    gathered[k] = 0;
                } else {
                    inner_eax = loaded & !0xFF;
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
                    // Ignored: the mix loop below reloads EAX first thing.
                    let _ = observe(arg2, b5, w6, n, gathered.as_ptr() as u32);
                }
                let mut eax = n;
                let mut k = 0usize;
                while k < count {
                    if voices[k] != 0 {
                        eax = delta;
                        let sample = f32::from_bits(gathered[k]);
                        if sample > 0.0 {
                            let entry = voices[k];
                            let dl = *((obj.wrapping_add(4)) as *const u8);
                            eax = (eax & !0xFF) | ((dl & 0x0f) as u32);
                            match dl & 0x0f {
                                1 => {
                                    eax = callee_thiscall!(2, u32, obj, entry);
                                }
                                0 => {
                                    eax = callee_thiscall!(3, u32, obj, entry);
                                }
                                2 if dl & 0x10 == 0 => {
                                    let add = *((entry.wrapping_add(0x10)) as *const f32);
                                    let cur = *((obj.wrapping_add(0x10)) as *const f32);
                                    *((obj.wrapping_add(0x10)) as *mut f32) = cur + add * sample;
                                }
                                _ => {}
                            }
                        }
                    }
                    k += 1;
                }
                ret = eax;
            } else {
                ret = inner_eax;
            }
            i += 1;
        }
        ret
    }
});
