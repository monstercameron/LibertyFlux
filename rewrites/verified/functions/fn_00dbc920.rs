// original: 0x00DBC920 heap_sift_down (proposed)
/// Sink the record at `hole` down a min-heap ordered by the key field.
///
/// The object points at a record array through its second word and holds the
/// last valid index in its first; records are 48 bytes and compare by the
/// dword at offset `0x20` as an ordered float (`NaN` loses every comparison,
/// matching the original's `comiss`/`ja` pair). The smaller child is moved
/// into the hole until no child is smaller, then the saved record is stored
/// at the final hole. Each record's last dword points at a cell holding the
/// record's address; moved records repoint their cell. Returns the saved
/// record's backlink when anything moved, otherwise index-arithmetic residue
/// (`2*hole+1`, or `6*hole` when the right child exists).
///
/// Quirk reproduced by contract: the original never saves the record fields
/// at offsets `0x0c`/`0x1c`; the final store reloads them from an
/// uninitialized word of its own stack frame. The contract defines that word
/// as zero (`stack_fill`), so the rewrite stores zero there.
lf_checker_rt::export!(thiscall, rw_dbc920(this_ptr: u32, hole: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 48;
        const FIELDS: usize = 12;
        const KEY: usize = 8;
        const LINK: usize = 11;
        let base = ((this_ptr.wrapping_add(8)) as *const u32).read();
        let count = ((this_ptr.wrapping_add(4)) as *const u32).read();
        let rec = |idx: u32| base.wrapping_add(idx.wrapping_mul(STRIDE)) as *mut u32;
        let key_of = |idx: u32| f32::from_bits((rec(idx) as *const u32).add(KEY).read());
        let h0 = rec(hole);
        let mut saved = [0u32; FIELDS];
        let mut k = 0usize;
        while k < FIELDS {
            saved[k] = (h0 as *const u32).add(k).read();
            k += 1;
        }
        let mut selected = hole;
        let mut h = hole;
        loop {
            let left = h.wrapping_mul(2);
            if left <= count && key_of(h) > key_of(left) {
                selected = left;
            }
            let right = h.wrapping_mul(2).wrapping_add(1);
            if right <= count && key_of(selected) > key_of(right) {
                selected = right;
            }
            if selected == h {
                break;
            }
            let src = rec(selected);
            let dst = rec(h);
            k = 0;
            while k < FIELDS {
                (dst as *mut u32).add(k).write((src as *const u32).add(k).read());
                k += 1;
            }
            let link = (dst as *const u32).add(LINK).read() as *mut u32;
            link.write(dst as u32);
            (src as *mut u32).add(KEY).write(saved[KEY]);
            h = selected;
        }
        if h == hole {
            let right1 = hole.wrapping_mul(2).wrapping_add(1);
            if right1 > count {
                right1
            } else {
                hole.wrapping_mul(6)
            }
        } else {
            let dst = rec(h);
            k = 0;
            while k < FIELDS {
                // Fields 3 (+0x0c) and 7 (+0x1c) read the original's
                // uninitialized stack slot: zero under this contract.
                let v = if k == 3 || k == 7 { 0 } else { saved[k] };
                (dst as *mut u32).add(k).write(v);
                k += 1;
            }
            let link = saved[LINK] as *mut u32;
            link.write(dst as u32);
            saved[LINK]
        }
    }
});
