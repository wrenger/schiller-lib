extends AcceptDialog
class_name MessageDialog

onready var window_content := $"../Content" as Control

static func alert(scene: SceneTree, text: String):
    var nodes = scene.get_nodes_in_group("MessageDialog")
    if nodes: nodes.front()._alert(text)

func _alert(text: String):
    if not visible:
        window_content.modulate.a = 0.5
        dialog_text = text
        window_title = tr(".alert.info")
        popup_centered()


func _popup_hide():
    window_content.modulate.a = 1
