#!/bin/bash

# List of all mod.rs files to process
files=(
    "crates/openai/src/admin/mod.rs"
    "crates/openai/src/audio/mod.rs"
    "crates/openai/src/containers/mod.rs"
    "crates/openai/src/evals/mod.rs"
    "crates/openai/src/responses/mod.rs"
    "crates/openai/src/spec/mod.rs"
    "crates/openai/src/spec/realtime/mod.rs"
    "crates/openai/src/types/admin/api_keys/mod.rs"
    "crates/openai/src/types/admin/audit_logs/mod.rs"
    "crates/openai/src/types/admin/certificates/mod.rs"
    "crates/openai/src/types/admin/groups/mod.rs"
    "crates/openai/src/types/admin/invites/mod.rs"
    "crates/openai/src/types/admin/mod.rs"
    "crates/openai/src/types/admin/project_api_keys/mod.rs"
    "crates/openai/src/types/admin/project_rate_limits/mod.rs"
    "crates/openai/src/types/admin/project_service_accounts/mod.rs"
    "crates/openai/src/types/admin/project_users/mod.rs"
    "crates/openai/src/types/admin/projects/mod.rs"
    "crates/openai/src/types/admin/roles/mod.rs"
    "crates/openai/src/types/admin/usage/mod.rs"
    "crates/openai/src/types/admin/users/mod.rs"
    "crates/openai/src/types/assistants/mod.rs"
    "crates/openai/src/types/audio/mod.rs"
    "crates/openai/src/types/batches/mod.rs"
    "crates/openai/src/types/chat/mod.rs"
    "crates/openai/src/types/chatkit/mod.rs"
    "crates/openai/src/types/completions/mod.rs"
    "crates/openai/src/types/containers/mod.rs"
    "crates/openai/src/types/embeddings/mod.rs"
    "crates/openai/src/types/evals/mod.rs"
    "crates/openai/src/types/files/mod.rs"
    "crates/openai/src/types/finetuning/mod.rs"
    "crates/openai/src/types/graders/mod.rs"
    "crates/openai/src/types/images/mod.rs"
    "crates/openai/src/types/mcp/mod.rs"
    "crates/openai/src/types/models/mod.rs"
    "crates/openai/src/types/moderations/mod.rs"
    "crates/openai/src/types/realtime/mod.rs"
    "crates/openai/src/types/responses/mod.rs"
    "crates/openai/src/types/shared/mod.rs"
    "crates/openai/src/types/uploads/mod.rs"
    "crates/openai/src/types/vectorstores/mod.rs"
    "crates/openai/src/types/videos/mod.rs"
    "crates/openai/src/types/webhooks/mod.rs"
    "crates/openai/src/vectorstores/mod.rs"
)

conflicts=()
processed=()

for mod_file in "${files[@]}"; do
    if [ ! -f "$mod_file" ]; then
        echo "Skipping (not found): $mod_file"
        continue
    fi
    
    # Extract folder name from path
    folder=$(dirname "$mod_file")
    folder_name=$(basename "$folder")
    parent_dir=$(dirname "$folder")
    target_file="${parent_dir}/${folder_name}.rs"
    
    echo "Processing: $mod_file -> $target_file"
    
    # Read current content
    mod_content=$(cat "$mod_file")
    
    if [ -f "$target_file" ]; then
        # Target file exists
        target_content=$(cat "$target_file")
        
        if [ "$mod_content" = "$target_content" ]; then
            echo "  -> Identical content, moving to .new.rs"
            conflicts+=("$mod_file -> ${target_file}.new.rs (identical content)")
            mv "$mod_file" "${target_file}.new.rs"
        else
            echo "  -> Different content, renaming to .new.rs"
            conflicts+=("$mod_file -> ${target_file}.new.rs (different content)")
            mv "$mod_file" "${target_file}.new.rs"
        fi
    else
        # Target doesn't exist
        echo "  -> Moving to new location"
        mv "$mod_file" "$target_file"
        processed+=("$mod_file -> $target_file")
    fi
    
    # Try to remove parent directory if empty
    if [ -d "$folder" ] && [ -z "$(ls -A "$folder")" ]; then
        echo "  -> Removing empty directory: $folder"
        rmdir "$folder"
    fi
done

# Summary
echo ""
echo "========== SUMMARY =========="
echo "Successfully processed:"
for item in "${processed[@]}"; do
    echo "  ✓ $item"
done

echo ""
echo "Conflicts detected (moved to .new.rs):"
for item in "${conflicts[@]}"; do
    echo "  ⚠ $item"
done

echo ""
echo "Total processed: ${#processed[@]}"
echo "Total conflicts: ${#conflicts[@]}"