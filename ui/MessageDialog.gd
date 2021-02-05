extends AcceptDialog
class_name MessageDialog

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
