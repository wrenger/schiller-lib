extends Node

signal search_results(results)

onready var _basic = $Tabs/Basic

onready var _account: LineEdit = $Tabs/Advanced/Account
onready var _forename: LineEdit = $Tabs/Advanced/Grid/Forename
onready var _surname: LineEdit = $Tabs/Advanced/Grid/Surname
onready var _role: LineEdit = $Tabs/Advanced/Grid/Role
onready var _may_borrow: OptionButton = $Tabs/Advanced/Grid/MayBorrow


func _on_search(_t = null):
    var result: Dictionary = Project.user_search(_basic.text)
    if result.has("Ok"):
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_advanced_search(_t = null):
    var id := _may_borrow.get_selected_id()
    var result: Dictionary = Project.user_search_advanced({
        account = _account.text,
        forename = _forename.text,
        surname = _surname.text,
        role = _role.text,
        may_borrow = null if id >= 2 else (id != 0),
    })

    if result.has("Ok"):
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])
