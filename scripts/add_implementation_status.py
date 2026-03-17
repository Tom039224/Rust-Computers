#!/usr/bin/env python3
"""
Add implementation status sections to all API documentation files.
"""

import os
import re
from pathlib import Path

# Implementation status data based on EVALUATION_REPORT.md
IMPLEMENTATION_STATUS = {
    "computer_craft": {
        "Modem": {
            "implemented": [
                "book_next_open / read_last_open",
                "book_next_is_open / read_last_is_open",
                "book_next_close / read_last_close",
                "book_next_close_all / read_last_close_all",
                "book_next_transmit / read_last_transmit",
                "book_next_transmit_raw / read_last_transmit_raw",
                "book_next_try_receive_raw / read_last_try_receive_raw",
                "receive_wait_raw (async)"
            ],
            "missing": [
                "async_* variants for all methods (except receive_wait_raw)",
                "is_wireless() method (all variants)",
                "get_names_remote() method (all variants)",
                "modem_message event system"
            ]
        },
        "Inventory": {
            "implemented": [
                "book_next_size / read_last_size / async_size",
                "book_next_list / read_last_list / async_list",
                "book_next_get_item_detail / read_last_get_item_detail / async_get_item_detail",
                "book_next_push_items / read_last_push_items / async_push_items",
                "book_next_pull_items / read_last_pull_items / async_pull_items"
            ],
            "missing": [
                "get_item_limit() method (all variants)"
            ]
        },
        "Monitor": {
            "implemented": [
                "All book_next_* / read_last_* methods",
                "*_imm() methods (immediate execution variants)",
                "poll_touch() (async) - for touch events",
                "book_next_try_poll_touch / read_last_try_poll_touch"
            ],
            "missing": [
                "async_* variants for all methods (except poll_touch)",
                "monitor_resize event"
            ]
        },
        "Speaker": {
            "implemented": [
                "book_next_play_note / read_last_play_note",
                "book_next_play_sound / read_last_play_sound",
                "book_next_stop / read_last_stop"
            ],
            "missing": [
                "async_* variants for all methods",
                "speaker_audio_empty event"
            ]
        }
    },
    "advanced_peripherals": {
        "MEBridge": {
            "implemented": [
                "All book_next_* / read_last_* methods (~60 methods)"
            ],
            "missing": [
                "async_* variants for all methods (~60 methods)"
            ]
        },
        "PlayerDetector": {
            "implemented": [
                "All book_next_* / read_last_* methods"
            ],
            "missing": [
                "async_* variants for all methods",
                "playerJoin / playerLeave events"
            ]
        },
        "ChatBox": {
            "implemented": [
                "All book_next_* / read_last_* methods"
            ],
            "missing": [
                "async_* variants for all methods",
                "chat event"
            ]
        },
        "BlockReader": {
            "implemented": [
                "All book_next_* / read_last_* methods"
            ],
            "missing": [
                "async_* variants for all methods"
            ]
        },
        "GeoScanner": {
            "implemented": [
                "All book_next_* / read_last_* methods"
            ],
            "missing": [
                "async_* variants for all methods"
            ]
        },
        "EnvironmentDetector": {
            "implemented": [
                "All book_next_* / read_last_* methods"
            ],
            "missing": [
                "async_* variants for all methods"
            ]
        },
        "EnergyDetector": {
            "implemented": [
                "All book_next_* / read_last_* methods"
            ],
            "missing": [
                "async_* variants for all methods"
            ]
        },
        # Default for other AdvancedPeripherals
        "_default": {
            "implemented": [
                "All book_next_* / read_last_* methods"
            ],
            "missing": [
                "async_* variants for all methods"
            ]
        }
    },
    # Default for other mods
    "_default": {
        "implemented": [
            "All book_next_* / read_last_* methods"
        ],
        "missing": [
            "async_* variants for all methods"
        ]
    }
}

