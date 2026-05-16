# Intents.framework coverage audit (v0.2.0)

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped

## Carried forward from v0.1.0

| Header / API | Status | Notes |
| --- | --- | --- |
| `INPreferences.siriAuthorizationStatus` / `requestSiriAuthorization:` | ✅ | `Preferences` wrapper plus callback bridge in `Preferences.swift` / `preferences.rs`. |
| `INVoiceShortcutCenter.shared`, `getAllVoiceShortcuts`, `getVoiceShortcut(with:)` | ✅ | `VoiceShortcutCenter` + `VoiceShortcut` wrappers retained from v0.1.0. |

## Requested logical areas

### IntentDefinition (`INIntent.h`, `INShortcut.h`)

| API | Status | Notes |
| --- | --- | --- |
| `INIntent.identifier` | ✅ | `Intent::identifier`. |
| `INIntent.intentDescription` | ✅ | `Intent::intent_description`. |
| `INIntent.suggestedInvocationPhrase` | ✅ | Getter/setter on `Intent`. |
| `INIntent.donationMetadata` | ✅ | Getter/setter on `Intent` via `IntentDonationMetadata`. |
| `INIntent.keyImage` | ✅ | `Intent::key_image`. |
| `INIntent.setImage:forParameterNamed:` / `imageForParameterNamed:` | ✅ | Dynamic Objective-C bridge in `IntentDefinition.swift`. |
| `INIntent.shortcutAvailability` | ⏭️ | API is marked unavailable on macOS in the public headers. |
| `INShortcut.initWithIntent:` | ✅ | `Shortcut::new`. |
| `INShortcut.initWithUserActivity:` | ✅ | `Shortcut::new_with_user_activity`. |
| `INShortcut.intent` / `userActivity` | ✅ | Exposed as `Shortcut::intent` / `user_activity`. |

### IntentResponse (`INIntentResponse.h`)

| API | Status | Notes |
| --- | --- | --- |
| `INIntentResponse.userActivity` | ✅ | `IntentResponse::user_activity`, `set_user_activity`, and `set_user_activity_type`. |

### IntentHandler (`INIntentHandlerProviding.h`)

| API | Status | Notes |
| --- | --- | --- |
| `handlerForIntent:` | ✅ | Runtime-backed `IntentHandlerProvider` test helper maps intent class names to handler names. |

### IntentDonation (`INIntentDonationMetadata.h`, `INSendMessageIntentDonationMetadata.h`)

| API | Status | Notes |
| --- | --- | --- |
| `INIntentDonationMetadata` wrapper | ✅ | Opaque `IntentDonationMetadata` handle for attached metadata. |
| `INIntentDonationMetadata.init` | ⏭️ | Explicitly unavailable in Apple’s header. |
| `INSendMessageIntentDonationMetadata.init` | ✅ | `SendMessageIntentDonationMetadata::new`. |
| `mentionsCurrentUser` | ✅ | Getter/setter exposed. |
| `replyToCurrentUser` | ✅ | Getter/setter exposed as `is_reply_to_current_user` / `set_reply_to_current_user`. |
| `notifyRecipientAnyway` | ✅ | Getter/setter exposed. |
| `recipientCount` | ✅ | Getter/setter exposed. |

### IntentExtension (`INExtension.h`)

| API | Status | Notes |
| --- | --- | --- |
| `INExtension` construction | ✅ | `IntentExtension::new`. |
| `handlerForIntent:` inherited behavior | ✅ | `IntentExtension::handler_class_name_for_intent`. |

### INParameter (`INParameter.h`)

| API | Status | Notes |
| --- | --- | --- |
| `parameterForClass:keyPath:` | ✅ | `IntentParameter::new`. |
| `parameterClass` / `parameterKeyPath` | ✅ | Getter surface exposed. |
| `isEqualToParameter:` | ✅ | `IntentParameter::is_equal_to_parameter`. |
| `setIndex:forSubKeyPath:` / `indexForSubKeyPath:` | ✅ | Getter/setter exposed, with `NSNotFound` mapped to `None`. |
| `INInteraction.parameterValueForParameter:` | ⏭️ | Category API is marked unavailable on macOS. |

### INObject (`INObject.h`)

