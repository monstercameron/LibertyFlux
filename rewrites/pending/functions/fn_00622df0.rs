// original: 0x00622df0 net_session_find_by_addr_key
/// Find the session entry matching a composite address key.
///
/// Scans the entry-pointer table at `this+0x1d94` (up to `count` at
/// `this+0x1e14` entries) for the first entry whose tag byte (+0x50)
/// equals the key tag (`key+8`, required to be 3), whose words +0x48/+0x4c
/// are not both zero and equal the key words, and whose subtag (+0x51)
/// equals `key+9`. The key words must not both be zero either. Returns the
/// entry or null.
export!(thiscall, rw_00622df0(this: u32, key: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let count = (base.add(0x1e14) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let bl = (key as *const u8).add(8).read();
        let table = base.add(0x1d94) as *const u32;
        let mut i: i32 = 0;
        while i < count {
            let e = table.add(i as usize).read_unaligned() as *const u8;
            let al = e.add(0x50).read();
            if al == bl && al >= 1 && al == 3 {
                let w0 = (e.add(0x48) as *const u32).read_unaligned();
                let w1 = (e.add(0x4c) as *const u32).read_unaligned();
                if (w0 | w1) != 0 && bl >= 1 && bl == 3 {
                    let ebp = (key as *const u32).read_unaligned();
                    let edi = (key as *const u32).add(1).read_unaligned();
                    if (ebp | edi) != 0
                        && e.add(0x51).read() == (key as *const u8).add(9).read()
                        && w0 == ebp
                        && w1 == edi
                    {
                        return e as u32;
                    }
                }
            }
            i += 1;
        }
        0
    }
});
