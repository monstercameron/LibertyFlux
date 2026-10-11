// original: 0x005DA100 sorted_pair_insert_if_absent

/// Insert a key and its paired value into two arrays owned by the receiver.
///
/// The receiver stores an array pointer at `+0x10` and its 16-bit element
/// count at `+0x14`; the paired-value array pointer and count are at `+0x18`
/// and `+0x1c`. Keys are kept in unsigned ascending order. If the key already
/// exists, the function leaves both arrays and counts unchanged and returns
/// false. Otherwise it shifts the key tail, inserts the key, then independently
/// shifts the paired-value tail and inserts that value at the same index. The
/// caller supplies the receiver in ECX and two 32-bit values on the stack; the
/// callee removes those eight stack bytes and returns its result in AL.
lf_checker_rt::export!(thiscall, rw_005da100(this: *mut u8, key: u32, paired_value: u32) -> u32 {
    const KEYS_PTR: usize = 0x10;
    const KEYS_LEN: usize = 0x14;
    const VALUES_PTR: usize = 0x18;
    const VALUES_LEN: usize = 0x1c;

    unsafe {
        let keys = (this.add(KEYS_PTR) as *const *mut u32).read_unaligned();
        let key_len_ptr = this.add(KEYS_LEN) as *mut u16;
        let key_len = key_len_ptr.read_unaligned() as usize;

        let mut insert_at = 0usize;
        while insert_at < key_len {
            let current = keys.add(insert_at).read_unaligned();
            if current == key {
                return 0;
            }
            if current > key {
                break;
            }
            insert_at += 1;
        }

        for slot in (insert_at..key_len).rev() {
            let value = keys.add(slot).read_unaligned();
            keys.add(slot + 1).write_unaligned(value);
        }
        keys.add(insert_at).write_unaligned(key);
        key_len_ptr.write_unaligned((key_len as u16).wrapping_add(1));

        let values = (this.add(VALUES_PTR) as *const *mut u32).read_unaligned();
        let value_len_ptr = this.add(VALUES_LEN) as *mut u16;
        let value_len = value_len_ptr.read_unaligned() as usize;
        if value_len > insert_at {
            for slot in (insert_at..value_len).rev() {
                let value = values.add(slot).read_unaligned();
                values.add(slot + 1).write_unaligned(value);
            }
        }
        values.add(insert_at).write_unaligned(paired_value);
        value_len_ptr.write_unaligned((value_len as u16).wrapping_add(1));
        1
    }
});
