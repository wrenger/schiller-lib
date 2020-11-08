extends ConfirmationDialog
class_name ConfirmDialog

signal response(confirmed)

onready var window_content := $"../Content" as Control

var _is_only_dialog := false


static func open(text: String) -> ConfirmDialog:
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("ConfirmDialog")
    nodes.front()._open(text)
    return nodes.front()


func _open(text: String):
    if not visible:
        dialog_text = text
        popup_centered()


func _ready() -> void:
    var result := OK
    result = connect("popup_hide", self, "_popup_hide")
    assert(result == OK)
    result = connect("about_to_show", self, "_about_to_show")
    assert(result == OK)
    result = connect("confirmed", self, "_confirmed")
    assert(result == OK)
    result = get_cancel().connect("pressed", self, "_cancelled")
    assert(result == OK)
    result = get_close_button().connect("pressed", self, "_cancelled")
    assert(result == OK)


func _cancelled():
    emit_signal("response", false)


func _confirmed():
    emit_signal("response", true)


func _popup_hide():
    if _is_only_dialog: window_content.modulate.a = 1


func _about_to_show():
    _is_only_dialog = window_content.modulate.a >= 1
    window_content.modulate.a = 0.5
