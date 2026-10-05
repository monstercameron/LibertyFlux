// original: 0x00c0a920 stream_entry_detach (proposed)

/// Detach one entry from its owner and run it through the three helpers.
///
/// `this` owns the counters, `entry` is the entry and `owner` the object the
/// finisher (callee 3) runs on. When the entry's link at `LINK` is set, the
/// flag bit `KEEP` in its word at `FLAGS` is cleared. Unless the entry's kind
/// (the masked `KINDMASK` bits of the word at `KIND` past its object at `OBJ`)
/// equals `KEEPKIND`, the owner's count at `COUNT` is dropped by one. The
/// releaser (callee 1) receives the object and zero; a set object then runs
/// its own teardown (callee 2, through the object table) with the constant 1.
/// The entry's object field is cleared, and when its mark byte at `MARK` is
/// set the owner's total at `TOTAL` is dropped by one and the mark cleared.
/// The finisher runs with the entry, and the writer (callee 4) with the
/// global owner and the entry, whose answer is returned.
///
/// Original: 0x00c0a920 (thiscall, two stack words; 1 is cdecl, rest thiscall).
lf_checker_rt::export!(thiscall, rw_00c0a920(this: u32, entry: u32, owner: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x10;
        const FLAGS: u32 = 0x24;
        const KEEP: u32 = 0x10000000;
        const OBJ: u32 = 0x08;
        const KIND: u32 = 0x28;
        const KINDMASK: u32 = 0x3c0;
        const KEEPKIND: u32 = 0x40;
        const COUNT: u32 = 0x04;
        const TOTAL: u32 = 0x00;
        const MARK: u32 = 0x14;
        const OWNER: u32 = 0x1683290;
        const RELEASE: u32 = 1;
        const TEARDOWN: u32 = 2;
        const FINISH: u32 = 3;
        const WRITE: u32 = 4;
        let link = (entry.wrapping_add(LINK) as *const u32).read_unaligned();
        if link != 0 {
            let f = link.wrapping_add(FLAGS) as *mut u32;
            f.write_unaligned(f.read_unaligned() & !KEEP);
        }
        let obj = (entry.wrapping_add(OBJ) as *const u32).read_unaligned();
        let kind = ((obj.wrapping_add(KIND)) as *const u32).read_unaligned() & KINDMASK;
        if kind != KEEPKIND {
            let c = this.wrapping_add(COUNT) as *mut u32;
            c.write_unaligned(c.read_unaligned().wrapping_sub(1));
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, obj, 0);
        let obj = (entry.wrapping_add(OBJ) as *const u32).read_unaligned();
        if obj != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, obj, 1);
        }
        (entry.wrapping_add(OBJ) as *mut u32).write_unaligned(0);
        if (entry.wrapping_add(MARK) as *const u8).read() != 0 {
            let t = this as *mut u32;
            t.write_unaligned(t.read_unaligned().wrapping_sub(1));
            (entry.wrapping_add(MARK) as *mut u8).write(0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(FINISH, u32, owner, entry);
        lf_checker_rt::callee_thiscall!(WRITE, u32, lf_checker_rt::relocated(OWNER), entry)
    }
});
