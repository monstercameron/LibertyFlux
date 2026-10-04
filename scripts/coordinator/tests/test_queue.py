"""Tests for queue.py on a small synthetic inventory.

Rules under test: no function is handed out twice, holds (library, runtime,
encrypted, uncheckable) are respected, and families of neighbouring functions
stay together in one batch.
"""

import random
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import queue as queue_rules
from common import va

BASE = 0x10000000


def feature(addr, size=100, sub="audio", category="as-is", blockers=(), name=None, callees=1):
    return {"a": f"0x{addr:08X}", "size": size, "name": name, "sub": sub, "level": 0,
            "n_dcallees": callees, "category": category, "blockers": list(blockers)}


def confirmed(addr, size=100):
    return {"start": f"0x{addr:08X}", "size": size, "class": "confirmed-ok"}


def synthetic(n_subs=3, per_sub=40, size=100):
    """Three subsystems of contiguous neighbouring functions."""
    features, conf = [], {}
    addr = BASE
    for s in range(n_subs):
        for _ in range(per_sub):
            features.append(feature(addr, size, sub=f"sub{s}"))
            conf[addr] = confirmed(addr, size)
            addr += size + 16
        addr += 0x10000  # subsystems live far apart
    return features, conf


class TestFilterPool(unittest.TestCase):
    def test_holds_respected(self):
        features, conf = synthetic(1, 10)
        addrs = sorted(conf)
        features[6]["category"] = "needs-game"  # uncheckable category
        features[7]["category"] = "needs-extension"  # handled blockers pass
        features[7]["blockers"] = ["X-indirect-calls"]
        features[8]["category"] = "needs-extension"
        features[8]["blockers"] = ["X-indirect-calls", "X-thread-local"]
        conf[addrs[9]] = confirmed(addrs[9], 8)  # under 12 bytes
        small, big, held = queue_rules.filter_pool(
            features, conf, assigned={addrs[2]}, handlers={addrs[4]}, replaced={addrs[3]},
            crt_block=(addrs[5], addrs[5] + 1), enc_end=addrs[2])
        got = {va(f["start"]) for f in small + big}
        self.assertEqual(got, {addrs[7]})  # everything else held out for one reason each
        self.assertGreaterEqual(held, 2)  # library hold + runtime block hold

    def test_small_big_split(self):
        features = [feature(BASE, 100), feature(BASE + 0x1000, 249), feature(BASE + 0x2000, 250),
                    feature(BASE + 0x3000, 1500)]
        conf = {BASE: confirmed(BASE, 100), BASE + 0x1000: confirmed(BASE + 0x1000, 249),
                BASE + 0x2000: confirmed(BASE + 0x2000, 250), BASE + 0x3000: confirmed(BASE + 0x3000, 1500)}
        small, big, _ = queue_rules.filter_pool(
            features, conf, assigned=set(), handlers=set(), replaced=set(), crt_block=(0, 0), enc_end=0)
        self.assertEqual({f["size"] for f in small}, {100, 249})
        self.assertEqual({f["size"] for f in big}, {250, 1500})

    def test_unconfirmed_excluded(self):
        features = [feature(BASE, 100)]
        small, big, _ = queue_rules.filter_pool(
            features, {}, assigned=set(), handlers=set(), replaced=set(), crt_block=(0, 0), enc_end=0)
        self.assertEqual(small + big, [])


class TestChunks(unittest.TestCase):
    def test_families_kept_together(self):
        features, conf = synthetic(2, 30)
        small, _, _ = queue_rules.filter_pool(
            features, conf, assigned=set(), handlers=set(), replaced=set(), crt_block=(0, 0), enc_end=0)
        found = queue_rules.chunks_of(small, lambda run: len(run) >= 20)
        self.assertEqual(set(found), {"sub0", "sub1"})
        for sub, chunks in found.items():
            for chunk in chunks:
                self.assertEqual(len(chunk), 20)
                self.assertTrue(all(f["sub"] == sub for f in chunk))
                span = va(chunk[-1]["start"]) - va(chunk[0]["start"])
                self.assertLessEqual(span, 0x8000)

    def test_run_breaks_on_subsystem_change(self):
        funcs = [{"start": f"0x{BASE + i * 0x100:08X}", "sub": "a" if i < 5 else "b"} for i in range(10)]
        found = queue_rules.chunks_of(funcs, lambda run: len(run) >= 5)
        self.assertEqual(set(found), {"a", "b"})
        self.assertEqual(len(found["a"][0]), 5)
        self.assertEqual(len(found["b"][0]), 5)

    def test_run_breaks_beyond_span(self):
        funcs = [{"start": f"0x{BASE:08X}", "sub": "a"},
                 {"start": f"0x{BASE + 0x8001:08X}", "sub": "a"}]
        found = queue_rules.chunks_of(funcs, lambda run: len(run) >= 2)
        self.assertEqual(found, {})

    def test_pick_spreads_over_subsystems(self):
        found = {"a": [[1], [2], [3]], "b": [[4], [5], [6]]}
        picked = queue_rules.pick(found, 4, random.Random(1))
        subs = [sub for sub, _ in picked]
        self.assertEqual(len(picked), 4)
        self.assertEqual(sorted(subs), ["a", "a", "b", "b"])


