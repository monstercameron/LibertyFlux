// original: 0x005D8010 run_row_session (proposed)

/// Primes a descriptor row, fills a buffer, opens a handle, runs a
/// three-entry descriptor block through it, then closes and releases.
///
/// `this` is the owner (never dereferenced, only stored into the block
/// and tested against zero); the single stack argument `node` carries a
/// tag word at `+0xE4` (non-zero selects the row index at `+0xE0`,
/// otherwise the constant `NO_ROW`) and is published to the global
/// `CUR_VA` while the block runs. Session state lives in TLS slot 0:
/// `[tls0+8]` and `[tls0+0x10]` are compared for equality (equal
/// increments the counter at `[tls0+0x68]`, otherwise the first word
/// rotates into the spare slot at `[tls0+0x64]` and the second becomes
/// current).
///
/// Callee 1 (stdcall: row index) answers the row number. Rows live at
/// `base + stride * row` (`BASE_VA`, `STRIDE_VA` globals); the flag
/// table (`FLAGTAB_VA`) is indexed by the same row. When the flag byte
/// has `0x80` set the row address is forced to null and the original
/// faults reading `[0+0x59C]`; otherwise, when the row's flag byte at
/// `+0x59C` is zero, callee 2 (stdcall: row, tag) primes it and the
/// byte is set to 1. The first check always settles the byte, so the
/// second check (same row, same byte) never fires its prime call. A
/// dword at row `+0x598` is passed on to callee 3. Callee 3 (cdecl:
/// `buf, 0x40, tag, row+0x20, extra, 0, 0`, where `extra` is the
/// dword at row `+0x598`) fills the 64-byte frame buffer, whose address
/// callee 4 reuses. Callee 4 (thiscall on `OP_VA`: `buf, 0, 0, 1`)
/// opens the handle.
///
/// The global `CUR_VA` is set to `node`, then callee 5 (thiscall:
/// block address, handle) attaches: it writes the two words at block
/// `+0`/`+4`. The block at `+0` gets `DESC0` (over the first attached
/// word), at `+8` a flag byte of 1, at `+0x3C` `0x1000`, and three
/// 16-byte entries at `+0xC`/`+0x14`/`+0x24` (copied with `movq` in the
/// original): each is `(func, 0, this, addr)` with funcs
/// `0x5D6E60/0x5D6FF0/0x5D7010` and addrs `0x404B60/0x404B80/0x404B90`
/// (relocated image addresses; the addrs land unconditionally - the
/// temp holding them is set outside the branch - while func and owner
/// are zero when `this` is null).
/// Callee 6 (thiscall: block address) runs it. When
/// `[handle+0x14] == 0` and `[handle+0x10] != 0`, callee 7 (thiscall:
/// handle) closes. The release is a double-indirect call: `ecx =
/// [handle]`, target `[[ecx]+0x2C]` (thiscall: table, `[handle+4]`);
/// then `[handle+4]` is set to `-1`, `[handle]` to 0 and `CUR_VA` to 0.
///
/// No callee answer is range- or sign-compared: the row number feeds
/// only indexing and multiplication, the ECX values of callees 1 and 2
/// are stale (whatever the globals loads left) so both are stdcall,
/// and the handle words are compared for equality with zero only.
///
/// Returns the session counter minus one, or the spare allocator word
/// when the counter reached zero (restoring and clearing the spare).
/// The trailing security-cookie check runs natively and is not
/// intercepted: it always passes and writes nothing.
///
/// Original: 0x005D8010 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_005D8010(this: u32, node: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0xE4;
        const ROW_OFF: u32 = 0xE0;
        const NO_ROW: u32 = 0x00FC9C85;
        const STRIDE_VA: u32 = 0x0110E8F0;
        const BASE_VA: u32 = 0x0110E8E4;
        const FLAGTAB_VA: u32 = 0x0110E8E8;
        const ROW_FLAG: u32 = 0x59C;
        const ROW_EXTRA: u32 = 0x598;
        const PRIME_TAG: u32 = 0x00F924F4;
        const FILL_TAG: u32 = 0x00F1C728;
        const CUR_VA: u32 = 0x018B6FAC;
        const OP_VA: u32 = 0x0110C0A0;
        const DESC0: u32 = 0x00FE0B68;
        const FUNC0: u32 = 0x005D6E60;
        const FUNC1: u32 = 0x005D6FF0;
        const FUNC2: u32 = 0x005D7010;
        const ADDR0: u32 = 0x00404B60;
        const ADDR1: u32 = 0x00404B80;
        const ADDR2: u32 = 0x00404B90;
        const ROW_OF: u32 = 1;
        const PRIME_ROW: u32 = 2;
        const FILL_BUF: u32 = 3;
        const OPEN_H: u32 = 4;
        const ATTACH: u32 = 5;
        const RUN_DESC: u32 = 6;
        const CLOSE_H: u32 = 7;
        let tls0 = lf_checker_rt::tls_slot(0);
        let ea = rd32(tls0.wrapping_add(8));
        let ec = rd32(tls0.wrapping_add(0x10));
        if ea == ec {
            let c = rd32(tls0.wrapping_add(0x68));
            wr32(tls0.wrapping_add(0x68), c.wrapping_add(1));
        } else {
            wr32(tls0.wrapping_add(0x64), ea);
            wr32(tls0.wrapping_add(8), ec);
        }
        let arg0 = if rd16(node.wrapping_add(TAG_OFF)) != 0 {
            rd32(node.wrapping_add(ROW_OFF))
        } else {
            lf_checker_rt::relocated(NO_ROW)
        };
        let ebp = lf_checker_rt::callee_stdcall!(ROW_OF, u32, arg0);
        let stride = rd32(lf_checker_rt::relocated(STRIDE_VA));
        let base = rd32(lf_checker_rt::relocated(BASE_VA));
        let flagtab = rd32(lf_checker_rt::relocated(FLAGTAB_VA));
        let fb = rd8(flagtab.wrapping_add(ebp));
        let row = base.wrapping_add(stride.wrapping_mul(ebp));
        let hi = (fb & 0x80) != 0;
        let rowptr = if hi { 0 } else { row };
        let ebx0 = if hi { 0 } else { row };
        if rd8(ebx0.wrapping_add(ROW_FLAG)) == 0 {
            let _ = lf_checker_rt::callee_stdcall!(
                PRIME_ROW,
                u32,
                ebp,
                lf_checker_rt::relocated(PRIME_TAG)
            );
            wr8(ebx0.wrapping_add(ROW_FLAG), 1);
        }
        let extra = rd32(rowptr.wrapping_add(ROW_EXTRA));
        let fb2 = rd8(flagtab.wrapping_add(ebp));
        let hi2 = (fb2 & 0x80) != 0;
        let ebx1 = if hi2 { 0 } else { row };
        let edi0 = if hi2 { 0 } else { row };
        if rd8(edi0.wrapping_add(ROW_FLAG)) == 0 {
            let _ = lf_checker_rt::callee_stdcall!(
                PRIME_ROW,
                u32,
                ebp,
                lf_checker_rt::relocated(PRIME_TAG)
            );
            wr8(edi0.wrapping_add(ROW_FLAG), 1);
        }
        let mut buf = [0u32; 16];
        let _ = lf_checker_rt::callee_cdecl!(
            FILL_BUF,
            u32,
            buf.as_mut_ptr() as u32,
            0x40,
            lf_checker_rt::relocated(FILL_TAG),
            ebx1.wrapping_add(0x20),
            extra,
            0,
            0
        );
        let edi = lf_checker_rt::callee_thiscall!(
            OPEN_H,
            u32,
            lf_checker_rt::relocated(OP_VA),
            buf.as_mut_ptr() as u32,
            0,
            0,
            1
        );
        wr32(lf_checker_rt::relocated(CUR_VA), node);
        let mut st = [0u32; 20];
        let stp = st.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(ATTACH, u32, stp, edi);
        st[0] = lf_checker_rt::relocated(DESC0);
        wr8(stp.wrapping_add(8), 1);
        st[15] = 0x1000;
        if this != 0 {
            st[3] = lf_checker_rt::relocated(FUNC0);
            st[4] = 0;
            st[5] = this;
            st[6] = lf_checker_rt::relocated(ADDR0);
        } else {
            st[3] = 0;
            st[4] = 0;
            st[5] = 0;
            st[6] = lf_checker_rt::relocated(ADDR0);
        }
        if this != 0 {
            st[7] = lf_checker_rt::relocated(FUNC1);
            st[8] = 0;
            st[9] = this;
            st[10] = lf_checker_rt::relocated(ADDR1);
        } else {
            st[7] = 0;
            st[8] = 0;
            st[9] = 0;
            st[10] = lf_checker_rt::relocated(ADDR1);
        }
        if this != 0 {
            st[11] = lf_checker_rt::relocated(FUNC2);
            st[12] = 0;
            st[13] = this;
            st[14] = lf_checker_rt::relocated(ADDR2);
        } else {
            st[11] = 0;
            st[12] = 0;
            st[13] = 0;
            st[14] = lf_checker_rt::relocated(ADDR2);
        }
        let _ = lf_checker_rt::callee_thiscall!(RUN_DESC, u32, stp);
        if rd32(edi.wrapping_add(0x14)) == 0 && rd32(edi.wrapping_add(0x10)) != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CLOSE_H, u32, edi);
        }
        let ov = rd32(edi);
        let t2 = rd32(ov);
        let tgt = rd32(t2.wrapping_add(0x2C));
        let site: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(tgt as usize) };
        let _ = site(ov, rd32(edi.wrapping_add(4)));
        wr32(edi.wrapping_add(4), 0xFFFFFFFF);
        wr32(edi, 0);
        wr32(lf_checker_rt::relocated(CUR_VA), 0);
        let e = rd32(tls0.wrapping_add(0x68));
        if e != 0 {
            wr32(tls0.wrapping_add(0x68), e.wrapping_sub(1));
            e.wrapping_sub(1)
        } else {
            let s = rd32(tls0.wrapping_add(0x64));
            wr32(tls0.wrapping_add(8), s);
            wr32(tls0.wrapping_add(0x64), 0);
            s
        }
    }
});
