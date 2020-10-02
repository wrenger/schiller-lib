extends Reference
class_name Util

static func create_shortcut(name: String, key: int) -> ShortCut:
    var event := InputEventKey.new()
    event.control = true
    event.scancode = key
    var shortcut := ShortCut.new()
    shortcut.resource_name = name
    shortcut.shortcut = event
    return shortcut


enum SbvError {
    InvalidArguments = 0,
    LogicError,
    NoProject,
    FileNotFound,
    FileOpenError,
    SQLError,
}


static func error_msg(error: int) -> String:
    match error:
        SbvError.InvalidArguments: return "Invalid Arguments Error"
        SbvError.LogicError: return "Logic Error"
        SbvError.NoProject: return ".error.no-project"
        SbvError.FileNotFound: return ".error.file-open"
        SbvError.FileOpenError: return ".error.file-open"
        SbvError.SQLError: return ".error.sql"
        _: return "Unknown Error"
