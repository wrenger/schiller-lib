extends GridContainer

export var editable := false setget set_editable

onready var project := get_node("/root/Project") as Project

var medium: Reference = null setget set_medium, get_medium

var _borrower := ""
var _deadline := ""
var _reservation := ""
var _id_before_edit := ""

func _ready():
    set_editable(editable)
    for node in get_tree().get_nodes_in_group("CategoryChanger"):
        assert(node.connect("categories_changed", self, "_on_categories_changed") == OK)


func _on_categories_changed(categories: Array):
    var ctrl := $Category as OptionButton
    ctrl.clear()
    for category in categories:
        var text: String = category.id + " - " + category.name + " - " + category.section
        ctrl.add_item(text,  category.id.hash())


func set_medium(m: Reference):
    if m != null:
        $ID/Input.text = m.id
        $ISBN.text = m.isbn
        $Title.text = m.title
        $Publisher.text = m.publisher
        $Price.value = m.costs
        $Year.value = m.year
        $Authors/List.clear()
        var root := $Authors/List.create_item() as TreeItem
        for author in m.authors:
            var item := $Authors/List.create_item(root) as TreeItem
            item.set_text(0, author)
        $Category.select($Category.get_item_index(m.category.hash()))
        $Notes.text = m.note
        $Borrowable.pressed = m.borrowable
    else:
        $ID/Input.clear()
        $ISBN.clear()
        $Title.clear()
        $Publisher.clear()
        $Price.value = 0
        $Year.value = 2000
        $Authors/List.clear()
        $Category.select(0)
        $Notes.text = ""
        $Borrowable.pressed = true
        _borrower = ""
        _deadline = ""
        _reservation = ""


func get_medium() -> Reference:
    var medium := Medium.new()
    medium.id = $ID/Input.text
    medium.isbn = $ISBN.text
    medium.title = $Title.text
    medium.publisher = $Publisher.text
    medium.costs = $Price.value
    medium.year = $Year.value as int
    var authors := []
    if $Authors/List.get_root():
        var child := $Authors/List.get_root().get_children() as TreeItem
        while child:
            authors.push_back(child.get_text(0))
            child = child.get_next()
    medium.authors = PoolStringArray(authors)
    if $Category.selected >= 0:
        var text: String = $Category.get_item_text($Category.selected)
        medium.category = text.split(" - ", true, 1)[0]
    medium.note = $Notes.text
    medium.borrowable = $Borrowable.pressed
    medium.borrower = _borrower
    medium.deadline = _deadline
    medium.reservation = _reservation
    return medium


func set_editable(e: bool):
    if e: _id_before_edit = $ID/Input.text
    else: _id_before_edit = ""

    editable = e
    $ID/Input.editable = e
    $ID/GenerateID.visible = e
    $ISBN.editable = e
    $Title.editable = e
    $Publisher.editable = e
    $Price.editable = e
    $Year.editable = e
    $Category.disabled = not e
    $Notes.readonly = not e
    $Borrowable.disabled = not e
    $Authors/Box.visible = e
    $Authors/Box/Remove.disabled = true
    if $Authors/List.get_root():
        var child := $Authors/List.get_root().get_children() as TreeItem
        while child:
            child.set_editable(0, e)
            child = child.get_next()


func _on_generate_id() -> void:
    if editable:
        var medium = get_medium()
        medium.id = _id_before_edit
        var result = project.medium_generate_id(medium)
        if result.has("Ok"):
            $ID/Input.text = result["Ok"]
        else:
            MessageDialog.error(get_tree(), tr(Util.error_msg(result["Err"])))


func _on_author_add() -> void:
    var item = $Authors/List.create_item($Authors/List.get_root())
    item.set_text(0, tr(".medium.authors.def"))
    item.set_editable(0, true)


func _on_author_remove() -> void:
    print("_on_author_remove")
    var selected = $Authors/List.get_selected()
    if selected:
        selected.deselect(0)
        $Authors/List.get_root().remove_child(selected)


func _on_author_selected() -> void:
     $Authors/Box/Remove.disabled = $Authors/List.get_selected() == null

