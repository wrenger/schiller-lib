extends Control

signal add_user(user)
signal update_user(user)
signal show_books(user)

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
    emit_signal("show_books", _user_pane.user)


func _on_edit_cancel():
    set_user(before_edit)
    before_edit = {}


func _on_edit_add():
    var user: Dictionary = _user_pane.user
    var result: Dictionary = Project.user_add(user)
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    set_user(_user_pane.user)
    emit_signal("add_user", user)
    before_edit = {}


func _on_edit_apply():
    var user: Dictionary = _user_pane.user
    var result: Dictionary = Project.user_update(before_edit.account, user)
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    set_user(user)
    emit_signal("update_user", user)
    before_edit = {}


func _on_edit_delete():
    var result: Dictionary = Project.user_delete(before_edit.account)
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    set_user({})
    emit_signal("update_user", {})
    before_edit = {}
