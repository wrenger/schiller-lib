extends AcceptDialog
class_name CategoryDialog

signal categories_changed(categories)

onready var _list: Tree = $Box/Box/Tree
onready var _status: Label = $Box/Status
onready var _delete_btn: Button = $Box/Box/Buttons/Delete
onready var _add_id: LineEdit = $Box/New/ID
onready var _add_name: LineEdit = $Box/New/Name
onready var _add_section: LineEdit = $Box/New/Section


func _ready() -> void:
    var result := OK
    result = connect("popup_hide", self, "_popup_hide")
    assert(result == OK)
    _list.set_column_min_width(0, 100)
    _list.set_column_title(0, tr(".category.id"))
    _list.set_column_min_width(1, 200)
    _list.set_column_title(1, tr(".category.name"))
    _list.set_column_min_width(2, 150)
    _list.set_column_title(2, tr(".category.section"))
    _list.set_column_titles_visible(true)


static func open():
    var scene: SceneTree = Engine.get_main_loop()
    var nodes := scene.get_nodes_in_group("CategoryDialog")
    if nodes: nodes.front()._open()


func _open():
    _delete_btn.disabled = true
    _add_id.clear()
    _add_name.clear()
    _add_section.clear()
    _list.clear()
    _status.text = ""

    var result: Dictionary = Project.category_list()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    var root := _list.create_item()
    for category in result["Ok"]:
        set_category(_list.create_item(root), category)
    popup_centered()


func set_category(item: TreeItem, category: Dictionary):
    item.set_text(0, category.id)
    item.set_editable(0, true)
    item.set_text(1, category.name)
    item.set_editable(1, true)
    item.set_text(2, category.section)
    item.set_editable(2, true)
    item.set_meta("category_backup", category)


func _on_add() -> void:
    var category := {
        id = _add_id.text,
        name = _add_name.text,
        section = _add_section.text,
    }
    var result: Dictionary = Project.category_add(category)
    if result.has("Ok"):
        var item := _list.create_item(_list.get_root())
        set_category(item, category)
        item.select(0)
        _add_id.clear()
        _add_name.clear()
        _add_section.clear()
        _status.text = ""
    elif result["Err"] == Util.SbvError.InvalidArguments:
        _status.text = tr(".category.empty-input")
    else:
        _status.text = Util.error_msg(result["Err"])


func _on_delete() -> void:
    var item := _list.get_selected()
    if item:
        var result: Dictionary = Project.category_remove(item.get_text(0))
        if result.has("Ok"):
            item.deselect(0)
            _list.get_root().remove_child(item)
            _status.text = ""
        elif result["Err"] == Util.SbvError.LogicError:
            _status.text = tr(".category.not-empty.del")
        else:
            _status.text = Util.error_msg(result["Err"])


func _on_edited() -> void:
    var item := _list.get_edited()
    var category := {
        id = item.get_text(0),
        name = item.get_text(1),
        section = item.get_text(2),
    }

    var category_backup = item.get_meta("category_backup")
    if category == category_backup: return

    var result: Dictionary = Project.category_update(category_backup.id, category)
    if result.has("Ok"):
        item.set_meta("category_backup", category)
        _status.text = ""
    elif result["Err"] == Util.SbvError.InvalidArguments:
        set_category(item, category_backup)
        _status.text = tr(".category.empty-input")
    else:
        set_category(item, category_backup)
        _status.text = Util.error_msg(result["Err"])


func _popup_hide():
    var result: Dictionary = Project.category_list()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    emit_signal("categories_changed", result["Ok"])


func _on_selected() -> void:
    var item := _list.get_selected()
    _delete_btn.disabled = not item