| API | Status | Notes |
| --- | --- | --- |
| All public initializers | ✅ | `IntentObject::new` / `with_details`. |
| `identifier` / `displayString` / `pronunciationHint` | ✅ | Getter surface exposed. |
| `subtitleString` | ✅ | Getter/setter exposed. |
| `displayImage` | ✅ | Getter/setter exposed via `Image`. |
| `alternativeSpeakableMatches` setter | ✅ | `set_alternative_speakable_matches`. |
| `alternativeSpeakableMatches` getter | 🟡 | Count is exposed; a fully typed array getter is not yet provided. |

### INVocabulary (`INVocabulary.h`)

| API | Status | Notes |
| --- | --- | --- |
| `sharedVocabulary` | ✅ | `IntentVocabulary::shared`. |
| `setVocabularyStrings:ofType:` | ✅ | `set_vocabulary_strings`. |
| `setVocabulary:ofType:` | ✅ | `set_vocabulary_speakables`. |
| `removeAllVocabularyStrings` | ✅ | `remove_all_vocabulary_strings`. |

### INRelevantShortcut (`INRelevantShortcut.h`, `INRelevanceProvider.h`)

| API | Status | Notes |
| --- | --- | --- |
| `INRelevantShortcut.initWithShortcut:` | ✅ | `RelevantShortcut::new`. |
| `relevanceProviders` setter | ✅ | `set_relevance_providers`. |
| `relevanceProviders` getter | 🟡 | Count is exposed; a typed array getter is not yet provided. |
| `watchTemplate` | 🟡 | Template presence is inspectable via `watch_template_class_name`; no `INDefaultCardTemplate` wrapper yet. |
| `widgetKind` | ✅ | Getter/setter exposed. |
| `shortcutRole` | ✅ | Getter/setter exposed. |
| `shortcut` | ✅ | `RelevantShortcut::shortcut`. |
| `INDateRelevanceProvider.initWithStartDate:endDate:` | ✅ | `RelevanceProvider::date`. |
| `INDailyRoutineRelevanceProvider.initWithSituation:` | ✅ | `RelevanceProvider::daily_routine`. |
| `INLocationRelevanceProvider.initWithRegion:` | ⏭️ | Requires `CLRegion` / CoreLocation bridging plus location authorization. |

### INInteraction (`INInteraction.h`)

| API | Status | Notes |
| --- | --- | --- |
| `initWithIntent:response:` | ✅ | `Interaction::new`. |
| `donateInteractionWithCompletion:` | ✅ | `Interaction::donate`. |
| `deleteAllInteractionsWithCompletion:` | ✅ | `Interaction::delete_all`. |
| `deleteInteractionsWithIdentifiers:completion:` | ✅ | `Interaction::delete_by_identifiers`. |
| `deleteInteractionsWithGroupIdentifier:completion:` | ✅ | `Interaction::delete_by_group_identifier`. |
| `intent` / `intentResponse` | ✅ | Getter surface exposed. |
| `intentHandlingStatus` | ✅ | Enum-mapped getter exposed. |
| `direction` | ✅ | Getter/setter exposed. |
| `dateInterval` | ✅ | Getter/setter exposed. |
| `identifier` / `groupIdentifier` | ✅ | Getter/setter exposed. |

### INRelevantShortcutStore (`INRelevantShortcutStore.h`)

| API | Status | Notes |
| --- | --- | --- |
| `defaultStore` | ✅ | `RelevantShortcutStore::default_store`. |
| `setRelevantShortcuts:completionHandler:` | ✅ | `set_relevant_shortcuts`. |

## Deferred items

| API | Status | Reason |
| --- | --- | --- |
| `INIntent.shortcutAvailability` | ⏭️ | Public header marks the property unavailable on macOS. |
| `INInteraction.parameterValueForParameter:` | ⏭️ | Public header marks the category unavailable on macOS. |
| `INLocationRelevanceProvider` | ⏭️ | Requires additional CoreLocation bridging and location authorization. |
| `INDefaultCardTemplate` / `watchTemplate` full wrapper | 🟡 | Presence is surfaced, but there is no dedicated card-template type in v0.2.0. |
| `alternativeSpeakableMatches` / `relevanceProviders` typed getters | 🟡 | Setter + count are covered; returning typed arrays is deferred. |
