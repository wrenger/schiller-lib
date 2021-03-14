extends Panel

signal categories_changed(categories)
signal project_changed()

var _project_path: String = ""


func _ready():
    get_tree().set_auto_accept_quit(false)
    if _project_path:
        _on_project_selected(_project_path, false)
    else:
        ProjectDialog.open(get_tree())

    for node in get_tree().get_nodes_in_group("ThemeChanger"):
        node.connect("theme_changed", self, "_on_theme_changed")


func _notification(what: int):
    match what:
        MainLoop.NOTIFICATION_WM_QUIT_REQUEST:
            Project.close()
            get_tree().quit()


func _on_open_project():
    ProjectDialog.open(get_tree())


func _on_new_project() -> void:
    ProjectDialog.create(get_tree())


func _on_close_project():
    Project.close()
    _project_path = ""
    emit_signal("project_changed")
    emit_signal("categories_changed", [])
    OS.set_window_title(ProjectSettings.get("application/config/name"))


func _on_project_selected(path: String, new: bool):
    var result: Dictionary
    if new:
        result = Project.create(path)
        if result.has("Ok"):
            result = _add_new_dummy_data()
    else:
        result = Project.open(path)

    if result.has("Ok"):
        if not new and result["Ok"]: # updated
            MessageDialog.alert(Util.trf(".alert.update", [Project.version()]))

        result = Project.category_list()
        if result.has("Ok"):
            _project_path = path
            OS.set_window_title(ProjectSettings.get("application/config/name") + " - " + path.get_file())
            emit_signal("project_changed")
            emit_signal("categories_changed", result["Ok"])
            return

    _on_close_project()
    OS.set_window_title(ProjectSettings.get("application/config/name"))
    MessageDialog.error_code(result["Err"])


func persist_save() -> Dictionary:
    return {
        "path": _project_path,
        "width": OS.window_size.x,
        "height": OS.window_size.y,
        "x": OS.window_position.x,
        "y": OS.window_position.y,
    }


func persist_load(data: Dictionary):
    _project_path = data.get("path", "")
    OS.window_size = Vector2(data.get("width", OS.window_size.x), data.get("height", OS.window_size.y))
    OS.window_position = Vector2(data.get("x", OS.window_position.x), data.get("y", OS.window_position.y))


func _on_theme_changed(theme):
    self.theme = theme


func _add_new_dummy_data() -> Dictionary:
    var result: Dictionary
    result = Project.category_add({
        id = tr(".category.t1.id"),
        name = tr(".category.t1.name"),
        section = tr(".category.t1.section"),
    })
    if result.has("Err"): return result
    result = Project.category_add({
        id = tr(".category.t2.id"),
        name = tr(".category.t2.name"),
        section = tr(".category.t2.section"),
    })
    return result
