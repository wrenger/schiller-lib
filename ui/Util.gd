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