def get_implementation_status(mod_name, peripheral_name):
    """Get implementation status for a peripheral."""
    if mod_name in IMPLEMENTATION_STATUS:
        mod_status = IMPLEMENTATION_STATUS[mod_name]
        if peripheral_name in mod_status:
            return mod_status[peripheral_name]
        elif "_default" in mod_status:
            return mod_status["_default"]
    return IMPLEMENTATION_STATUS["_default"]

def create_status_section_en(implemented, missing):
    """Create English implementation status section."""
    section = "\n## Implementation Status\n\n"
    section += "### ✅ Implemented\n\n"
    for item in implemented:
        section += f"- {item}\n"
    section += "\n### 🚧 Not Yet Implemented\n\n"
    for item in missing:
        section += f"- {item}\n"
    section += "\n"
    return section

def create_status_section_ja(implemented, missing):
    """Create Japanese implementation status section."""
    section = "\n## 実装状況\n\n"
    section += "### ✅ 実装済み\n\n"
    for item in implemented:
        section += f"- {item}\n"
    section += "\n### 🚧 未実装\n\n"
    for item in missing:
        section += f"- {item}\n"
    section += "\n"
    return section

def add_status_to_file(file_path, is_japanese=False):
    """Add implementation status section to a documentation file."""
    # Read the file
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Check if status section already exists
    if "## Implementation Status" in content or "## 実装状況" in content:
        print(f"  Status section already exists in {file_path}, skipping...")
        return False
    
    # Extract mod name and peripheral name from path
    parts = Path(file_path).parts
    if "api_en" in parts:
        lang_idx = parts.index("api_en")
    elif "api_ja" in parts:
        lang_idx = parts.index("api_ja")
    else:
        return False
    
    mod_name = parts[lang_idx + 1]
    peripheral_name = Path(file_path).stem
    
    # Get implementation status
    status = get_implementation_status(mod_name, peripheral_name)
    
    # Create status section
    if is_japanese:
        status_section = create_status_section_ja(status["implemented"], status["missing"])
    else:
        status_section = create_status_section_en(status["implemented"], status["missing"])
    
    # Find insertion point (after Overview section, before Methods section)
    # Look for "## Methods" or similar section
    methods_match = re.search(r'\n## (Methods|メソッド)', content)
    if methods_match:
        insert_pos = methods_match.start()
        new_content = content[:insert_pos] + status_section + content[insert_pos:]
    else:
        # If no Methods section, insert before first ## after Overview
        overview_match = re.search(r'\n## Overview', content)
        if overview_match:
            # Find next ## section
            next_section = re.search(r'\n## ', content[overview_match.end():])
            if next_section:
                insert_pos = overview_match.end() + next_section.start()
                new_content = content[:insert_pos] + status_section + content[insert_pos:]
            else:
                # Insert at end
                new_content = content + status_section
        else:
            # Insert after first heading
            first_heading = re.search(r'\n## ', content)
            if first_heading:
                next_section = re.search(r'\n## ', content[first_heading.end():])
                if next_section:
                    insert_pos = first_heading.end() + next_section.start()
                    new_content = content[:insert_pos] + status_section + content[insert_pos:]
                else:
                    new_content = content + status_section
            else:
                new_content = content + status_section
    
    # Write back
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(new_content)
    
    return True

def main():
    """Main function."""
    docs_root = Path("docs")
    
    # Process English docs
    print("Processing English documentation...")
    api_en = docs_root / "api_en"
    for md_file in api_en.rglob("*.md"):
        if md_file.name == "core.md":
            continue
        print(f"  Processing {md_file}...")
        add_status_to_file(md_file, is_japanese=False)
    
    # Process Japanese docs
    print("\nProcessing Japanese documentation...")
    api_ja = docs_root / "api_ja"
    for md_file in api_ja.rglob("*.md"):
        if md_file.name == "core.md":
            continue
        print(f"  Processing {md_file}...")
        add_status_to_file(md_file, is_japanese=True)
    
    print("\nDone!")

if __name__ == "__main__":
    main()
