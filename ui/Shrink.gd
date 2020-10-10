extends Control

onready var width = rect_size.x

var shrinking := false

func _ready() -> void:
    var error := get_parent().connect("resized", self, "_resized")
    assert(error == OK)
    _resized()

func _resized() -> void:
    var parent: Control = get_parent()
    if not shrinking and parent.rect_size.x <= width:
        shrinking = true
        anchor_left = 0.0
        anchor_right = 1.0
        rect_size.x = parent.rect_size.x
        rect_position.x = parent.rect_position.x
    elif shrinking and parent.rect_size.x >= width:
        shrinking = false
        anchor_left = 0.5
        anchor_right = 0.5
        rect_size.x = width
        rect_position.x = (parent.rect_size.x - width) / 2
