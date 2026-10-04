// original: 0x00bf39c0 derive_entity_shadow (proposed name)
/// Derive a shadow record from an entity's live fields.
///
/// `this_` is the shadow record under construction, `arg1` the source
/// entity. After initializing the header, the function copies the key
/// fields, splices three flag bits from packed source words, copies or
/// clears the detail bytes depending on one spliced bit, checks two
/// helper answers against stored slots, applies a global-gated tag, and
/// resolves the record's link field through virtual calls (with a deep
/// fallback chain). A final flag splice and a closing helper call finish
/// the record. Returns the closing helper's answer.
export!(thiscall, rw_00bf39c0(this_: *mut u8, arg1: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ as u32, 0x20);
        *(this_.add(0x10) as *mut u32) = *(arg1.add(0x64) as *const u32);
        let aux = *(arg1.add(0x280) as *const u32);
        let mut edx = if aux == 0 {
            0xffffffffu32
        } else {
            *((aux + 0x64) as *const u32)
        };
        *(this_.add(0x14) as *mut u32) = edx;
        *(this_.add(0x0a) as *mut u16) = *(arg1.add(0x2e) as *const u16);
        let e24 = *(arg1.add(0x24) as *const u32);
        let mut b8 = *this_.add(8);
        b8 ^= (((e24 >> 5) as u8 ^ b8) & 1);
        *this_.add(8) = b8;
        *this_.add(9) = *(arg1.add(0x63) as *const u8);
        let cond = ((e24 >> 27) & 1) != 0
            && *(arg1.add(0x48) as *const u32) != 0xffffffff
            && *(arg1.add(0x40) as *const u8) != 0x3f;
        let mut al = (cond as u8) << 1;
        al = (al ^ b8) & 2;
        b8 ^= al;
        *this_.add(8) = b8;
        *this_.add(0x18) = *(arg1.add(0xb8) as *const u8);
        let e210 = *(arg1.add(0x210) as *const u32);
        let mut al2 = ((e210 >> 27) as u8).wrapping_shl(4);
        al2 ^= b8;
        al2 &= 0x10;
        al2 ^= b8;
        *this_.add(8) = al2;
        if (al2 & 0x10) != 0 {
            *this_.add(0x0d) = *(arg1.add(0x24c) as *const u8);
            *this_.add(0x0e) = *(arg1.add(0x24d) as *const u8);
            *this_.add(0x0f) = *(arg1.add(0x24e) as *const u8);
            if edx == 0xffffffff {
                edx = *(arg1.add(0x240) as *const u32);
                *(this_.add(0x14) as *mut u32) = edx;
            }
            let d = *this_.add(0x0d) as u32;
            let e = *this_.add(0x0e) as u32;
            let f = *this_.add(0x0f) as u32;
            let r1 = callee_cdecl!(2, u32, f, d);
            if *(arg1.add(0x244) as *const u32) != r1 {
                *this_.add(8) &= !0x10;
                *(this_.add(0x14) as *mut u32) = 0xffffffff;
                *this_.add(0x0f) = 0;
                *(this_.add(0x0d) as *mut u16) = 0;
            } else {
                let r2 = callee_cdecl!(3, u32, f, d, e);
                if *(arg1.add(0x248) as *const u32) != r2 {
                    *this_.add(8) &= !0x10;
                    *(this_.add(0x14) as *mut u32) = 0xffffffff;
                    *this_.add(0x0f) = 0;
                    *(this_.add(0x0d) as *mut u16) = 0;
                }
            }
        } else {
            *this_.add(0x0f) = 0;
            *(this_.add(0x0d) as *mut u16) = 0;
        }
        if *global::<u32>(0x011D6FD4) == 2 {
            let tag = *(this_.add(0x0a) as *const u16) as u32;
            if tag == *global::<u32>(0x012FA4F4) || tag == *global::<u32>(0x012FA404) {
                *this_.add(4) |= 4;
            }
        }
        if (*this_.add(4) & 4) != 0
            && *(this_.add(0x14) as *const u32) == 0xffffffff
        {
            let p = *(arg1.add(0x284) as *const u32);
            // The final gate compares a read-only constant that is -1,
            // so the load below never runs; it is kept for fidelity.
            if p != 0 && *global::<i32>(0x01050B90) != -1 {
                *(this_.add(0x14) as *mut u32) = *((p + 0x64) as *const u32);
            }
        }
        *this_.add(8) &= !8;
        *this_.add(0x0c) = 0;
        let vt = *(arg1 as *const u32);
        let slot = ((vt + 0xa0) as *const u32).read();
        let vcall1: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let mut done = false;
        if vcall1(arg1 as u32) != 0 {
            let b = vcall1(arg1 as u32);
            if callee_thiscall!(4, u32, b) != 0 {
                *this_.add(8) |= 8;
                let c = vcall1(arg1 as u32);
                let mut bl = 0u8;
                let o = *((c + 0x64) as *const u32);
                if o != 0 {
                    let cc = *((o + 0x160) as *const u32);
                    if cc != 0 {
                        let ee = *((o + 0x168) as *const u32);
                        let mut ebx = *((cc + 0x10) as *const u32);
                        if ee != 0 {
                            ebx = ebx.wrapping_add(*((ee + 0x10) as *const u32));
                        }
                        bl = ebx as u8;
                    }
                }
                *this_.add(0x0c) = bl;
                done = true;
            }
        }
        if !done {
            if (*this_.add(4) & 4) != 0 {
                let fetch = |vcall1: extern "thiscall" fn(u32) -> u32| -> u32 {
                    let a = vcall1(arg1 as u32);
                    if a == 0 {
                        *(arg1.add(0x100) as *const u32)
                    } else {
                        let b = vcall1(arg1 as u32);
                        let vt2 = *(b as *const u32);
                        let s2 = ((vt2 + 0xe0) as *const u32).read();
                        let vcall2: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(s2 as usize);
                        vcall2(b)
                    }
                };
                let e1 = fetch(vcall1);
                if e1 != 0 {
                    let e2 = fetch(vcall1);
                    if *((e2 + 0x10) as *const i32) > 0 {
                        let n = callee_thiscall!(5, u32, arg1 as u32);
                        *this_.add(0x0c) = *((n + 0x10) as *const u8);
                    }
                }
            }
        }
        let mut al3 = (((e24 >> 8) as u8).wrapping_shl(2) ^ *this_.add(8)) & 4;
        *this_.add(8) ^= al3;
        callee_thiscall!(6, u32, this_ as u32, *(arg1.add(0x20) as *const u32))
    }
});
