#!/usr/bin/env python3
"""
Script to sync async-openai files to novel/crates/openai/ with comparison
"""

import os
import shutil
import filecmp


def compare_files(file1, file2):
    """Compare two files, return True if identical"""
    try:
        return filecmp.cmp(file1, file2, shallow=False)
    except:
        return False


def sync_directory(src_dir, dst_dir):
    """Sync files from src_dir to dst_dir with comparison rules"""
    operations = []
    conflicts = []

    for root, dirs, files in os.walk(src_dir):
        # Calculate relative path from src_dir
        rel_path = os.path.relpath(root, src_dir)
        if rel_path == ".":
            rel_path = ""

        # Create corresponding destination directory
        dst_root = os.path.join(dst_dir, rel_path)

        # Ensure destination directory exists
        if not os.path.exists(dst_root):
            os.makedirs(dst_root, exist_ok=True)
            operations.append(f"CREATED DIR: {dst_root}")

        # Process each file
        for file in files:
            src_file = os.path.join(root, file)
            dst_file = os.path.join(dst_root, file)

            if os.path.exists(dst_file):
                # File exists, compare content
                if compare_files(src_file, dst_file):
                    # Files are identical, rename source to .new.rs
                    new_name = file.replace(".rs", ".new.rs")
                    new_src_file = os.path.join(root, new_name)
                    shutil.move(src_file, new_src_file)
                    operations.append(
                        f"RENAMED (identical): {src_file} -> {new_src_file}"
                    )
                else:
                    # Files are different, rename source to .new.rs
                    new_name = file.replace(".rs", ".new.rs")
                    new_src_file = os.path.join(root, new_name)
                    shutil.move(src_file, new_src_file)
                    operations.append(
                        f"RENAMED (different): {src_file} -> {new_src_file}"
                    )
            else:
                # File doesn't exist, copy it
                shutil.copy2(src_file, dst_file)
                operations.append(f"COPIED: {src_file} -> {dst_file}")

    return operations, conflicts


def main():
    # Define source and destination directories
    src_dir = os.path.abspath(
        os.path.join("..", "..", "..", "async-openai", "async-openai", "src")
    )
    dst_dir = os.path.abspath(
        os.path.join("..", "..", "novel", "crates", "openai", "src")
    )

    print("=== Step 3: Syncing files ===")
    print(f"Source: {src_dir}")
    print(f"Destination: {dst_dir}")

    if not os.path.exists(src_dir):
        print(f"Error: Source directory {src_dir} not found")
        return

    if not os.path.exists(dst_dir):
        print(f"Note: Destination directory {dst_dir} doesn't exist, will be created")
        os.makedirs(dst_dir, exist_ok=True)

    # Perform sync
    operations, conflicts = sync_directory(src_dir, dst_dir)

    print("\n=== Operations ===")
    for op in operations:
        print(f"  {op}")

    print(f"\n=== Summary ===")
    print(f"Operations performed: {len(operations)}")
    print(f"Conflicts: {len(conflicts)}")

    if conflicts:
        print("\nConflicts:")
        for conflict in conflicts:
            print(f"  {conflict}")


if __name__ == "__main__":
    main()
