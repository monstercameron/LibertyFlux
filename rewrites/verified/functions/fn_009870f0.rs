// original: 0x009870F0 audEmitterAudioEntity::vf2

/// Emitter audio entity virtual slot 2: tears down every live slot, frees the
/// shared buffer, then tail-calls the base slot.
///
/// Clears the flag byte at `+DONE_OFF` of `this`, then walks the slot count
/// (UNSIGNED) at `+COUNT_OFF`, slots starting at `+SLOT0_OFF` and spaced
/// `SLOT_STRIDE` bytes apart. A slot with its byte at `+ACTIVE_OFF` (relative
/// to the slot cursor) clear is skipped; otherwise its dword at `-LINK_ARG_OFF`
/// (when nonzero) is released with argument 0 (callee id 1), its head dword
/// (when nonzero) is destroyed (callee id 2) and cleared, and its helper
/// object at `-HELPER_OFF` runs virtual slot 3 (indirect callee id 4): a
/// nonzero answer is destroyed too (callee id 2 again) and the object runs
/// virtual slot 4 with argument 0 (indirect callee id 5). Afterwards, when
/// the byte at `+KEEP_OFF` is clear, the buffer word at `+BUFFER_OFF` is
/// handed to the free callee (id 3, cdecl) and cleared. Finally the base slot
/// (id 6) is tail-called with `this`, whose answer is returned.
/// Original: thiscall, no stack words, ends in a jump.
lf_checker_rt::export!(thiscall, rw_009870F0(this: u32) -> u32 {
    const DONE_OFF: u32 = 0x4a550;
    const COUNT_OFF: u32 = 0x8230;
    const SLOT0_OFF: u32 = 0xe0;
    const SLOT_STRIDE: u32 = 0xd0;
    const ACTIVE_OFF: u32 = 0x15;
    const LINK_ARG_OFF: u32 = 0x18;
    const HELPER_OFF: u32 = 0xa0;
    const VT_SLOT3: u32 = 0x0c;
    const VT_SLOT4: u32 = 0x10;
    const KEEP_OFF: u32 = 0x4a552;
    const BUFFER_OFF: u32 = 0x38240;
    const RELEASE_ARG: u32 = 1;
    const DESTROY: u32 = 2;
    const FREE_BUF: u32 = 3;
    const HELPER_VF3: u32 = 4;
    const HELPER_VF4: u32 = 5;
    const TAIL_BASE: u32 = 6;
    unsafe {
        ((this + DONE_OFF) as *mut u8).write(0);
        let count = ((this + COUNT_OFF) as *const u32).read_unaligned();
        if count > 0 {
            let mut i = 0u32;
            while i < count {
                let cur = this
                    .wrapping_add(SLOT0_OFF)
                    .wrapping_add(i.wrapping_mul(SLOT_STRIDE));
                if ((cur + ACTIVE_OFF) as *const u8).read() != 0 {
                    let link =
                        (cur.wrapping_sub(LINK_ARG_OFF) as *const u32)
                            .read_unaligned();
                    if link != 0 {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(RELEASE_ARG, u32, link, 0);
                    }
                    let head = (cur as *const u32).read_unaligned();
                    if head != 0 {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(DESTROY, u32, head);
                        (cur as *mut u32).write_unaligned(0);
                    }
                    let obj = cur.wrapping_sub(HELPER_OFF);
                    let vt = (obj as *const u32).read_unaligned();
                    let vf3: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(
                            ((vt + VT_SLOT3) as *const u32).read_unaligned()
                                as usize,
                        );
                    let r = vf3(obj);
                    if r != 0 {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(DESTROY, u32, r);
                        let vt2 = (obj as *const u32).read_unaligned();
                        let vf4: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(
                                ((vt2 + VT_SLOT4) as *const u32).read_unaligned()
                                    as usize,
                            );
                        vf4(obj, 0);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        if ((this + KEEP_OFF) as *const u8).read() == 0 {
            let old = ((this + BUFFER_OFF) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(FREE_BUF, u32, old);
            ((this + BUFFER_OFF) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(TAIL_BASE, u32, this)
    }
});
