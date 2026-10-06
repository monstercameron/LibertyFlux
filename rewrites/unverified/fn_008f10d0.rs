// original: 0x008f10d0 bind_control_slot (proposed)

/// Bind one control slot in a binding group, if its current entry allows it.
///
/// `this` is the controls manager; `group` (0-5, UNSIGNED compare: values
/// above 5 return it unchanged) picks a binding sub-object (`+0x0`, `+0xf70`,
/// `+0x7b8`, `+0x1ee0`, `+0x1728`, `+0x32ac`). Up to six search rounds look
/// through that group's pointer array (`+0x528`) for the 16-byte slot
/// `this+0x2698+16*slot`, where `slot` is `a3`. The round counter lives in
/// the caller's `a2` argument slot in the original.
///
/// A found index is accepted only when the kind array (`+0x8`) holds `want`
/// (`a1`) there and, when the low byte of `a8` is set, the masked value
/// `a2` equals the masked flag word (`+0x298`, low 24 bits): no masking when
/// the low byte of `a6` is clear, the low byte kept when `a7`'s is also set,
/// else the second byte kept. A rejected index resumes the scan just past it;
/// a scan that finds nothing restarts from zero. Six fruitless rounds return
/// 0 after a miss, or one past the last rejected index.
///
/// On accept, the setter callee runs with (index, shape, word, entry, -1):
/// `shape` is 0xe with `word` mixing `a5` and the flag bytes (low byte of
/// `a5` over the flag's second byte, or the reverse) in the masked modes, or
/// (`a4`, `a5`) unmasked. Its answer is returned. The round counter in the
/// `a2` slot is purely internal: it is never passed on or returned, so the
/// stack comparison is off for this function (see the contract).
/// Note the group sub-objects are NOT in address order: group 3 selects
/// `+0x1ee0`, group 4 selects `+0x1728` and group 5 selects `+0x32ac`.
/// Original: 0x008f10d0 (thiscall, nine stack words).
lf_checker_rt::export!(
    thiscall,
    rw_008f10d0(
        this: u32,
        group: u32,
        want: u32,
        value: u32,
        slot: u32,
        aux: u32,
        mix: u32,
        m6: u32,
        m7: u32,
        gate: u32
    ) -> u32 {
        unsafe {
            const SUBS: [u32; 6] = [0x0, 0xf70, 0x7b8, 0x1ee0, 0x1728, 0x32ac];
            const SLOTS: u32 = 0x2698;
            const ROUNDS: u32 = 6;
            const SHAPE: u32 = 0xe;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }

            if group > 5 {
                return group;
            }
            let sub = this.wrapping_add(SUBS[group as usize]);
            if sub == 0 {
                return group;
            }
            let g6 = (m6 as u8) != 0;
            let g7 = (m7 as u8) != 0;
            let masked_value = if !g6 {
                value
            } else if g7 {
                value & 0xff
            } else {
                value & 0xff00
            };
            let slot_addr =
                this.wrapping_add(SLOTS).wrapping_add(slot.wrapping_mul(16));
            let mut counter: u32 = 0;
            let mut eax: i32 = 0;
            loop {
                let count = rd32(sub) as i32;
                // Inner scan from eax for the slot address.
                let mut j = eax;
                let mut found = false;
                while j < count {
                    let p = rd32(
                        sub.wrapping_add(0x528)
                            .wrapping_add((j as u32).wrapping_mul(4)),
                    );
                    if p == slot_addr {
                        found = true;
                        break;
                    }
                    j = j.wrapping_add(1);
                }
                if !found {
                    counter = counter.wrapping_add(1);
                    eax = 0;
                    if counter >= ROUNDS {
                        return 0;
                    }
                    continue;
                }
                let idx = j;
                if idx < 0 {
                    counter = counter.wrapping_add(1);
                    eax = idx.wrapping_add(1);
                    if counter >= ROUNDS {
                        return eax as u32;
                    }
                    continue;
                }
                let flags = rd32(
                    sub.wrapping_add(0x298)
                        .wrapping_add((idx as u32).wrapping_mul(4)),
                ) & 0xffffff;
                let masked_flags = if !g6 {
                    flags
                } else if g7 {
                    flags & 0xff
                } else {
                    flags & 0xff00
                };
                let kind = rd32(
                    sub.wrapping_add(0x8)
                        .wrapping_add((idx as u32).wrapping_mul(4)),
                );
                let gated = (gate as u8) != 0 && masked_value != masked_flags;
                if want != kind || gated {
                    counter = counter.wrapping_add(1);
                    eax = idx.wrapping_add(1);
                    if counter >= ROUNDS {
                        return eax as u32;
                    }
                    continue;
                }
                let entry = rd32(
                    sub.wrapping_add(0x528)
                        .wrapping_add((idx as u32).wrapping_mul(4)),
                );
                if g6 {
                    let word = if g7 {
                        (flags & 0xff00) | (mix & 0xff)
                    } else {
                        (mix & 0xff00) | (flags & 0xff)
                    };
                    return lf_checker_rt::callee_thiscall!(
                        0,
                        u32,
                        sub,
                        idx as u32,
                        SHAPE,
                        word,
                        entry,
                        0xffffffff
                    );
                }
                return lf_checker_rt::callee_thiscall!(
                    0,
                    u32,
                    sub,
                    idx as u32,
                    aux,
                    mix,
                    entry,
                    0xffffffff
                );
            }
        }
    }
);
