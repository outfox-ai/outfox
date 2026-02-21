#!/usr/bin/env python3
"""
Script to rewrite async-openai codebase by converting mod.rs files to <parent>.rs
"""

import os
import shutil
import sys


def get_mod_files(base_path):
    """Find all mod.rs files"""
    mod_files = []
    for root, dirs, files in os.walk(base_path):
        for file in files:
            if file == "mod.rs":
                full_path = os.path.join(root, file)
                mod_files.append(full_path)
    return sorted(mod_files)


def rewrite_mod_file(mod_path):
    """Rename mod.rs to <parent>.rs and move up one directory"""
    parent_dir = os.path.dirname(mod_path)
    grandparent_dir = os.path.dirname(parent_dir)
    parent_folder_name = os.path.basename(parent_dir)

    new_filename = f"{parent_folder_name}.rs"
    new_path = os.path.join(grandparent_dir, new_filename)

    # Check if target file already exists
    if os.path.exists(new_path):
        return False, f"CONFLICT: {new_path} already exists"

    # Move the file
    try:
        shutil.move(mod_path, new_path)

        # Remove the parent directory if it's empty
        try:
            if len(os.listdir(parent_dir)) == 0:
                os.rmdir(parent_dir)
        except OSError:
            pass  # Directory not empty or can't be removed

        return True, f"RENAMED: {mod_path} -> {new_path}"
    except Exception as e:
        return False, f"ERROR: {mod_path} -> {str(e)}"


def main():
    src_path = os.path.join("..", "..", "..", "async-openai", "async-openai", "src")
    src_path = os.path.abspath(src_path)

    if not os.path.exists(src_path):
        print(f"Error: {src_path} not found")
        sys.exit(1)

    print("=== Step 1: Finding mod.rs files ===")
    mod_files = get_mod_files(src_path)
    print(f"Found {len(mod_files)} mod.rs files:")
    for f in mod_files:
        print(f"  {f}")

    print("\n=== Step 2: Rewriting files ===")
    results = []
    for mod_file in mod_files:
        success, message = rewrite_mod_file(mod_file)
        results.append((success, message))
        print(f"  {message}")

    conflicts = [r for r in results if not r[0] and "CONFLICT" in r[1]]
    errors = [r for r in results if not r[0] and "ERROR" in r[1]]
    successes = [r for r in results if r[0]]

    print(f"\n=== Summary ===")
    print(f"Files processed: {len(results)}")
    print(f"Successful: {len(successes)}")
    print(f"Conflicts: {len(conflicts)}")
    print(f"Errors: {len(errors)}")

    if conflicts:
        print("\nConflicts detected:")
        for _, msg in conflicts:
            print(f"  {msg}")


if __name__ == "__main__":
    main()
