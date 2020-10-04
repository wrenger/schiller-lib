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
        SbvError.NoProject: return ".error.no-project"
        SbvError.FileNotFound: return ".error.file-open"
        SbvError.FileOpenError: return ".error.file-open"
        SbvError.SQLError: return ".error.sql"
        SbvError.RentalUserMayNotBorrow: return ".error.rental.user"
        SbvError.RentalMediumNotBorrowable: return ".error.rental.medium"
        SbvError.RentalMediumAlreadyBorrowed: return ".error.rental.already-borrowed"
        SbvError.RentalMediumAlreadyBorrowedByUser: return ".error.rental.already-borrowed-by"
        SbvError.RentalMediumNotBorrowed: return ".error.rental.not-borrowed"
        SbvError.RentalMediumAlreadyReserved: return ".error.rental.already-reserved"
        _: return "Internal: Unknown Error"
