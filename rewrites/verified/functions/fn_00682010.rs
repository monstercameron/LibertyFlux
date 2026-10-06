// original: 0x00682010 file_open_dispatch

/// Open a file through the global file manager and dispatch on its type tag.
///
/// `this` is the requester object (only written at `+0x1c` on path B); `a0`
/// is an identifier passed to the open and resolve helpers (the other three
/// stack words are unread). The manager at file VA 0x110C0A0 opens the file
/// with flags `0xFA00E4`; a null handle returns 0 at once.
///
/// Otherwise a probe helper writes one tag word through a stack out-slot and
/// the tag steers three paths, compared as SIGNED 32-bit (`jg` after the
/// compare, so negative tags sort below every tag):
/// * tag `0x31494E41`: allocate a 0x30-byte record from the calling thread's
///   allocator (thread-local state at TLS slot 0, object links at `+8`/`+10`
///   with a save slot at `+0x64` and a nest count at `+0x68`), wrap it,
///   resolve `a0` into its `+0x1c` link, run two helpers over it, then
///   release it through virtual slot 0. The status is the wrap helper's
///   low answer byte. A null allocation faults on the `+0x1c` store, on both
///   sides alike.
/// * tag `0x38494E41`: resolve `a0` into the requester's `+0x1c` link and run
///   the finish helper over the handle. The status is its low answer byte.
///   The helper's middle argument is whatever the resolver left in ECX, which
///   no rewrite can observe, so the contract skips it.
/// * any other tag (including `0x35494E41`): no work; the status is 0.
///
/// The tail releases the handle: a cleanup helper runs when the `+0x14`
/// counter is zero but the `+0x10` counter is not, then virtual slot 0x2c closes it
/// and the handle links are cleared. The full return is the closer's answer
/// with its low byte replaced by the status. The original spills the status
/// into its own incoming `a0` slot; the rewrite keeps it in a local, so the
/// contract switches the stack check off (the value is observed in the
/// low return byte instead).
///
/// Original: 0x00682010 (thiscall, four stack words; callee ids 1-12).
lf_checker_rt::export!(thiscall, rw_00682010(this: u32, a0: u32, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const FILE_MGR_VA: u32 = 0x110C0A0;
        const OPEN_FLAGS_VA: u32 = 0x0FA00E4;
        const TAG_ALLOC: u32 = 0x3149_4E41;
        const TAG_PLAIN: u32 = 0x3549_4E41;
        const TAG_DIRECT: u32 = 0x3849_4E41;
        const RECORD_SIZE: u32 = 0x30;
        const RECORD_ALIGN: u32 = 0x10;
        const LINK_OFF: u32 = 0x1c;
        const TLS_CUR: u32 = 8;
        const TLS_NEXT: u32 = 0x10;
        const TLS_SAVED: u32 = 0x64;
        const TLS_NEST: u32 = 0x68;
        const HANDLE_COUNT_A: u32 = 0x14;
        const HANDLE_COUNT_B: u32 = 0x10;
        const VT_RELEASE: u32 = 0x2c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// The thread-allocator lock dance shared by both critical sections.
        #[inline(always)]
        unsafe fn tls_enter(tls: u32) {
            unsafe {
                let cur = rd32(tls + TLS_CUR);
                let nxt = rd32(tls + TLS_NEXT);
                if cur == nxt {
                    wr32(tls + TLS_NEST, rd32(tls + TLS_NEST).wrapping_add(1));
                } else {
                    wr32(tls + TLS_SAVED, cur);
                    wr32(tls + TLS_CUR, nxt);
                }
            }
        }

        let handle: u32 = lf_checker_rt::callee_thiscall!(
            1, u32,
            lf_checker_rt::relocated(FILE_MGR_VA),
            a0,
            lf_checker_rt::relocated(OPEN_FLAGS_VA),
            0,
            1
        );
        if handle == 0 {
            return 0;
        }
        let mut tag: u32 = 0;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            2, u32,
            handle,
            &mut tag as *mut u32 as u32,
            4
        );
        let mut status: u8 = 0;
        if (tag as i32) > (TAG_ALLOC as i32) {
            if tag != TAG_PLAIN && tag == TAG_DIRECT {
                let resolved: u32 = lf_checker_rt::callee_stdcall!(5, u32, a0);
                wr32(this + LINK_OFF, resolved);
                let done: u32 =
                    lf_checker_rt::callee_thiscall!(10, u32, this, handle, 0, 0);
                status = done as u8;
            }
        } else if tag == TAG_ALLOC {
            let tls = lf_checker_rt::tls_slot(0);
            tls_enter(tls);
            let alloc_obj = rd32(tls + TLS_CUR);
            let alloc_fn: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(alloc_obj) + 8) as usize);
            let record = alloc_fn(alloc_obj, RECORD_SIZE, RECORD_ALIGN, 0);
            let wrapped = if record == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(4, u32, record)
            };
            let resolved: u32 = lf_checker_rt::callee_stdcall!(5, u32, a0);
            wr32(wrapped + LINK_OFF, resolved);
            let done: u32 = lf_checker_rt::callee_thiscall!(6, u32, wrapped, handle);
            status = done as u8;
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, this, wrapped);
            tls_enter(tls);
            let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(wrapped)) as usize);
            let _: u32 = free_fn(wrapped, 1);
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        }
        if rd32(handle + HANDLE_COUNT_A) == 0 && rd32(handle + HANDLE_COUNT_B) != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, handle);
        }
        let close_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(rd32(handle)) + VT_RELEASE) as usize);
        let close_ans: u32 = close_fn(rd32(handle), rd32(handle + 4));
        wr32(handle + 4, 0xffff_ffff);
        wr32(handle, 0);
        (close_ans & 0xffff_ff00) | (status as u32)
    }
});
