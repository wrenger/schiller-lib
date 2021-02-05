extends Control

var _open_dialogs := 0

func _ready() -> void:
    for child in get_children():
        var popup := child as Popup
        if popup:
            var result: int
            result = popup.connect("about_to_show", self, "_about_to_show")
            assert(result == OK)
            result = popup.connect("popup_hide", self, "_popup_hide")
            assert(result == OK)


func _about_to_show():
    if _open_dialogs == 0: visible = true
    _open_dialogs += 1


func _popup_hide():
    _open_dialogs -= 1
    if _open_dialogs == 0: visible = false
