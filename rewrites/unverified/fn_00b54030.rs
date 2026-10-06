// original: 0x00B54030 rage::crFrameBuffer::vf0

/// Destroy the frame buffer, releasing its resource and maybe itself.
///
/// Stamps the table address, runs the teardown (callee 1), then when the
/// byte at +0x20 is non-zero and the resource at +0x14 is non-null
/// releases the resource through the thread-local manager chain: slot
/// `idx` (held by the global at 0x17ABA14) points at A, +0x8 at B, +0 at
/// the table, slot 3 is called with (B, resource) (callee 2). After the
/// release step (callee 3), a set bit 0 in `flag` deletes `this` through
/// callee 4. Returns `this`. All tests are exact null or bit tests.
///
/// Original: 0x00B54030 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b54030(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xeaf2dc;
        const TLSIDX_G: u32 = 0x17aba14;
        const TEARDOWN: u32 = 1;
        const RELEASE: u32 = 3;
        const DELETE: u32 = 4;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
        let need = ((this + 0x20) as *const u8).read();
        if need != 0 {
            let res = ((this + 0x14) as *const u32).read_unaligned();
            if res != 0 {
                let idx = lf_checker_rt::global::<u32>(TLSIDX_G).read_unaligned();
                let a = lf_checker_rt::tls_slot(idx as usize);
                let b = ((a + 8) as *const u32).read_unaligned();
                let vt = (b as *const u32).read_unaligned();
                let fptr = ((vt + 0xc) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(fptr as usize);
                let _: u32 = f(b, res);
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, this);
        if (flag & 1) != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(DELETE, u32, this);
        }
        this
    }
});
