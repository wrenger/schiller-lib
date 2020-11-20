extends TabContainer
class_name NamedTabs

export (Array, String) var editor_icons: Array = []
export (Array, String) var labels: Array = []


func _ready() -> void:
    for i in range(len(labels)):
        set_tab_title(i, tr(labels[i]))

    _on_theme_changed_()
    for node in get_tree().get_nodes_in_group("ThemeChanger"):
        node.connect("theme_changed", self, "_on_theme_changed_")


func _on_theme_changed_(_x = null):
    if editor_icons:
        for i in range(len(labels)):
            set_tab_icon(i, get_icon(editor_icons[i], "EditorIcons"))
