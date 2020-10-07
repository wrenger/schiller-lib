extends Node
class_name Test


var _counter := 0


func _ready() -> void:
    for method in get_method_list():
        if method.name.begins_with("test_"):
            call(method.name)
            if _counter > 0:
                printerr(name, ".", method.name, ": ", _counter, " Assertions failed!")
            _counter = 0


func assert_eq(a, b):
    if a != b:
        printerr(name, ": Assert failed: ", a, " != ", b)
        _counter += 1
