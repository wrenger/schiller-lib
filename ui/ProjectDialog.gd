extends FileDialog
class_name ProjectDialog

onready var window_content: Control = $"../Content"


static func open(scene: SceneTree):
    var nodes = scene.get_nodes_in_group("ProjectDialog")
    if nodes: nodes.front()._open()


func _open():
    if not visible: popup_centered()


func _popup_hide():
    window_content.modulate.a = 1


func _about_to_show():
    window_content.modulate.a = 0.5
