extends LineEdit

signal user_entered(user)

onready var _project: Project = $"/root/Project"
onready var _popup: Popup = $Popup
onready var _state: Label = $"../State"

var _result := []

func _on_text_entered(new_text: String) -> void:
    print(name, " _on_text_entered")
    var result: Dictionary = _project.user_search(text)
    if result.has("Ok"):
        _popup.clear()
        _result = result["Ok"]
        for user in _result:
            _popup.add_item(user.account + " - " + user.forename + " " + user.surname + " (" + user.role + ")")
        _popup.popup_centered()
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_user_selected(index: int) -> void:
    var user = _result[index]
    text = user.account
    _state.text = user.forename + " " + user.surname + " (" + user.role + ")"
    emit_signal("user_entered", user)
