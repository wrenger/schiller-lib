extends ConfirmationDialog
class_name LendDialog

onready var _window_content: Control = $"../Content"
onready var _state: Label = $Box/State

onready var _user_search: Control = $Box/User
onready var _user_search_input: LineEdit = $Box/User/Search
onready var _user_search_popup: PopupMenu = $Box/User/Search/Popup
onready var _user_search_state: Label = $Box/User/State

onready var _user_add: Control = $Box/AddUser

onready var _period: SpinBox = $Box/Period/Days
onready var _period_panel: Control = $Box/Period

var _book_panel: Control = null
var _book: Dictionary = {}
var _user: Dictionary = {}

var _user_result := []


static func lend(book_panel: Control, book: Dictionary, account: String = ""):
    var nodes = book_panel.get_tree().get_nodes_in_group("LendDialog")
    if nodes: nodes.front()._lend(book_panel, book, account)


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


func _lend(book_panel: Control, book: Dictionary, account: String):
    if not visible:
        _book_panel = book_panel
        _book = book

        if account:
            var result: Dictionary = Project.user_fetch(account)
            if result.has("Err"): return MessageDialog.error_code(result["Err"])
            _set_user(result["Ok"])
        else:
            _set_user({})

        var result: Dictionary = Project.settings_get()
        if result.has("Err"): return MessageDialog.error_code(result["Err"])

        _period.value = result["Ok"].borrowing_duration
        _period_panel.visible = true
        _state.text = ""
        _user_search.visible = true
        _user_add.visible = false
        window_title = tr(".book.lend") + " - " + book.id + ": " + book.title
        get_ok().text = tr(".book.lend")
        popup_centered(Vector2(rect_size.x, 0))


func _reserve(book_panel: Control, book: Dictionary):
    if not visible:
        _book_panel = book_panel
        _book = book
        _set_user({})
        _period_panel.visible = false
        _state.text = ""
        _user_search.visible = true
        _user_add.visible = false
        window_title = tr(".book.reserve") + " - " + book.id + ": " + book.title
        get_ok().text = tr(".book.reserve")
        popup_centered(Vector2(rect_size.x, 0))


func _popup_hide():
    _window_content.modulate.a = 1


func _about_to_show():
    _window_content.modulate.a = 0.5


func _on_user_input_entered(new_text: String):
    if not new_text: new_text = _user_search_input.text
    var result: Dictionary = Project.user_search(new_text)
    if result.has("Ok"):
        _user = {}
        _user_search_state.text = ""
        _user_search_popup.clear()
        _user_result = result["Ok"]
        for user in _user_result:
            _user_search_popup.add_radio_check_item(user.account + " - " + user.forename + " " + user.surname + " (" + user.role + ")")
        _user_search_popup.add_radio_check_item(tr(".user.new"))
        var gp := _user_search_input.rect_global_position
        var s := _user_search_input.rect_size
        _user_search_popup.popup(Rect2(gp + Vector2(0, s.y), s))
    else:
        _state.text = tr(result["Err"])


func _on_user_selected(index: int):
    if index >= 0:
        if index < len(_user_result):
            _set_user(_user_result[index])
        else:
            _show_add_user()
    else: _set_user({})


func _show_add_user():
    _user_search.visible = false
    _user_add.visible = true
    _user_add.set_user({account = _user_search_input.text})


func _set_user(user: Dictionary):
    _user = user
    if user:
        _user_search_input.text = user.account
        _user_search_state.text = user.forename + " " + user.surname + " (" + user.role + ")"
    else:
        _user_search_input.text = ""
        _user_search_state.text = ""


func _on_confirmed():
    # Add user if user add panel is shown
    if _user_add.visible:
        _user = _user_add.get_user()
        var result: Dictionary = Project.user_add(_user)
        if result.has("Err"):
            _state.text = Util.error_msg(result["Err"])
            popup_centered()
            return

    if not _user or not _book:
        _state.text = tr(".error.input")
        popup_centered()
        return

    var result: Dictionary
    if _period_panel.visible:
        result = Project.lending_lend(_book, _user, int(_period.value))
    else:
        result = Project.lending_reserve(_book, _user)

    if result.has("Ok"):
        _book_panel._on_lend_update(result["Ok"])
    else:
        _state.text = Util.error_msg(result["Err"])
        popup_centered()

