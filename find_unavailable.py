#!/usr/bin/env python3
import os
import re
import subprocess

SDK = subprocess.check_output(['xcrun', '--sdk', 'macosx', '--show-sdk-path']).decode().strip()
FRAMEWORK_PATH = os.path.join(SDK, 'System/Library/Frameworks/Intents.framework/Headers')

unavailable_symbols = []

headers = sorted([f for f in os.listdir(FRAMEWORK_PATH) if f.endswith('.h')])

for header in headers:
    filepath = os.path.join(FRAMEWORK_PATH, header)
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
    except:
        continue
    
    # Look for API_UNAVAILABLE(macos...
    for match in re.finditer(r'API_UNAVAILABLE\s*\([^)]*macos[^)]*\)', content):
        # Find what symbol this applies to
        before = content[:match.start()]
        after = content[match.end():]
        
        # Look back for @interface, @protocol, typedef, extern, etc.
        lines_before = before[-500:].split('\n')
        for line in reversed(lines_before):
            if '@interface' in line:
                sym = re.search(r'@interface\s+(\w+)', line)
                if sym:
                    unavailable_symbols.append((sym.group(1), 'interface', header))
                break
            elif '@protocol' in line:
                sym = re.search(r'@protocol\s+(\w+)', line)
                if sym:
                    unavailable_symbols.append((sym.group(1), 'protocol', header))
                break
            elif 'typedef' in line and ('NS_ENUM' in line or 'NS_OPTIONS' in line):
                sym = re.search(r'typedef\s+NS_(?:ENUM|OPTIONS)[^)]*\)\s*(\w+)', line)
                if sym:
                    unavailable_symbols.append((sym.group(1), 'enum/options', header))
                break

print(f"Symbols with API_UNAVAILABLE(macos...):")
for sym, kind, header in sorted(set(unavailable_symbols)):
    print(f"  {sym:60} {kind:15} {header}")

print(f"\nTotal: {len(set(unavailable_symbols))}")

