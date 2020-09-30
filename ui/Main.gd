extends Panel

signal categories_changed(categories)

onready var project := get_node("/root/Project") as Project

var project_path: String = ""


func _ready() -> void:
    print("_ready")
    get_tree().set_auto_accept_quit(false)
    if project_path:
        _on_project_selected(project_path)
    else:
        ProjectDialog.open(get_tree())


func _notification(what: int) -> void:
    match what:
        MainLoop.NOTIFICATION_WM_QUIT_REQUEST:
            project.close()
            get_tree().quit()

func _unhandled_key_input(event):
    if event is InputEventKey:
        var key_event := event as InputEventKey
        if key_event.scancode == KEY_P and key_event.control and key_event.pressed:
            MessageDialog.alert(get_tree(), "ctrl+P pressed")


func _on_project_selected(path: String) -> void:
    if project.open(path):
        var result = project.categories()
        if result.has("Ok"):
            project_path = path
            emit_signal("categories_changed", result["Ok"])
        else:
            print("ERROR: loading categories")
    else:
        print("ERROR: opening project")


func persistant_save() -> Dictionary:
    return {
        "path": project_path
    }


func persistant_load(data: Dictionary):
    project_path = data.get("path", "")
