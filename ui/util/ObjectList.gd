extends Tree
class_name ObjectList

signal object_selected(object)

# Column names that are translated on runtime
export var column_names: PoolStringArray
# Relative column sizes
export var column_sizes: PoolIntArray
# If the column should be resizable
export var column_expand: PoolByteArray

# Selectors: keys of the rows dictionary
# If a selector has the format '#<name>' the 'format_<name>' method is called instead.
# Also for sorting the '<name>' function on the RowSorter is used.
export var column_selectors: PoolStringArray

var _column_sorted_asc := -1

func _ready():
    assert(columns == len(column_names))
    assert(columns == len(column_sizes))
    assert(columns == len(column_expand))
    assert(columns == len(column_selectors))

    set_column_titles_visible(true)

    for i in range(columns):
        set_column_title(i, tr(column_names[i]))
        set_column_min_width(i, column_sizes[i])
        set_column_expand(i, column_expand[i] != 0)

    var result := OK
    for node in get_tree().get_nodes_in_group("ProjectChanger"):
        result = node.connect("project_changed", self, "fill")
        assert(result == OK)
    result = connect("item_selected", self, "_on_item_selected")
    assert(result == OK)
    result = connect("column_title_pressed", self, "sort")
    assert(result == OK)


func fill(rows: Array = []):
    clear()
    var _root := create_item()
    for object in rows:
        # warning-ignore:return_value_discarded
        add_object(object)
    if rows:
        get_root().get_children().select(0)
    else:
        _on_item_selected()


func update_selected(object: Dictionary):
    var item := get_selected()
    if object:
        item.set_meta("object", object)
        var fields = format(object)
        for i in range(columns):
            item.set_text(i, fields[i])
    else:
        get_root().remove_child(item)
        update()


func add_object(object: Dictionary) -> TreeItem:
    var fields = format(object)
    var item := create_item(get_root())
    item.set_meta("object", object)
    for i in range(columns):
        item.set_text(i, fields[i])
    return item


func add_and_select_object(object: Dictionary):
    var item := add_object(object)
    item.select(0)


func _on_item_selected():
    var selected := get_selected()
    if selected:
        emit_signal("object_selected", selected.get_meta("object"))
    else:
        emit_signal("object_selected", {})


func format(object: Dictionary) -> PoolStringArray:
    var result = PoolStringArray([])
    for selector in column_selectors:
        result.append(format_value(object, selector))
    return result


func format_value(object: Dictionary, selector: String) -> String:
    if selector.begins_with("#"):
        return call("format_" + selector.substr(1), object)
    else:
        return object[selector]


# format method for the '#book_authors' selector
func format_book_authors(object: Dictionary) -> String:
    return PoolStringArray(object.authors).join(", ")


# format method for the '#book_state' selector
func format_book_state(object: Dictionary) -> String:
    var state := ""
    if object.reservation:
        state = tr(".book.reserved")
    elif object.borrower:
        state = tr(".book.borrowed")
    return state


# Sort by column
func sort(column: int):
    prints("column", column, "selected")
    if get_root():
        var rows := []
        var child := get_root().get_children()
        while child:
            rows.push_back(child.get_meta("object"))
            child = child.get_next()

        var selector := column_selectors[column]
        var sorter := "default"
        if selector.begins_with("#"):
            sorter = selector.substr(1)
        rows.sort_custom(RowSorter.new(selector), sorter)

        # Reverse if clicked a second time
        if column == _column_sorted_asc:
            rows.invert()
            _column_sorted_asc = -1
        else: _column_sorted_asc = column

        fill(rows)


# Sorter class for sorting rows by a selector value
class RowSorter:
    var _selector: String

    func _init(selector: String) -> void:
        self._selector = selector

    func default(a: Dictionary, b: Dictionary) -> bool:
        return a[_selector].nocasecmp_to(b[_selector]) < 0

    # sort method for the '#book_authors' selector
    func book_authors(a: Dictionary, b: Dictionary) -> bool:
        var a_name := ""
        if a.authors: a_name = a.authors[0]
        var b_name := ""
        if b.authors: b_name = b.authors[0]

        # Sort by surnames
        var idx = a_name.find_last(" ")
        if idx >= 0: a_name = a_name.substr(idx)
        idx = b_name.find_last(" ")
        if idx >= 0: b_name = b_name.substr(idx)

        return a_name.nocasecmp_to(b_name) < 0

    # sort method for the '#book_state' selector
    func book_state(a: Dictionary, b: Dictionary) -> bool:
        return a.borrower < b.borrower or (a.borrower == b.borrower and a.reservation < b.reservation)
