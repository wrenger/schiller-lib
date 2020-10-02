extends ConfirmationDialog
class_name LendDialog

onready var window_content := $"../Content" as Control

static func lend(scene: SceneTree, medium: Reference):
    var nodes = scene.get_nodes_in_group("LendDialog")
    if nodes: nodes.front()._lend(medium)


func _lend(medium: Reference):
    if not visible:
        window_content.modulate.a = 0.5
        window_title = tr(".medium.lend")
        $Box/Lend/Medium/Data.text = medium.id + ": " + medium.title
        popup_centered()


func _popup_hide():
    window_content.modulate.a = 1


func _on_confirmed() -> void:
    print("TODO: lend")
