// original: 0x00b659f0 veh_lazy_bind_slot
/// Lazily bind a slot and report whether the object is bound.
///
/// Reads the head word at `[this]`: zero returns 1 at once. Otherwise, when
/// the head is positive and byte `[this+8]` is clear, resolves a manager
/// from the head, asks it for a handle, and — unless the handle is -1 —
/// runs a two-call attach pair keyed by a shared global; when the second
/// call answers nonzero, notifies the table slot for the handle and sets
/// byte `[this+8]`. The -1 handle also sets the byte. Returns the last
/// value seen with its low byte replaced by whether `[this+8]` is now set
/// (for the zero head that is exactly 1).
///
/// Original: thiscall, ECX only.
lf_checker_rt::export!(thiscall, rw_00b659f0(this: u32) -> u32 {
    unsafe {
        const V_GET: u32 = 1;
        const H_GET: u32 = 2;
        const PAIR_A: u32 = 3;
        const PAIR_B: u32 = 4;
        const NOTIFY: u32 = 5;
        const SHARED_VA: u32 = 0x012b4138;
        const TABLE_VA: u32 = 0x01295cd8;
        const NO_HANDLE: u32 = 0xFFFFFFFF;
        let rd8 = |a: u32| (a as *const u8).read();
        let head = (this as *const u32).read_unaligned();
        if head == 0 {
            return 1;
        }
        let mut eax = head;
        if (head as i32) > 0 && rd8(this + 8) == 0 {
            let mgr: u32 = lf_checker_rt::callee_cdecl!(V_GET, u32, head);
            let h: u32 = lf_checker_rt::callee_thiscall!(H_GET, u32, mgr);
            eax = h;
            if rd8(this + 8) == 0 {
                if h == NO_HANDLE {
                    ((this + 8) as *mut u8).write(1);
                } else {
                    let shared = lf_checker_rt::global::<u32>(SHARED_VA).read_unaligned();
                    let _: u32 = lf_checker_rt::callee_cdecl!(PAIR_A, u32, h, shared, 8);
                    let r: u32 = lf_checker_rt::callee_cdecl!(PAIR_B, u32, h, shared);
                    eax = r;
                    if (r & 0xFF) != 0 {
                        let entry = lf_checker_rt::relocated(TABLE_VA)
                            .wrapping_add(h.wrapping_mul(4));
                        let slot = (entry as *const u32).read_unaligned();
                        let n: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, slot);
                        eax = n;
                        ((this + 8) as *mut u8).write(1);
                    }
                }
            }
        }
        let bit = if rd8(this + 8) != 0 { 1u32 } else { 0u32 };
        (eax & 0xFFFFFF00) | bit
    }
});

/// Wrong version of rw_00b659f0: the notify path does not set `[this+8]`.
lf_checker_rt::export!(thiscall, mut_00b659f0(this: u32) -> u32 {
    unsafe {
        const V_GET: u32 = 1;
        const H_GET: u32 = 2;
        const PAIR_A: u32 = 3;
        const PAIR_B: u32 = 4;
        const NOTIFY: u32 = 5;
        const SHARED_VA: u32 = 0x012b4138;
        const TABLE_VA: u32 = 0x01295cd8;
        const NO_HANDLE: u32 = 0xFFFFFFFF;
        let rd8 = |a: u32| (a as *const u8).read();
        let head = (this as *const u32).read_unaligned();
        if head == 0 {
            return 1;
        }
        let mut eax = head;
        if (head as i32) > 0 && rd8(this + 8) == 0 {
            let mgr: u32 = lf_checker_rt::callee_cdecl!(V_GET, u32, head);
            let h: u32 = lf_checker_rt::callee_thiscall!(H_GET, u32, mgr);
            eax = h;
            if rd8(this + 8) == 0 {
                if h == NO_HANDLE {
                    ((this + 8) as *mut u8).write(1);
                } else {
                    let shared = lf_checker_rt::global::<u32>(SHARED_VA).read_unaligned();
                    let _: u32 = lf_checker_rt::callee_cdecl!(PAIR_A, u32, h, shared, 8);
                    let r: u32 = lf_checker_rt::callee_cdecl!(PAIR_B, u32, h, shared);
                    eax = r;
                    if (r & 0xFF) != 0 {
                        let entry = lf_checker_rt::relocated(TABLE_VA)
                            .wrapping_add(h.wrapping_mul(4));
                        let slot = (entry as *const u32).read_unaligned();
                        let n: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, slot);
                        eax = n;
                        // MUTANT: store of 1 to [this+8] omitted.
                    }
                }
            }
        }
        let bit = if rd8(this + 8) != 0 { 1u32 } else { 0u32 };
        (eax & 0xFFFFFF00) | bit
    }
});
