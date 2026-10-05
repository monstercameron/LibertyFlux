// original: 0x00a353d0 vehicle_event_create (proposed)

/// Allocate and fill a vehicle event node under the table lock.
///
/// Acquires the lock (id 1, thiscall/1) on a two-word scratch slot, takes
/// a node from the allocator (id 2, cdecl/0) -- a null node releases the
/// lock (ids 3 and 4) and answers null -- copies the two 3-word vectors
/// from `a` and `b` into `+0x4`/`+0x8`/`+0xc` and `+0x10`/`+0x14`/`+0x18`,
/// stores `e`/`f`/`g` into `+0x24`/`+0x1c`/`+0x20`, stamps the global head
/// word into `+0`, merges `(h << 2) | ((d & 1) << 1) | (c & 1)` into the
/// low 10 bits of `+0x28`, runs the fixup (id 5, cdecl/0), releases the
/// lock, notifies (id 6, cdecl/1) with the notify global, releases again,
/// and answers the head word. Cdecl/8, returns EAX.
lf_checker_rt::export!(cdecl, rw_00a353d0(
    a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32,
) -> u32 {
    unsafe {
        const LOCK_ARG: u32 = 0x012D_FD80;
        const HEAD_GLOBAL: u32 = 0x0103_CB10;
        const NOTIFY_GLOBAL: u32 = 0x012D_FD78;
        const ACQUIRE: u32 = 1;
        const ALLOC: u32 = 2;
        const UNLOCK_A: u32 = 3;
        const UNLOCK_B: u32 = 4;
        const FIXUP: u32 = 5;
        const NOTIFY: u32 = 6;
        let mut slot = [0u32; 2];
        let p = core::ptr::addr_of_mut!(slot) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(ACQUIRE, u32, p, lf_checker_rt::relocated(LOCK_ARG));
        let node: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32,);
        if node == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_A, u32, p);
            let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_B, u32, p);
            return 0;
        }
        for (i, off) in [0u32, 4, 8].iter().enumerate() {
            let v = core::ptr::read_unaligned((a + off) as *const u32);
            core::ptr::write_unaligned((node + 4 + i as u32 * 4) as *mut u32, v);
        }
        for (i, off) in [0u32, 4, 8].iter().enumerate() {
            let v = core::ptr::read_unaligned((b + off) as *const u32);
            core::ptr::write_unaligned((node + 0x10 + i as u32 * 4) as *mut u32, v);
        }
        core::ptr::write_unaligned((node + 0x24) as *mut u32, e);
        let head = core::ptr::read(lf_checker_rt::global::<u32>(HEAD_GLOBAL));
        core::ptr::write_unaligned(node as *mut u32, head);
        core::ptr::write_unaligned((node + 0x1C) as *mut u32, f);
        core::ptr::write_unaligned((node + 0x20) as *mut u32, g);
        let bits = ((h & 0xFF) << 2) | (((d & 1) << 1) | (c & 1));
        let w = core::ptr::read_unaligned((node + 0x28) as *const u32);
        core::ptr::write_unaligned((node + 0x28) as *mut u32, (w & 0xFFFF_FC00) | (bits & 0x3FF));
        let _: u32 = lf_checker_rt::callee_cdecl!(FIXUP, u32,);
        let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_A, u32, p);
        let n = core::ptr::read(lf_checker_rt::global::<u32>(NOTIFY_GLOBAL));
        let _: u32 = lf_checker_rt::callee_cdecl!(NOTIFY, u32, n);
        let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_B, u32, p);
        core::ptr::read_unaligned(node as *const u32)
    }
});
