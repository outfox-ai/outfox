#!/usr/bin/env python3
"""
Sync files from ../async-openai/async-openai/src/ to crates/openai/src/
with specific renaming rules for existing files.
"""

import os
import shutil
import filecmp
from pathlib import Path
from typing import Tuple, List

# Configuration
SRC_ROOT = Path("../async-openai/async-openai/src")
DST_ROOT = Path("crates/openai/src")


def get_relative_files(root_dir: Path) -> List[Path]:
    """Get all files relative to root_dir"""
    files = []
    for item in root_dir.rglob("*"):
        if item.is_file():
            files.append(item.relative_to(root_dir))
    return sorted(files)


def ensure_dir_exists(path: Path) -> None:
    """Ensure parent directory exists"""
    path.parent.mkdir(parents=True, exist_ok=True)


def files_are_identical(src: Path, dst: Path) -> bool:
    """Compare two files, return True if identical"""
    try:
        return filecmp.cmp(src, dst, shallow=False)
    except Exception:
        return False


def sync_files() -> Tuple[int, int, int, int, List[str]]:
    """
    Sync files according to rules.
    Returns: (copied_directly, renamed_identical, renamed_different, conflicts, errors)
    """
    src_files = get_relative_files(SRC_ROOT)

    copied_directly = 0
    renamed_identical = 0
    renamed_different = 0
    conflicts = 0
    errors = []

    for src_rel in src_files:
        src_path = SRC_ROOT / src_rel
        dst_path = DST_ROOT / src_rel
        dst_new_path = dst_path.parent / f"{dst_path.name}.new.rs"

        try:
            if not dst_path.exists():
                # File doesn't exist in destination - copy directly
                ensure_dir_exists(dst_path)
                shutil.copy2(src_path, dst_path)
                copied_directly += 1
                print(f"COPIED: {src_rel}")
            else:
                # File exists - need to create .new.rs version
                if dst_new_path.exists():
                    # Conflict: .new.rs already exists
                    conflicts += 1
                    errors.append(f"CONFLICT: {src_rel} - .new.rs already exists")
                    print(f"CONFLICT: {src_rel}")
                else:
                    # Create .new.rs version
                    ensure_dir_exists(dst_new_path)
                    shutil.copy2(src_path, dst_new_path)

                    # Check if identical
                    if files_are_identical(src_path, dst_path):
                        renamed_identical += 1
                        print(f"RENAMED (identical): {src_rel} -> {src_rel}.new.rs")
                    else:
                        renamed_different += 1
                        print(f"RENAMED (different): {src_rel} -> {src_rel}.new.rs")

        except Exception as e:
            errors.append(f"ERROR processing {src_rel}: {str(e)}")
            print(f"ERROR: {src_rel} - {str(e)}")

    return copied_directly, renamed_identical, renamed_different, conflicts, errors


def main():
    print("=" * 60)
    print("Syncing files from async-openai to outfox")
    print("=" * 60)
    print(f"Source: {SRC_ROOT.resolve()}")
    print(f"Destination: {DST_ROOT.resolve()}")
    print("=" * 60)

    # Verify source exists
    if not SRC_ROOT.exists():
        print(f"ERROR: Source directory does not exist: {SRC_ROOT}")
        return 1

    # Ensure destination exists
    DST_ROOT.mkdir(parents=True, exist_ok=True)

    # Perform sync
    copied, identical, different, conflicts, errors = sync_files()

    # Print summary
    print("\n" + "=" * 60)
    print("SYNC SUMMARY")
    print("=" * 60)
    print(f"Files copied directly: {copied}")
    print(f"Files renamed to .new.rs (identical): {identical}")
    print(f"Files renamed to .new.rs (different): {different}")
    print(f"Total .new.rs files created: {identical + different}")
    print(f"Conflicts encountered: {conflicts}")
    print(f"Errors: {len(errors)}")

    if errors:
        print("\nDetailed errors:")
        for error in errors:
            print(f"  - {error}")

    print("=" * 60)
    return 0 if not errors else 1


if __name__ == "__main__":
    exit(main())
