import Foundation
import Intents

private func inxRelevantShortcuts(
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> [AnyObject]? {
    guard count >= 0 else { return nil }
    guard count == 0 || values != nil else { return nil }
    guard let values else { return [] }
    return (0..<count).compactMap { index in
        values[index].map { Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() }
    }
}

@_cdecl("inx_relevant_shortcut_store_default")
public func inx_relevant_shortcut_store_default() -> UnsafeMutableRawPointer? {
    guard let storeClass = NSClassFromString("INRelevantShortcutStore") as? NSObject.Type,
          let store = storeClass.value(forKey: "defaultStore") as AnyObject?
    else {
        return nil
    }
    return inxRetain(store)
}

@_cdecl("inx_relevant_shortcut_store_set")
public func inx_relevant_shortcut_store_set(
    _ storePtr: UnsafeMutableRawPointer?,
    _ shortcuts: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ callback: @escaping INXErrorCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    let box = ErrorCallbackBox(callback: callback, refcon: refcon)
    guard let storePtr else {
        box.send(errorMessage: "relevant shortcut store pointer was NULL")
        return
    }
    guard let relevantShortcuts = inxRelevantShortcuts(shortcuts, count), relevantShortcuts.count == count else {
        box.send(errorMessage: "relevant shortcut array was invalid")
        return
    }

    let store: AnyObject = inxUnretained(storePtr)
    let selector = NSSelectorFromString("setRelevantShortcuts:completionHandler:")
    guard store.responds(to: selector) else {
        box.send(errorMessage: "INRelevantShortcutStore.setRelevantShortcuts is unavailable")
        return
    }

    typealias Fn = @convention(c) (AnyObject, Selector, NSArray, (@convention(block) (NSError?) -> Void)?) -> Void
    let function = unsafeBitCast(store.method(for: selector), to: Fn.self)
    let block: @convention(block) (NSError?) -> Void = { error in
        box.send(errorMessage: error?.localizedDescription)
    }
    function(store, selector, relevantShortcuts as NSArray, block)
}
