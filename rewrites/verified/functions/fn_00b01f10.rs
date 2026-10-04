// original: 0x00b01f10 heap_sort
/// Heapsort over 8-byte records keyed by the f32 at offset 0. Heapifies
/// through the wrapper, copies each out-of-place record down and sifts it,
/// then extracts from the back. Five stack words: base, cursor, range
/// limit, one unread slot, and a value forwarded to all three callees.
/// (The limit/forward loads run BEFORE the call-argument cleanup, so they
/// read earlier slots than a post-cleanup reading suggests.) Returns the
/// final extract span rounded down to 8.
/// Note: the inventory lists size 154 but the epilogue (5 bytes) sits just
/// past it; true size is 159.
export!(cdecl, rw_00b01f10(p0: u32, p1: u32, limit: u32, a3: u32, fwd: u32) -> u32 {
    let _ = a3;
    let _: u32 = callee_cdecl!(1, u32, p0, p1, fwd);
    let mut edi = p1;
    if p1 < limit {
        // unsigned range walk; the post-call limit reload reads this same slot
        loop {
            let pv = unsafe { (p0 as *const f32).read_unaligned() };
            let ev = unsafe { (edi as *const f32).read_unaligned() };
            if pv > ev {
                // comiss+jbe: copy down unless not-greater (NaN-safe)
                unsafe {
                    let w = (p0 as *const u64).read_unaligned();
                    let old = (edi as *const u64).read_unaligned();
                    (edi as *mut u64).write_unaligned(w);
                    let cnt = ((p1.wrapping_sub(p0) as i32) >> 3) as u32;
                    let _: u32 = callee_cdecl!(
                        2,
                        u32,
                        p0,
                        0,
                        cnt,
                        old as u32,
                        (old >> 32) as u32,
                        fwd
                    );
                }
            }
            edi = edi.wrapping_add(8);
            if !(edi < limit) {
                break;
            }
        }
    }
    let mut bx = p1;
    let mut di = bx.wrapping_sub(p0);
    if ((di & !7) as i32) > 8 {
        loop {
            let _: u32 = callee_cdecl!(3, u32, p0, bx, fwd);
            di = di.wrapping_sub(8);
            bx = bx.wrapping_sub(8);
            if !(((di & !7) as i32) > 8) {
                break;
            }
        }
    }
    di & !7
});
