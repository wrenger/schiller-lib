extends FileDialog
class_name ProjectDialog

signal project_selected(path, new)

onready var window_content: Control = $"../Content"


static func open(scene: SceneTree):
    var nodes = scene.get_nodes_in_group("ProjectDialog")
    if nodes: nodes.front()._open()


static func create(scene: SceneTree):
    var nodes = scene.get_nodes_in_group("ProjectDialog")
    if nodes: nodes.front()._create()


func _ready() -> void:
    var result := connect("file_selected", self, "_on_file_selected")
    assert(result == OK)


func _open():
    if not visible:
        popup_centered()
        window_title = tr(".alert.open-project")
        mode = FileDialog.MODE_OPEN_FILE


func _create():
    if not visible:
        popup_centered()
        window_title = tr(".alert.create-project")
        mode = FileDialog.MODE_SAVE_FILE


func _on_file_selected(path):
    emit_signal("project_selected", path, mode == FileDialog.MODE_SAVE_FILE)


func _popup_hide():
    window_content.modulate.a = 1


func _about_to_show():
    window_content.modulate.a = 0.5
