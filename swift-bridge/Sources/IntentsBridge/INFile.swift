import Foundation
import Intents

private func inxFileJSON(_ object: Any) -> String? {
    guard JSONSerialization.isValidJSONObject(object),
          let data = try? JSONSerialization.data(withJSONObject: object, options: []),
          let string = String(data: data, encoding: .utf8)
    else {
        return nil
    }
    return string
}

@_cdecl("inx_file_create_with_data")
public func inx_file_create_with_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ count: Int,
    _ filename: UnsafePointer<CChar>?,
    _ typeIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard count >= 0 else {
        outError?.pointee = inxCString("file byte count must be non-negative")
        return nil
    }
    guard count == 0 || bytes != nil else {
        outError?.pointee = inxCString("file bytes were NULL")
        return nil
    }
    guard let filename else {
        outError?.pointee = inxCString("file name was NULL")
        return nil
    }

    let data = count == 0 ? Data() : Data(bytes: bytes!, count: count)
    let file = INFile(
        data: data,
        filename: String(cString: filename),
        typeIdentifier: typeIdentifier.map(String.init(cString:))
    )
    return inxRetain(file)
}

@_cdecl("inx_file_create_with_file_url")
public func inx_file_create_with_file_url(
    _ fileURL: UnsafePointer<CChar>?,
    _ filename: UnsafePointer<CChar>?,
    _ typeIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let fileURL else {
        outError?.pointee = inxCString("file URL was NULL")
        return nil
    }
    guard let url = URL(string: String(cString: fileURL)), url.isFileURL else {
        outError?.pointee = inxCString("file URL must be a valid file:// URL")
        return nil
    }

    let file = INFile(
        fileURL: url,
        filename: filename.map(String.init(cString:)),
        typeIdentifier: typeIdentifier.map(String.init(cString:))
    )
    return inxRetain(file)
}

@_cdecl("inx_file_copy_data_json")
public func inx_file_copy_data_json(
    _ filePtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let filePtr else { return nil }
    let file: INFile = inxUnretained(filePtr)
    return inxFileJSON(Array(file.data)).flatMap(inxCString)
}
