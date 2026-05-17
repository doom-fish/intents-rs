#!/usr/bin/env python3
import os
import re
import subprocess

SDK = subprocess.check_output(['xcrun', '--sdk', 'macosx', '--show-sdk-path']).decode().strip()
FRAMEWORK_PATH = os.path.join(SDK, 'System/Library/Frameworks/Intents.framework/Headers')

print("=== STRICT V2 AUDIT: RE-VERIFYING V1 EXEMPT ENTRIES ===\n")

# All EXEMPT entries from v1 (from lines 165-193 of COVERAGE_AUDIT.md)
exempt_v1 = [
    ("INAddTasksIntent (Deprecated)", "category", "INAddTasksIntent_Deprecated.h"),
    ("INCallRecord (Deprecated)", "category", "INCallRecord_Deprecated.h"),
    ("INMediaSearch (Deprecated)", "category", "INMediaSearch_Deprecated.h"),
    ("INPerson (Deprecated)", "category", "INPerson_Deprecated.h"),
    ("INPlayMediaIntent (Deprecated)", "category", "INPlayMediaIntent_Deprecated.h"),
    ("INRequestRideIntent (Deprecated)", "category", "INRequestRideIntent_Deprecated.h"),
    ("INRideDriver (Deprecated)", "category", "INRideDriver_Deprecated.h"),
    ("INRideOption (Deprecated)", "category", "INRideOption.h"),
    ("INSaveProfileInCarIntent (Deprecated)", "category", "INSaveProfileInCarIntent_Deprecated.h"),
    ("INSearchCallHistoryIntent (Deprecated)", "category", "INSearchCallHistoryIntent_Deprecated.h"),
    ("INSearchForMessagesIntent (DeprecatedSearchForMessages)", "category", "INSearchForMessagesIntent_Deprecated.h"),
    ("INSearchForNotebookItemsIntent (Deprecated)", "category", "INSearchForNotebookItemsIntent_Deprecated.h"),
    ("INSendMessageIntentResponse (Deprecated)", "category", "INSendMessageIntentResponse_Deprecated.h"),
    ("INSendMessageIntent (Deprecated)", "category", "INSendMessageIntent_Deprecated.h"),
    ("INSetClimateSettingsInCarIntent (Deprecated)", "category", "INSetClimateSettingsInCarIntent_Deprecated.h"),
    ("INSetDefrosterSettingsInCarIntent (Deprecated)", "category", "INSetDefrosterSettingsInCarIntent_Deprecated.h"),
    ("INSetProfileInCarIntent (Deprecated)", "category", "INSetProfileInCarIntent_Deprecated.h"),
    ("INSetSeatSettingsInCarIntent (Deprecated)", "category", "INSetSeatSettingsInCarIntent_Deprecated.h"),
    ("INSetTaskAttributeIntent (Deprecated)", "category", "INSetTaskAttributeIntent_Deprecated.h"),
    ("INStartAudioCallIntent (Deprecated)", "category", "INStartAudioCallIntent_Deprecated.h"),
    ("INStartCallIntent (Deprecated)", "category", "INStartCallIntent_Deprecated.h"),
    ("INBookRestaurantReservationIntentCode", "enum", "INBookRestaurantReservationIntentResponse.h"),
    ("INGetAvailableRestaurantReservationBookingDefaultsIntentResponseCode", "enum", "INGetAvailableRestaurantReservationBookingDefaultsIntentResponse.h"),
    ("INGetAvailableRestaurantReservationBookingsIntentCode", "enum", "INGetAvailableRestaurantReservationBookingsIntentResponse.h"),
    ("INGetRestaurantGuestIntentResponseCode", "enum", "INGetRestaurantGuestIntentResponse.h"),
    ("INGetUserCurrentRestaurantReservationBookingsIntentResponseCode", "enum", "INGetUserCurrentRestaurantReservationBookingsIntentResponse.h"),
    ("INRestaurantReservationBooking", "interface", "INRestaurantReservationBooking.h"),
    ("INRestaurantReservationUserBookingStatus", "enum", "INRestaurantReservationUserBooking.h"),
    ("NSExtensionContext (ShareExtension)", "category", "NSExtensionContext+ShareExtension.h"),
]

verified_attrs = {}

for sym, kind, header in exempt_v1:
    filepath = os.path.join(FRAMEWORK_PATH, header)
    if not os.path.exists(filepath):
        print(f"✗ {sym:55} HEADER NOT FOUND: {header}")
        continue
    
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
    except:
        print(f"✗ {sym:55} CANNOT READ: {header}")
        continue
    
    # Check for API_UNAVAILABLE(macos...)
    has_api_unavailable = bool(re.search(r'API_UNAVAILABLE\s*\([^)]*macos[^)]*\)', content))
    
    # Check if header name ends with _Deprecated
    is_deprecated_header = '_Deprecated' in header
    
    attrs = []
    if is_deprecated_header:
        attrs.append("_Deprecated header")
    if has_api_unavailable:
        attrs.append("API_UNAVAILABLE(macos)")
    
    if attrs:
        print(f"✓ {sym:55} {', '.join(attrs)}")
        verified_attrs[sym] = ', '.join(attrs)
    else:
        print(f"? {sym:55} NO CLEAR EXEMPTION ATTRIBUTE")

print(f"\n=== SUMMARY ===")
print(f"Total v1 EXEMPT entries: {len(exempt_v1)}")
print(f"Verified with attributes: {len(verified_attrs)}")
print(f"Missing attributes: {len(exempt_v1) - len(verified_attrs)}")

