extends GridContainer

export var editable := false setget set_editable

var book: Dictionary = {} setget set_book, get_book

onready var _project: Project = $"/root/Project"

onready var _id := $ID/Input as LineEdit
onready var _id_btn := $ID/Generate as Button
onready var _isbn := $ISBN/Input as LineEdit
onready var _isbn_btn := $ISBN/Request as Button
onready var _title := $Title as LineEdit
onready var _publisher := $Publisher as LineEdit
onready var _costs := $Price as SpinBox
onready var _year := $Year as SpinBox
onready var _authors := $Authors/List as Tree
onready var _authors_btns := $Authors/Box as Control
onready var _authors_remove := $Authors/Box/Remove as Button
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
        if m.has("authors"): _set_authors(m.authors)
        else: _authors.clear()
        if m.has("category"): _category.select_category(m.category)
        else: _category.select(0)
        _note.text = m.get("note", "")
        _borrowable.pressed = m.get("borrowable", true)
        _borrower = m.get("borrower", "")
        _deadline = m.get("deadline", "")
        _reservation = m.get("reservation", "")


func get_book() -> Dictionary:
    var authors := []
    if _authors.get_root():
        var child := _authors.get_root().get_children() as TreeItem
        while child:
            authors.push_back(child.get_text(0))
            child = child.get_next()
    return {
        id = _id.text,
        isbn = _isbn.text,
        title = _title.text,
        publisher = _publisher.text,
        costs = _costs.value,
        year = _year.value as int,
        authors = authors,
        category = _category.get_selected_category_id(),
        note = _note.text,
        borrowable = _borrowable.pressed,
        borrower = _borrower,
        deadline = _deadline,
        reservation = _reservation,
    }


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
        _authors_btns.visible = e
        _authors_remove.disabled = true
        if _authors.get_root():
            var child := _authors.get_root().get_children() as TreeItem
            while child:
                child.set_editable(0, e)
                child = child.get_next()


func _set_authors(authors):
    _authors.clear()
    var root := _authors.create_item() as TreeItem
    for author in authors:
        var item := _authors.create_item(root) as TreeItem
        item.set_text(0, author)


func generate_id():
    if editable:
        var book = get_book()
        book.id = _id_before_edit # fallback if nothing changed
        var result = _project.book_generate_id(book)
        if result.has("Ok"):
            _id.text = result["Ok"]
        else:
            MessageDialog.error_code(result["Err"])


func _on_author_add():
    var item = _authors.create_item(_authors.get_root())
    item.set_text(0, tr(".book.authors.def"))
    item.set_editable(0, true)


func _on_author_remove():
    var selected = _authors.get_selected()
    if selected:
        selected.deselect(0)
        _authors.get_root().remove_child(selected)


func _on_author_selected():
     _authors_remove.disabled = _authors.get_selected() == null


func _on_request(x = null):
    var result: Dictionary = _project.settings_get()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var settings: Dictionary = result["Ok"]

    # TODO: flexible provider selection & configuration
    var provider = BookDNBProvider.new()
    provider.token = settings.dnb_token
    result = provider.request(_isbn.text)
    if result.has("Err"): return MessageDialog.error_code(result["Err"])

    var data: Dictionary = result["Ok"]
    print(data)
    _title.text = data.title
    _set_authors(data.authors)
    _costs.value = data.costs
    _publisher.text = data.publisher
