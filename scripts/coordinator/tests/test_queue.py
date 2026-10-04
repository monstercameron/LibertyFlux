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


if __name__ == "__main__":
    unittest.main()
