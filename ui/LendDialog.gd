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

var _book_panel: Control = null
var _book: Dictionary = {}
var _user: Dictionary = {}

var _user_result := []


static func lend(book_panel: Control, book: Dictionary, user: Dictionary = {}):
    var nodes = book_panel.get_tree().get_nodes_in_group("LendDialog")
    if nodes: nodes.front()._lend(book_panel, book, user)


static func reserve(book_panel: Control, book: Dictionary):
    var nodes = book_panel.get_tree().get_nodes_in_group("LendDialog")
    if nodes: nodes.front()._reserve(book_panel, book)


func _ready() -> void:
    var result: int
    result = connect("popup_hide", self, "_popup_hide")
    assert(result == OK)
    result = connect("about_to_show", self, "_about_to_show")
    assert(result == OK)
    result = connect("confirmed", self, "_on_confirmed")
    assert(result == OK)


func _lend(book_panel: Control, book: Dictionary, user: Dictionary):
    if not visible:
        _book_panel = book_panel
        _book = book
        _set_user(user)
        var result: Dictionary = _project.settings_get()
        if result.has("Err"):
            MessageDialog.error_code(result["Err"])
            return
        _period.value = result["Ok"].borrowing_duration
        _period_panel.visible = true
        _state.text = ""
        window_title = tr(".book.lend") + " - " + book.id + ": " + book.title
        get_ok().text = tr(".book.lend")
        popup_centered()


func _reserve(book_panel: Control, book: Dictionary):
    if not visible:
        _book_panel = book_panel
        _book = book
        _set_user({})
        _period_panel.visible = false
        _state.text = ""
        window_title = tr(".book.reserve") + " - " + book.id + ": " + book.title
        get_ok().text = tr(".book.reserve")
        popup_centered()


func _popup_hide():
    _window_content.modulate.a = 1


func _about_to_show():
    _window_content.modulate.a = 0.5


func _on_user_input_entered(new_text: String):
    if not new_text: new_text = _user_input.text
    var result: Dictionary = _project.user_search(new_text)
    if result.has("Ok"):
        _set_user({})
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
    if index >= 0: _set_user(_user_result[index])
    else: _set_user({})


func _set_user(user: Dictionary):
    _user = user
    if user:
        _user_input.text = user.account
        _user_state.text = user.forename + " " + user.surname + " (" + user.role + ")"
    else:
        _user_input.text = ""
        _user_state.text = ""


func _on_confirmed():
    if not _user or not _book:
        _state.text = tr(".error.input")
        popup_centered()
        return

    var result: Dictionary
    if _period_panel.visible:
        result = _project.lending_lend(_book, _user, int(_period.value))
    else:
        result = _project.lending_reserve(_book, _user)

    if result.has("Ok"):
        _book_panel._on_lend_update(result["Ok"])
    else:
        _state.text = Util.error_msg(result["Err"])
        popup_centered()

