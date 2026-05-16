import Foundation
import Intents

private func inxObject(_ ptr: UnsafeMutableRawPointer?) -> NSObject? {
    guard let ptr else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as? NSObject
}

private func inxKey(_ key: UnsafePointer<CChar>?) -> String? {
    key.map(String.init(cString:))
}

private func inxJSON(_ object: Any) -> String? {
    guard JSONSerialization.isValidJSONObject(object),
          let data = try? JSONSerialization.data(withJSONObject: object, options: []),
          let string = String(data: data, encoding: .utf8)
    else {
        return nil
    }
    return string
}

@_cdecl("inx_object_class_name")
public func inx_object_class_name(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let object = inxObject(ptr) else { return nil }
    return inxCString(NSStringFromClass(type(of: object)))
}

@_cdecl("inx_object_is_equal")
public func inx_object_is_equal(
    _ lhsPtr: UnsafeMutableRawPointer?,
    _ rhsPtr: UnsafeMutableRawPointer?
) -> Bool {
    guard let lhs = inxObject(lhsPtr), let rhs = inxObject(rhsPtr) else {
        return false
    }
    return lhs.isEqual(rhs)
}

@_cdecl("inx_object_copy_string_property")
public func inx_object_copy_string_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let object = inxObject(ptr), let key = inxKey(key), let value = object.value(forKey: key) else {
        return nil
    }

    switch value {
    case let string as String:
        return inxCString(string)
    case let string as NSString:
        return inxCString(string as String)
    case let uuid as UUID:
        return inxCString(uuid.uuidString)
    case let uuid as NSUUID:
        return inxCString(uuid.uuidString)
    case let url as URL:
        return inxCString(url.absoluteString)
    case let url as NSURL:
        return inxCString(url.absoluteString ?? url.description)
    case let activity as NSUserActivity:
        return inxCString(activity.activityType)
    case let cls as AnyClass:
        return inxCString(NSStringFromClass(cls))
    default:
        return nil
    }
}

@_cdecl("inx_object_copy_object_property")
public func inx_object_copy_object_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let object = inxObject(ptr), let key = inxKey(key) else { return nil }
    guard let value = object.value(forKey: key) as AnyObject? else { return nil }
    return inxRetain(value)
}

@_cdecl("inx_object_copy_string_array_property_json")
public func inx_object_copy_string_array_property_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let object = inxObject(ptr), let key = inxKey(key) else { return nil }
    guard let array = object.value(forKey: key) as? [Any] else { return nil }
    let strings = array.compactMap { element -> String? in
        switch element {
        case let string as String:
            return string
        case let string as NSString:
            return string as String
        default:
            return nil
        }
    }
    guard strings.count == array.count else { return nil }
    return inxJSON(strings).flatMap(inxCString)
}

@_cdecl("inx_object_get_integer_property")
public func inx_object_get_integer_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outPresent: UnsafeMutablePointer<Bool>?
) -> Int64 {
    guard let object = inxObject(ptr), let key = inxKey(key), let value = object.value(forKey: key) else {
        outPresent?.pointee = false
        return 0
    }

    outPresent?.pointee = true
    if let number = value as? NSNumber {
        return number.int64Value
    }
    if let value = value as? Int {
        return Int64(value)
    }
    if let value = value as? Int64 {
        return value
    }
    outPresent?.pointee = false
    return 0
}

@_cdecl("inx_object_get_double_property")
public func inx_object_get_double_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outPresent: UnsafeMutablePointer<Bool>?
) -> Double {
    guard let object = inxObject(ptr), let key = inxKey(key), let value = object.value(forKey: key) else {
        outPresent?.pointee = false
        return 0
    }

    outPresent?.pointee = true
    if let number = value as? NSNumber {
        return number.doubleValue
    }
    if let value = value as? Double {
        return value
    }
    outPresent?.pointee = false
    return 0
}

@_cdecl("inx_object_get_bool_property")
public func inx_object_get_bool_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outPresent: UnsafeMutablePointer<Bool>?
) -> Bool {
    guard let object = inxObject(ptr), let key = inxKey(key), let value = object.value(forKey: key) else {
        outPresent?.pointee = false
        return false
    }

    outPresent?.pointee = true
    if let number = value as? NSNumber {
        return number.boolValue
    }
    if let value = value as? Bool {
        return value
    }
    outPresent?.pointee = false
    return false
}

@_cdecl("inx_object_get_array_count_property")
public func inx_object_get_array_count_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outPresent: UnsafeMutablePointer<Bool>?
) -> Int {
    guard let object = inxObject(ptr), let key = inxKey(key), let array = object.value(forKey: key) as? NSArray else {
        outPresent?.pointee = false
        return 0
    }
    outPresent?.pointee = true
    return array.count
}

@_cdecl("inx_object_get_date_interval_property")
public func inx_object_get_date_interval_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outStart: UnsafeMutablePointer<Double>?,
    _ outEnd: UnsafeMutablePointer<Double>?,
    _ outPresent: UnsafeMutablePointer<Bool>?
) -> Bool {
    guard let object = inxObject(ptr), let key = inxKey(key) else {
        outPresent?.pointee = false
        return false
    }
    guard let value = object.value(forKey: key) else {
        outPresent?.pointee = false
        return true
    }

    let interval: DateInterval?
    if let dateInterval = value as? DateInterval {
        interval = dateInterval
    } else if let dateInterval = value as? NSDateInterval {
        interval = DateInterval(start: dateInterval.startDate as Date, end: dateInterval.endDate as Date)
    } else {
        interval = nil
    }

    guard let interval else {
        outPresent?.pointee = false
        return false
    }

    outPresent?.pointee = true
    outStart?.pointee = interval.start.timeIntervalSince1970
    outEnd?.pointee = interval.end.timeIntervalSince1970
    return true
}

@_cdecl("inx_object_set_string_property")
public func inx_object_set_string_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?
) -> Bool {
    guard let object = inxObject(ptr), let key = inxKey(key), let value else { return false }
    object.setValue(String(cString: value), forKey: key)
    return true
}

@_cdecl("inx_object_set_integer_property")
public func inx_object_set_integer_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: Int64
) -> Bool {
    guard let object = inxObject(ptr), let key = inxKey(key) else { return false }
    object.setValue(NSNumber(value: value), forKey: key)
    return true
}

@_cdecl("inx_object_set_object_property")
public func inx_object_set_object_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valuePtr: UnsafeMutableRawPointer?
) -> Bool {
    guard let object = inxObject(ptr), let key = inxKey(key) else { return false }
    let value = valuePtr.map { Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() }
    object.setValue(value, forKey: key)
    return true
}

@_cdecl("inx_object_set_date_interval_property")
public func inx_object_set_date_interval_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ start: Double,
    _ end: Double
) -> Bool {
    guard let object = inxObject(ptr), let key = inxKey(key) else { return false }
    let interval = DateInterval(
        start: Date(timeIntervalSince1970: start),
        end: Date(timeIntervalSince1970: end)
    )
    object.setValue(interval, forKey: key)
    return true
}

@_cdecl("inx_object_set_object_array_property")
public func inx_object_set_object_array_property(
    _ ptr: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> Bool {
    guard let object = inxObject(ptr), let key = inxKey(key) else { return false }
    let objects: [AnyObject]
    if count == 0 {
        objects = []
    } else if let values {
        objects = (0..<count).compactMap { index in
            guard let value = values[index] else { return nil }
            return Unmanaged<AnyObject>.fromOpaque(value).takeUnretainedValue()
        }
        guard objects.count == count else { return false }
    } else {
        return false
    }
    object.setValue(objects, forKey: key)
    return true
}
