extends Control

signal add_medium(medium)
signal update_medium(medium)

onready var _project: Project = $"/root/Project"

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


func _ready():
    set_medium(null)


func set_medium(medium: Reference):
    visible = medium != null

    if medium != null:
        _medium_pane.medium = medium

        var borrowed := bool(medium.borrower)
        var reserved := bool(medium.reservation)

        _medium_state.text = ""
        if borrowed:
            _medium_state.text = tr(".medium.borrowed.by").replace("{0}", medium.borrower).replace("{1}", medium.deadline_local())
        if reserved:
            if medium.borrower:
                _medium_state.text += "\n"
            _medium_state.text += tr(".medium.reserved.by").replace("{0}", medium.reservation)
        if not borrowed and not reserved:
            _medium_state.text = tr(".medium.available")

        _medium_lend.visible = medium.borrowable and not borrowed and not reserved
        _medium_lend_to.visible = not borrowed and reserved
        if reserved: _medium_lend_to.text = tr(".medium.lend.to").replace("{0}", medium.reservation)
        _medium_revoke.visible = borrowed
        _medium_reserve.visible = borrowed and not reserved
        _medium_release.visible = reserved
        _medium_edit.visible = true
        _medium_editing.visible = false
        _medium_adding.visible = false

    _medium_pane.editable = false


func _on_medium_selected(medium: Reference):
    set_medium(medium)


func _on_lend():
    LendDialog.lend(self, _medium_pane.medium)


func _on_lend_to():
    var medium = _medium_pane.medium
    var result: Dictionary = _project.user_get(medium.reservation)
    if result.has("Ok"):
        LendDialog.lend(self, medium, result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_reserve():
    LendDialog.reserve(self, _medium_pane.medium)


func _on_release():
    var medium = _medium_pane.medium
    var result: Dictionary = _project.rental_release(medium)
    if result.has("Ok"):
        set_medium(result["Ok"])
        emit_signal("update_medium", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_revoke():
    var medium = _medium_pane.medium
    var result: Dictionary = _project.rental_revoke(medium)
    print(result)
    if result.has("Ok"):
        set_medium(result["Ok"])
        emit_signal("update_medium", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_lend_update(medium: Reference):
    set_medium(medium)
    emit_signal("update_medium", medium)


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


func _on_add_medium():
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


func _on_edit_cancel():
    set_medium(_before_edit)
    _before_edit = null


func _on_edit_add():
    var medium = _medium_pane.medium
    var result: Dictionary = _project.medium_add(medium)
    if result.has("Ok"):
        set_medium(medium)
        emit_signal("add_medium", medium)
        _before_edit = null
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.error(tr(".medium.invalid"))
        else:
            MessageDialog.error_code(result["Err"])


func _on_edit_apply():
    var medium = _medium_pane.medium
    var result: Dictionary = _project.medium_update(_before_edit.id, medium)
    if result.has("Ok"):
        set_medium(medium)
        emit_signal("update_medium", medium)
        _before_edit = null
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.error(tr(".medium.invalid"))
        else:
            MessageDialog.error_code(result["Err"])


func _on_edit_delete():
    var result: Dictionary = _project.medium_delete(_before_edit.id)
    if result.has("Ok"):
        set_medium(null)
        emit_signal("update_medium", null)
        _before_edit = null
    else:
        MessageDialog.error_code(result["Err"])
