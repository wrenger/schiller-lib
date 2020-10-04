extends Control

signal add_user(user)
signal update_user(user)
signal show_media(user)

onready var _project := $"/root/Project" as Project

onready var _user_pane := $User as Control
onready var _user_show_media := $ShowMedia as Control
onready var _user_edit := $Edit as Control
onready var _user_editing := $Editing as Control
onready var _user_adding := $Adding as Control

var before_edit: Reference = null

func _ready() -> void:
    set_user(null)


func set_user(user: Reference):
    visible = user != null

    if user:
        _user_pane.user = user
        _user_show_media.visible = true
        _user_edit.visible = true
        _user_editing.visible = false
        _user_adding.visible = false

    _user_pane.editable = false


func _on_user_selected(user: Reference) -> void:
    set_user(user)


func _on_edit() -> void:
    before_edit = _user_pane.user
    _user_show_media.visible = false
    _user_edit.visible = false
    _user_adding.visible = false

    _user_editing.visible = true
    _user_pane.editable = true


func _on_add() -> void:
    before_edit = null
    _user_pane.user = null
    _user_show_media.visible = false
    _user_edit.visible = false
    _user_editing.visible = false

    visible = true
    _user_adding.visible = true
    _user_pane.editable = true


func _on_show_media() -> void:
    print("Show media")
    emit_signal("show_media", _user_pane.user)


func _on_edit_cancel() -> void:
    set_user(before_edit)
    before_edit = null


func _on_edit_add() -> void:
    var result: Dictionary = _project.user_add(_user_pane.user)
    if result.has("Ok"):
        set_user(_user_pane.user)
        emit_signal("add_user", _user_pane.user)
        before_edit = null
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.alert(get_tree(), tr(".user.invalid"))
        else:
            MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_edit_apply() -> void:
    var result: Dictionary = _project.user_update(before_edit.account, _user_pane.user)
    if result.has("Ok"):
        set_user(_user_pane.user)
        emit_signal("update_user", _user_pane.user)
        before_edit = null
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.alert(get_tree(), tr(".user.invalid"))
        else:
            MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_edit_delete() -> void:
    var result: Dictionary = _project.user_delete(before_edit.account)
    if result.has("Ok"):
        set_user(null)
        emit_signal("update_user", null)
        before_edit = null
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))
