extends OptionButton


export var allow_empty: bool


func _ready():
    for node in get_tree().get_nodes_in_group("CategoryChanger"):
        assert(node.connect("categories_changed", self, "_on_categories_changed") == OK)


func _on_categories_changed(categories: Array):
    clear()
    if allow_empty:
        add_item(tr(".category"))
    for category in categories:
        var text: String = category.id + " - " + category.name + " - " + category.section
        add_item(text,  category.id.hash())
