extends VBoxContainer

onready var _account: LineEdit = $Account
onready var _forename: LineEdit = $Name/Forename
onready var _surname: LineEdit = $Name/Surname
onready var _role: LineEdit = $Role


func set_account(account: String):
    _account.text = account


func clear():
    _account.clear()
    _forename.clear()
    _surname.clear()
    _role.clear()


func get_user() -> Dictionary:
    return {
        account = _account.text.strip_edges(),
        forename = _forename.text.strip_edges(),
        surname = _surname.text.strip_edges(),
        role = _role.text.strip_edges(),
        may_borrow = true,
    }
