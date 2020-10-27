extends AcceptDialog
class_name MessageDialog

onready var window_content := $"../Content" as Control

var _is_only_dialog := false

static func alert(text: String):
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("MessageDialog")
    if nodes: nodes.front()._alert(text)


static func error(text: String):
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("MessageDialog")
    if nodes: nodes.front()._error(text)


static func error_code(code: int):
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("MessageDialog")
    if nodes: nodes.front()._error(Util.error_msg(code))


func _alert(text: String):
    if not visible:
        dialog_text = text
        window_title = tr(".alert.info")
        popup_centered()


func _error(text: String):
    if not visible:
        dialog_text = text
        window_title = tr(".alert.error")
        popup_centered()


func _popup_hide():
    if _is_only_dialog: window_content.modulate.a = 1


func _about_to_show():
    _is_only_dialog = window_content.modulate.a >= 1
    window_content.modulate.a = 0.5
