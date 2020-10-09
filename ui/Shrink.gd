extends ScrollContainer

onready var width = rect_min_size.x + 120

func _ready() -> void:
    assert(get_parent().connect("resized", self, "_resized") == OK)

func _resized() -> void:
    var parent: Control = get_parent()
    print("_resized: ", parent.rect_size, rect_min_size)
    if rect_min_size.x > 0 and parent.rect_size.x <= width:
        print("enable shrink")
        rect_min_size.x = 0
        size_flags_horizontal |= SIZE_FILL
    elif rect_min_size.x == 0 and parent.rect_size.x >= width:
        print("disable shrink")
        rect_min_size.x = width - 120
        size_flags_horizontal &= ~SIZE_FILL
