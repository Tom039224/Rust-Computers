#!/usr/bin/env python3
"""
Remove Lua signatures and examples from Rust API documentation.
This script processes all Markdown files in docs/api_en and docs/api_ja directories.
"""

import os
import re
import sys
from pathlib import Path

def remove_lua_signatures(content):
    """Remove Lua signatures and replace Lua examples with Rust examples."""
    
    # Remove Lua signature sections
    # Pattern: **Lua Signature:** followed by code block
    content = re.sub(
        r'\*\*Lua Signature:\*\*\s*```lua[\s\S]*?```\s*',
        '',
        content
    )
    
    # Remove Lua examples and replace with Rust examples where possible
    # This is more complex and may need manual review
    # For now, just remove Lua code blocks that are clearly examples
    # Pattern: **Example:** followed by Lua code block
    content = re.sub(
        r'\*\*Example:\*\*\s*```lua[\s\S]*?```\s*',
        '**Example:**\n```rust\n// Rust example to be added\n```\n',
        content
    )
    
    # Remove standalone Lua code blocks in examples sections
    content = re.sub(
        r'```lua\n[\s\S]*?\n```\s*',
        '```rust\n// Rust example to be added\n```\n',
        content
    )
    
    # Remove Lua type definitions
    content = re.sub(
        r'### .*Type Definitions.*\s*```lua[\s\S]*?```\s*',
        '',
        content
    )
    
    return content

def process_file(filepath):
    """Process a single Markdown file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        content = remove_lua_signatures(content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Processed: {filepath}")
            return True
        else:
            print(f"No changes: {filepath}")
            return False
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    # Directories to process
    directories = [
        'docs/api_en',
        'docs/api_ja'
    ]
    
    total_processed = 0
    total_changed = 0
    
    for dir_path in directories:
        if not os.path.exists(dir_path):
            print(f"Directory not found: {dir_path}")
            continue
        
        for root, dirs, files in os.walk(dir_path):
            for file in files:
                if file.endswith('.md'):
                    filepath = os.path.join(root, file)
                    total_processed += 1
                    if process_file(filepath):
                        total_changed += 1
    
    print(f"\nSummary:")
    print(f"  Total files processed: {total_processed}")
    print(f"  Files changed: {total_changed}")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())