extends GridContainer

export var editable := false setget set_editable

var user: Dictionary = {} setget set_user, get_user

func _ready():
    set_editable(editable)


func set_user(m: Dictionary):
    if not m.empty():
        $Account.text = m.account
        $Forename.text = m.forename
        $Surname.text = m.surname
        $Role.text = m.role
        $MayBorrow.pressed = m.may_borrow
    else:
        $Account.clear()
        $Forename.clear()
        $Surname.clear()
        $Role.clear()
        $MayBorrow.pressed = true


func get_user() -> Dictionary:
    return {
        account = $Account.text,
        forename = $Forename.text,
        surname = $Surname.text,
        role = $Role.text,
        may_borrow = $MayBorrow.pressed,
    }


func set_editable(e: bool):
    editable = e
    $Account.editable = e
    $Forename.editable = e
    $Surname.editable = e
    $Role.editable = e
    $MayBorrow.disabled = not e
