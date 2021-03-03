extends GridContainer

export var book: Dictionary = {} setget set_book, get_book
export var editable := false setget set_editable

onready var _id := $ID/Input as LineEdit
onready var _id_btn := $ID/Generate as Button
onready var _isbn := $ISBN/Input as LineEdit
onready var _isbn_btn := $ISBN/Request as Button
onready var _title := $Title as LineEdit
onready var _publisher := $Publisher as LineEdit
onready var _costs := $Price as SpinBox
onready var _year := $Year as SpinBox
onready var _authors := $Authors/List as AuthorList
onready var _category := $Category as CategorySelect
onready var _note := $Notes as TextEdit
onready var _borrowable := $Borrowable as CheckBox

var _borrower := ""
var _deadline := ""
var _reservation := ""
var _id_before_edit := ""


func _ready():
    set_editable(editable)


func set_book(m: Dictionary):
    if is_inside_tree():
        _id.text = m.get("id", "")
        _isbn.text = m.get("isbn", "")
        _title.text = m.get("title", "")
        _publisher.text = m.get("publisher", "")
        _costs.value = m.get("costs", 0.0)
        if m.has("year"): _year.value = m.year
        else: _year.value = Date.new().get_year()
        _authors.authors = m.get("authors", [])
        _category.select_category(m.get("category", ""))
        _note.text = m.get("note", "")
        _borrowable.pressed = m.get("borrowable", true)
        _borrower = m.get("borrower", "")
        _deadline = m.get("deadline", "")
        _reservation = m.get("reservation", "")


func get_book() -> Dictionary:
    if is_inside_tree():
        return {
            id = _id.text,
            isbn = _isbn.text,
            title = _title.text,
            publisher = _publisher.text,
            costs = _costs.value,
            year = _year.value as int,
            authors = _authors.authors,
            category = _category.get_selected_category_id(),
            note = _note.text,
            borrowable = _borrowable.pressed,
            borrower = _borrower,
            deadline = _deadline,
            reservation = _reservation,
        }
    return {}


func set_editable(e: bool):
    editable = e
    if is_inside_tree():
        if e: _id_before_edit = _id.text
        else: _id_before_edit = ""

        _id.editable = e
        _id_btn.visible = e
        _isbn.editable = e
        _isbn_btn.visible = e
        _title.editable = e
        _publisher.editable = e
        _costs.editable = e
        _year.editable = e
        _category.disabled = not e
        _note.readonly = not e
        _borrowable.disabled = not e
        _authors.editable = e


func generate_id():
    if editable:
        var b := get_book()
        b.id = _id_before_edit # fallback if nothing changed
        var result: Dictionary = Project.book_generate_id(b)
        if result.has("Err"): return MessageDialog.error_code(result["Err"])
        _id.text = result["Ok"]


# Requesting metadata from the book provider
func _on_request(_x = null):
    var result: Dictionary = Project.settings_get()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var settings: Dictionary = result["Ok"]

    # provider selection & configuration
    var provider_plugin = load("res://plugins/BookProvider.gd")
    if provider_plugin == null:
        return MessageDialog.error(Util.trf(".error.provider.none", ["BookProvider"]))

    var provider = provider_plugin.new()
    provider.token = settings.dnb_token
    result = provider.request(_isbn.text)
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    var data: Dictionary = result["Ok"]
    _title.text = data.title
    _authors.authors = data.authors
    _costs.value = data.costs
    _publisher.text = data.publisher
