// original: 0x00E48880 title_menu_update_episode_frame

/// Resolve the title-menu frame by its mapped image string and pass the
/// caller's episode value to the frame formatter. The global tree receiver
/// lookup and formatter are both thiscall helpers that remove their one stack
/// argument; this wrapper uses cdecl and leaves its own input for the caller.
lf_checker_rt::export!(cdecl, rw_00e48880(episode_value: u32) -> u32 {
    const MENU_TREE: u32 = 0x0198_1a4c;
    const FRAME_NAME: u32 = 0x00f1_6bb4;
    const TREE_LOOKUP: u32 = 1;
    const FORMAT_EPISODE: u32 = 2;

    let frame_object = lf_checker_rt::callee_thiscall!(
        TREE_LOOKUP,
        u32,
        lf_checker_rt::relocated(MENU_TREE),
        lf_checker_rt::relocated(FRAME_NAME),
    );
    lf_checker_rt::callee_thiscall!(FORMAT_EPISODE, u32, frame_object, episode_value)
});
