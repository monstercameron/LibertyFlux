// original: 0x005FD620 streaming_poll_active_items

/// Poll the current streaming context and each active item context.
///
/// The receiver points to the current context at `+0xb4`, an array of item
/// pointers at `+0xd4`, and a byte item count at `+0x1f3`. If the current
/// context's float at `+0x70` is ordered-greater than the relocated engine
/// limit, the helper is called once and the context's virtual slot `+0x40` is
/// invoked with zero. Otherwise, each item's contexts at `+0x90` and `+0x94`
/// are scanned for a nonzero word in their four-word block at `+0x40`; active
/// contexts receive the same helper and virtual call. The return is the OR of
/// the virtual calls' EAX values, with one call result returned directly on
/// the greater-than path. The function uses thiscall and has no stack args.
lf_checker_rt::export!(thiscall, rw_005fd620(this: u32) -> u32 {
    unsafe {
        const CURRENT_CONTEXT: u32 = 0xb4;
        const ITEM_ARRAY: u32 = 0xd4;
        const ITEM_COUNT: u32 = 0x1f3;
        const CONTEXT_VALUE: u32 = 0x70;
        const ITEM_CONTEXTS: [u32; 2] = [0x90, 0x94];
        const ACTIVE_WORDS: u32 = 0x40;
        const ACTIVE_WORD_COUNT: u32 = 4;
        const CONTEXT_VTABLE: u32 = 0;
        const POLL_SLOT: u32 = 0x40;
        const LIMIT_FILE_VA: u32 = 0x00fe8628;
        const DIRECT_HELPER: u32 = 1;

        let current = (this.wrapping_add(CURRENT_CONTEXT) as *const u32).read_unaligned();
        let current_bits = (current.wrapping_add(CONTEXT_VALUE) as *const u32).read_unaligned();
        let current_value = f32::from_bits(current_bits);
        let limit = lf_checker_rt::global::<f32>(LIMIT_FILE_VA).read_unaligned();

        if current_value > limit {
            let helper_arg = (current.wrapping_add(8) as *const u32).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(
                DIRECT_HELPER,
                u32,
                current.wrapping_add(0x10),
                helper_arg,
            );
            let table = (current.wrapping_add(CONTEXT_VTABLE) as *const u32).read_unaligned();
            let target = (table.wrapping_add(POLL_SLOT) as *const u32).read_unaligned();
            let poll: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            return poll(current, 0);
        }

        let count = (this.wrapping_add(ITEM_COUNT) as *const u8).read() as u32;
        let items = (this.wrapping_add(ITEM_ARRAY) as *const u32).read_unaligned();
        let mut combined_status = 0u32;
        for item_index in 0..count {
            let item_address = (items.wrapping_add(item_index.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            for context_offset in ITEM_CONTEXTS {
                let context = (item_address.wrapping_add(context_offset) as *const u32)
                    .read_unaligned();
                if context == 0 {
                    continue;
                }
                let mut active = false;
                for word_index in 0..ACTIVE_WORD_COUNT {
                    let word = (context
                        .wrapping_add(ACTIVE_WORDS)
                        .wrapping_add(word_index.wrapping_mul(4)) as *const u32)
                        .read_unaligned();
                    if word != 0 {
                        active = true;
                        break;
                    }
                }
                if !active {
                    continue;
                }

                let helper_arg = (context.wrapping_add(8) as *const u32).read_unaligned();
                let _ = lf_checker_rt::callee_thiscall!(
                    DIRECT_HELPER,
                    u32,
                    context.wrapping_add(0x10),
                    helper_arg,
                );
                let table = (context.wrapping_add(CONTEXT_VTABLE) as *const u32).read_unaligned();
                let target = (table.wrapping_add(POLL_SLOT) as *const u32).read_unaligned();
                let poll: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                combined_status |= poll(context, 0);
            }
        }
        combined_status
    }
});
