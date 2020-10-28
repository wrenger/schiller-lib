extends FileDialog
class_name ProjectDialog

signal project_selected(path, new)
signal userfile_selected(path)

onready var window_content: Control = $"../Content"

enum DialogType { PROJECT, USERFILE }
var type: int = DialogType.PROJECT


static func open(scene: SceneTree):
    var nodes = scene.get_nodes_in_group("ProjectDialog")
    if nodes: nodes.front()._open()


static func create(scene: SceneTree):
    var nodes = scene.get_nodes_in_group("ProjectDialog")
    if nodes: nodes.front()._create()


static func userfile(scene: SceneTree):
    var nodes = scene.get_nodes_in_group("ProjectDialog")
    if nodes: nodes.front()._userfile()


func _ready() -> void:
    var result := connect("file_selected", self, "_on_file_selected")
    assert(result == OK)


func _open():
    if not visible:
        popup_centered()
        window_title = tr(".alert.open-project")
        filters = PoolStringArray(["*.db ; SQLite DB"])
        mode = FileDialog.MODE_OPEN_FILE
        type = DialogType.PROJECT


func _create():
    if not visible:
        popup_centered()
        window_title = tr(".alert.create-project")
        filters = PoolStringArray(["*.db ; SQLite DB"])
        mode = FileDialog.MODE_SAVE_FILE
        type = DialogType.PROJECT


func _userfile():
    if not visible:
        popup_centered()
        window_title = tr(".alert.userfile")
        filters = PoolStringArray(["*.csv ; CSV File"])
        mode = FileDialog.MODE_OPEN_FILE
        type = DialogType.USERFILE


func _on_userfile_selected(path):
    emit_signal("userfile_selected", path)


func _on_file_selected(path):
    match type:
        DialogType.PROJECT: emit_signal("project_selected", path, mode == FileDialog.MODE_SAVE_FILE)
        DialogType.USERFILE: emit_signal("userfile_selected", path)


func _popup_hide():
    window_content.modulate.a = 1


func _about_to_show():
    window_content.modulate.a = 0.5
