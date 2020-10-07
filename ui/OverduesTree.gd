extends Tree

onready var _project: Project = $"/root/Project"


func _ready():
    set_column_min_width(0, 100)
    set_column_min_width(1, 200)
    set_column_min_width(2, 100)


func reload():
    print("reload")
    clear()
    var result: Dictionary = _project.rental_overdues()
    var root := create_item()
    if result.has("Ok"):
        var role: TreeItem = null
        for period in result["Ok"]:
            var medium = period[0]
            var user = period[1]

            if not role or role.get_text(0) != user.role:
                role = create_item(root)
                role.set_text(0, user.role)

            var item := create_item(role)
            item.set_text(0, user.forename + " " + user.surname)
            item.set_text(1, medium.title + " (" + medium.id + ")")
            item.set_text(2, Util.trf(".medium.period", [medium.deadline_local(), medium.deadline_days()]))

    else:
        MessageDialog.error_code(result["Err"])


func _on_visibility_changed():
    if is_visible_in_tree(): reload()


func _on_copy() -> void:
    var text := ""
    var role := get_root().get_children()
    while role:
        if text: text += "\n"
        text += role.get_text(0) + "\n"
        var period := role.get_children()
        while period:
            text += period.get_text(0) + " - " + period.get_text(1) + " - " + period.get_text(2) + "\n"
            period = period.get_next()
        role = role.get_next()
    OS.clipboard = text
