extends TabContainer

signal show_overdues()

export (Array, Texture) var icons: Array = []
export (Array, String) var labels: Array = []


func _ready():
    for i in range(len(icons)):
        set_tab_title(i, tr(labels[i]))
        set_tab_icon(i, icons[i])


func _on_tab_changed(tab: int) -> void:
    match tab:
        3: emit_signal("show_overdues")
