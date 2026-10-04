// original: 0x009766a0 audio_entity_pair_max_reach
/// Maximum reach of an audio entity pair.
///
/// The game keeps two optional sub-entities on the pair object (at +0x20 and
/// +0x24). An entity contributes only when present and its kind field
/// (bits 6..10 of the word at +0x28) is 2, 3 or 4. A contributing entity is
/// sampled through its slot-59 virtual function into a small buffer, then a
/// helper resolves that buffer to three coordinates; the contribution is the
/// length of that vector. Absent or wrong-kind entities contribute 0.0. The
/// result is the larger of the two contributions (ties and unordered cases
/// yield the second one, matching the original's `comiss`/`jbe`).
export!(stdcall, rb37_9766a0(pair: *const u8) -> f32 {
    unsafe {
        let first = sample_entity(*((pair as *const u8).add(0x20) as *const u32), 2);
        let second = sample_entity(*((pair as *const u8).add(0x24) as *const u32), 3);
        let d1 = dist3(first);
        let d2 = dist3(second);
        if d1 > d2 { d1 } else { d2 }
    }
});
