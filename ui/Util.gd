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
    # Rental errors
    RentalUserMayNotBorrow,
    RentalMediumNotBorrowable,
    RentalMediumAlreadyBorrowed,
    RentalMediumAlreadyBorrowedByUser,
    RentalMediumNotBorrowed,
    RentalMediumAlreadyReserved,
}


static func error_msg(error: int) -> String:
    match error:
        SbvError.InvalidArguments: return "Internal: Invalid Arguments Error"
        SbvError.LogicError: return "Internal: Logic Error"
        SbvError.NoProject: return TranslationServer.tr(".error.no-project")
        SbvError.FileNotFound: return TranslationServer.tr(".error.file-open")
        SbvError.FileOpenError: return TranslationServer.tr(".error.file-open")
        SbvError.SQLError: return TranslationServer.tr(".error.sql")
        SbvError.RentalUserMayNotBorrow: return TranslationServer.tr(".error.rental.user")
        SbvError.RentalMediumNotBorrowable: return TranslationServer.tr(".error.rental.medium")
        SbvError.RentalMediumAlreadyBorrowed: return TranslationServer.tr(".error.rental.already-borrowed")
        SbvError.RentalMediumAlreadyBorrowedByUser: return TranslationServer.tr(".error.rental.already-borrowed-by")
        SbvError.RentalMediumNotBorrowed: return TranslationServer.tr(".error.rental.not-borrowed")
        SbvError.RentalMediumAlreadyReserved: return TranslationServer.tr(".error.rental.already-reserved")
        _: return "Internal: Unknown Error"



static func trf(key: String, values: Array = []) -> String:
    var text := TranslationServer.tr(key)
    if false:
        var re := RegEx.new()
        var error := re.compile("\\{(\\d{1,2})(:(([\\p{L}\\p{N}]*)\\|([\\p{L}\\p{N}]*)))?}")
        assert(error == OK)
        var output := PoolStringArray([])
        var pos := 0
        for result in re.search_all(text):
            if len(result.strings) == 6:
                var idx := int(result.strings[1])
                if idx < len(values):
                    output.push_back(text.substr(pos, result.get_start() - pos))
                    if not result.strings[3]:
                        output.push_back(String(values[idx]))
                    else:
                        if values[idx] is int and values[idx] == 1:
                            output.push_back(result.strings[4])
                        else:
                            output.push_back(result.strings[5])
                    pos = result.get_end()
        output.push_back(text.substr(pos, len(text) - pos))
        return output.join("")
    return text
