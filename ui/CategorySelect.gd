extends OptionButton
class_name CategorySelect

export var allow_empty: bool


func _ready():
    for node in get_tree().get_nodes_in_group("CategoryChanger"):
        var error: int = node.connect("categories_changed", self, "_on_categories_changed")
        assert(error == OK)


func _on_categories_changed(categories: Array):
    clear()
    if allow_empty:
        add_item(tr(".category"))
    for category in categories:
        var text: String = category.id + " - " + category.name + " - " + category.section
        add_item(text,  category.id.hash())


func select_category(id: String):
    select(get_item_index(id.hash()))


func get_selected_category_id() -> String:
    if selected >= 0:
        var text: String = get_item_text(selected)
        return text.split(" - ", true, 1)[0]
    return ""
