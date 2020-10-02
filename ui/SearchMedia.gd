extends MarginContainer

onready var project := get_node("/root/Project") as Project

export var media_list: NodePath

export var medium_box: NodePath
export var medium_pane: NodePath
export var medium_state: NodePath
export var medium_lend: NodePath
export var medium_lend_to: NodePath
export var medium_revoke: NodePath
export var medium_reserve: NodePath
export var medium_release: NodePath
export var medium_edit: NodePath
export var medium_editing: NodePath
export var medium_adding: NodePath

onready var _media_list := get_node(media_list) as Tree
onready var _medium_box := get_node(medium_box) as Control
onready var _medium_pane := get_node(medium_pane) as Control
onready var _medium_state := get_node(medium_state) as Label
onready var _medium_lend := get_node(medium_lend) as Control
onready var _medium_lend_to := get_node(medium_lend_to) as Control
onready var _medium_revoke := get_node(medium_revoke) as Control
onready var _medium_reserve := get_node(medium_reserve) as Control
onready var _medium_release := get_node(medium_release) as Control
onready var _medium_edit := get_node(medium_edit) as Control
onready var _medium_editing := get_node(medium_editing) as Control
onready var _medium_adding := get_node(medium_adding) as Control


var before_edit: Reference = null

func _ready() -> void:
    set_medium(null)


func set_medium(medium: Reference, update_pane = true):
    _medium_box.visible = medium != null

    if medium != null:
        if update_pane: _medium_pane.medium = medium

        var borrowed := bool(medium.borrower)
        var reserved := bool(medium.reservation)

        _medium_state.text = ""
        if borrowed:
            _medium_state.text = tr(".medium.borrowed.by").replace("{0}", medium.borrower).replace("{1}", medium.deadline)
        if reserved:
            if medium.borrower:
                _medium_state.text += "\n"
            _medium_state.text += tr(".medium.reserved.by").replace("{0}", medium.reservation)
        if not borrowed and not reserved:
            _medium_state.text = tr(".medium.available")

        _medium_lend.visible = not borrowed and not reserved
        _medium_lend_to.visible = reserved
        if reserved: _medium_lend_to.text = tr(".medium.lend.to").replace("{0}", medium.reservation)
        _medium_revoke.visible = borrowed
        _medium_reserve.visible = borrowed
        _medium_release.visible = reserved
        _medium_edit.visible = true
        _medium_editing.visible = false
        _medium_adding.visible = false

    _medium_pane.editable = false


func _on_basic_search(new_text):
    var result: Dictionary = project.medium_search(new_text)

    if result.has("Ok"):
        _media_list.fill(result["Ok"])
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_medium_selected(medium: Reference):
    set_medium(medium)


func _on_lend():
    LendDialog.lend(get_tree(), _medium_pane.medium)


func _on_lend_to():
    MessageDialog.alert(get_tree(), "WIP")


func _on_reserve():
    MessageDialog.alert(get_tree(), "WIP")


func _on_release():
    MessageDialog.alert(get_tree(), "WIP")


func _on_revoke():
    MessageDialog.alert(get_tree(), "WIP")


func _on_edit():
    before_edit = _medium_pane.medium
    _medium_lend.visible = false
    _medium_lend_to.visible = false
    _medium_revoke.visible = false
    _medium_reserve.visible = false
    _medium_release.visible = false
    _medium_edit.visible = false
    _medium_adding.visible = false

    _medium_editing.visible = true
    _medium_pane.editable = true


func _on_add_medium() -> void:
    before_edit = null
    _medium_pane.medium = null
    _medium_lend.visible = false
    _medium_lend_to.visible = false
    _medium_revoke.visible = false
    _medium_reserve.visible = false
    _medium_release.visible = false
    _medium_edit.visible = false
    _medium_editing.visible = false

    _medium_adding.visible = true
    _medium_box.visible = true
    _medium_pane.editable = true


func _on_edit_cancel() -> void:
    set_medium(before_edit)
    before_edit = null


func _on_edit_add() -> void:
    var medium = _medium_pane.medium
    var result: Dictionary = project.medium_add(medium)
    if result.has("Ok"):
        set_medium(medium, false)
        _media_list.add_and_select_object(medium)
        before_edit = null
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))
        _on_edit_cancel()


func _on_edit_apply() -> void:
    var medium = _medium_pane.medium
    var result: Dictionary = project.medium_update(before_edit.id, medium)
    if result.has("Ok"):
        set_medium(medium, false)
        _media_list.update_selected(medium)
        before_edit = null
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))
        _on_edit_cancel()


func _on_edit_delete() -> void:
    var result: Dictionary = project.medium_delete(before_edit.id)
    if result.has("Ok"):
        set_medium(null)
        _media_list.update_selected(null)
        before_edit = null
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))
        _on_edit_cancel()

