#!/bin/bash

FRAMEWORK_PATH="$SDK/System/Library/Frameworks/Intents.framework/Headers"

echo "=== Verifying v1 EXEMPT entries ==="
echo ""

# List from v1 audit: 29 EXEMPT items
# 1-7: deprecated headers  
# 22-29: Restaurant + Share

declare -a deprecated_headers=(
  "INAddTasksIntent_Deprecated.h"
  "INCallRecord_Deprecated.h"
  "INMediaSearch_Deprecated.h"
  "INPerson_Deprecated.h"
  "INPlayMediaIntent_Deprecated.h"
  "INRequestRideIntent_Deprecated.h"
  "INRideDriver_Deprecated.h"
  "INSaveProfileInCarIntent_Deprecated.h"
  "INSearchCallHistoryIntent_Deprecated.h"
  "INSearchForMessagesIntent_Deprecated.h"
  "INSearchForNotebookItemsIntent_Deprecated.h"
  "INSendMessageIntentResponse_Deprecated.h"
  "INSendMessageIntent_Deprecated.h"
  "INSetClimateSettingsInCarIntent_Deprecated.h"
  "INSetDefrosterSettingsInCarIntent_Deprecated.h"
  "INSetProfileInCarIntent_Deprecated.h"
  "INSetSeatSettingsInCarIntent_Deprecated.h"
  "INSetTaskAttributeIntent_Deprecated.h"
  "INStartAudioCallIntent_Deprecated.h"
  "INStartCallIntent_Deprecated.h"
  "INRideOption.h"
)

echo "Checking deprecated files exist:"
for hdr in "${deprecated_headers[@]}"; do
  if [ -f "$FRAMEWORK_PATH/$hdr" ]; then
    echo "  ✓ $hdr"
  else
    echo "  ✗ $hdr NOT FOUND"
  fi
done

echo ""
echo "Checking API_UNAVAILABLE(macos) in restaurant/share headers:"
grep -l "API_UNAVAILABLE.*macos" "$FRAMEWORK_PATH/INBookRestaurantReservationIntentResponse.h" "$FRAMEWORK_PATH/NSExtensionContext+ShareExtension.h" 2>/dev/null | while read f; do
  echo "  ✓ $(basename $f)"
done

