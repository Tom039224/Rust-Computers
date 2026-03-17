#!/usr/bin/env python3
"""
Add 🚧 marks to unimplemented features in API documentation.
"""

import os
import re
from pathlib import Path

def mark_unimplemented_methods(content, is_japanese=False):
    """Add 🚧 marks to method signatures that are not implemented."""
    
    # Pattern to match method headings with async_* variants
    # Example: ### `async_open(channel)`
    async_pattern = r'(###\s+`async_[^`]+`)'
    
    # Replace with marked version if not already marked
    def replace_async(match):
        heading = match.group(1)
        if '🚧' not in heading:
            return heading + ' 🚧'
        return heading
    
    content = re.sub(async_pattern, replace_async, content)
    
    # Mark specific unimplemented methods based on peripheral type
    # is_wireless, get_names_remote, get_item_limit
    unimplemented_methods = [
        r'(###\s+`is_wireless\([^)]*\)[^`]*`)',
        r'(###\s+`get_names_remote\([^)]*\)[^`]*`)',
        r'(###\s+`getNamesRemote\([^)]*\)[^`]*`)',
        r'(###\s+`get_item_limit\([^)]*\)[^`]*`)',
        r'(###\s+`getItemLimit\([^)]*\)[^`]*`)',
    ]
    
    for pattern in unimplemented_methods:
        def replace_method(match):
            heading = match.group(1)
            if '🚧' not in heading:
                return heading + ' 🚧'
            return heading
        content = re.sub(pattern, replace_method, content)
    
    # Mark event sections
    event_patterns = [
        r'(###\s+`modem_message`)',
        r'(###\s+`monitor_resize`)',
        r'(###\s+`speaker_audio_empty`)',
        r'(###\s+`chat`)',
        r'(###\s+`playerJoin`)',
        r'(###\s+`playerLeave`)',
    ]
    
    for pattern in event_patterns:
        def replace_event(match):
            heading = match.group(1)
            if '🚧' not in heading:
                return heading + ' 🚧'
            return heading
        content = re.sub(pattern, replace_event, content)
    
    return content

def process_file(file_path, is_japanese=False):
    """Process a single documentation file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Check if file needs processing (has async_ methods or unimplemented features)
    if 'async_' not in content and 'is_wireless' not in content and 'get_names_remote' not in content:
        return False
    
    new_content = mark_unimplemented_methods(content, is_japanese)
    
    if new_content != content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        return True
    
    return False

def main():
    """Main function."""
    docs_root = Path("docs")
    
    # Process English docs
    print("Processing English documentation...")
    api_en = docs_root / "api_en"
    count = 0
    for md_file in api_en.rglob("*.md"):
        if md_file.name == "core.md":
            continue
        if process_file(md_file, is_japanese=False):
            print(f"  Marked unimplemented features in {md_file}")
            count += 1
    print(f"  Updated {count} files")
    
    # Process Japanese docs
    print("\nProcessing Japanese documentation...")
    api_ja = docs_root / "api_ja"
    count = 0
    for md_file in api_ja.rglob("*.md"):
        if md_file.name == "core.md":
            continue
        if process_file(md_file, is_japanese=True):
            print(f"  Marked unimplemented features in {md_file}")
            count += 1
    print(f"  Updated {count} files")
    
    print("\nDone!")

if __name__ == "__main__":
    main()
