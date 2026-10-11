// original: 0x0066ABD0 streaming_find_record_for_pointer

/// Search each record's pointer list for the key at `needle + 0x28`. The
/// receiver stores a 16-bit record count at `+0x08` and a record-array pointer
/// at `+0x04`; records are `0x4c` bytes, with a 16-bit list count at `+0x44`
/// and the list pointer at `+0x40`. Every matching record writes its address
/// plus `0x48` to `needle + 0x2c`, so the last matching record wins. The
/// `thiscall` function returns the receiver's count as an unsigned 32-bit
/// value. Empty receiver or list counts do not read their associated pointers.
const RECEIVER_RECORDS_OFFSET: u32 = 0x04;
const RECEIVER_COUNT_OFFSET: u32 = 0x08;
const RECORD_STRIDE: u32 = 0x4c;
const RECORD_LIST_OFFSET: u32 = 0x40;
const RECORD_LIST_COUNT_OFFSET: u32 = 0x44;
const FOUND_RECORD_OFFSET: u32 = 0x48;
const NEEDLE_KEY_OFFSET: u32 = 0x28;
const NEEDLE_RESULT_OFFSET: u32 = 0x2c;

#[inline(always)]
unsafe fn read_u16(address: u32) -> u32 {
    unsafe { (address as *const u16).read_unaligned() as u32 }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

lf_checker_rt::export!(thiscall, rw_0066abd0(receiver: u32, needle: u32) -> u32 {
    unsafe {
        let record_count = read_u16(receiver.wrapping_add(RECEIVER_COUNT_OFFSET));
        if record_count == 0 {
            return 0;
        }
        let records = read_u32(receiver.wrapping_add(RECEIVER_RECORDS_OFFSET));
        for index in 0..record_count {
            let record = records.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            let list_count = read_u16(record.wrapping_add(RECORD_LIST_COUNT_OFFSET));
            if list_count == 0 {
                continue;
            }
            let list = read_u32(record.wrapping_add(RECORD_LIST_OFFSET));
            for item in 0..list_count {
                let key = read_u32(needle.wrapping_add(NEEDLE_KEY_OFFSET));
                let candidate = read_u32(list.wrapping_add(item.wrapping_mul(4)));
                if candidate == key {
                    write_u32(
                        needle.wrapping_add(NEEDLE_RESULT_OFFSET),
                        record.wrapping_add(FOUND_RECORD_OFFSET),
                    );
                    break;
                }
            }
        }
        record_count
    }
});
