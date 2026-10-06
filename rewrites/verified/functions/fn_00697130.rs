// original: 0x00697130 comp_group_teardown (proposed)

/// Tear down a three-part component group.
///
/// Runs the base teardown helper (six calls: parts at `this+0x14`, `+0x28`,
/// `this`, `+0x28` again, then `+0x14` and `this` once more) interleaved with
/// three identical release blocks. Each block closes a handle word when set
/// (`this+0x38`, `+0x24`, `+0x10`) through the imported close routine, then,
/// when its flag half (`+0x32`, `+0x1e`, `+0x0a`) and pointer word (`+0x2c`,
/// `+0x18`, `+0x04`) are both set, releases the pointer through the TLS
/// allocator's vtable slot 3. Writes no memory itself; no defined result.
///
/// Original: thiscall, no stack words, plain return.
lf_checker_rt::export!(thiscall, rw_00697130(this: u32) -> u32 {
    unsafe {
        const BASE_HELPER: u32 = 2;
        const CLOSE: u32 = 12;
        const RELEASE_SLOT: u32 = 0x0c;
        const HANDLES: [u32; 3] = [0x38, 0x24, 0x10];
        const FLAGS: [u32; 3] = [0x32, 0x1e, 0x0a];
        const PTRS: [u32; 3] = [0x2c, 0x18, 0x04];
        const BASES: [&[u32]; 3] = [&[0x14, 0x28, 0x00, 0x28], &[0x14], &[0x00]];
        let owner = lf_checker_rt::tls_slot(0);
        let alloc = (owner as *const u32).byte_offset(8).read_unaligned();
        let vtable = (alloc as *const u32).read_unaligned();
        let routine =
            (vtable as *const u32).byte_offset(RELEASE_SLOT as isize).read_unaligned();
        let release: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(routine as usize);
        for round in 0..3 {
            for &part in BASES[round] {
                let _ = lf_checker_rt::callee_thiscall!(
                    BASE_HELPER,
                    u32,
                    this.wrapping_add(part)
                );
            }
            let handle =
                (this as *const u32).byte_offset(HANDLES[round] as isize).read_unaligned();
            if handle != 0 {
                let _ = lf_checker_rt::callee_stdcall!(CLOSE, u32, handle);
            }
            let flag =
                (this as *const u16).byte_offset(FLAGS[round] as isize).read_unaligned();
            if flag != 0 {
                let ptr =
                    (this as *const u32).byte_offset(PTRS[round] as isize).read_unaligned();
                if ptr != 0 {
                    let _ = release(alloc, ptr);
                }
            }
        }
        0
    }
});
