extends TextEdit

export var content: String

func _ready():
    text = tr(content)
    add_color_override("font_color_readonly", get_color("font_color"))
