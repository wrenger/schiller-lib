extends Tree

signal object_selected(object)

export var column_names: PoolStringArray
export var column_sizes: PoolIntArray
export var column_expand: PoolByteArray

var results := {}


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
    self.results.clear()
    clear()
    var root := create_item()
    for object in rows:
        var fields = object.list_item()
        assert(len(fields) == columns)

        self.results[fields[0]] = object

        var item := create_item(root)
        for i in range(columns):
            item.set_text(i, fields[i])


func update_selected(object):
    var item := get_selected()
    if object:
        var fields = object.list_item()
        self.results.erase(fields[0])
        self.results[fields[0]] = object
        assert(len(fields) == columns)
        for i in range(columns):
            item.set_text(i, fields[i])
    else:
        get_root().remove_child(item)
        update()


func _on_item_selected():
    var selected := get_selected()
    var object = results[selected.get_text(0)]
    emit_signal("object_selected", object)

