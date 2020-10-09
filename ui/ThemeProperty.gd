extends Control

export var theme_class: String = ""
export var theme_name: String = ""

enum ThemeType { COLOR, FONT, ICON }
export(ThemeType) var theme_type: int = ThemeType.COLOR

export var target_property: String = ""

func _ready() -> void:
    _on_theme_changed(null)
    for node in get_tree().get_nodes_in_group("ThemeChanger"):
        node.connect("theme_changed", self, "_on_theme_changed")


func _on_theme_changed(x) -> void:
    print("theme changed for ", get_path())
    match theme_type:
        ThemeType.COLOR:
            set(target_property, get_color(theme_name, theme_class))
            print(theme_class, "/", theme_name, ": ", get_color(theme_name, theme_class))
        ThemeType.FONT:
            set(target_property, get_font(theme_name, theme_class))
            print(theme_class, "/", theme_name, ": ", get_font(theme_name, theme_class))
        ThemeType.ICON: set(target_property, get_icon(theme_name, theme_class))
