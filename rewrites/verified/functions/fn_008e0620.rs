// original: 0x008e0620 release_under_tls_guard

/// Release slot `key`'s object under the thread-local pool guard: resolve the
/// entry (dead slots fault on a null header read, exactly like the lookup),
/// bump the current thread's guard counter, drop the object's reference, run
/// its virtual release when the count hits zero, then release the guard and
/// clear the entry.
///
/// The guard object is found through the worker thread-local area at the slot
/// index held in the fixed selector global; the guard word sits at +0x14.
///
/// Original: cdecl (key); no return value.
lf_checker_rt::export!(cdecl, rw_008e0620(key: u32) -> () {
    unsafe {
        let ctx = *lf_checker_rt::global::<u32>(0x11764C0) as *mut u32;
        let base = *ctx as *mut u32;
        let flags = *ctx.add(1) as *const u8;
        let flag = *flags.add(key as usize);
        let entry = if flag & 0x80 == 0 {
            let stride = *ctx.add(3);
            base.byte_add((key.wrapping_mul(stride)) as usize) as *mut u32
        } else {
            core::hint::black_box(core::ptr::null_mut())
        };
        let slot = *lf_checker_rt::global::<u32>(0x17ABA14);
        let tlsobj = lf_checker_rt::tls_slot(slot as usize) as *mut u32;
        *tlsobj.add(5) = (*tlsobj.add(5)).wrapping_add(1);
        let obj = *entry as *mut u32;
        let refs = obj.add(3);
        *refs = (*refs).wrapping_sub(1);
        if *refs == 0 {
            lf_checker_rt::callee_thiscall!(1, u32, obj as u32, 1u32);
        }
        *tlsobj.add(5) = (*tlsobj.add(5)).wrapping_sub(1);
        *entry = 0;
    }
});
