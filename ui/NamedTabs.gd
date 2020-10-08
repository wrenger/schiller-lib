extends TabContainer

export (Array, Texture) var icons: Array = []
export (Array, String) var labels: Array = []


func _ready():
    for i in range(len(icons)):
        set_tab_title(i, tr(labels[i]))
        if icons: set_tab_icon(i, icons[i])
