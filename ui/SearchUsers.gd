extends MarginContainer

onready var project := $"/root/Project" as Project

export var user_list : NodePath
export var user_box : NodePath
export var user_pane : NodePath
export var user_show_media : NodePath
export var user_edit : NodePath
export var user_editing : NodePath

onready var _user_list := get_node(user_list) as Tree
onready var _user_box := get_node(user_box) as Control
onready var _user_pane := get_node(user_pane) as Control
onready var _user_show_media := get_node(user_show_media) as Control
onready var _user_edit := get_node(user_edit) as Control
onready var _user_editing := get_node(user_editing) as Control

var before_edit = null

func _ready() -> void:
    set_user(null)


func set_user(user, update_pane = true):
    _user_box.visible = user != null

    if user:
        if update_pane: _user_pane.user = user
        _user_show_media.visible = true
        _user_edit.visible = true
        _user_editing.visible = false

    _user_pane.editable = false


func _on_search(text: String) -> void:
    var result = project.search_user_basic(text)
    if result.has("Ok"):
        _user_list.fill(result["Ok"])
    else:
        MessageDialog.alert(get_tree(), "Search Error: " + result["Err"])


func _on_user_selected(user) -> void:
    assert(not _user_pane.editable)
    set_user(user)


func _on_edit() -> void:
    before_edit = _user_pane.user
    _user_show_media.visible = false
    _user_edit.visible = false

    _user_editing.visible = true
    _user_pane.editable = true


func _on_show_media() -> void:
    print("Show media")
    # TODO: Based on Advanced search -> medium.borrower || medium.reservation


func _on_edit_cancel() -> void:
    set_user(before_edit)
    before_edit = null


func _on_edit_apply() -> void:
    var result = project.update_user(before_edit.account, _user_pane.user)
    if result.has("Err"):
        MessageDialog.alert(get_tree(), "Error: " + result["Err"])
    set_user(_user_pane.user, false)
    _user_list.update_selected(_user_pane.user)
    before_edit = null


func _on_edit_delete() -> void:
    var result = project.delete_user(before_edit.account)
    if result.has("Err"):
        MessageDialog.alert(get_tree(), "Error: " + result["Err"])
    set_user(null)
    _user_list.update_selected(null)
    before_edit = null
