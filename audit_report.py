#!/usr/bin/env python3
import os
import re
import subprocess
import json

SDK = subprocess.check_output(['xcrun', '--sdk', 'macosx', '--show-sdk-path']).decode().strip()
FRAMEWORK_PATH = os.path.join(SDK, 'System/Library/Frameworks/Intents.framework/Headers')

# == PART 1: ENUMERATE SDK SYMBOLS ==

sdk_symbols = {}  # {symbol: (kind, header, attributes_list)}

headers = sorted([f for f in os.listdir(FRAMEWORK_PATH) if f.endswith('.h')])

for header in headers:
    filepath = os.path.join(FRAMEWORK_PATH, header)
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
    except:
        continue
    
    # Check if entire header is API_UNAVAILABLE
    header_level_unavailable = bool(re.search(r'API_UNAVAILABLE\s*\([^)]*macos[^)]*\)\s*$', content, re.MULTILINE))
    
    # @interface (with optional categories)
    for match in re.finditer(r'@interface\s+(\w+)(?:\s*\((\w+)\))?', content):
        name = match.group(1)
        category = match.group(2)
        
        if category:
            key = f"{name} ({category})"
            kind = "category"
        else:
            key = name
            kind = "interface"
        
        # Extract attributes around the @interface line
        start = max(0, match.start() - 300)
        end = min(len(content), match.end() + 300)
        context = content[start:end]
        
        attrs = []
        if '_Deprecated' in header or 'DEPRECATED' in header:
            attrs.append('header_name_deprecated')
        if 'API_UNAVAILABLE' in context and 'macos' in context:
            attrs.append('API_UNAVAILABLE(macos)')
        elif header_level_unavailable:
            attrs.append('header_level_unavailable')
        
        sdk_symbols[key] = (kind, header, attrs)
    
    # @protocol
    for match in re.finditer(r'@protocol\s+(\w+)(?:\s|;|<|$)', content):
        name = match.group(1)
        if name:
            start = max(0, match.start() - 300)
            end = min(len(content), match.end() + 300)
            context = content[start:end]
            
            attrs = []
            if 'API_UNAVAILABLE' in context and 'macos' in context:
                attrs.append('API_UNAVAILABLE(macos)')
            
            if name not in sdk_symbols:
                sdk_symbols[name] = ("protocol", header, attrs)
    
    # NS_ENUM / NS_OPTIONS
    for match in re.finditer(r'typedef\s+NS_(ENUM|OPTIONS)\s*\([^)]*\)\s*(\w+)', content):
        kind_type = match.group(1)
        name = match.group(2)
        
        start = max(0, match.start() - 300)
        end = min(len(content), match.end() + 300)
        context = content[start:end]
        
        attrs = []
        if 'API_UNAVAILABLE' in context and 'macos' in context:
            attrs.append('API_UNAVAILABLE(macos)')
        
        kind = "enum" if kind_type == "ENUM" else "options"
        if name not in sdk_symbols:
            sdk_symbols[name] = (kind, header, attrs)
    
    # Constants (IntentsVersionNumber, IntentsVersionString, etc.)
    for match in re.finditer(r'(?:FOUNDATION_EXPORT|extern\s+const)\s+\w+\s+(\w+)(?:\s|;)', content):
        name = match.group(1)
        if name not in sdk_symbols:
            sdk_symbols[name] = ("constant", header, [])

# == PART 2: DETERMINE WHICH SYMBOLS ARE EXEMPT ==

exempt_symbols = set()
macOS_unavailable = set()

# 1. Restaurant-related (API_UNAVAILABLE(macos))
restaurant_symbols = {
    'INBookRestaurantReservationIntent',
    'INBookRestaurantReservationIntentResponse',
    'INBookRestaurantReservationIntentCode',
    'INGetAvailableRestaurantReservationBookingDefaultsIntent',
    'INGetAvailableRestaurantReservationBookingDefaultsIntentResponse',
    'INGetAvailableRestaurantReservationBookingDefaultsIntentResponseCode',
    'INGetAvailableRestaurantReservationBookingsIntent',
    'INGetAvailableRestaurantReservationBookingsIntentResponse',
    'INGetAvailableRestaurantReservationBookingsIntentCode',
    'INGetRestaurantGuestIntent',
    'INGetRestaurantGuestIntentResponse',
    'INGetRestaurantGuestIntentResponseCode',
    'INGetUserCurrentRestaurantReservationBookingsIntent',
    'INGetUserCurrentRestaurantReservationBookingsIntentResponse',
    'INGetUserCurrentRestaurantReservationBookingsIntentResponseCode',
    'INRestaurantReservationBooking',
    'INRestaurantReservationUserBooking',
    'INRestaurantReservationUserBookingStatus',
}

