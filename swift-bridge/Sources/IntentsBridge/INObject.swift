import Foundation
import Intents

@_cdecl("inx_in_object_create")
public func inx_in_object_create(
    _ identifier: UnsafePointer<CChar>?,
    _ displayString: UnsafePointer<CChar>?,
    _ pronunciationHint: UnsafePointer<CChar>?,
    _ subtitleString: UnsafePointer<CChar>?,
    _ displayImagePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let displayString else {
        outError?.pointee = inxCString("display string was NULL")
        return nil
    }

    let displayImage: INImage? = displayImagePtr.map { inxUnretained($0) }
    let object = INObject(
        identifier: identifier.map(String.init(cString:)),
        display: String(cString: displayString),
        pronunciationHint: pronunciationHint.map(String.init(cString:)),
        subtitle: subtitleString.map(String.init(cString:)),
        image: displayImage
    )
    return inxRetain(object)
}
