extends Control

signal add_user(user)
signal update_user(user)
signal show_books(user)

onready var _project: Project = $"/root/Project"

onready var _user_pane := $User as Control
onready var _user_show_books := $ShowBooks as Control
onready var _user_edit := $Edit as Control
onready var _user_editing := $Editing as Control
onready var _user_adding := $Adding as Control

var before_edit: Dictionary = {}

func _ready():
    set_user({})


func set_user(user: Dictionary):
    visible = not user.empty()

    if user:
        _user_pane.user = user
        _user_show_books.visible = true
        _user_edit.visible = true
        _user_editing.visible = false
        _user_adding.visible = false

    _user_pane.editable = false


func _on_user_selected(user: Dictionary):
    set_user(user)


func _on_edit():
    before_edit = _user_pane.user
    _user_show_books.visible = false
    _user_edit.visible = false
    _user_adding.visible = false

    _user_editing.visible = true
    _user_pane.editable = true


func _on_add():
    before_edit = {}
    _user_pane.user = {}
    _user_show_books.visible = false
    _user_edit.visible = false
    _user_editing.visible = false

    visible = true
    _user_adding.visible = true
    _user_pane.editable = true


func _on_show_books():
    print("Show books")
    emit_signal("show_books", _user_pane.user)


func _on_edit_cancel():
    set_user(before_edit)
    before_edit = {}


func _on_edit_add():
    var result: Dictionary = _project.user_add(_user_pane.user)
    if result.has("Ok"):
        set_user(_user_pane.user)
        emit_signal("add_user", _user_pane.user)
        before_edit = {}
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.error(tr(".user.invalid"))
        else:
            MessageDialog.error_code(result["Err"])


func _on_edit_apply():
    var result: Dictionary = _project.user_update(before_edit.account, _user_pane.user)
    if result.has("Ok"):
        set_user(_user_pane.user)
        emit_signal("update_user", _user_pane.user)
        before_edit = {}
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.error(tr(".user.invalid"))
        else:
            MessageDialog.error_code(result["Err"])


func _on_edit_delete():
    var result: Dictionary = _project.user_delete(before_edit.account)
    if result.has("Ok"):
        set_user({})
        emit_signal("update_user", {})
        before_edit = {}
    else:
        MessageDialog.error_code(result["Err"])
