extends MenuButton

signal theme_changed(dark)

export var theme_dark: Theme
export var theme_light: Theme

var _dark: bool = true

func _ready():
    var menu := get_popup()
    menu.add_check_item(tr(".pref.appearance.dark"))
    menu.set_item_checked(0, _dark)

    assert(menu.connect("index_pressed", self, "_on_index_pressed") == OK)


func _on_index_pressed(index: int):
    match index:
        0: _set_theme(not get_popup().is_item_checked(0))


func _set_theme(dark: bool):
    _dark = dark
    if get_popup().get_item_count() > 0:
        get_popup().set_item_checked(0, dark)
    if dark: emit_signal("theme_changed", theme_dark)
    else: emit_signal("theme_changed", theme_light)


func persist_save() -> Dictionary:
    return { "dark_theme": get_popup().is_item_checked(0) }


func persist_load(data: Dictionary):
    _set_theme(data.get("dark_theme", true))
