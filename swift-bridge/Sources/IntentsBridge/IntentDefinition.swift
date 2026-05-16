import Foundation
import Intents

@_cdecl("inx_intent_create")
public func inx_intent_create() -> UnsafeMutableRawPointer? {
    inxRetain(INIntent())
}

@_cdecl("inx_user_activity_create")
public func inx_user_activity_create(
    _ activityType: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let activityType else {
        outError?.pointee = inxCString("activity type was NULL")
        return nil
    }

    let activityTypeValue = String(cString: activityType)
    let activity = NSUserActivity(activityType: activityTypeValue)
    activity.title = activityTypeValue
    return inxRetain(activity)
}

@_cdecl("inx_speakable_string_create")
public func inx_speakable_string_create(
    _ vocabularyIdentifier: UnsafePointer<CChar>?,
    _ spokenPhrase: UnsafePointer<CChar>?,
    _ pronunciationHint: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let vocabularyIdentifier else {
        outError?.pointee = inxCString("vocabulary identifier was NULL")
        return nil
    }
    guard let spokenPhrase else {
        outError?.pointee = inxCString("spoken phrase was NULL")
        return nil
    }

    let speakable = INSpeakableString(
        vocabularyIdentifier: String(cString: vocabularyIdentifier),
        spokenPhrase: String(cString: spokenPhrase),
        pronunciationHint: pronunciationHint.map(String.init(cString:))
    )
    return inxRetain(speakable)
}

@_cdecl("inx_image_create_named")
public func inx_image_create_named(
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let name else {
        outError?.pointee = inxCString("image name was NULL")
        return nil
    }

    return inxRetain(INImage(named: String(cString: name)))
}

@_cdecl("inx_image_create_with_data")
public func inx_image_create_with_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard count >= 0 else {
        outError?.pointee = inxCString("image byte count must be non-negative")
        return nil
    }
    guard count == 0 || bytes != nil else {
        outError?.pointee = inxCString("image bytes were NULL")
        return nil
    }

    let data = count == 0 ? Data() : Data(bytes: bytes!, count: count)
    return inxRetain(INImage(imageData: data))
}

@_cdecl("inx_image_create_with_url")
public func inx_image_create_with_url(
    _ urlString: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let urlString else {
        outError?.pointee = inxCString("image URL was NULL")
        return nil
    }
    guard let url = URL(string: String(cString: urlString)) else {
        outError?.pointee = inxCString("image URL was invalid")
        return nil
    }
    guard let image = INImage(url: url) else {
        outError?.pointee = inxCString("INImage rejected the provided URL")
        return nil
    }
    return inxRetain(image)
}

@_cdecl("inx_shortcut_create_with_intent")
public func inx_shortcut_create_with_intent(
    _ intentPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let intentPtr else {
        outError?.pointee = inxCString("intent pointer was NULL")
        return nil
    }

    let intent: INIntent = inxUnretained(intentPtr)
    guard let shortcut = INShortcut(intent: intent) else {
        outError?.pointee = inxCString("INShortcut rejected the provided intent")
        return nil
    }
    return inxRetain(shortcut as AnyObject)
}

@_cdecl("inx_shortcut_create_with_user_activity")
public func inx_shortcut_create_with_user_activity(
    _ userActivityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let userActivityPtr else {
        outError?.pointee = inxCString("user activity pointer was NULL")
        return nil
    }

    let userActivity: NSUserActivity = inxUnretained(userActivityPtr)
    return inxRetain(INShortcut(userActivity: userActivity) as AnyObject)
}

@_cdecl("inx_intent_copy_key_image")
public func inx_intent_copy_key_image(
    _ intentPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let intentPtr else { return nil }
    let intent: INIntent = inxUnretained(intentPtr)
    return intent.keyImage().map(inxRetain)
}

@_cdecl("inx_intent_copy_image_for_parameter_named")
public func inx_intent_copy_image_for_parameter_named(
    _ intentPtr: UnsafeMutableRawPointer?,
    _ parameterName: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let intentPtr, let parameterName else { return nil }

    let intent: INIntent = inxUnretained(intentPtr)
    let selector = NSSelectorFromString("imageForParameterNamed:")
    guard intent.responds(to: selector),
          let image = intent
            .perform(selector, with: NSString(string: String(cString: parameterName)))?
            .takeUnretainedValue() as AnyObject?
    else {
        return nil
    }
    return inxRetain(image)
}

@_cdecl("inx_intent_set_image_for_parameter_named")
public func inx_intent_set_image_for_parameter_named(
    _ intentPtr: UnsafeMutableRawPointer?,
    _ parameterName: UnsafePointer<CChar>?,
    _ imagePtr: UnsafeMutableRawPointer?
) -> Bool {
    guard let intentPtr, let parameterName else { return false }

    let intent: INIntent = inxUnretained(intentPtr)
    let selector = NSSelectorFromString("setImage:forParameterNamed:")
    guard intent.responds(to: selector) else { return false }

    let image = imagePtr.map { Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() }
    _ = intent.perform(
        selector,
        with: image,
        with: NSString(string: String(cString: parameterName))
    )
    return true
}
