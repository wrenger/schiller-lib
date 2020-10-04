extends Control

signal add_medium(medium)
signal update_medium(medium)

onready var project := get_node("/root/Project") as Project

onready var _medium_pane := $Medium as Control
onready var _medium_state := $State as Label
onready var _medium_lend := $Lend as Control
onready var _medium_lend_to := $LendTo as Control
onready var _medium_revoke := $Revoke as Control
onready var _medium_reserve := $Reserve as Control
onready var _medium_release := $Release as Control
onready var _medium_edit := $Edit as Control
onready var _medium_editing := $Editing as Control
onready var _medium_adding := $Adding as Control

var _before_edit: Reference = null


func _ready() -> void:
    set_medium(null)


func set_medium(medium: Reference):
    visible = medium != null

    if medium != null:
        _medium_pane.medium = medium

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


func _on_medium_selected(medium: Reference):
    set_medium(medium)


func _on_lend():
    LendDialog.lend(get_tree(), _medium_pane.medium)


func _on_lend_to():
    MessageDialog.alert(get_tree(), "WIP")


func _on_reserve():
    MessageDialog.alert(get_tree(), "WIP")


func _on_release():
    var medium = _medium_pane.medium
    var result: Dictionary = project.rental_release(medium.id)
    if result.has("Ok"):
        set_medium(medium)
        emit_signal("update_medium", medium)
    else:
        MessageDialog.error(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_revoke():
    var medium = _medium_pane.medium
    var result: Dictionary = project.rental_revoke(medium.id)
    if result.has("Ok"):
        set_medium(medium)
        emit_signal("update_medium", medium)
    else:
        MessageDialog.error(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_edit():
    _before_edit = _medium_pane.medium
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
    _before_edit = null
    _medium_pane.medium = null
    _medium_lend.visible = false
    _medium_lend_to.visible = false
    _medium_revoke.visible = false
    _medium_reserve.visible = false
    _medium_release.visible = false
    _medium_edit.visible = false
    _medium_editing.visible = false

    visible = true
    _medium_adding.visible = true
    _medium_pane.editable = true


func _on_edit_cancel() -> void:
    set_medium(_before_edit)
    _before_edit = null


func _on_edit_add() -> void:
    var medium = _medium_pane.medium
    var result: Dictionary = project.medium_add(medium)
    if result.has("Ok"):
        set_medium(medium)
        emit_signal("add_medium", medium)
        _before_edit = null
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.alert(get_tree(), tr(".medium.invalid"))
        else:
            MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_edit_apply() -> void:
    var medium = _medium_pane.medium
    var result: Dictionary = project.medium_update(_before_edit.id, medium)
    if result.has("Ok"):
        set_medium(medium)
        emit_signal("update_medium", medium)
        _before_edit = null
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.alert(get_tree(), tr(".medium.invalid"))
        else:
            MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_edit_delete() -> void:
    var result: Dictionary = project.medium_delete(_before_edit.id)
    if result.has("Ok"):
        set_medium(null)
        emit_signal("update_medium", null)
        _before_edit = null
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))
