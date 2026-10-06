// original: 0x00B53DB0 crmt_buffer_teardown (proposed)

/// Tear down the double buffer, releasing both resources, then chain on.
///
/// Releases the object at +0x58 through its table slot 1 (callee 1) and
/// runs its teardown (callee 2), stamps the table address at +0x24 and
/// runs that teardown (callee 3), then releases the resource at +0x38
/// when the byte at +0x44 is set (callee 4, the thread-local manager
/// chain of slot `idx` from the global at 0x17ABA14, as in the sibling
/// destructor). After the release step (callee 5) the same pattern
/// repeats for +0x20/+0x14 (callee 6, then callee 4 again), and the
/// function tail-jumps to the final teardown (callee 7) with `this`,
/// whose answer is returned. All tests are exact null or bit tests.
///
/// Original: 0x00B53DB0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b53db0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xeaf2dc;
        const TLSIDX_G: u32 = 0x17aba14;
        const DOWN58: u32 = 2;
        const DOWN24: u32 = 3;
        const REL24: u32 = 5;
        const DOWN00: u32 = 6;
        const TAIL: u32 = 7;
        let e58 = this + 0x58;
        let vt = (e58 as *const u32).read_unaligned();
        let fptr = ((vt + 4) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(fptr as usize);
        let _: u32 = f(e58);
        let _: u32 = lf_checker_rt::callee_thiscall!(DOWN58, u32, e58);
        ((this + 0x24) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(DOWN24, u32, this + 0x24);
        let idx = lf_checker_rt::global::<u32>(TLSIDX_G).read_unaligned();
        let need1 = ((this + 0x44) as *const u8).read();
        if need1 != 0 {
            let res = ((this + 0x38) as *const u32).read_unaligned();
            if res != 0 {
                let a = lf_checker_rt::tls_slot(idx as usize);
                let b = ((a + 8) as *const u32).read_unaligned();
                let vt = (b as *const u32).read_unaligned();
                let fptr = ((vt + 0xc) as *const u32).read_unaligned();
                let g: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(fptr as usize);
                let _: u32 = g(b, res);
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(REL24, u32, this + 0x24);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(DOWN00, u32, this);
        let need2 = ((this + 0x20) as *const u8).read();
        if need2 != 0 {
            let res = ((this + 0x14) as *const u32).read_unaligned();
            if res != 0 {
                let a = lf_checker_rt::tls_slot(idx as usize);
                let b = ((a + 8) as *const u32).read_unaligned();
                let vt = (b as *const u32).read_unaligned();
                let fptr = ((vt + 0xc) as *const u32).read_unaligned();
                let g: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(fptr as usize);
                let _: u32 = g(b, res);
            }
        }
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
