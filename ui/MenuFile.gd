extends MenuButton

signal menu_open()
signal menu_close()

func _ready():
    var menu := get_popup()
    menu.add_shortcut(Util.create_shortcut("Open", KEY_O))
    menu.add_item("Close")

    assert(menu.connect("index_pressed", self, "_on_index_pressed") == OK)


func _on_index_pressed(index: int) -> void:
    match index:
        0: emit_signal("menu_open")
        1: emit_signal("menu_close")

