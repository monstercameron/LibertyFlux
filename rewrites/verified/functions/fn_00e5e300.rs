// original: 0x00e5e300 CJ_MOBILE_2
/// Register the three CJ_MOBILE phone nodes with the node registrar.
///
/// Registers the nodes at `0x019D309C`, `0x019D30A8` and `0x019D30B4` with
/// their names (`CJ_MOBILE_1`, `CJ_MOBILE_2`, `CJ_MOBILE_3` in read-only
/// data), in order, and returns the last registration answer.
export!(cdecl, rw_00e5e300() -> u32 {
    unsafe {
        /// First node object and its name (file VAs).
        const OBJ1: u32 = 0x019D309C;
        /// Second node object and its name (file VAs).
        const OBJ2: u32 = 0x019D30A8;
        /// Third node object and its name (file VAs).
        const OBJ3: u32 = 0x019D30B4;
        const NAME1: u32 = 0x00F8E270;
        const NAME2: u32 = 0x00F8D87C;
        const NAME3: u32 = 0x00F8E29C;
        let _: u32 = callee_thiscall!(1, u32, relocated(OBJ1), relocated(NAME1));
        let _: u32 = callee_thiscall!(1, u32, relocated(OBJ2), relocated(NAME2));
        callee_thiscall!(1, u32, relocated(OBJ3), relocated(NAME3))
    }
});
