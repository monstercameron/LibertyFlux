// original: 0x009466F0 emit_node_items_and_flag_table (proposed)

/// Emit one call per item of a node list, then flag a handle table.
///
/// Runs callee 1 once with (`BUF`, 0, 0x100) — the call is compared but
/// the callee's own zeroing is not modelled on either side — then walks the
/// node list at `HEAD`. Each node holds flags at +5, the next pointer at +0x3B
/// and an item count byte at +0x3F. For each item `j` of a node, callee 2
/// runs with (`BUF` + running total + `j`, mode), where mode is 2 when
/// bits 0-1 of the flags equal 1 and 1 otherwise (that is what the
/// original's and/dec/movsx/neg/sbb/add sequence computes). The running
/// total grows by each node's count. Afterwards the status bytes are
/// written (`READY` = 1, the rest 0) and, for each of the `N` handles in
/// the table at `TABLE` (`N` from the count byte `COUNT`, compared
/// unsigned), a non-null entry gets byte 1 at +0x1920. No return channel.
///
/// Original: 0x009466F0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_009466F0() -> u32 {
    unsafe {
        const HEAD: u32 = 0x11D7644;
        const BUF: u32 = 0x11D7520;
        const COUNT: u32 = 0x11D74F1;
        const READY: u32 = 0x1037606;
        const ST_A: u32 = 0x11D7628;
        const ST_B: u32 = 0x11D7629;
        const ST_C: u32 = 0x11D762C;
        const TABLE: u32 = 0x11D76AC;
        const FLAG_OFF: u32 = 0x1920;
        const ZERO_MEM: u32 = 1;
        const EMIT: u32 = 2;
        lf_checker_rt::callee_cdecl!(ZERO_MEM, u32, lf_checker_rt::relocated(BUF), 0, 0x100);
        let mut node = (lf_checker_rt::global::<u32>(HEAD) as *const u32).read();
        let mut total = 0u32;
        while node != 0 {
            let count = (node.wrapping_add(0x3F) as *const u8).read_unaligned();
            if count != 0 {
                let flags = (node.wrapping_add(5) as *const u32).read_unaligned();
                let mode = if (flags & 3) == 1 { 2u32 } else { 1u32 };
                let mut j = 0u32;
                while j < count as u32 {
                    let dst = lf_checker_rt::relocated(BUF)
                        .wrapping_add(total)
                        .wrapping_add(j);
                    lf_checker_rt::callee_cdecl!(EMIT, u32, dst, mode);
                    j += 1;
                }
            }
            total = total.wrapping_add(count as u32);
            node = (node.wrapping_add(0x3B) as *const u32).read_unaligned();
        }
        (lf_checker_rt::global::<u8>(READY) as *mut u8).write(1);
        (lf_checker_rt::global::<u8>(ST_A) as *mut u8).write(0);
        (lf_checker_rt::global::<u8>(ST_B) as *mut u8).write(0);
        (lf_checker_rt::global::<u32>(ST_C) as *mut u32).write(0);
        let n = (lf_checker_rt::global::<u8>(COUNT) as *const u8).read();
        if n != 0 {
            let tab = (lf_checker_rt::global::<u32>(TABLE) as *const u32).read();
            let mut d = 0u32;
            while d < n as u32 {
                let e =
                    (tab.wrapping_add(d.wrapping_mul(4)) as *const u32).read_unaligned();
                if e != 0 {
                    (e.wrapping_add(FLAG_OFF) as *mut u8).write(1);
                }
                d += 1;
            }
        }
        0
    }
});
