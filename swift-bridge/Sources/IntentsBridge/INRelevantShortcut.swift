import Foundation
import Intents

private func inxInitWithObject(
    className: String,
    selectorName: String,
    object: AnyObject
) -> AnyObject? {
    guard let allocated = inxAllocObject(className: className) else { return nil }
    let selector = NSSelectorFromString(selectorName)
    guard allocated.responds(to: selector) else { return nil }
    return allocated.perform(selector, with: object)?.takeUnretainedValue()
}

private func inxInitWithTwoObjects(
    className: String,
    selectorName: String,
    first: AnyObject,
    second: AnyObject?
) -> AnyObject? {
    guard let allocated = inxAllocObject(className: className) else { return nil }
    let selector = NSSelectorFromString(selectorName)
    guard allocated.responds(to: selector) else { return nil }
    return allocated.perform(selector, with: first, with: second)?.takeUnretainedValue()
}

private func inxInitWithInt(
    className: String,
    selectorName: String,
    rawValue: Int
) -> AnyObject? {
    guard let allocated = inxAllocObject(className: className) else { return nil }
    let selector = NSSelectorFromString(selectorName)
    guard allocated.responds(to: selector) else { return nil }

    typealias Fn = @convention(c) (AnyObject, Selector, Int) -> AnyObject
    let imp = allocated.method(for: selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    return function(allocated, selector, rawValue)
}

@_cdecl("inx_relevant_shortcut_create")
public func inx_relevant_shortcut_create(
    _ shortcutPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let shortcutPtr else {
        outError?.pointee = inxCString("shortcut pointer was NULL")
        return nil
    }
    let shortcut: AnyObject = inxUnretained(shortcutPtr)
    guard let relevantShortcut = inxInitWithObject(
        className: "INRelevantShortcut",
        selectorName: "initWithShortcut:",
        object: shortcut
    ) else {
        outError?.pointee = inxCString("failed to construct INRelevantShortcut")
        return nil
    }
    return inxRetain(relevantShortcut)
}

@_cdecl("inx_date_relevance_provider_create")
public func inx_date_relevance_provider_create(
    _ start: Double,
    _ end: Double,
    _ hasEnd: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let startDate = Date(timeIntervalSince1970: start) as NSDate
    let endDate = hasEnd ? Date(timeIntervalSince1970: end) as NSDate : nil
    guard let provider = inxInitWithTwoObjects(
        className: "INDateRelevanceProvider",
        selectorName: "initWithStartDate:endDate:",
        first: startDate,
        second: endDate
    ) else {
        outError?.pointee = inxCString("failed to construct INDateRelevanceProvider")
        return nil
    }
    return inxRetain(provider)
}

@_cdecl("inx_daily_routine_relevance_provider_create")
public func inx_daily_routine_relevance_provider_create(
    _ situation: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let provider = inxInitWithInt(
        className: "INDailyRoutineRelevanceProvider",
        selectorName: "initWithSituation:",
        rawValue: Int(situation)
    ) else {
        outError?.pointee = inxCString("failed to construct INDailyRoutineRelevanceProvider")
        return nil
    }
    return inxRetain(provider)
}
