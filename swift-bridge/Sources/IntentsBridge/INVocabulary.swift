import Foundation
import Intents

private func inxVocabularyStrings(
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int
) -> [String]? {
    guard count >= 0 else { return nil }
    guard count == 0 || values != nil else { return nil }
    guard let values else { return [] }
    return (0..<count).compactMap { index in
        values[index].map(String.init(cString:))
    }
}

private func inxVocabularySpeakables(
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

@_cdecl("inx_vocabulary_shared")
public func inx_vocabulary_shared() -> UnsafeMutableRawPointer? {
    guard let vocabularyClass = NSClassFromString("INVocabulary") as? NSObject.Type,
          let vocabulary = vocabularyClass.perform(NSSelectorFromString("sharedVocabulary"))?.takeUnretainedValue()
    else {
        return nil
    }
    return inxRetain(vocabulary as AnyObject)
}

@_cdecl("inx_vocabulary_set_strings")
public func inx_vocabulary_set_strings(
    _ vocabularyPtr: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ type: Int64
) -> Bool {
    guard let vocabularyPtr,
          let strings = inxVocabularyStrings(values, count),
          strings.count == count
    else {
        return false
    }

    let vocabulary: AnyObject = inxUnretained(vocabularyPtr)
    let selector = NSSelectorFromString("setVocabularyStrings:ofType:")
    guard vocabulary.responds(to: selector) else { return false }

    typealias Fn = @convention(c) (AnyObject, Selector, NSOrderedSet, Int) -> Void
    let function = unsafeBitCast(vocabulary.method(for: selector), to: Fn.self)
    function(vocabulary, selector, NSOrderedSet(array: strings), Int(type))
    return true
}

@_cdecl("inx_vocabulary_set_speakables")
public func inx_vocabulary_set_speakables(
    _ vocabularyPtr: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ type: Int64
) -> Bool {
    guard let vocabularyPtr,
          let speakables = inxVocabularySpeakables(values, count),
          speakables.count == count
    else {
        return false
    }

    let vocabulary: AnyObject = inxUnretained(vocabularyPtr)
    let selector = NSSelectorFromString("setVocabulary:ofType:")
    guard vocabulary.responds(to: selector) else { return false }

    typealias Fn = @convention(c) (AnyObject, Selector, NSOrderedSet, Int) -> Void
    let function = unsafeBitCast(vocabulary.method(for: selector), to: Fn.self)
    function(vocabulary, selector, NSOrderedSet(array: speakables), Int(type))
    return true
}

@_cdecl("inx_vocabulary_remove_all")
public func inx_vocabulary_remove_all(_ vocabularyPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let vocabularyPtr else { return false }
    let vocabulary: AnyObject = inxUnretained(vocabularyPtr)
    let selector = NSSelectorFromString("removeAllVocabularyStrings")
    guard vocabulary.responds(to: selector) else { return false }
    _ = vocabulary.perform(selector)
    return true
}
