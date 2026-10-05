// original: 0x00887860 stream_hash_lookup (proposed)

/// Find a stream entry by name in the hash table.
///
/// `this` points to the table (`[this]` buckets, word count at `+4`); the
/// argument points to a word holding the name pointer. The name's hash
/// (callee 1) modulo the count selects the bucket, whose chain (link at
/// entry `+0xc`) is walked comparing names byte-wise like `strcmp`.
/// Entries with a zero word at `+4` compare against the empty-name
/// constant. A match returns the entry plus 8; an empty table, an empty
/// bucket or a full walk without a match returns 0.
///
/// Original: 0x00887860 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00887860(this: u32, namepp: u32) -> u32 {
    unsafe {
        const BUCKETS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const ENTRY_NAME: u32 = 0x00;
        const ENTRY_KIND: u32 = 0x04;
        const ENTRY_NEXT: u32 = 0x0c;
        const HIT_BIAS: u32 = 8;
        const EMPTY_NAME_FILE_VA: u32 = 0x00fc_9c85;
        const HASH: u32 = 1;
        let count = ((this + COUNT) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        let name = ((namepp) as *const u32).read_unaligned();
        let h = lf_checker_rt::callee_cdecl!(HASH, u32, name);
        let table = ((this + BUCKETS) as *const u32).read_unaligned();
        let mut e = (table.wrapping_add((h % count).wrapping_mul(4))
            as *const u32)
            .read_unaligned();
        if e == 0 {
            return 0;
        }
        loop {
            let kind = ((e + ENTRY_KIND) as *const u16).read_unaligned();
            let key = if kind == 0 {
                lf_checker_rt::relocated(EMPTY_NAME_FILE_VA)
            } else {
                ((e + ENTRY_NAME) as *const u32).read_unaligned()
            };
            if streq_00887860(name, key) {
                return e.wrapping_add(HIT_BIAS);
            }
            e = ((e + ENTRY_NEXT) as *const u32).read_unaligned();
            if e == 0 {
                return 0;
            }
        }
    }
});

/// Byte-wise string equality with the original's two-bytes-at-a-time loop.
unsafe fn streq_00887860(a: u32, b: u32) -> bool {
    unsafe {
        let mut x = a;
        let mut y = b;
        loop {
            let ca = (x as *const u8).read();
            let cb = (y as *const u8).read();
            if ca != cb {
                return false;
            }
            if ca == 0 {
                return true;
            }
            let da = ((x.wrapping_add(1)) as *const u8).read();
            let db = ((y.wrapping_add(1)) as *const u8).read();
            if da != db {
                return false;
            }
            x = x.wrapping_add(2);
            y = y.wrapping_add(2);
            if da == 0 {
                return true;
            }
        }
    }
}
