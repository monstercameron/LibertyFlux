// original: 0x0065bcb0 rage::EnviromentDomeLighting::vf3
use lf_checker_rt::{export, relocated};

/// Search a counted table of fixed-size rows for the supplied resource key.
/// The stack argument points to an owner whose +0x18 field points to a
/// collection; its +0x0C word is the row count, and its +0x08 field points to
/// the row storage. Each 0x30-byte row has two candidate keys at offsets 0 and
/// 4. A direct helper call supplies the lookup key.
export!(stdcall, rw_0065bcb0(table: *const u8) -> u8 {
    const RESOURCE_VA: u32 = 0x00FE2F08;
    const TABLE_FIELD: usize = 0x18;
    const COUNT_FIELD: usize = 0x0C;
    const ROWS_FIELD: usize = 0x08;
    const FIRST_ROW_FIELD: usize = 0x0C;
    const ROW_STRIDE: usize = 0x30;
    const KEY_OFFSETS: [usize; 2] = [0x04, 0x00];

    unsafe {
        let collection = table.add(TABLE_FIELD).cast::<*const u8>().read_unaligned();
        let wanted = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(RESOURCE_VA), 0u32);
        let count = collection.add(COUNT_FIELD).cast::<u16>().read_unaligned() as usize;
        if count == 0 {
            return 0;
        }
        let row_base = collection.add(ROWS_FIELD).cast::<*const u8>().read_unaligned();
        let first_row = row_base.add(FIRST_ROW_FIELD);
        for index in 0..count {
            let row = first_row.add(index * ROW_STRIDE);
            for field in KEY_OFFSETS {
                if row.add(field).cast::<u32>().read_unaligned() == wanted {
                    return 1;
                }
            }
        }
        0
    }
});