class TestNoDoubleHandout(unittest.TestCase):
    def test_successive_calls_are_disjoint(self):
        features, conf = synthetic(2, 60)
        handed = set()
        for _ in range(3):
            small, _, _ = queue_rules.filter_pool(
                features, conf, assigned=handed, handlers=set(), replaced=set(), crt_block=(0, 0), enc_end=0)
            for _, chunk in queue_rules.pick(queue_rules.chunks_of(small, lambda r: len(r) >= 20), 2, random.Random(7)):
                for f in chunk:
                    self.assertNotIn(va(f["start"]), handed)
                    handed.add(va(f["start"]))
        self.assertEqual(len(handed), 120)  # everything handed out exactly once


class TestNativesAndNaming(unittest.TestCase):
    def test_native_batches(self):
        natives = [{"name": f"n{i:02d}"} for i in range(60)]
        batches = queue_rules.split_native_batches(natives, 25, 3)
        self.assertEqual([len(b) for b in batches], [25, 25, 10])
        self.assertEqual(batches[0][0]["name"], "n00")  # sorted by name

    def test_naming_batches_spread(self):
        todo = list(range(100, 200))
        batches = queue_rules.naming_batches(todo, 2, 10)
        self.assertEqual(len(batches), 2)
        self.assertEqual(batches[0], list(range(100, 110)))
        self.assertEqual(batches[1], list(range(150, 160)))

    def test_naming_batches_short(self):
        self.assertEqual(queue_rules.naming_batches([], 2, 10), [])
        self.assertEqual(queue_rules.naming_batches([1, 2], 0, 10), [])

    def test_next_lane_numbers(self):
        stems = ["r-n01", "r-n12", "r-s03", "r-b07", "q-01", "nm-04", "natives_remaining"]
        self.assertEqual(queue_rules.next_lane_numbers(stems), {"n": 12, "s": 3, "b": 7})


class TestTails(unittest.TestCase):
    """Remnant runs (cut short by a subsystem's end or the span) used to be dropped on every call."""

    def funcs(self, n, sub="a", start=BASE, step=0x100):
        return [{"start": f"0x{start + i * step:08X}", "sub": sub, "size": 50} for i in range(n)]

    def test_chunks_of_unchanged(self):
        funcs = self.funcs(12)
        full = lambda run: len(run) >= 5  # noqa: E731
        found, tails = queue_rules.chunks_and_tails(funcs, full)
        self.assertEqual(dict(found), dict(queue_rules.chunks_of(funcs, full)))
        self.assertEqual([len(r) for r in tails["a"]], [2])

    def test_tails_from_subsystem_change_span_and_end(self):
        funcs = self.funcs(3, "a") + self.funcs(2, "b", BASE + 0x1000) + self.funcs(1, "b", BASE + 0x20000)
        found, tails = queue_rules.chunks_and_tails(funcs, lambda run: len(run) >= 5)
        self.assertEqual(dict(found), {})
        self.assertEqual({k: [len(r) for r in v] for k, v in tails.items()}, {"a": [3], "b": [2, 1]})

    def test_every_function_in_exactly_one_batch(self):
        funcs = self.funcs(13, "a") + self.funcs(7, "b", BASE + 0x40000) + self.funcs(4, "b", BASE + 0x80000)
        full = lambda run: len(run) >= 5  # noqa: E731
        found, tails = queue_rules.chunks_and_tails(funcs, full)
        seen = [f["start"] for runs in list(found.values()) + list(tails.values()) for run in runs for f in run]
        self.assertEqual(sorted(seen), sorted(f["start"] for f in funcs))

    def test_merge_tails_packs_to_batch_size(self):
        tails = {"b": [self.funcs(3, "b"), self.funcs(4, "b", BASE + 0x10000)]}
        merged = queue_rules.merge_tails(tails, lambda run: len(run) >= 5)
        self.assertEqual([len(b) for b in merged["b"]], [5, 2])
        starts = [va(f["start"]) for f in merged["b"][0] + merged["b"][1]]
        self.assertEqual(starts, sorted(starts))

    def test_tails_offered_only_once_full_batches_are_gone(self):
        full = lambda run: len(run) >= 5  # noqa: E731
        found, tails = queue_rules.chunks_and_tails(self.funcs(12, "a") + self.funcs(3, "b", BASE + 0x40000), full)
        offered = queue_rules.with_tails(found, tails, full)
        self.assertEqual([len(b) for b in offered["a"]], [5, 5])  # a still has full batches: its tail waits
        self.assertEqual([len(b) for b in offered["b"]], [3])     # b has none: its remnant is served
        # Once a's full batches are handed out, the next call sees only the remnant and serves it.
        found, tails = queue_rules.chunks_and_tails(self.funcs(2, "a", BASE + 0xA00), full)
        self.assertEqual([len(b) for b in queue_rules.with_tails(found, tails, full)["a"]], [2])

    def test_whole_pool_drains_over_successive_calls(self):
        features, conf = synthetic(3, 47)  # 47 per subsystem: 2 full batches of 20 and a remnant of 7 each
        handed = set()
        full = lambda run: len(run) >= 20  # noqa: E731
        for _ in range(10):
            small, _, _ = queue_rules.filter_pool(
                features, conf, assigned=handed, handlers=set(), replaced=set(), crt_block=(0, 0), enc_end=0)
            for _, chunk in queue_rules.pick(queue_rules.with_tails(*queue_rules.chunks_and_tails(small, full), full),
                                             2, random.Random(3)):
                for f in chunk:
                    self.assertNotIn(va(f["start"]), handed)
                    handed.add(va(f["start"]))
        self.assertEqual(len(handed), 3 * 47)


if __name__ == "__main__":
    unittest.main()
