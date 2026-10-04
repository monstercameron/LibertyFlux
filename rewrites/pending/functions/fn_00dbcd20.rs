// original: 0x00DBCD20 record_swap (proposed)
/// Swap two records in a 48-byte-stride table and repair their backlinks.
///
/// Original `0x00DBCD20` is a thiscall taking two indices. The object points
/// at a record array through its second word; each record holds twelve
/// dwords and its last dword points at a cell that must hold the record's
/// own address. The function exchanges the two records field by field and
/// then repoints both backlink cells. Returns the backlink pointer stored
/// in record `j` after the swap.
lf_checker_rt::export!(thiscall, rw_dbcd20(this_ptr: u32, i: u32, j: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 48;
        const FIELDS: usize = 12;
        let base = ((this_ptr.wrapping_add(8)) as *const u32).read();
        let rec_i = base.wrapping_add(i.wrapping_mul(STRIDE)) as *mut u32;
        let rec_j = base.wrapping_add(j.wrapping_mul(STRIDE)) as *mut u32;
        let mut a = [0u32; FIELDS];
        let mut b = [0u32; FIELDS];
        let mut k = 0usize;
        while k < FIELDS {
            a[k] = (rec_i as *const u32).add(k).read();
            b[k] = (rec_j as *const u32).add(k).read();
            k += 1;
        }
        k = 0;
        while k < FIELDS {
            (rec_i as *mut u32).add(k).write(b[k]);
            (rec_j as *mut u32).add(k).write(a[k]);
            k += 1;
        }
        let link_i = (rec_i as *const u32).add(FIELDS - 1).read() as *mut u32;
        link_i.write(rec_i as u32);
        let link_j = (rec_j as *const u32).add(FIELDS - 1).read() as *mut u32;
        link_j.write(rec_j as u32);
        link_j as u32
    }
});
