extends FileDialog
class_name ProjectDialog

onready var window_content := $"../Content" as Control

static func open(scene: SceneTree):
    var nodes = scene.get_nodes_in_group("ProjectDialog")
    if nodes: nodes.front()._open()

func _open():
    if not visible:
        window_content.modulate.a = 0.5
        window_title = tr(".alert.data.title")
        popup_centered()


func _popup_hide():
    window_content.modulate.a = 1