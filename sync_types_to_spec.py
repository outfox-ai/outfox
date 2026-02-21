#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import shutil
from pathlib import Path


def sync_types_to_spec(types_dir, spec_dir):
    """
    将 types/ 文件夹内容同步到 spec/
    - 如果 spec/ 中不存在该文件/文件夹，直接移动
    - 如果已存在，重命名为 .new2.rs 后缀后移动
    """
    types_path = Path(types_dir)
    spec_path = Path(spec_dir)

    if not types_path.exists():
        print(f"错误: {types_dir} 不存在")
        return

    if not spec_path.exists():
        print(f"错误: {spec_dir} 不存在")
        return

    processed = []
    moved_directly = []
    renamed_moved = []

    print(f"扫描 {types_dir} 文件...\n")

    # 获取所有文件和文件夹（递归）
    all_items = []
    for item in types_path.rglob("*"):
        if item.is_file():
            rel_path = item.relative_to(types_path)
            all_items.append((item, rel_path))

    print(f"找到 {len(all_items)} 个文件\n")

    # 处理每个文件
    for src_file, rel_path in sorted(all_items):
        # 构造目标路径
        dst_path = spec_path / rel_path

        # 确保目标目录存在
        dst_dir = dst_path.parent
        dst_dir.mkdir(parents=True, exist_ok=True)

        # 检查目标文件/文件夹是否已存在
        if dst_path.exists():
            # 已存在，重命名为 .new2.rs
            if dst_path.is_file():
                if str(dst_path).endswith(".rs"):
                    new_name = str(dst_path)[:-3] + ".new2.rs"
                    new_dst = Path(new_name)
                else:
                    # 非 .rs 文件，后面加 .new2
                    new_dst = dst_path.parent / (dst_path.name + ".new2")

                shutil.move(str(src_file), str(new_dst))
                renamed_moved.append((src_file, new_dst))
                print(f"  [重命名] {rel_path} -> {new_dst.relative_to(spec_path)}")
            else:
                # 处理文件夹逻辑（不应该有.rs文件夹，这里忽略）
                pass
        else:
            # 不存在，直接移动
            shutil.move(str(src_file), str(dst_path))
            moved_directly.append((src_file, dst_path))
            print(f"  [移动] {rel_path}")

        processed.append(rel_path)

    # 统计结果
    print(f"\n处理完成:")
    print(f"  直接移动: {len(moved_directly)} 个文件/文件夹")
    print(f"  重命名后移动: {len(renamed_moved)} 个文件/文件夹")

    if moved_directly:
        print(f"\n直接移动的文件:")
        for src, dst in moved_directly[:10]:  # 只显示前10个
            print(f"  {src.relative_to(types_path)} -> {dst.relative_to(spec_path)}")
        if len(moved_directly) > 10:
            print(f"  ... 还有 {len(moved_directly) - 10} 个文件")

    if renamed_moved:
        print(f"\n重命名后移动的文件:")
        for src, dst in renamed_moved[:10]:  # 只显示前10个
            print(f"  {src.relative_to(types_path)} -> {dst.relative_to(spec_path)}")
        if len(renamed_moved) > 10:
            print(f"  ... 还有 {len(renamed_moved) - 10} 个文件")

    return processed, moved_directly, renamed_moved


if __name__ == "__main__":
    types_dir = "crates/openai/src/types"
    spec_dir = "crates/openai/src/spec"

    print(f"开始同步 {types_dir} -> {spec_dir}\n")
    sync_types_to_spec(types_dir, spec_dir)
