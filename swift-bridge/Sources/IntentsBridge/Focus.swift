import Dispatch
import Foundation
import Intents

@_cdecl("inx_focus_status_create")
public func inx_focus_status_create(
    _ hasValue: Bool,
    _ isFocused: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INFocusStatus requires macOS 12+")
        return nil
    }

    let value: Bool? = hasValue ? isFocused : nil
    return inxRetain(INFocusStatus(isFocused: value))
}

@_cdecl("inx_focus_status_center_copy_default")
public func inx_focus_status_center_copy_default() -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else { return nil }
    return inxRetain(INFocusStatusCenter.default)
}

@_cdecl("inx_focus_status_center_request_authorization")
public func inx_focus_status_center_request_authorization(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ outStatus: UnsafeMutablePointer<Int64>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INFocusStatusCenter requires macOS 12+")
        return false
    }
    guard let centerPtr else {
        outError?.pointee = inxCString("focus status center pointer was NULL")
        return false
    }

    let center: INFocusStatusCenter = inxUnretained(centerPtr)
    let semaphore = DispatchSemaphore(value: 0)
    var resolvedStatus: INFocusStatusAuthorizationStatus?
    center.requestAuthorization { status in
        resolvedStatus = status
        semaphore.signal()
    }
    _ = semaphore.wait(timeout: .now() + 5)

    guard let resolvedStatus else {
        outError?.pointee = inxCString("focus status authorization request timed out")
        return false
    }

    outStatus?.pointee = Int64(resolvedStatus.rawValue)
    return true
}
