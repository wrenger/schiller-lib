extends ConfirmationDialog
class_name CategoryDialog

static func open():
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("CategoryDialog")
    if nodes: nodes.front()._open()


func _open():
    pass
