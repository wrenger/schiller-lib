extends ConfirmationDialog
class_name LendDialog

onready var _window_content: Control = $"../Content"
onready var _project: Project = $"/root/Project"
onready var _state: Label = $Box/Lend/State
onready var _user_input: LineEdit = $Box/Lend/User/Search
onready var _user_state: Label = $Box/Lend/User/State
onready var _period: SpinBox = $Box/Lend/Period/Days

var _medium_panel: Control = null
var _medium: Reference = null
var _user: Reference = null

static func lend(medium_panel: Control, medium: Reference):
    var nodes = medium_panel.get_tree().get_nodes_in_group("LendDialog")
    if nodes: nodes.front()._lend(medium_panel, medium)


func _ready() -> void:
    $Box/Lend/Period/Days.suffix = tr(".medium.period.days")


func _lend(medium_panel: Control, medium: Reference):
    if not visible:
        _medium_panel = medium_panel
        _medium = medium
        _user = null
        _user_input.clear()
        _user_state.text = ""
        window_title = tr(".medium.lend") + " - " + medium.id + ": " + medium.title
        get_ok().text = tr(".medium.lend")
        popup_centered()


func _popup_hide():
    _window_content.modulate.a = 1


func _on_confirmed() -> void:
    if _user == null or _medium == null:
        _state.text = tr(".error.input")
        popup_centered()
        return

    var result: Dictionary = _project.rental_lend(_medium, _user, int(_period.value))
    if result.has("Ok"):
        _medium_panel._on_lend_update(result["Ok"])
    else:
        _state.text = tr(Util.error_msg(result["Err"]))
        popup_centered()


func _on_about_to_show() -> void:
    _window_content.modulate.a = 0.5


func _on_user_entered(user) -> void:
    _user = user
