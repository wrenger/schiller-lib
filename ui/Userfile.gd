extends Button


func _ready() -> void:
    for node in get_tree().get_nodes_in_group("ProjectDialog"):
        node.connect("userfile_selected", self, "_on_selected")


func _pressed():
    ProjectDialog.userfile(get_tree())


func _on_selected(path: String):
    hint_tooltip = path

