extends TextEdit

export var content: String

func _ready():
    text = tr(content)
