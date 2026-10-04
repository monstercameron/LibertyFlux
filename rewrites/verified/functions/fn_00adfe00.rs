// original: 0x00adfe00 ui_store_indexed_values
/// Store a value triplet into this object's indexed row.
///
/// Reads the shared row selector, stamps it at 0xc, then stores the first
/// value, the third-slot value, and (third + fourth * 8) into the row's
/// three columns. Returns the selector. The second slot is reserved and
/// unread.
export!(thiscall, rw_00adfe00(this: *mut u8, first: u32, _reserved: u32, second: u32, third: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0x01174790;
        let row = *global::<u32>(SELECTOR);
        *(this.add(0x0c) as *mut u32) = row;
        let col = |off: u32| (this as u32).wrapping_add(off).wrapping_add(row.wrapping_mul(4)) as *mut u32;
        *col(0x1c) = first;
        *col(0x24) = second;
        *col(0x2c) = second.wrapping_add(third.wrapping_mul(8));
        row
    }
});
