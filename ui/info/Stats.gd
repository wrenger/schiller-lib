extends Control

onready var _books: Label = $Entries/Box/Books
onready var _authors: Label = $Entries/Box/Authors
onready var _users: Label = $Entries/Box/Users
onready var _borrows: Label = $Borrowing/Box/Borrows
onready var _reservations: Label = $Borrowing/Box/Reservations
onready var _overdues: Label = $Borrowing/Box/Overdues


func _ready() -> void:
    var result := OK
    for node in get_tree().get_nodes_in_group("ProjectChanger"):
        result = node.connect("project_changed", self, "_reload")
        assert(result == OK)
    result = connect("visibility_changed", self, "_reload")
    assert(result == OK)


func _reload():
    if visible:
        var result: Dictionary = Project.stats()
        if result.has("Err"): return MessageDialog.error_code(result["Err"])

        var stats: Dictionary = result["Ok"]
        _books.text = Util.trf(".info.books", [stats.books])
        _authors.text = Util.trf(".info.authors", [stats.authors])
        _users.text = Util.trf(".info.users", [stats.users])
        _borrows.text = Util.trf(".info.borrows", [stats.borrows])
        _reservations.text = Util.trf(".info.reservations", [stats.reservations])
        _overdues.text = Util.trf(".info.overdues", [stats.overdues])
