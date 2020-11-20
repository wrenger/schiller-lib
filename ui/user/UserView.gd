extends Control

export var editable := false setget set_editable
var user: Dictionary = {} setget set_user, get_user

export var account_path: NodePath
export var forename_path: NodePath
export var surname_path: NodePath
export var role_path: NodePath
export var may_borrow_path: NodePath
export var user_request: NodePath

onready var _account: LineEdit = get_node(account_path)
onready var _forename: LineEdit = get_node(forename_path)
onready var _surname: LineEdit = get_node(surname_path)
onready var _role: LineEdit = get_node(role_path)
onready var _may_borrow: CheckBox = null
onready var _user_request: Button = get_node(user_request)

func _ready():
    set_editable(editable)
    if may_borrow_path: _may_borrow = get_node(may_borrow_path)


func set_user(u: Dictionary):
    if is_inside_tree():
        _account.text = u.get("account", "")
        _forename.text = u.get("forename", "")
        _surname.text = u.get("surname", "")
        _role.text = u.get("role", "")
        if _may_borrow: _may_borrow.pressed = u.get("may_borrow", true)


func get_user() -> Dictionary:
    if is_inside_tree():
        var may_borrow := true
        if _may_borrow: may_borrow = _may_borrow.pressed
        return {
            account = _account.text,
            forename = _forename.text,
            surname = _surname.text,
            role = _role.text,
            may_borrow = may_borrow,
        }
    return {}


func set_editable(e: bool):
    editable = e
    if is_inside_tree():
        _account.editable = e
        _forename.editable = e
        _surname.editable = e
        _role.editable = e
        if _may_borrow: _may_borrow.disabled = not e
        _user_request.visible = e


func _on_request(_x = null):
    var result: Dictionary = Project.settings_get()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var settings: Dictionary = result["Ok"]

    # provider selection & configuration
    var provider = UserCSVProvider.new()
    provider.path = settings.user_path
    if len(settings.user_delimiter) != 1:
        return MessageDialog.error(Util.trf(".error.provider.config", [tr(".pref.user.delimiter")]))
    provider.delimiter = settings.user_delimiter
    result = provider.request(_account.text)
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    var data: Dictionary = result["Ok"]
    _account.text = data.account
    _forename.text = data.forename
    _surname.text = data.surname
    _role.text = data.role
