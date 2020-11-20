extends Tree
class_name AuthorList

export var authors: Array setget set_authors, get_authors
export var editable := false setget set_editable

onready var _authors_btns := $"../Box" as Control
onready var _authors_remove := $"../Box/Remove" as Button


func set_authors(value: Array):
    clear()
    if value:
        var root := create_item()
        for author in value:
            var item := create_item(root)
            item.set_text(0, author)


func get_authors() -> Array:
    var value := []
    if get_root():
        var child := get_root().get_children()
        while child:
            value.push_back(child.get_text(0))
            child = child.get_next()
    return value


func set_editable(value: bool):
    editable = value
    _authors_btns.visible = value
    _authors_remove.disabled = true
    if get_root():
        var child := get_root().get_children()
        while child:
            child.set_editable(0, editable)
            child = child.get_next()


func _on_author_add():
    var item = create_item(get_root())
    item.set_text(0, tr(".book.authors.def"))
    item.set_editable(0, true)


func _on_author_remove():
    var selected = get_selected()
    if selected:
        selected.deselect(0)
        get_root().remove_child(selected)


func _on_author_selected():
     _authors_remove.disabled = get_selected() == null
