import CoreLocation
import Foundation
import Intents

private func inxObjectArray(_ values: UnsafePointer<UnsafeMutableRawPointer?>?, count: Int) -> [AnyObject]? {
    if count == 0 {
        return []
    }
    guard let values else { return nil }
    let objects = (0..<count).compactMap { index -> AnyObject? in
        guard let value = values[index] else { return nil }
        return Unmanaged<AnyObject>.fromOpaque(value).takeUnretainedValue()
    }
    return objects.count == count ? objects : nil
}

@_cdecl("inx_deferred_localized_intents_string_copy")
public func inx_deferred_localized_intents_string_copy(
    _ format: UnsafePointer<CChar>?,
    _ table: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let format else { return nil }
    guard let metaClass = object_getClass(NSString.self) else { return nil }

    let formatValue = NSString(string: String(cString: format))
    let localized: NSString?
    if let table {
        let selector = NSSelectorFromString("deferredLocalizedIntentsStringWithFormat:fromTable:")
        guard class_respondsToSelector(metaClass, selector) else { return nil }
        typealias Fn = @convention(c) (AnyClass, Selector, NSString, NSString?) -> AnyObject
        let imp = class_getMethodImplementation(metaClass, selector)
        let function = unsafeBitCast(imp, to: Fn.self)
        localized = function(NSString.self, selector, formatValue, NSString(string: String(cString: table))) as? NSString
    } else {
        let selector = NSSelectorFromString("deferredLocalizedIntentsStringWithFormat:")
        guard class_respondsToSelector(metaClass, selector) else { return nil }
        typealias Fn = @convention(c) (AnyClass, Selector, NSString) -> AnyObject
        let imp = class_getMethodImplementation(metaClass, selector)
        let function = unsafeBitCast(imp, to: Fn.self)
        localized = function(NSString.self, selector, formatValue) as? NSString
    }

    return localized.flatMap { inxCString($0 as String) }
}

@_cdecl("inx_placemark_create")
public func inx_placemark_create(
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let location = CLLocation(latitude: 0, longitude: 0)
    let selector = NSSelectorFromString("placemarkWithLocation:name:postalAddress:")
    guard let metaClass = object_getClass(CLPlacemark.self), class_respondsToSelector(metaClass, selector) else {
        outError?.pointee = inxCString("CLPlacemark INIntentsAdditions category is unavailable")
        return nil
    }

    typealias Fn = @convention(c) (AnyClass, Selector, CLLocation, NSString?, AnyObject?) -> AnyObject
    let imp = class_getMethodImplementation(metaClass, selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    let placemark = function(
        CLPlacemark.self,
        selector,
        location,
        name.map { NSString(string: String(cString: $0)) },
        nil
    )
    return inxRetain(placemark)
}

@_cdecl("inx_object_section_create")
public func inx_object_section_create(
    _ title: UnsafePointer<CChar>?,
    _ items: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let objects = inxObjectArray(items, count: count) else {
        outError?.pointee = inxCString("object section items were invalid")
        return nil
    }

    let section = INObjectSection(title: title.map { String(cString: $0) }, items: objects)
    return inxRetain(section)
}

@_cdecl("inx_object_collection_create_with_items")
public func inx_object_collection_create_with_items(
    _ items: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let objects = inxObjectArray(items, count: count) else {
        outError?.pointee = inxCString("object collection items were invalid")
        return nil
    }

    return inxRetain(INObjectCollection(items: objects))
}

@_cdecl("inx_object_collection_create_with_sections")
public func inx_object_collection_create_with_sections(
    _ sections: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let objects = inxObjectArray(sections, count: count) else {
        outError?.pointee = inxCString("object collection sections were invalid")
        return nil
    }

    let typedSections = objects.compactMap { $0 as? INObjectSection<AnyObject> }
    guard typedSections.count == objects.count else {
        outError?.pointee = inxCString("object collection sections were not INObjectSection instances")
        return nil
    }

    return inxRetain(INObjectCollection(sections: typedSections))
}
