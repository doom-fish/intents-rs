#!/usr/bin/env python3
import subprocess
import re
import os

SDK = subprocess.check_output(['xcrun', '--sdk', 'macosx', '--show-sdk-path']).decode().strip()
FRAMEWORK_PATH = os.path.join(SDK, 'System/Library/Frameworks/Intents.framework/Headers')

# Get all headers
headers = sorted([f for f in os.listdir(FRAMEWORK_PATH) if f.endswith('.h')])

# Main Intents.h
main_header = os.path.join(FRAMEWORK_PATH, 'Intents.h')
with open(main_header) as f:
    content = f.read()

# Find all @interface
interfaces = re.findall(r'@interface\s+(\w+)(?:\s+\(|:|<|$)', content)
print(f"Interfaces in Intents.h: {len(set(interfaces))}")

# Enum patterns
enum_patterns = [
    r'typedef\s+NS_ENUM\s*\([^)]+\)\s*(\w+)',
    r'typedef\s+NS_OPTIONS\s*\([^)]+\)\s*(\w+)',
    r'typedef\s+enum\s+(\w+)',
]

all_enums = set()
for pattern in enum_patterns:
    matches = re.findall(pattern, content)
    all_enums.update(matches)

print(f"Enums/Options in Intents.h: {len(all_enums)}")

# Total headers
print(f"Total .h files in framework: {len(headers)}")

