extends OptionButton
class_name CategorySelect

export var allow_empty: bool


func _ready():
    for node in get_tree().get_nodes_in_group("CategoryChanger"):
        var error = node.connect("categories_changed", self, "_on_categories_changed")
        assert(error == OK)


func _on_categories_changed(categories: Array):
    clear()
    if allow_empty:
        add_item(tr(".category"))
        set_item_metadata(get_item_count() - 1, "")
        add_separator()
    for category in categories:
        var text: String = category.id + " - " + category.name + " - " + category.section
        add_item(text,  category.id.hash())
        set_item_metadata(get_item_count() - 1, category.id)


func select_category(id: String):
    select(get_item_index(id.hash()))


func get_selected_category_id() -> String:
    if selected >= 0: return get_selected_metadata()
    return ""
