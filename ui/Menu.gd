extends MenuButton

signal menu_new()
signal menu_open()
signal menu_close()
signal theme_changed(dark)

export var btn_icon := ""
export var new_icon := ""
export var open_icon := ""
export var close_icon := ""
export var theme_icon := ""
export var settings_icon := ""

export var theme_dark: Theme
export var theme_light: Theme

var _dark: bool = true

enum Items {
    NEW = 0,
    OPEN = 1,
    CLOSE = 3,
    THEME = 5,
    SETTINGS = 7
}

func _ready() -> void:
    var menu := get_popup()
    menu.add_item(tr(".action.new"))
    menu.add_shortcut(Util.create_shortcut(tr(".action.open"), KEY_O))
    menu.add_separator()
    menu.add_item(tr(".action.close"))

    menu.add_separator()
    menu.add_check_item(tr(".pref.appearance.dark"))
    menu.set_item_checked(Items.THEME, _dark)
    menu.add_separator()
    menu.add_shortcut(Util.create_shortcut(tr(".pref.title"), KEY_COMMA))

    var error := menu.connect("index_pressed", self, "_on_index_pressed")
    assert(error == OK)

    _on_theme_changed_()


func _on_index_pressed(index: int):
    match index:
        Items.NEW: emit_signal("menu_new")
        Items.OPEN: emit_signal("menu_open")
        Items.CLOSE: emit_signal("menu_close")
        Items.THEME: _set_theme(not get_popup().is_item_checked(Items.THEME))
        Items.SETTINGS: SettingsDialog.open()


func _set_theme(dark: bool):
    _dark = dark
    if get_popup().get_item_count() > Items.THEME:
        get_popup().set_item_checked(Items.THEME, dark)
    if dark: emit_signal("theme_changed", theme_dark)
    else: emit_signal("theme_changed", theme_light)
    _on_theme_changed_()


func _on_theme_changed_(_x = null):
    if btn_icon: icon = get_icon(btn_icon, "EditorIcons")
    if get_popup().get_item_count() > Items.SETTINGS:
        if new_icon:
            get_popup().set_item_icon(Items.NEW, get_icon(new_icon, "EditorIcons"))
        if open_icon:
            get_popup().set_item_icon(Items.OPEN, get_icon(open_icon, "EditorIcons"))
        if close_icon:
            get_popup().set_item_icon(Items.CLOSE, get_icon(close_icon, "EditorIcons"))
        if theme_icon:
            get_popup().set_item_icon(Items.THEME, get_icon(theme_icon, "EditorIcons"))
        if settings_icon:
            get_popup().set_item_icon(Items.SETTINGS, get_icon(settings_icon, "EditorIcons"))


func persist_save() -> Dictionary:
    return { "dark_theme": get_popup().is_item_checked(Items.THEME) }


func persist_load(data: Dictionary):
    _set_theme(data.get("dark_theme", true))
