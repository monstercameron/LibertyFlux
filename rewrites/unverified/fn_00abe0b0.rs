// original: 0x00ABE0B0 target_records_validate

/// Return whether every record in the owner's collection satisfies two validity checks.
///
/// The owner points to a collection at byte offset 64. That collection holds
/// a 32-bit record-pointer array at offset 0 and a 16-bit count at offset 4.
/// Each non-null record must have a nonzero word at offset 12 and a zero byte
/// at offset 23. An empty collection is valid. Null record pointers are not a
/// supported input: the original faults when it reads a record field.
/// The function is cdecl with one pointer argument and returns its result in AL.
lf_checker_rt::export!(cdecl, rw_00abe0b0(owner_address: u32) -> u8 {
    unsafe {
        const COLLECTION_FIELD: usize = 0x40;
        const RECORDS_POINTER: usize = 0;
        const RECORD_COUNT: usize = 4;
        const REQUIRED_WORD: usize = 0x0c;
        const REQUIRED_ZERO_BYTE: usize = 0x17;

        let owner = owner_address as *const u8;
        let collection_address = owner.add(COLLECTION_FIELD).cast::<u32>().read_unaligned();
        let collection = collection_address as *const u8;
        let count = collection.add(RECORD_COUNT).cast::<u16>().read_unaligned();
        let records = collection.add(RECORDS_POINTER).cast::<u32>();

        for index in 0..usize::from(count) {
            let record_address = records.add(index).read_unaligned();
            let record = record_address as *const u8;
            let required_word = record.add(REQUIRED_WORD).cast::<u32>().read_unaligned();
            if required_word == 0 || record.add(REQUIRED_ZERO_BYTE).read() != 0 {
                return 0;
            }
        }

        1
    }
});
