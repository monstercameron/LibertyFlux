// original: 0x0097F4E0 WET_PED_IMPACT

/// Fire the wet-ped-impact audio event chain for a ped task object.
///
/// `this` is the task object; the seven stack arguments are a spare word,
/// an unread word, a ped object (or null), a float factor, an integer
/// class, a spare word and a flag word. After the shared audio-ready
/// guards the first float comes from the ped path (callee 1 arbitrates a
/// vehicle-bike collision and the clock is latched into `+0x14C`, the
/// float is the factor) or the timer path (the unsigned clock delta since
/// `+0x14C`, converted exactly through double, transformed by callee 2
/// and scaled by the factor).
///
/// A non-pedestrian ped then runs a cooldown gate (`+0x150` plus 0x7D0
/// against the clock) and a float gate (callee 3, callee 4 must answer
/// true): unless the flag word is zero the class must index the shared
/// word table to the table's second word, and callee 5 (the flag-indexed
/// event itself) fires. The class picks a pointer table (`+0x7C` under 7,
/// `+0x80` otherwise); a null entry ends the call unless the class is
/// 0xC. The float passes callee 6 and must beat a threshold; class 0xC
/// then resolves its word through a predicate plus a lazily hashed name
/// (defaulting to a gap word), other classes read it at the entry's
/// `+0xA`. A first post block (callees 9-15, descriptor 0x45) runs, the
/// float is scaled by `[this+0x144]` and must beat the threshold again,
/// and a second post block (callees 16-23, descriptor 0x46, name hashed
/// fresh) runs. Both threshold compares are exact `comiss`+`jbe`
/// semantics (unordered counts as below-or-equal). The two intermediate
/// floats live on the original's incoming argument slots, which a Rust
/// rewrite cannot address; locals stand in and every value is observed
/// through the call arguments that consume it.
///
/// Original: 0x0097F4E0 (thiscall, seven stack arguments, no return).
lf_checker_rt::export!(
    thiscall,
    rw_0097F4E0(
        this: u32,
        spare0: u32,
        _spare1: u32,
        ped: u32,
        factor: u32,
        class: u32,
        spare5: u32,
        flagw: u32,
    ) -> u32 {
        unsafe {
            const G_QUIT: u32 = 0x011F7060;
            const G_SESS_A: u32 = 0x012088B4;
            const G_SESS_B: u32 = 0x00F1C040;
            const G_MODE: u32 = 0x01037720;
            const G_CLOCK: u32 = 0x011735B4;
            const SKIP_MODE: u32 = 0x12;
            const G_TABLE_T: u32 = 0x01231618;
            const TABLE_A: u32 = 0x01231360;
            const TABLE_B: u32 = 0x01231320;
            const G_HASH_FLAG: u32 = 0x01231768;
            const G_HASH: u32 = 0x01231764;
            const G_DEFAULT_WORD: u32 = 0x017ACC5C;
            const NAME_C: u32 = 0x00E8CB58;
            const NAME_ARB2: u32 = 0x00E8CB7C;
            const NAME_EV2: u32 = 0x00E8CB9C;
            const THRESH: u32 = 0x00FE8670;
            const FLOAT_MGR_B: u32 = 0x012314F4;
            const FLOAT_MGR_C: u32 = 0x0123151C;
            const FLOAT_MGR_F: u32 = 0x012313A0;
            const KIND_MASK: u32 = 0x3C0;
            const KIND_PED: u32 = 0x80;
            const COOLDOWN: u32 = 0x7D0;
            const OFF_LAST: u32 = 0x14C;
            const OFF_COOL: u32 = 0x150;
            const OFF_LEVEL: u32 = 0x144;
            const OFF_SUB: u32 = 0x08;
            const OFF_PED: u32 = 0x120;
            const OFF_CLASS_A: u32 = 0x7C;
            const OFF_CLASS_B: u32 = 0x80;
            const PED_KIND: u32 = 0x28;
            const WORD_OFF: u32 = 0x0A;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn gget(va: u32) -> u32 {
                unsafe { lf_checker_rt::global::<u32>(va).read() }
            }
            #[inline(always)]
            fn mul(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) * core::hint::black_box(b)
            }
            #[inline(always)]
            unsafe fn thresh() -> f32 {
                unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(THRESH))) }
            }

            if gget(G_QUIT) == 1 {
                return 0;
            }
            if gget(G_SESS_A) != gget(G_SESS_B) {
                return 0;
            }
            if gget(G_MODE) == SKIP_MODE {
                return 0;
            }
            let clock = gget(G_CLOCK);
            let is_ped =
                ped != 0 && rd32(ped.wrapping_add(PED_KIND)) & KIND_MASK == KIND_PED;
            // First float (the original keeps it on an argument slot).
            let slot_x: u32;
            if is_ped {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    1, u32, this, spare0, ped, factor, class, spare5
                );
                (this.wrapping_add(OFF_LAST) as *mut u32).write_unaligned(clock);
                slot_x = factor;
            } else {
                let dt = clock.wrapping_sub(rd32(this.wrapping_add(OFF_LAST)));
                let as_f = (dt as f64) as f32;
                let f1: f32 = lf_checker_rt::callee_thiscall!(
                    2,
                    f32,
                    lf_checker_rt::relocated(FLOAT_MGR_B),
                    as_f.to_bits()
                );
                slot_x = mul(f1, f32::from_bits(factor)).to_bits();
            }
            if !is_ped {
                if clock > rd32(this.wrapping_add(OFF_COOL)).wrapping_add(COOLDOWN) {
                    let f2: f32 = lf_checker_rt::callee_thiscall!(
                        3,
                        f32,
                        lf_checker_rt::relocated(FLOAT_MGR_C),
                        slot_x
                    );
                    let ok: u32 = lf_checker_rt::callee_cdecl!(4, u32, f2.to_bits());
                    if (ok & 0xFF) != 0 {
                        let mut fire = flagw == 0;
                        if !fire {
                            let w = rd32(
                                lf_checker_rt::relocated(G_TABLE_T)
                                    .wrapping_add(class.wrapping_mul(4)),
                            );
                            fire = w == gget(G_TABLE_T.wrapping_add(4));
                        }
                        if fire {
                            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, class);
                        }
                    }
                }
            }
            let entry = if class < 7 {
                rd32(
                    lf_checker_rt::relocated(TABLE_A)
                        .wrapping_add(rd32(this.wrapping_add(OFF_CLASS_A)).wrapping_mul(4)),
                )
            } else {
                rd32(
                    lf_checker_rt::relocated(TABLE_B)
                        .wrapping_add(rd32(this.wrapping_add(OFF_CLASS_B)).wrapping_mul(4)),
                )
            };
            if entry == 0 && class != 0xC {
                return 0;
            }
            let f3: f32 = lf_checker_rt::callee_thiscall!(
                6,
                f32,
                lf_checker_rt::relocated(FLOAT_MGR_F),
                slot_x
            );
            // Second float (also on an argument slot in the original).
            let slot_y = f3.to_bits();
            if !(f3 > thresh()) {
                return 0;
            }
            let word = if class != 0xC {
                rd32(entry.wrapping_add(WORD_OFF))
            } else {
                let mut w = gget(G_DEFAULT_WORD);
                let ok: u32 = lf_checker_rt::callee_thiscall!(
                    7,
                    u32,
                    rd32(this.wrapping_add(OFF_PED))
                );
                if (ok & 0xFF) != 0 {
                    let flag = gget(G_HASH_FLAG);
                    if flag & 1 == 0 {
                        lf_checker_rt::global::<u32>(G_HASH_FLAG).write(flag | 1);
                        let h: u32 = lf_checker_rt::callee_cdecl!(
                            8,
                            u32,
                            lf_checker_rt::relocated(NAME_C),
                            0
                        );
                        lf_checker_rt::global::<u32>(G_HASH).write(h);
                        w = h;
                    } else {
                        w = gget(G_HASH);
                    }
                }
                w
            };
            // First post block.
            let mut buf = [0u32; 16];
            let buf_ptr = buf.as_mut_ptr() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, buf_ptr);
            let f4: f32 = lf_checker_rt::callee_cdecl!(10, f32, slot_y);
            buf[0] = f4.to_bits();
            buf[5] = spare0;
            buf[8] = rd32(this.wrapping_add(OFF_SUB));
            let handle: u32 = lf_checker_rt::callee_thiscall!(11, u32, buf_ptr);
            let param: u32 = lf_checker_rt::callee_cdecl!(12, u32, handle);
            let arb: u32 =
                lf_checker_rt::callee_thiscall!(13, u32, this, word, buf_ptr, handle, param, 0);
            if (arb & 0xFF) == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(15, u32, handle);
            } else {
                let mut desc = [0u32, 0xFFFF_FFFF, 0x45];
                let desc_ptr = desc.as_mut_ptr() as u32;
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    14,
                    u32,
                    word,
                    0,
                    0,
                    1,
                    buf_ptr,
                    desc_ptr,
                    rd32(this.wrapping_add(OFF_PED)),
                    handle
                );
            }
            // Third float reuses the second slot.
            let slot_y2 =
                mul(f32::from_bits(rd32(this.wrapping_add(OFF_LEVEL))), f3).to_bits();
            if !(f32::from_bits(slot_y2) > thresh()) {
                return 0;
            }
            // Second post block. The original reuses the same stack area,
            // so the buffer still holds the first block's words here.
            let _: u32 = lf_checker_rt::callee_thiscall!(16, u32, buf_ptr);
            let f5: f32 = lf_checker_rt::callee_cdecl!(17, f32, slot_y2);
            buf[0] = f5.to_bits();
            buf[5] = spare0;
            buf[8] = rd32(this.wrapping_add(OFF_SUB));
            let handle2: u32 = lf_checker_rt::callee_thiscall!(18, u32, buf_ptr);
            let param2: u32 = lf_checker_rt::callee_cdecl!(19, u32, handle2);
            let arb2: u32 = lf_checker_rt::callee_thiscall!(
                20,
                u32,
                this,
                lf_checker_rt::relocated(NAME_ARB2),
                buf_ptr,
                handle2,
                param2,
                0
            );
            if (arb2 & 0xFF) == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(23, u32, handle2);
                return 0;
            }
            let mut desc2 = [0u32, 0xFFFF_FFFF, 0x46];
            let desc2_ptr = desc2.as_mut_ptr() as u32;
            let hash: u32 = lf_checker_rt::callee_cdecl!(
                21,
                u32,
                lf_checker_rt::relocated(NAME_EV2),
                0
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(
                22,
                u32,
                hash,
                0,
                0,
                1,
                buf_ptr,
                desc2_ptr,
                rd32(this.wrapping_add(OFF_PED)),
                handle2
            );
            0
        }
    }
);
