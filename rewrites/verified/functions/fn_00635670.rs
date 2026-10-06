// original: 0x00635670 table_iter_advance (proposed)

/// Advance a six-global row iterator, or reset it when the index is -1.
///
/// State lives in six adjacent global dwords: `IDX` (current row, -1 means
/// reset), `CUR` (current node pointer), `SUB` (position inside the node),
/// `LIM` (node length), `BND` (row bound) and `TBL` (table pointer). The
/// table points at a header whose first dword is a row array (48-byte rows,
/// first field of each row a node pointer) and whose halfword at `+4` is the
/// row count. Nodes form a chain through their first dword and carry their
/// length as one byte at `+0x1F3`.
///
/// On the reset path (`IDX == -1`) a null table or a zero row count leaves
/// the state alone and just tests the current position; otherwise the bound
/// is reloaded from the table (an unsigned halfword, zero-extended), `SUB`
/// is cleared and rows are scanned from the top for the first non-null node.
/// On the advance path `SUB` is bumped and, when it reaches `LIM`, the next
/// node is taken, then later rows are scanned the same way. Every bound
/// comparison is SIGNED (`jge`/`jl`): row against bound, position against
/// length. Returns 1 in AL when the resulting position is live
/// (`CUR != 0` and `SUB < LIM`), else 0; the upper 24 bits of EAX keep the
/// caller's value on the plain-advance path (never written), so only AL is
/// meaningful. No arguments, cdecl, no calls, no floating point.
lf_checker_rt::export!(cdecl, rw_00635670() -> u32 {
    unsafe {
        const G_IDX: u32 = 0x0110_ED00;
        const G_CUR: u32 = 0x0110_ED04;
        const G_SUB: u32 = 0x0110_ED08;
        const G_LIM: u32 = 0x0110_ED0C;
        const G_BND: u32 = 0x0110_ED10;
        const G_TBL: u32 = 0x0110_ED14;
        const ROW_STRIDE: u32 = 48;
        const NODE_LEN_OFF: u32 = 0x1F3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let idx = lf_checker_rt::global::<i32>(G_IDX);
        let cur = lf_checker_rt::global::<u32>(G_CUR);
        let sub = lf_checker_rt::global::<i32>(G_SUB);
        let lim = lf_checker_rt::global::<i32>(G_LIM);
        let bnd = lf_checker_rt::global::<i32>(G_BND);
        let tbl = lf_checker_rt::global::<u32>(G_TBL);

        let mut edx: i32 = idx.read_unaligned();
        if edx == -1 {
            let ebp: u32 = tbl.read_unaligned();
            if ebp == 0 {
                let edi: i32 = sub.read_unaligned();
                let ecx: u32 = cur.read_unaligned();
                return tail_check(ecx, edi, lim);
            }
            let count: u32 = rd16(ebp.wrapping_add(4));
            if count == 0 {
                let edi: i32 = sub.read_unaligned();
                let ecx: u32 = cur.read_unaligned();
                return tail_check(ecx, edi, lim);
            }
            let mut ecx: u32 = cur.read_unaligned();
            let ebx: i32 = count as i32;
            bnd.write_unaligned(ebx);
            wr32(lf_checker_rt::relocated(G_SUB), 0);
            if ecx == 0 {
                // `or esi,edx` with edx == -1 forces esi to -1 whatever the
                // caller held; scan rows from the top for a live node.
                let mut esi: i32 = -1;
                loop {
                    if esi >= ebx {
                        return tail_check(ecx, 0, lim);
                    }
                    edx = edx.wrapping_add(1);
                    esi = edx;
                    idx.write_unaligned(edx);
                    if esi >= ebx {
                        if ecx == 0 {
                            continue;
                        }
                        return load_lim_and_check(ecx, lim);
                    }
                    let rows: u32 = rd32(ebp);
                    let off = (edx as u32).wrapping_mul(ROW_STRIDE);
                    ecx = rd32(rows.wrapping_add(off));
                    cur.write_unaligned(ecx);
                    if ecx == 0 {
                        continue;
                    }
                    return load_lim_and_check(ecx, lim);
                }
            }
            return tail_check(ecx, 0, lim);
        }
        // Advance path.
        let mut ecx: u32 = cur.read_unaligned();
        if ecx == 0 {
            return 0;
        }
        let mut edi: i32 = sub.read_unaligned().wrapping_add(1);
        sub.write_unaligned(edi);
        if edi < lim.read_unaligned() {
            return tail_check(ecx, edi, lim);
        }
        edi = 0;
        sub.write_unaligned(0);
        ecx = rd32(ecx);
        cur.write_unaligned(ecx);
        if ecx != 0 {
            return load_lim_and_check(ecx, lim);
        }
        let ebp: u32 = tbl.read_unaligned();
        let ebx: i32 = bnd.read_unaligned();
        let mut esi: i32 = edx;
        loop {
            if esi >= ebx {
                return 0;
            }
            edx = edx.wrapping_add(1);
            esi = edx;
            idx.write_unaligned(edx);
            if esi >= ebx {
                if ecx == 0 {
                    continue;
                }
                return load_lim_and_check(ecx, lim);
            }
            let rows: u32 = rd32(ebp);
            let off = (edx as u32).wrapping_mul(ROW_STRIDE);
            ecx = rd32(rows.wrapping_add(off));
            cur.write_unaligned(ecx);
            if ecx == 0 {
                continue;
            }
            return load_lim_and_check(ecx, lim);
        }

        #[inline(always)]
        unsafe fn tail_check(ecx: u32, edi: i32, lim: *mut i32) -> u32 {
            unsafe {
                if ecx == 0 {
                    return 0;
                }
                // Signed: `(an instruction of the original); jge fail`.
                if edi >= lim.read_unaligned() {
                    return 0;
                }
                1
            }
        }

        #[inline(always)]
        unsafe fn load_lim_and_check(ecx: u32, lim: *mut i32) -> u32 {
            unsafe {
                let n: u32 = (ecx.wrapping_add(NODE_LEN_OFF) as *const u8).read() as u32;
                (lim as *mut u32).write_unaligned(n);
                // edi is 0 here on both paths (cleared before the scan).
                tail_check(ecx, 0, lim)
            }
        }
    }
});
