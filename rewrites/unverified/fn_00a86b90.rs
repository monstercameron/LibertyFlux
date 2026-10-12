// original: 0x00A86B90 find_first_cam_of_type_32
/// Finds the first camera of query type 32 for this camera manager.
///
/// The 32-bit manager object stores its camera-list pointer at byte offset 4.
/// A null list returns zero without calling the search helper. Otherwise the
/// helper receives the list in ECX, the query type, and a zero query flag;
/// its 32-bit result is returned unchanged.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86b90(manager_object: u32) -> u32 {
    const CAMERA_LIST_OFFSET_WORDS: usize = 1;
    const CAMERA_QUERY_TYPE: u32 = 32;
    const CAMERA_QUERY_FLAGS: u32 = 0;

    let camera_list = unsafe {
        ((manager_object as *const u32).add(CAMERA_LIST_OFFSET_WORDS)).read_unaligned()
    };
    if camera_list == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(1, u32, camera_list, CAMERA_QUERY_TYPE, CAMERA_QUERY_FLAGS)
}
