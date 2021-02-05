extends ConfirmationDialog
class_name ConfirmDialog

signal response(confirmed)


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
