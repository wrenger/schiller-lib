tool
extends TextureRect


export var theme_class: String = "EditorIcons"
export var frame_names: PoolStringArray = PoolStringArray([
    "Progress1",
    "Progress2",
    "Progress3",
    "Progress4",
    "Progress5",
    "Progress6",
    "Progress7",
    "Progress8",
])
export var frame_time: float = 0.3

var _timer := 0.0
var _frame := 0

func _ready() -> void:
    texture = get_icon(frame_names[_frame], theme_class)

func _process(delta: float) -> void:
    if is_visible_in_tree():
        _timer += delta
        if _timer > frame_time:
            _timer -= frame_time
            texture = get_icon(frame_names[_frame], theme_class)
            _frame = (_frame + 1) % len(frame_names)
