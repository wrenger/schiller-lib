extends Panel

signal categories_changed(categories)
signal project_changed()

onready var _project: Project = $"/root/Project"

var project_path: String = ""


func _ready():
    get_tree().set_auto_accept_quit(false)
    if project_path:
        _on_project_selected(project_path)
    else:
        ProjectDialog.open(get_tree())


func _notification(what: int):
    match what:
        MainLoop.NOTIFICATION_WM_QUIT_REQUEST:
            _project.close()
            get_tree().quit()


func _unhandled_key_input(event):
    if event is InputEventKey:
        var key_event := event as InputEventKey
        if key_event.scancode == KEY_P and key_event.control and key_event.pressed:
            MessageDialog.alert("ctrl+P pressed")


func _on_open_project():
    ProjectDialog.open(get_tree())


func _on_close_project():
    _project.close()
    emit_signal("project_changed")
    emit_signal("categories_changed", [])
    OS.set_window_title(ProjectSettings.get("application/config/name"))


func _on_project_selected(path: String):
    if _project.open(path):
        var result: Dictionary = _project.category_list()
        if result.has("Ok"):
            project_path = path
            OS.set_window_title(ProjectSettings.get("application/config/name") + " - " + path.get_file())
            emit_signal("project_changed")
            emit_signal("categories_changed", result["Ok"])
        else:
            MessageDialog.error(tr(result["Err"]))
            _on_close_project()
    else:
        OS.set_window_title(ProjectSettings.get("application/config/name"))
        MessageDialog.error(tr(".error.db"))


func persist_save() -> Dictionary:
    return {
        "path": project_path,
        "width": OS.window_size.x,
        "height": OS.window_size.y,
        "x": OS.window_position.x,
        "y": OS.window_position.y,
    }


func persist_load(data: Dictionary):
    project_path = data.get("path", "")
    OS.window_size = Vector2(data.get("width", OS.window_size.x), data.get("height", OS.window_size.y))
    OS.window_position = Vector2(data.get("x", OS.window_position.x), data.get("y", OS.window_position.y))


func _enter_tree():
    pass
    # TranslationServer.set_locale("de")


func _on_theme_changed(theme):
    self.theme = theme