# 2. Deprecated headers (header name ends with _Deprecated.h)
deprecated_headers = {
    'INAddTasksIntent_Deprecated.h',
    'INCallRecord_Deprecated.h',
    'INMediaSearch_Deprecated.h',
    'INPerson_Deprecated.h',
    'INPlayMediaIntent_Deprecated.h',
    'INRequestRideIntent_Deprecated.h',
    'INRideDriver_Deprecated.h',
    'INSaveProfileInCarIntent_Deprecated.h',
    'INSearchCallHistoryIntent_Deprecated.h',
    'INSearchForMessagesIntent_Deprecated.h',
    'INSearchForNotebookItemsIntent_Deprecated.h',
    'INSendMessageIntentResponse_Deprecated.h',
    'INSendMessageIntent_Deprecated.h',
    'INSetClimateSettingsInCarIntent_Deprecated.h',
    'INSetDefrosterSettingsInCarIntent_Deprecated.h',
    'INSetProfileInCarIntent_Deprecated.h',
    'INSetSeatSettingsInCarIntent_Deprecated.h',
    'INSetTaskAttributeIntent_Deprecated.h',
    'INStartAudioCallIntent_Deprecated.h',
    'INStartCallIntent_Deprecated.h',
}

# 3. Share extension (API_UNAVAILABLE(macos))
share_ext_symbols = {
    'NSExtensionContext (ShareExtension)',
}

# 4. Other unavailable
other_unavailable = {
    'INImage',
    'INInteraction',
    'INPerson',
    'INRecurrenceRule',
    'INRelevanceProvider',
    'INSendMessageIntentResponse',
    'INStartAudioCallIntent',
}

# Domain handling protocols (iOS-only)
domain_handling = {
    'INCallsDomainHandling',
    'INCarCommandsDomainHandling',
    'INCarPlayDomainHandling',
    'INMessagesDomainHandling',
    'INNotebookDomainHandling',
    'INPaymentsDomainHandling',
    'INPhotosDomainHandling',
    'INRadioDomainHandling',
    'INRidesharingDomainHandling',
    'INWorkoutsDomainHandling',
}

# Mark symbols as exempt
for sym, (kind, header, attrs) in sdk_symbols.items():
    base_name = sym.split('(')[0].strip()
    
    if header in deprecated_headers:
        exempt_symbols.add(sym)
        macOS_unavailable.add(sym)
    elif base_name in restaurant_symbols:
        exempt_symbols.add(sym)
        macOS_unavailable.add(sym)
    elif sym in share_ext_symbols:
        exempt_symbols.add(sym)
        macOS_unavailable.add(sym)
    elif base_name in domain_handling:
        exempt_symbols.add(sym)
        macOS_unavailable.add(sym)
    elif 'API_UNAVAILABLE(macos)' in attrs:
        exempt_symbols.add(sym)
        macOS_unavailable.add(sym)
    elif 'header_name_deprecated' in attrs:
        exempt_symbols.add(sym)

# Count
total = len(sdk_symbols)
exempt = len(exempt_symbols)
available = total - exempt

print(f"=== SDK Symbol Summary ===")
print(f"Total SDK public symbols: {total}")
print(f"Exempt (unavailable/deprecated): {exempt}")
print(f"Available for coverage check: {available}")
print()

# Write full list
with open('sdk_all_symbols.txt', 'w') as f:
    for sym in sorted(sdk_symbols.keys()):
        kind, header, attrs = sdk_symbols[sym]
        is_exempt = sym in exempt_symbols
        f.write(f"{'E' if is_exempt else '.'} {sym:60} {kind:15} {header:50} {','.join(attrs)}\n")

print(f"Wrote sdk_all_symbols.txt ({len(sdk_symbols)} symbols)")

