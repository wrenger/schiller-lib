extends GridContainer

export var editable := false setget set_editable

var user: Reference = null setget set_user, get_user

func _ready():
    set_editable(editable)


func set_user(m: Reference):
    if m != null:
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


func get_user() -> Reference:
    var user := User.new()
    user.account = $Account.text
    user.forename = $Forename.text
    user.surname = $Surname.text
    user.role = $Role.text
    user.may_borrow = $MayBorrow.pressed
    return user


func set_editable(e: bool):
    editable = e
    $Account.editable = e
    $Forename.editable = e
    $Surname.editable = e
    $Role.editable = e
    $MayBorrow.disabled = not e
