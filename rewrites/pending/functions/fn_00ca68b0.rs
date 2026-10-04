// original: 0x00ca68b0 CEventHandler::vf65
/// Event reaction 65: resolve the owner's current task record, derive a
/// behaviour flavour from the owner's stance word, and commit it.
///
/// The record comes from the task slot, either straight from the polled
/// object or by searching its kind chain for kind 0x2de. The stance field
/// (bits 14:10, -1 meaning "keep the polled value") picks between a
/// plain commit and a remapped flavour that also consults a global toggle.
/// The committed handle (or zero when no manager answers) lands at
/// owner+0xc.
export!(thiscall, rw_rb02_vf65(this_ptr: u32, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        let p1 = *((this_ptr.wrapping_add(4)) as *const u32);
        let a1: u32 = callee_thiscall!(1, u32, p1);
        if a1 == 0 {
            return 0;
        }
        let ebx = a1.wrapping_add(8);
        let ans2: u32 = callee_thiscall!(2, u32, ebx);
        if ans2 == 0 {
            return 0;
        }
        if ans2 == p1 {
            return p1;
        }
        let segslot = *((p1.wrapping_add(0x224)) as *const u32);
        if *((segslot.wrapping_add(0x50)) as *const u32) != 0 {
            return ans2;
        }
        // Resolve the task record: attached block or kind-chain search.
        let edi: u32;
        let via_chain: bool;
        if (*((ans2.wrapping_add(0x26c)) as *const u8) & 4) != 0 {
            let attached = *((ans2.wrapping_add(0xb30)) as *const u32);
            if attached != 0 {
                edi = attached;
                via_chain = false;
            } else {
                edi = 0;
                via_chain = true;
            }
        } else {
            edi = 0;
            via_chain = true;
        }
        let mut edi = edi;
        if via_chain {
            let s = *((ans2.wrapping_add(0x224)) as *const u32);
            let mut node = *((s.wrapping_add(0x2e0)) as *const u32);
            if node == 0 {
                return ans2;
            }
            let mut tag0 = (*((node.wrapping_add(8)) as *const u32) >> 1) & 7;
            let mut last_tag = tag0;
            loop {
                let tag = (*((node.wrapping_add(8)) as *const u32) >> 1) & 7;
                last_tag = tag;
                if tag0 < tag {
                    return tag;
                }
                tag0 = tag;
                if *((node.wrapping_add(4)) as *const u32) == 0x2de {
                    break;
                }
                node = *((node.wrapping_add(12)) as *const u32);
                if node == 0 {
                    return last_tag;
                }
            }
            let found: u32 = callee_thiscall!(3, u32, s.wrapping_add(0x2e0), 0x2deu32, 5u32);
            if found == 0 {
                return 0;
            }
            edi = found;
        }
        if (*((p1.wrapping_add(0x26c)) as *const u8) & 4) != 0 {
            let cur = *((p1.wrapping_add(0xb30)) as *const u32);
            if cur == edi {
                return if via_chain { edi } else { ans2 };
            }
        }
        let chk4: u32 = callee_thiscall!(4, u32, segslot.wrapping_add(0x44), 0x2deu32);
        if chk4 != 0 {
            return chk4;
        }
        let mode = *((edi.wrapping_add(0x1304)) as *const u32);
        if mode == 4 {
            let ok5: u32 = callee_thiscall!(5, u32, edi);
            if ok5 == 0 {
                return 0;
            }
        }
        let ans6: u32 = callee_thiscall!(6, u32, ebx, p1);
        let ans7: u32 = callee_thiscall!(7, u32, ebx);
        // Stance field: bits 14:10 of the word, sign-extended; the
        // all-set field reads as -1 and keeps the polled value.
        let raw = *((p1.wrapping_add(0x264)) as *const u32);
        let stance = ((((raw << 17) as i32) >> 27) as u32);
        let orig_flavour: u32 = if stance == 0xffffffff { ans6 } else { stance };
        let mut flavour = orig_flavour;
        let limit = *((edi.wrapping_add(0x1070)) as *const u8) as u32;
        let bus = *global::<u32>(0x167e2a0);
        // Plain commit when the flavour reaches the record's limit, or
        // when the remap below explicitly falls through with -1.
        let mut plain = (flavour as i32) >= (limit as i32);
        if !plain {
            if mode == 3 {
                flavour = 0xfffffffa;
            } else if orig_flavour == 0 {
                flavour = if ans7 == 1 { 0xfffffffc } else { 2 };
            } else if orig_flavour != 1 {
                flavour = if orig_flavour == 2 { 3 } else { 0xffffffff };
            }
            let toggle = *global::<u8>(0x171bbd0);
            let ecx8 = if mode != 3 && orig_flavour == 0 {
                0xfffffffc
            } else {
                stance
            };
            let sw: u32 = callee_thiscall!(8, u32, ecx8);
            if (sw & 0xff) != 0 || toggle != 0 {
                flavour = 0xfffffffc;
            } else if flavour == 0xffffffff {
                plain = true;
            }
        }
        if plain {
            if mode != 2 {
                return ans7;
            }
            let mgr: u32 = callee_thiscall!(9, u32, bus);
            if mgr == 0 {
                *((this_ptr.wrapping_add(0xc)) as *mut u32) = 0;
                return 0;
            }
            let ans: u32 = callee_thiscall!(10, u32, mgr, edi, 0xfffffff9u32, 0xbu32, 0u32, 0u32);
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = ans;
            return ans;
        }
        let mgr2: u32 = callee_thiscall!(9, u32, bus);
        if mgr2 == 0 {
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 = callee_thiscall!(10, u32, mgr2, edi, flavour, 0x1bu32, 0x200000u32, 0u32);
        *((this_ptr.wrapping_add(0xc)) as *mut u32) = ans;
        ans
    }
});
