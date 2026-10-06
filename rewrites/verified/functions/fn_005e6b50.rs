// original: 0x005e6b50 CREATE_EMERGENCY_SERVICES_CAR
/// Create an emergency-services car at a position.
///
/// Builds a four-word request (a tag word plus three coordinates from script
/// arguments 1-3), has the locator call filter it, then forwards the tag with
/// the filtered coordinates to the spawner call. Stores 1 in the return slot
/// when the spawner reports success, else the spawner's out flag.
///
/// The contract skips both request addresses and snapshots the pointed-to
/// words; the locator outputs are scripted, and the spawner out flag stays 0
/// in the proof (it sits below the snapped block, where the stub cannot write).
export!(cdecl, rw_005e6b50(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let mut req = [0xffffffff, *args.add(1), *args.add(2), *args.add(3)];
        callee_cdecl!(1, u32, *args, req.as_mut_ptr() as u32);
        let coords = [req[1], req[2], req[3]];
        let ok = callee_cdecl!(2, u32, req[0], coords.as_ptr() as u32, 0);
        let slot = *(ctx as *const u32);
        *(slot as *mut u32) = u32::from(ok != 0);
        slot
    }
});
