// original: 0x00bc59c0 GET_CAR_DEFORMATION_AT_POS
/// Read the deformation of a car at a position.
///
/// Logs the deformation-sample pointer (script argument 4) into the context's
/// sample list, copies the sample's three words into the context's current
/// row, then forwards a position vector (arguments 1-3) in ECX with the car
/// handle and the new row pointer to the engine implementation.
///
/// The contract skips the ECX address and snapshots the three pointed-to
/// words; the vector words are also compared as stack arguments 1-3, since
/// the original leaves them on the stack above the buried row pointer.
export!(cdecl, rw_00bc59c0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let sample = *args.add(4) as *const u32;
        let count = *(ctx as *const u32).add(3);
        *((ctx as *mut u32).add(4).add(count as usize)) = sample as u32;
        let row = (ctx as *mut u32).add(((count + 2) * 4) as usize);
        *row = *sample;
        *row.add(1) = *sample.add(1);
        *row.add(2) = *sample.add(2);
        let row_ptr = ctx + (count + 2) * 16;
        *((ctx as *mut u32).add(3)) = count + 1;
        let vec = [*args.add(1), *args.add(2), *args.add(3)];
        callee_thiscall!(1, u32, vec.as_ptr() as u32, *args,
                         *args.add(1), *args.add(2), *args.add(3), row_ptr)
    }
});
