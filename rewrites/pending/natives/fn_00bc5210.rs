// original: 0x00bc5210 ATTACH_CAR_TO_OBJECT_PHYSICALLY
/// Native handler `ATTACH_CAR_TO_OBJECT_PHYSICALLY`.
///
/// Attaches a vehicle to an object physically.
///
/// Handler mechanics: takes the native call context,
/// Passes its call context untouched to the shared physical-attach
/// argument unpacker together with the engine function address.
lf_rn21_rt::export!(cdecl, rw_00bc5210(ctx: u32) -> () {
    lf_rn21_rt::callee_cdecl!(1, u32, lf_rn21_rt::relocated(0x00BC9410), ctx);
});
