extends GridContainer

export var editable := false setget set_editable

var medium: Reference = null setget set_medium, get_medium

onready var _project: Project = $"/root/Project"

onready var _id := $ID/Input as LineEdit
onready var _id_btn := $ID/Generate as Button
onready var _isbn := $ISBN as LineEdit
onready var _title := $Title as LineEdit
onready var _publisher := $Publisher as LineEdit
onready var _price := $Price as SpinBox
onready var _year := $Year as SpinBox
onready var _authors := $Authors/List as Tree
onready var _authors_btns := $Authors/Box as Control
onready var _authors_remove := $Authors/Box/Remove as Button
onready var _category := $Category as OptionButton
onready var _notes := $Notes as TextEdit
onready var _borrowable := $Borrowable as CheckBox

var _borrower := ""
var _deadline := ""
var _reservation := ""
var _id_before_edit := ""

func _ready():
    set_editable(editable)
    for node in get_tree().get_nodes_in_group("CategoryChanger"):
        assert(node.connect("categories_changed", self, "_on_categories_changed") == OK)


func _on_categories_changed(categories: Array):
    var ctrl := _category as OptionButton
    ctrl.clear()
    for category in categories:
        var text: String = category.id + " - " + category.name + " - " + category.section
        ctrl.add_item(text,  category.id.hash())


func set_medium(m: Reference):
    if m != null:
        _id.text = m.id
        _isbn.text = m.isbn
        _title.text = m.title
        _publisher.text = m.publisher
        _price.value = m.costs
        _year.value = m.year
        _authors.clear()
        var root := _authors.create_item() as TreeItem
        for author in m.authors:
            var item := _authors.create_item(root) as TreeItem
            item.set_text(0, author)
        _category.select(_category.get_item_index(m.category.hash()))
        _notes.text = m.note
        _borrowable.pressed = m.borrowable
        _borrower = m.borrower
        _deadline = m.deadline
        _reservation = m.reservation
    else:
        _id.clear()
        _isbn.clear()
        _title.clear()
        _publisher.clear()
        _price.value = 0
        _year.value = 2000
        _authors.clear()
        _category.select(0)
        _notes.text = ""
        _borrowable.pressed = true
        _borrower = ""
        _deadline = ""
        _reservation = ""


func get_medium() -> Reference:
    var medium := Medium.new()
    medium.id = _id.text
    medium.isbn = _isbn.text
    medium.title = _title.text
    medium.publisher = _publisher.text
    medium.costs = _price.value
    medium.year = _year.value as int
    var authors := []
    if _authors.get_root():
        var child := _authors.get_root().get_children() as TreeItem
        while child:
            authors.push_back(child.get_text(0))
            child = child.get_next()
    medium.authors = PoolStringArray(authors)
    if _category.selected >= 0:
        var text: String = _category.get_item_text(_category.selected)
        medium.category = text.split(" - ", true, 1)[0]
    medium.note = _notes.text
    medium.borrowable = _borrowable.pressed
    medium.borrower = _borrower
    medium.deadline = _deadline
    medium.reservation = _reservation
    return medium


func set_editable(e: bool):
    if e: _id_before_edit = _id.text
    else: _id_before_edit = ""

    editable = e
    _id.editable = e
    _id_btn.visible = e
    _isbn.editable = e
    _title.editable = e
    _publisher.editable = e
    _price.editable = e
    _year.editable = e
    _category.disabled = not e
    _notes.readonly = not e
    _borrowable.disabled = not e
    _authors_btns.visible = e
    _authors_remove.disabled = true
    if _authors.get_root():
        var child := _authors.get_root().get_children() as TreeItem
        while child:
            child.set_editable(0, e)
            child = child.get_next()


func _on_generate_id():
    if editable:
        var medium = get_medium()
        medium.id = _id_before_edit
        var result = _project.medium_generate_id(medium)
        if result.has("Ok"):
            _id.text = result["Ok"]
        else:
            MessageDialog.error_code(result["Err"])


func _on_author_add():
    var item = _authors.create_item(_authors.get_root())
    item.set_text(0, tr(".medium.authors.def"))
    item.set_editable(0, true)


func _on_author_remove():
    var selected = _authors.get_selected()
    if selected:
        selected.deselect(0)
        _authors.get_root().remove_child(selected)


func _on_author_selected():
     _authors_remove.disabled = _authors.get_selected() == null
