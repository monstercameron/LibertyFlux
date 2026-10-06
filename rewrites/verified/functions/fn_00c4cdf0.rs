// original: 0x00C4CDF0 task_rows_load (proposed)

/// Load a task-row table from its text definition file.
///
/// `this` points to the table (42 rows of 0x68 bytes at `+0x10`, a header
/// pointer at `+0x00`, a row count at `+0x08`, a table float at `+0x04`).
/// The second stack word is never read. After blanking all rows (slots at
/// row `+0x0c/+0x10/+0x14/+0x18` to zero or the shared default object from
/// global `0x17ED954`, header words of the next row to 0/1.0/0), the file
/// is validated (callees 1-2); a zero handle ends the function at once.
/// A name tag is looked up (callee 3, falling back to callee 4 on -1) and
/// resolved (callee 5); a null resolution runs an error report (callees
/// 6-8) and rejoins. The reader is opened (callees 9-10), the count is
/// zeroed, and one table float is parsed.
///
/// Each loop pass reads a line (callee 12): `#` lines are skipped
/// (callee 13), a header-tag match (callee 14) arms row parsing, an
/// end-tag match leaves the loop. A row parses eight tagged pointer
/// fields (callee 14 against each tag; on match the name resolves through
/// callees 5, 15 and 17, stores into the slot and gains one reference),
/// four integers (callee 18) kept as low bytes at `+0x34..+0x37` and
/// `+0x60..+0x63`, two integers kept as SIGNED greater-than-zero flags at
/// `+0x24` and `+0x50`, eleven floats (callee 11) at `+0x1c/+0x20/+0x28/
/// +0x2c/+0x30/+0x48/+0x4c/+0x54/+0x58/+0x5c/+0x64`, one more
/// float at `+0x6c`, and two optional numbers (callee 19) at the next
/// row's `+0x00` and at `+0x70` (zero when their tag misses). The count
/// grows by one per row. The exit block resolves one more pointer into
/// the header, references it, releases the file handle (callee 20), and
/// every path ends through the security-cookie check (callee 21).
///
/// Frame pointers (line buffer, reader state) are the rewrite's own
/// scratch; the stubbed reader fills the buffer identically on both
/// sides. The cookie save is the original's frame guard, not behaviour,
/// and is not replicated; the check call is kept for call parity.
/// Original: 0x00C4CDF0 (thiscall, one unread stack word, no return value).
lf_checker_rt::export!(thiscall, rw_00c4cdf0(this: u32, _arg: u32) -> u32 {
    unsafe {
        const ROW_COUNT: u32 = 0x08;
        const TABLE_FLOAT: u32 = 0x04;
        const HEADER_PTR: u32 = 0x00;
        const ROW_BASE: u32 = 0x10;
        const ROW_STRIDE: u32 = 0x68;
        const ROWS: u32 = 42;
        const REF_COUNT: u32 = 0x0a;
        const DEFAULT_OBJECT: u32 = 0x17ED954;
        const SHARED_OBJ: u32 = 0x110C0A0;
        const MISSING: u32 = 0xFFFF_FFFF;
        const TAG_OPEN_A: u32 = 0xEC9C4C;
        const TAG_OPEN_B0: u32 = 0xEC9C64;
        const TAG_OPEN_B1: u32 = 0xEC9C68;
        const TAG_OPEN_C: u32 = 0xEC9B09;
        const TAG_LOOKUP: u32 = 0xEC9C74;
        const TAG_ERROR: u32 = 0xEC9C80;
        const TAG_READER: u32 = 0xEC9C94;
        const TAG_HDR: u32 = 0xEC9C9C;
        const TAG_END: u32 = 0xEC9CB0;
        const TAG_F0: u32 = 0xEC9CC0;
        const TAG_F1: u32 = 0xEC9CC4;
        const TAG_F2: u32 = 0xEC9CC8;
        const TAG_F3: u32 = 0xEC9CCC;
        const TAG_F4: u32 = 0xEC9CD4;
        const TAG_F5: u32 = 0xEC9CE0;
        const TAG_F6: u32 = 0xEC9CE4;
        const TAG_F7: u32 = 0xEC9CE8;
        const TAG_NUM0: u32 = 0xEC9CEC;
        const TAG_NUM1: u32 = 0xEC9CF0;
        const TAG_EXIT: u32 = 0xEC9CF4;
        const LINE_LEN: u32 = 0x80;
        const COMMENT: u8 = 0x23;
        const C_VALID_A: u32 = 1;
        const C_VALID_B: u32 = 2;
        const C_LOOKUP: u32 = 3;
        const C_LOOKUP_ALT: u32 = 4;
        const C_RESOLVE: u32 = 5;
        const C_ERR_A: u32 = 6;
        const C_ERR_B: u32 = 7;
        const C_ERR_C: u32 = 8;
        const C_ACQUIRE: u32 = 9;
        const C_OPEN: u32 = 10;
        const C_PARSE_F: u32 = 11;
        const C_READ: u32 = 12;
        const C_SKIP_COMMENT: u32 = 13;
        const C_TAG_CMP: u32 = 14;
        const C_HANDLE: u32 = 15;
        const C_HANDLE_EXIT: u32 = 16;
        const C_INTERN: u32 = 17;
        const C_PARSE_I: u32 = 18;
        const C_PARSE_N: u32 = 19;
        const C_RELEASE_FILE: u32 = 20;
        const C_COOKIE: u32 = 21;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                let _: u32 = lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            }
        }

        /// Resolve one tagged name and store it into the row slot with a
        /// fresh reference: handle (callee 5), lookup (callee 15), intern
        /// (callee 17), store, count + 1.
        #[inline(always)]
        unsafe fn do_resolve(ebp: u32, buf: u32, row: u32, off: u32, extra: u32) {
            unsafe {
                let h: u32 = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, ebp);
                let a: u32 = lf_checker_rt::callee_cdecl!(C_HANDLE, u32, buf, extra);
                let v: u32 = lf_checker_rt::callee_thiscall!(C_INTERN, u32, h, a);
                wr32(row.wrapping_add(off), v);
                wr16(
                    v.wrapping_add(REF_COUNT),
                    rd16(v.wrapping_add(REF_COUNT)).wrapping_add(1),
                );
            }
        }

        let shared = rd32(lf_checker_rt::relocated(DEFAULT_OBJECT));
        for i in 0..ROWS {
            let r = this
                .wrapping_add(ROW_BASE)
                .wrapping_add(i.wrapping_mul(ROW_STRIDE));
            wr32(r.wrapping_sub(4), 0);
            wr32(r, shared);
            wr32(r.wrapping_add(4), 0);
            wr32(r.wrapping_add(8), shared);
            wr32(r.wrapping_add(0x58), 0);
            wr32(r.wrapping_add(0x5c), 0x3f80_0000);
            wr32(r.wrapping_add(0x60), 0);
        }
        let _: u32 =
            lf_checker_rt::callee_cdecl!(C_VALID_A, u32, lf_checker_rt::relocated(TAG_OPEN_A));
        let file: u32 = lf_checker_rt::callee_cdecl!(
            C_VALID_B,
            u32,
            lf_checker_rt::relocated(TAG_OPEN_B1),
            lf_checker_rt::relocated(TAG_OPEN_B0)
        );
        let l_file = file;
        let _: u32 =
            lf_checker_rt::callee_cdecl!(C_VALID_A, u32, lf_checker_rt::relocated(TAG_OPEN_C));
        if file == 0 {
            cookie();
            return 0;
        }
        let mut ebp: u32 =
            lf_checker_rt::callee_cdecl!(C_LOOKUP, u32, lf_checker_rt::relocated(TAG_LOOKUP));
        if ebp == MISSING {
            ebp = lf_checker_rt::callee_cdecl!(
                C_LOOKUP_ALT,
                u32,
                lf_checker_rt::relocated(TAG_LOOKUP)
            );
        }
        let q: u32 = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, ebp);
        if q == 0 {
            let so = lf_checker_rt::relocated(SHARED_OBJ);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_ERR_A, u32, so, lf_checker_rt::relocated(TAG_ERROR));
            let _: u32 = lf_checker_rt::callee_cdecl!(
                C_ERR_B,
                u32,
                ebp,
                lf_checker_rt::relocated(TAG_LOOKUP)
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(C_ERR_C, u32, so);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(C_ACQUIRE, u32, ebp);
        let mut parser_scratch: u32 = 0;
        let mut line_buf: [u32; 2] = [0; 2];
        let parser = core::ptr::addr_of_mut!(parser_scratch) as u32;
        let buf = core::ptr::addr_of_mut!(line_buf) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            C_OPEN,
            u32,
            parser,
            lf_checker_rt::relocated(TAG_READER),
            file
        );
        let mut ebx = 0xFFFF_FFFFu32;
        let mut l_ebx = ebx;
        wr32(this.wrapping_add(ROW_COUNT), 0);
        let tf: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
        wrf(this.wrapping_add(TABLE_FLOAT), tf);
        loop {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            let first = (buf as *const u8).read();
            if first == COMMENT {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_SKIP_COMMENT, u32, parser);
                continue;
            }
            let s1: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_HDR)
            );
            if s1 == 0 {
                ebx = 0;
                l_ebx = 0;
                continue;
            }
            let s2: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_END)
            );
            if s2 == 0 {
                break;
            }
            if ebx != 0 {
                continue;
            }
            let n = rd32(this.wrapping_add(ROW_COUNT));
            let row = this.wrapping_add(n.wrapping_mul(ROW_STRIDE));
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x0c), ebx);
            let f0: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F0)
            );
            if f0 != 0 {
                do_resolve(ebp, buf, row, 0x0c, ebx);
            }
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x10), shared);
            let f1: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F1)
            );
            if f1 != 0 {
                do_resolve(ebp, buf, row, 0x10, 0);
            }
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x14), 0);
            let f2: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F2)
            );
            if f2 != 0 {
                do_resolve(ebp, buf, row, 0x14, 0);
            }
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x18), shared);
            let f3: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F3)
            );
            if f3 != 0 {
                do_resolve(ebp, buf, row, 0x18, 0);
            }
            let a1: u32 = lf_checker_rt::callee_thiscall!(C_PARSE_I, u32, parser, 1);
            let s1v = a1;
            let a2: u32 = lf_checker_rt::callee_thiscall!(C_PARSE_I, u32, parser, 1);
            let s2v = a2;
            let a3: u32 = lf_checker_rt::callee_thiscall!(C_PARSE_I, u32, parser, 1);
            ebx = a3;
            let a4: u32 = lf_checker_rt::callee_thiscall!(C_PARSE_I, u32, parser, 1);
            let edx = a4;
            wr8(row.wrapping_add(0x34), s1v as u8);
            wr8(row.wrapping_add(0x35), s2v as u8);
            wr8(row.wrapping_add(0x36), ebx as u8);
            wr8(row.wrapping_add(0x37), edx as u8);
            wr8(row.wrapping_add(0x60), s1v as u8);
            wr8(row.wrapping_add(0x61), s2v as u8);
            wr8(row.wrapping_add(0x62), ebx as u8);
            wr8(row.wrapping_add(0x63), edx as u8);
            let g1: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x1c), g1);
            let g2: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x20), g2);
            let a5: u32 = lf_checker_rt::callee_thiscall!(C_PARSE_I, u32, parser, 1);
            wr8(row.wrapping_add(0x24), if (a5 as i32) > 0 { 1 } else { 0 });
            let g3: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x28), g3);
            let g4: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x2c), g4);
            let g5: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x30), g5);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x38), 0);
            let f4: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F4)
            );
            if f4 != 0 {
                do_resolve(ebp, buf, row, 0x38, 0);
            }
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x3c), shared);
            let f5: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F5)
            );
            if f5 != 0 {
                do_resolve(ebp, buf, row, 0x3c, 0);
            }
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x40), 0);
            let f6: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F6)
            );
            if f6 != 0 {
                do_resolve(ebp, buf, row, 0x40, 0);
            }
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            wr32(row.wrapping_add(0x44), shared);
            let f7: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_F7)
            );
            if f7 != 0 {
                do_resolve(ebp, buf, row, 0x44, 0);
            }
            let h1: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x48), h1);
            let h2: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x4c), h2);
            let a6: u32 = lf_checker_rt::callee_thiscall!(C_PARSE_I, u32, parser, 1);
            wr8(row.wrapping_add(0x50), if (a6 as i32) > 0 { 1 } else { 0 });
            let h3: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x54), h3);
            let h4: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x58), h4);
            let h5: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x5c), h5);
            let h6: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x64), h6);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            let f8: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_NUM0)
            );
            let next = this.wrapping_add(n.wrapping_add(1).wrapping_mul(ROW_STRIDE));
            if f8 != 0 {
                let v: u32 = lf_checker_rt::callee_cdecl!(C_PARSE_N, u32, buf);
                wr32(next, v);
            } else {
                wr32(next, 0);
            }
            let h7: f32 = lf_checker_rt::callee_thiscall!(C_PARSE_F, f32, parser, 1);
            wrf(row.wrapping_add(0x6c), h7);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_READ, u32, parser, buf, LINE_LEN);
            let f9: u32 = lf_checker_rt::callee_cdecl!(
                C_TAG_CMP,
                u32,
                buf,
                lf_checker_rt::relocated(TAG_NUM1)
            );
            if f9 != 0 {
                let v: u32 = lf_checker_rt::callee_cdecl!(C_PARSE_N, u32, buf);
                wr32(row.wrapping_add(0x70), v);
            } else {
                wr32(row.wrapping_add(0x70), 0);
            }
            ebx = l_ebx;
            wr32(this.wrapping_add(ROW_COUNT), n.wrapping_add(1));
        }
        let esi: u32 = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, ebp);
        let a: u32 = lf_checker_rt::callee_cdecl!(
            C_HANDLE_EXIT,
            u32,
            lf_checker_rt::relocated(TAG_EXIT),
            0
        );
        let v: u32 = lf_checker_rt::callee_thiscall!(C_INTERN, u32, esi, a);
        wr32(this.wrapping_add(HEADER_PTR), v);
        wr16(
            v.wrapping_add(REF_COUNT),
            rd16(v.wrapping_add(REF_COUNT)).wrapping_add(1),
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(C_RELEASE_FILE, u32, l_file);
        cookie();
        0
    }
});
