extends MainLoop
class_name Test


var _counter := 0
var _current_method := ""
var _current_assert := 0


func assert_eq(expected, actual, msg = ""):
    if expected != actual:
        if expected is String and actual is String:
            expected = expected.c_escape()
            actual = actual.c_escape()
        var error: String = ("ERROR: Assert " + String(_current_assert)
            + " failed in " + _current_method + ": '"
            + String(expected) + "' != '" + String(actual) + "'")
        if msg:
            error += " - '" + msg + "'"
        printerr(error)
        _counter += 1
    _current_assert += 1


func assert_true(actual, msg = ""):
    assert_eq(true, actual, msg)


func _idle(_delta: float) -> bool:
    OS.exit_code = 1 # if the script panics during execution

    var _success := true
    var _script_path: String = get_script().resource_path

    print("Starting Tests for ", _script_path, "\n")
    for method in get_method_list():
        if method.name.begins_with("test_"):
            _current_assert = 0
            _current_method = method.name
            call(method.name)
            if _counter > 0:
                printerr(method.name + ": " + String(_counter) + " Assertions failed\n")
                _success = false
            else:
                printerr(method.name + ": Succeeded\n")
            _counter = 0

    print("Finalized:")
    print("  End time: %s ms" % str(OS.get_ticks_msec()))
    if _success: OS.exit_code = 0

    return true
