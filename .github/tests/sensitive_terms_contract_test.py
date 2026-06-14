from __future__ import annotations

import subprocess
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
FORBIDDEN_TERMS = [
    "\u5b58" + "\u8bc1",
    "recy" + "cloud",
]
TEXT_SUFFIXES = {
    ".bash",
    ".c",
    ".conf",
    ".cpp",
    ".css",
    ".csv",
    ".dockerignore",
    ".env",
    ".go",
    ".gradle",
    ".graphql",
    ".h",
    ".html",
    ".java",
    ".js",
    ".json",
    ".jsx",
    ".kt",
    ".lock",
    ".md",
    ".mjs",
    ".proto",
    ".py",
    ".rs",
    ".sh",
    ".sql",
    ".toml",
    ".ts",
    ".tsx",
    ".txt",
    ".xml",
    ".yaml",
    ".yml",
}
TEXT_FILENAMES = {
    "Dockerfile",
    "LICENSE",
    "Makefile",
    "README",
    "AGENTS.md",
}


def tracked_files() -> list[Path]:
    output = subprocess.check_output(["git", "ls-files"], cwd=ROOT, text=True)
    return [ROOT / line for line in output.splitlines() if line.strip()]


def is_text_candidate(path: Path) -> bool:
    return path.suffix in TEXT_SUFFIXES or path.name in TEXT_FILENAMES


class SensitiveTermsContractTest(unittest.TestCase):
    def test_tracked_text_files_do_not_expose_internal_sensitive_terms(self):
        offenders: list[str] = []
        for path in tracked_files():
            if not is_text_candidate(path):
                continue
            try:
                content = path.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                continue
            lowered = content.lower()
            for term in FORBIDDEN_TERMS:
                if term in lowered or term in content:
                    offenders.append(str(path.relative_to(ROOT)))
                    break

        self.assertEqual([], offenders)


if __name__ == "__main__":
    unittest.main()
