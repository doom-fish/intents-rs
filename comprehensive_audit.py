#!/usr/bin/env python3
import os
import re
import subprocess

SDK = subprocess.check_output(['xcrun', '--sdk', 'macosx', '--show-sdk-path']).decode().strip()
FRAMEWORK_PATH = os.path.join(SDK, 'System/Library/Frameworks/Intents.framework/Headers')

# Get all headers
headers = sorted([f for f in os.listdir(FRAMEWORK_PATH) if f.endswith('.h')])

sdk_symbols = {}  # symbol_name -> (kind, header, attributes)

# Scan all headers
for header in headers:
    filepath = os.path.join(FRAMEWORK_PATH, header)
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
    except:
        continue
    
    # Extract @interface definitions
    for match in re.finditer(r'@interface\s+(\w+)(?:\s*\((\w+)\))?', content):
        name = match.group(1)
        category = match.group(2)
        if category:
            key = f"{name} ({category})"
            kind = "category"
        else:
            key = name
            kind = "interface"
        
        # Check for API_UNAVAILABLE or NS_UNAVAILABLE
        before = content[max(0, match.start()-200):match.start()]
        after = content[match.end():min(len(content), match.end()+200)]
        context = before + after
        
        attrs = []
        if 'API_UNAVAILABLE' in context or 'API_UNAVAILABLE' in before:
            attrs.append('API_UNAVAILABLE')
        if '__deprecated' in context.lower() or '_Deprecated' in header:
            attrs.append('DEPRECATED')
        
        sdk_symbols[key] = (kind, header, ', '.join(attrs) if attrs else 'available')
    
    # Extract @protocol
    for match in re.finditer(r'@protocol\s+(\w+)', content):
        name = match.group(1)
        if name not in sdk_symbols:
            sdk_symbols[name] = ("protocol", header, "available")
    
    # Extract NS_ENUM and NS_OPTIONS
    for match in re.finditer(r'typedef\s+NS_(?:ENUM|OPTIONS)\s*\([^)]*\)\s*(\w+)', content):
        name = match.group(1)
        kind = "enum/options"
        before = content[max(0, match.start()-200):match.start()]
        if 'API_UNAVAILABLE' in before:
            attrs = 'API_UNAVAILABLE'
        else:
            attrs = 'available'
        if name not in sdk_symbols:
            sdk_symbols[name] = (kind, header, attrs)
    
    # Extract constants (FOUNDATION_EXPORT, etc.)
    for match in re.finditer(r'(?:FOUNDATION_EXPORT|extern\s+const)\s+\w+\s+(\w+)', content):
        name = match.group(1)
        if name not in sdk_symbols:
            sdk_symbols[name] = ("constant", header, "available")

print(f"Total SDK public symbols enumerated: {len(sdk_symbols)}")
print(f"Total headers scanned: {len(headers)}")
print()

# Group by kind
kinds = {}
for sym, (kind, _, _) in sdk_symbols.items():
    if kind not in kinds:
        kinds[kind] = 0
    kinds[kind] += 1

print("Symbols by kind:")
for kind in sorted(kinds.keys()):
    print(f"  {kind}: {kinds[kind]}")

