extends ConfirmationDialog
class_name LendDialog

onready var _project: Project = $"/root/Project"

onready var _window_content: Control = $"../Content"
onready var _state: Label = $Box/State
onready var _user_input: LineEdit = $Box/User/Search
onready var _user_popup: PopupMenu = $Box/User/Search/Popup
onready var _user_state: Label = $Box/User/State
onready var _period: SpinBox = $Box/Period/Days
onready var _period_panel: Control = $Box/Period

var _medium_panel: Control = null
var _medium: Reference = null
var _user: Reference = null

var _user_result := []


static func lend(medium_panel: Control, medium: Reference):
    var nodes = medium_panel.get_tree().get_nodes_in_group("LendDialog")
    if nodes: nodes.front()._lend(medium_panel, medium)


static func reserve(medium_panel: Control, medium: Reference):
    var nodes = medium_panel.get_tree().get_nodes_in_group("LendDialog")
    if nodes: nodes.front()._reserve(medium_panel, medium)


func _ready():
    _period.suffix = tr(".medium.period.days")


func _lend(medium_panel: Control, medium: Reference):
    if not visible:
        _medium_panel = medium_panel
        _medium = medium
        _user = null
        _user_input.clear()
        _user_state.text = ""
        _period_panel.visible = true
        window_title = tr(".medium.lend") + " - " + medium.id + ": " + medium.title
        get_ok().text = tr(".medium.lend")
        popup_centered()


func _reserve(medium_panel: Control, medium: Reference):
    if not visible:
        _medium_panel = medium_panel
        _medium = medium
        _user = null
        _user_input.clear()
        _user_state.text = ""
        _period_panel.visible = false
        window_title = tr(".medium.reserve") + " - " + medium.id + ": " + medium.title
        get_ok().text = tr(".medium.reserve")
        popup_centered()


func _popup_hide():
    _window_content.modulate.a = 1


func _about_to_show():
    _window_content.modulate.a = 0.5


func _on_user_input_entered(new_text: String):
    if not new_text: new_text = _user_input.text
    var result: Dictionary = _project.user_search(new_text)
    if result.has("Ok"):
        _user = null
        _user_state.text = ""
        _user_popup.clear()
        _user_result = result["Ok"]
        for user in _user_result:
            _user_popup.add_radio_check_item(user.account + " - " + user.forename + " " + user.surname + " (" + user.role + ")")
        var gp := _user_input.rect_global_position
        var s := _user_input.rect_size
        _user_popup.popup(Rect2(gp + Vector2(0, s.y), s))
    else:
        _state.text = tr(result["Err"])


func _on_user_selected(index: int):
    var user = _user_result[index]
    _user_input.text = user.account
    _user_state.text = user.forename + " " + user.surname + " (" + user.role + ")"
    _user = user


func _on_confirmed():
    if _user == null or _medium == null:
        _state.text = tr(".error.input")
        popup_centered()
        return

    var result: Dictionary
    if _period_panel.visible:
        result = _project.rental_lend(_medium, _user, int(_period.value))
    else:
        result = _project.rental_reserve(_medium, _user)

    if result.has("Ok"):
        _medium_panel._on_lend_update(result["Ok"])
    else:
        _state.text = Util.error_msg(result["Err"])
        popup_centered()

