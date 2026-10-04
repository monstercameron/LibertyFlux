"""End-to-end tests of the brief maker and the review and experiment drivers on a synthetic tree.

pefile is not part of the standard library and the batch lock needs Windows, so both are replaced by stand-ins;
everything else (file layout, queue rules, templates) is the real code.
"""

import contextlib
import io
import json
import os
import sys
import tempfile
import types
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import common

IMAGE_BASE, TEXT_RVA, ENC_OFFSET = 0x400000, 0x1000, 0xFB000
ENC_END = IMAGE_BASE + TEXT_RVA + ENC_OFFSET


class FakePE:
    def __init__(self, path, fast_load=True):
        self.sections = [types.SimpleNamespace(Name=b".text\0\0\0", VirtualAddress=TEXT_RVA)]
        self.OPTIONAL_HEADER = types.SimpleNamespace(ImageBase=IMAGE_BASE)


def import_with_fake_pefile(name):
    """Import a driver that needs pefile, with the stand-in in place."""
    with mock.patch.dict(sys.modules, {"pefile": types.SimpleNamespace(PE=FakePE)}):
        if name in sys.modules:
            del sys.modules[name]
        return __import__(name)


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data), encoding="utf-8")


class Tree(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.scratch = self.root / ".artifacts" / "scratch"
        self.coord = self.scratch / "coordinator"
        self.lists = self.coord / "lists"
        self.lists.mkdir(parents=True)
        self.env = mock.patch.dict(os.environ, {"LIBERTYFLUX_ROOT": str(self.root)})
        self.env.start()
        for key in ("LF_SMALL_N", "LF_BIG_BYTES", "LF_BIG_MAX", "LF_EXP_TAG", "LF_EXP_NOTE", "LIBERTYFLUX_LISTS_DIR",
                    "LIBERTYFLUX_BRIEFS_DIR"):
            os.environ.pop(key, None)

    def tearDown(self):
        self.env.stop()
        self.tmp.cleanup()

    def inventory(self, groups):
        """groups: [(subsystem, count, size)] laid out after the encrypted range."""
        functions, features, address = [], [], ENC_END + 0x1000
        for sub, count, size in groups:
            for _ in range(count):
                functions.append({"start": f"0x{address:08X}", "size": size, "class": "confirmed-ok", "kind": "game"})
                features.append({"a": f"0x{address:08X}", "category": "as-is", "blockers": [], "name": None, "sub": sub,
                                 "level": 0, "n_dcallees": 0})
                address += size + 16
            address += 0x10000
        write(self.scratch / "f-boundaries" / "functions_v2.json", functions)
        write(self.scratch / "q-order" / "features.json", features)
        return functions


class TestMakeBriefsProd(Tree):
    def setUp(self):
        super().setUp()
        self.mod = import_with_fake_pefile("make_briefs_prod")
        for name, value in (("ROOT", self.root), ("OUT", self.coord / "briefs"), ("LISTS", self.lists),
                            ("SCRATCH", self.scratch), ("COORD", self.coord)):
            patch = mock.patch.object(self.mod, name, value)
            patch.start()
            self.addCleanup(patch.stop)
        lock = mock.patch.object(common, "batch_lock", lambda lists: contextlib.nullcontext())
        lock.start()
        self.addCleanup(lock.stop)
        write(self.scratch / "f-leaf-candidates" / "pilot_50.json", [])
        write(self.scratch / "p0-natives" / "natives.json", [])
        write(self.scratch / "f-library-ranges" / "library_functions.json", [])

    def run_maker(self, *counts):
        out = io.StringIO()
        with mock.patch.object(sys, "argv", ["make_briefs_prod.py", *map(str, counts)]), contextlib.redirect_stdout(out):
            self.mod.main()
        return out.getvalue()

    def batch(self, lane):
        return json.loads((self.lists / f"{lane}.json").read_text())

    def test_default_batch_is_40(self):
        self.assertEqual(self.mod.SMALL_PER_LANE, 40)
        self.assertIn("40 neighbouring functions", self.mod.__doc__)
        self.inventory([("audio", 45, 60)])
        out = self.run_maker(0, 1)
        self.assertIn("LANES r-s01", out)
        self.assertEqual(len(self.batch("r-s01")), 40)
        self.assertTrue((self.lists / "r-s01.v4").exists())
        self.assertTrue((self.coord / "briefs" / "r-s01.txt").exists())

    def test_remnants_are_served(self):
        functions = self.inventory([("audio", 50, 60), ("zz", 5, 60)])
        self.run_maker(0, 2)
        # Round-robin by subsystem: audio's full batch, then zz, which has only a remnant.
        self.assertEqual((len(self.batch("r-s01")), len(self.batch("r-s02"))), (40, 5))
        self.run_maker(0, 3)
        self.assertEqual(len(self.batch("r-s03")), 10)  # audio's remnant, once its full batches are gone
        self.assertFalse((self.lists / "r-s04.json").exists())  # nothing left to hand out
        handed = [e["start"] for n in (1, 2, 3) for e in self.batch(f"r-s{n:02d}")]
        self.assertEqual(sorted(handed), sorted(f["start"] for f in functions))

    def test_env_override_still_works(self):
        self.inventory([("audio", 45, 60)])
        with mock.patch.dict(os.environ, {"LF_SMALL_N": "20"}):
            self.run_maker(0, 1)
        self.assertEqual(len(self.batch("r-s01")), 20)


class TestReviewMetrics(Tree):
    def test_reads_the_tracked_verified_index(self):
        import review_metrics
        functions = self.inventory([("audio", 4, 100)])
        index = [{"address": functions[0]["start"], "checker": "version 4", "file": "functions/x.rs"},
                 {"address": functions[1]["start"], "checker": "version 1", "file": "functions/y.rs"}]
        write(self.root / "rewrites" / "verified" / "index.json", index)
        write(self.root / "docs" / "data" / "progress.json",
              {"stages": {"named": 1, "rewritten": 2, "verified": 1}, "functions": {"game": 4}})
        out = io.StringIO()
        with mock.patch.object(sys, "argv", ["review_metrics.py", "60"]), contextlib.redirect_stdout(out):
            review_metrics.main()
        history = json.loads((self.coord / "review_history.json").read_text())
        self.assertEqual(history[-1]["verified_bytes"], 100)  # only the version 4 entry counts
        self.assertIn("25.0% of bytes", out.getvalue())
        self.assertFalse((self.root / "rewrites" / "pending").exists())


class TestRunExperiments(Tree):
    def test_families_skip_verified_functions(self):
        mod = import_with_fake_pefile("run_experiments")
        functions = self.inventory([("audio", 30, 48)])
        write(self.root / "rewrites" / "verified" / "index.json", [{"address": functions[0]["start"], "checker": "version 4"}])
        with mock.patch.object(mod, "ROOT", self.root), mock.patch.object(mod, "SCRATCH", self.scratch), \
                mock.patch.object(mod, "LISTS", self.lists), contextlib.redirect_stdout(io.StringIO()):
            runs = mod.families(1)
        self.assertEqual(len(runs), 1)
        self.assertEqual(len(runs[0]), 29)
        self.assertNotIn(functions[0]["start"], {f["start"] for f in runs[0]})


if __name__ == "__main__":
    unittest.main()
