import Foundation
import Intents

@_cdecl("inx_person_handle_create")
public func inx_person_handle_create(
    _ value: UnsafePointer<CChar>?,
    _ handleType: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let handleType = INPersonHandleType(rawValue: handleType) else {
        outError?.pointee = inxCString("invalid INPersonHandleType raw value")
        return nil
    }

    return inxRetain(
        INPersonHandle(
            value: value.map(String.init(cString:)),
            type: handleType
        )
    )
}

@_cdecl("inx_person_create")
public func inx_person_create(
    _ personHandlePtr: UnsafeMutableRawPointer?,
    _ displayName: UnsafePointer<CChar>?,
    _ imagePtr: UnsafeMutableRawPointer?,
    _ contactIdentifier: UnsafePointer<CChar>?,
    _ customIdentifier: UnsafePointer<CChar>?,
    _ aliasesPtr: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ aliasCount: Int,
    _ suggestionType: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let personHandlePtr else {
        outError?.pointee = inxCString("person handle pointer was NULL")
        return nil
    }
    guard let suggestionType = INPersonSuggestionType(rawValue: suggestionType) else {
        outError?.pointee = inxCString("invalid INPersonSuggestionType raw value")
        return nil
    }

    let personHandle: INPersonHandle = inxUnretained(personHandlePtr)
    let image: INImage? = imagePtr.map { inxUnretained($0) }
    let aliases: [INPersonHandle]
    if aliasCount == 0 {
        aliases = []
    } else if let aliasesPtr {
        let resolvedAliases = (0..<aliasCount).compactMap { index -> INPersonHandle? in
            guard let alias = aliasesPtr[index] else { return nil }
            return inxUnretained(alias)
        }
        guard resolvedAliases.count == aliasCount else {
            outError?.pointee = inxCString("person aliases contained a NULL pointer")
            return nil
        }
        aliases = resolvedAliases
    } else {
        outError?.pointee = inxCString("person aliases pointer was NULL")
        return nil
    }

    let person = INPerson(
        personHandle: personHandle,
        nameComponents: nil,
        displayName: displayName.map(String.init(cString:)),
        image: image,
        contactIdentifier: contactIdentifier.map(String.init(cString:)),
        customIdentifier: customIdentifier.map(String.init(cString:)),
        aliases: aliases.isEmpty ? nil : aliases,
        suggestionType: suggestionType
    )
    return inxRetain(person)
}
