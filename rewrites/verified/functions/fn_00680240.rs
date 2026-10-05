// original: 0x00680240 euphoria_cleanup_lists (proposed)

/// Release this object's two pointer lists, its spare block, and clear it.
///
/// `this` owns list A (array at `+0x20`, 16-bit count at `+0x24`), list B
/// (array at `+0x14`, count at `+0x18`) and a spare block at `+0x1c`. Bit 0
/// of the byte at `+7` selects the disposal shape for both lists. Every
/// block is released through the allocator reached through TLS slot 0:
/// `A = tls[0]; C = [A+8]; V = [C]` (the original reads the TLS array at
/// FS:0x2c and takes slot 0), called as `V+0xc` with the block on
/// the stack (callee pops one word); the rewrite performs the same
/// load-and-call through the same fabricated chain, so both sides land on
/// the same planted stub. The TLS value is trial-constant, so it is read
/// once; the original re-reads the slot and observes the same value.
///
/// List A, per element: with the flag set the element's sub-block at `+8`
/// is freed first (a null element faults here, on both sides alike) and
/// `+8`/`+0xc` are cleared; then a null element is skipped, otherwise the
/// direct helper (callee id 2) disposes it, a nonzero word at `+0xe` with a
/// live `+8` frees that block too, and the element itself is freed. Every
/// visited array slot is cleared, then the array, and `+0x20`/`+0x24` are
/// cleared (`+0x24` as a full word). List B, per element: with the flag set
/// a live slot is freed directly (null-safe); otherwise a null element is
/// skipped, a live block at `+0` is freed, `+0`/`+4` are cleared and the
/// element is freed. Slots, array, `+0x14`/`+0x18` (word-cleared) follow,
/// then the spare block, the words at `+6` and `+0x28`, and the dword at
/// `+0x10`. Returns 0.
///
/// Original: 0x00680240 (thiscall, ECX only, callee pops 0, returns EAX).
lf_checker_rt::export!(thiscall, rw_00680240(ecx: u32) -> u32 {
    unsafe {
        const LISTA_ARRAY: u32 = 0x20;
        const LISTA_COUNT: u32 = 0x24;
        const LISTB_ARRAY: u32 = 0x14;
        const LISTB_COUNT: u32 = 0x18;
        const SPARE: u32 = 0x1c;
        const FLAG: u32 = 0x07;
        const VTABLE_FREE_SLOT: u32 = 0x0c;
        const DIRECT_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        /// Free `block` through the TLS allocator chain, like the original.
        #[inline(always)]
        unsafe fn dispose(tls: u32, block: u32) {
            unsafe {
                let alloc = tls;
                let ctx = rd32(alloc + 8);
                let table = rd32(ctx);
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(table + VTABLE_FREE_SLOT) as usize);
                free(ctx, block);
            }
        }

        let this = ecx;
        let tls = lf_checker_rt::tls_slot(0);
        let flagged = rd8(this + FLAG) & 1 != 0;

        let count_a = rd16(this + LISTA_COUNT);
        for i in 0..count_a {
            if flagged {
                let elem = rd32(rd32(this + LISTA_ARRAY) + i * 4);
                let sub = rd32(elem + 8);
                if sub != 0 {
                    dispose(tls, sub);
                }
                wr32(elem + 8, 0);
                wr32(elem + 0x0c, 0);
            }
            let elem = rd32(rd32(this + LISTA_ARRAY) + i * 4);
            if elem != 0 {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(DIRECT_CALLEE, u32, elem);
                if rd16(elem + 0x0e) != 0 {
                    let sub = rd32(elem + 8);
                    if sub != 0 {
                        dispose(tls, sub);
                    }
                }
                dispose(tls, elem);
            }
            wr32(rd32(this + LISTA_ARRAY) + i * 4, 0);
        }
        let arr_a = rd32(this + LISTA_ARRAY);
        if arr_a != 0 {
            dispose(tls, arr_a);
        }
        wr32(this + LISTA_ARRAY, 0);
        wr32(this + LISTA_COUNT, 0);

        let count_b = rd16(this + LISTB_COUNT);
        for j in 0..count_b {
            if flagged {
                let slot = rd32(rd32(this + LISTB_ARRAY) + j * 4);
                if slot != 0 {
                    dispose(tls, slot);
                }
            } else {
                let elem = rd32(rd32(this + LISTB_ARRAY) + j * 4);
                if elem != 0 {
                    let sub = rd32(elem);
                    if sub != 0 {
                        dispose(tls, sub);
                    }
                    wr32(elem + 4, 0);
                    wr32(elem, 0);
                    dispose(tls, elem);
                }
            }
            wr32(rd32(this + LISTB_ARRAY) + j * 4, 0);
        }
        let arr_b = rd32(this + LISTB_ARRAY);
        if arr_b != 0 {
            dispose(tls, arr_b);
        }
        wr32(this + LISTB_ARRAY, 0);
        wr32(this + LISTB_COUNT, 0);

        let spare = rd32(this + SPARE);
        if spare != 0 {
            dispose(tls, spare);
        }
        wr32(this + SPARE, 0);
        wr16(this + 6, 0);
        wr16(this + 0x28, 0);
        wr32(this + 0x10, 0);
        0
    }
});
