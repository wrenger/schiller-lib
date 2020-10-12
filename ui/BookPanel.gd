extends Control

signal add_book(book)
signal update_book(book)

onready var _project: Project = $"/root/Project"

onready var _book_pane := $Book as Control
onready var _book_state := $State as Label
onready var _book_lend := $Lend as Control
onready var _book_lend_to := $LendTo as Control
onready var _book_revoke := $Revoke as Control
onready var _book_reserve := $Reserve as Control
onready var _book_release := $Release as Control
onready var _book_edit := $Edit as Control
onready var _book_editing := $Editing as Control
onready var _book_adding := $Adding as Control

var _before_edit: Dictionary = {}


func _ready():
    set_book({})


func set_book(book: Dictionary):
    visible = not book.empty()

    if not book.empty():
        _book_pane.book = book

        var borrowed := bool(book.borrower)
        var reserved := bool(book.reservation)

        _book_state.text = ""
        if borrowed:
            var date := Date.new()
            date.set_iso(book.deadline)
            _book_state.text = tr(".book.borrowed.by").replace("{0}", book.borrower).replace("{1}", date.get_local())
        if reserved:
            if book.borrower:
                _book_state.text += "\n"
            _book_state.text += tr(".book.reserved.by").replace("{0}", book.reservation)
        if not borrowed and not reserved:
            _book_state.text = tr(".book.available")

        _book_lend.visible = book.borrowable and not borrowed and not reserved
        _book_lend_to.visible = not borrowed and reserved
        if reserved: _book_lend_to.text = tr(".book.lend.to").replace("{0}", book.reservation)
        _book_revoke.visible = borrowed
        _book_reserve.visible = borrowed and not reserved
        _book_release.visible = reserved
        _book_edit.visible = true
        _book_editing.visible = false
        _book_adding.visible = false

    _book_pane.editable = false


func _on_book_selected(book: Dictionary):
    set_book(book)


func _on_lend():
    LendDialog.lend(self, _book_pane.book)


func _on_lend_to():
    var book: Dictionary = _book_pane.book
    var result: Dictionary = _project.user_fetch(book.reservation)
    if result.has("Ok"):
        LendDialog.lend(self, book, result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_reserve():
    LendDialog.reserve(self, _book_pane.book)


func _on_release():
    var book: Dictionary = _book_pane.book
    var result: Dictionary = _project.lending_release(book)
    if result.has("Ok"):
        set_book(result["Ok"])
        emit_signal("update_book", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_revoke():
    var book: Dictionary = _book_pane.book
    var result: Dictionary = _project.lending_return(book)
    if result.has("Ok"):
        set_book(result["Ok"])
        emit_signal("update_book", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_lend_update(book: Dictionary):
    set_book(book)
    emit_signal("update_book", book)


func _on_edit():
    _before_edit = _book_pane.book
    _book_lend.visible = false
    _book_lend_to.visible = false
    _book_revoke.visible = false
    _book_reserve.visible = false
    _book_release.visible = false
    _book_edit.visible = false
    _book_adding.visible = false

    _book_editing.visible = true
    _book_pane.editable = true


func _on_add_book():
    _before_edit = {}
    _book_pane.book = {}
    _book_lend.visible = false
    _book_lend_to.visible = false
    _book_revoke.visible = false
    _book_reserve.visible = false
    _book_release.visible = false
    _book_edit.visible = false
    _book_editing.visible = false

    visible = true
    _book_adding.visible = true
    _book_pane.editable = true


func _on_edit_cancel():
    set_book(_before_edit)
    _before_edit = {}


func _on_edit_add():
    var book: Dictionary = _book_pane.book
    var result: Dictionary = _project.book_add(book)
    if result.has("Ok"):
        set_book(book)
        emit_signal("add_book", book)
        _before_edit = {}
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.error(tr(".book.invalid"))
        else:
            MessageDialog.error_code(result["Err"])


func _on_edit_apply():
    var book: Dictionary = _book_pane.book
    var result: Dictionary = _project.book_update(_before_edit.id, book)
    if result.has("Ok"):
        set_book(book)
        emit_signal("update_book", book)
        _before_edit = {}
    else:
        if result["Err"] == Util.SbvError.LogicError:
            MessageDialog.error(tr(".book.invalid"))
        else:
            MessageDialog.error_code(result["Err"])


func _on_edit_delete():
    var result: Dictionary = _project.book_delete(_before_edit.id)
    if result.has("Ok"):
        set_book({})
        emit_signal("update_book", {})
        _before_edit = {}
    else:
        MessageDialog.error_code(result["Err"])
