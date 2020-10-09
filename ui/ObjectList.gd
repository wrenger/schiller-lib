extends Tree
class_name ObjectList

signal object_selected(object)

export var column_names: PoolStringArray
export var column_sizes: PoolIntArray
export var column_expand: PoolByteArray

enum Formatter { MEDIUM, USER }
export(Formatter) var formatter: int


func _ready():
    assert(columns == len(column_names))
    assert(columns == len(column_sizes))
    assert(columns == len(column_expand))

    set_column_titles_visible(true)

    for i in range(columns):
        set_column_title(i, tr(column_names[i]))
        set_column_min_width(i, column_sizes[i])
        set_column_expand(i, column_expand[i] != 0)


func fill(rows: Array):
    clear()
    var _root := create_item()
    for object in rows:
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
        assert(len(fields) == columns)
        for i in range(columns):
            item.set_text(i, fields[i])
    else:
        get_root().remove_child(item)
        update()


func add_object(object: Dictionary) -> TreeItem:
    var fields = format(object)
    assert(len(fields) == columns)

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
    match formatter:
        Formatter.MEDIUM: return format_medium(object)
        Formatter.USER: return format_user(object)
    return PoolStringArray([])


func format_medium(object: Dictionary) -> PoolStringArray:
    var state := ""
    if object.reservation:
        state = tr(".medium.reserved")
    elif object.borrower:
        state = tr(".medium.borrowed")
    return PoolStringArray([
        object.id,
        object.title,
        PoolStringArray(object.authors).join(", "),
        state,
    ])


func format_user(object: Dictionary) -> PoolStringArray:
    return PoolStringArray([
        object.account,
        object.forename,
        object.surname,
        object.role,
    ])
