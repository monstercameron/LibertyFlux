// original: 0x00948170 stamp_entry_fields
/// Stamp a computed field into each live entry of a 16-entry table.
///
/// Entries with a negative head word are skipped. Each live entry is built
/// through two helpers, then measured twice through its virtual probe; the
/// two readings are folded (signed mod-16 chain, truncated divide, shift)
/// into a bit-field that is masked into the entry's parameter word.
lf_checker_rt::export!(thiscall, rw_00948170(this_ptr: u32) -> () {
    unsafe {
        let mut esi = this_ptr.wrapping_add(4);
        let mut n = 0;
        while n < 16 {
            let head = *(esi as *const u32);
            if (head as i32) >= 0 {
                let a = lf_checker_rt::callee_cdecl!(1, u32, 8, 0);
                let ebx: u32;
                if a == 0 {
                    ebx = 0;
                } else {
                    let e = *(esi as *const u32);
                    ebx = lf_checker_rt::callee_thiscall!(2, u32, a, e);
                }
                let vt = *(ebx as *const u32);
                let tgt = *((vt.wrapping_add(8)) as *const u32);
                let probe: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let m1 = (probe(ebx) as i32) % 16;
                let m2 = (0x10i32.wrapping_sub(m1)) % 16;
                let sum = (probe(ebx) as i32).wrapping_add(m2);
                let v = ((sum / 16) << 14) as u32;
                let slot = ebx.wrapping_add(4) as *mut u32;
                let old = *slot;
                *slot = old ^ ((v ^ old) & 0x01ff_c000);
            }
            esi = esi.wrapping_add(0x20);
            n += 1;
        }
    }
});
