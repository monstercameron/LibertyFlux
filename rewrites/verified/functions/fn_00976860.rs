// original: 0x00976860 audio_entity_pair_max_span
/// Larger of two cross-entity distances for an audio entity pair.
///
/// Like `rb37_9766a0`, each of the two optional sub-entities (at +0x20/+0x24
/// on the pair, gated by presence and kind bits 6..10 of [+0x28]) is sampled
/// twice: once through its slot-59 virtual function and once through the
/// shared coordinate helper. The result is the larger of the distance between
/// the two first-samples and the distance between the two second-samples
/// (ties and unordered cases yield the second distance).
export!(stdcall, rb37_976860(pair: *const u8) -> f32 {
    unsafe {
        let ea = *((pair as *const u8).add(0x20) as *const u32);
        let eb = *((pair as *const u8).add(0x24) as *const u32);
        let a = sample_twice(ea, 2);
        let b = sample_twice(eb, 3);
        let d1 = dist3((a.0.0 - b.0.0, a.0.1 - b.0.1, a.0.2 - b.0.2));
        let d2 = dist3((a.1.0 - b.1.0, a.1.1 - b.1.1, a.1.2 - b.1.2));
        if d1 > d2 { d1 } else { d2 }
    }
});
