extends MenuButton

signal menu_new()
signal menu_open()
signal menu_close()

func _ready():
    var menu := get_popup()
    menu.add_item(tr(".action.new"))
    menu.add_shortcut(Util.create_shortcut(tr(".action.open"), KEY_O))
    menu.add_separator()
    menu.add_item(tr(".action.close"))

    var error := menu.connect("index_pressed", self, "_on_index_pressed")
    assert(error == OK)


func _on_index_pressed(index: int):
    print(name, ":_on_index_pressed ", index)
    match index:
        0: emit_signal("menu_new")
        1: emit_signal("menu_open")
        2: pass # separator
        3: emit_signal("menu_close")
