import Foundation
import Intents

@_cdecl("inx_call_group_create")
public func inx_call_group_create(
    _ groupName: UnsafePointer<CChar>?,
    _ groupId: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 11.3, *) else {
        outError?.pointee = inxCString("INCallGroup requires macOS 11.3+")
        return nil
    }

    let group = INCallGroup(
        groupName: groupName.map { String(cString: $0) },
        groupId: groupId.map { String(cString: $0) }
    )
    return inxRetain(group)
}

@_cdecl("inx_call_record_create")
public func inx_call_record_create(
    _ identifier: UnsafePointer<CChar>?,
    _ callRecordType: Int,
    _ callCapability: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INCallRecord requires macOS 12+")
        return nil
    }
    guard let identifier else {
        outError?.pointee = inxCString("call record identifier was NULL")
        return nil
    }
    guard let recordType = INCallRecordType(rawValue: callRecordType) else {
        outError?.pointee = inxCString("invalid INCallRecordType raw value")
        return nil
    }
    guard let capability = INCallCapability(rawValue: callCapability) else {
        outError?.pointee = inxCString("invalid INCallCapability raw value")
        return nil
    }
    guard let record = (NSClassFromString("INCallRecord") as? NSObject.Type)?.init() else {
        outError?.pointee = inxCString("INCallRecord class was unavailable")
        return nil
    }

    record.setValue(String(cString: identifier), forKey: "identifier")
    record.setValue(NSNumber(value: recordType.rawValue), forKey: "callRecordType")
    record.setValue(NSNumber(value: capability.rawValue), forKey: "callCapability")
    return inxRetain(record)
}

@_cdecl("inx_call_record_filter_create")
public func inx_call_record_filter_create(
    _ participants: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ callTypes: UInt,
    _ callCapability: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INCallRecordFilter requires macOS 12+")
        return nil
    }

    let people: [INPerson]
    if count == 0 {
        people = []
    } else if let participants {
        people = (0..<count).compactMap { index -> INPerson? in
            guard let value = participants[index] else { return nil }
            return Unmanaged<INPerson>.fromOpaque(value).takeUnretainedValue()
        }
        guard people.count == count else {
            outError?.pointee = inxCString("call record filter participants were invalid")
            return nil
        }
    } else {
        outError?.pointee = inxCString("call record filter participants were NULL")
        return nil
    }

    guard let capability = INCallCapability(rawValue: callCapability) else {
        outError?.pointee = inxCString("invalid INCallCapability raw value")
        return nil
    }
    guard let filter = (NSClassFromString("INCallRecordFilter") as? NSObject.Type)?.init() else {
        outError?.pointee = inxCString("INCallRecordFilter class was unavailable")
        return nil
    }

    filter.setValue(people.isEmpty ? nil : people, forKey: "participants")
    filter.setValue(NSNumber(value: callTypes), forKey: "callTypes")
    filter.setValue(NSNumber(value: capability.rawValue), forKey: "callCapability")
    return inxRetain(filter)
}
