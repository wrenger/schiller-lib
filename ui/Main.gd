extends Panel

func _ready():
    var project := get_node("/root/Project") as Project
    if not project.open("/home/lars/Documents/projects/sbv-gd/test/demo.db"):
        print("ERROR: opening project")


func _unhandled_key_input(event):
    if event is InputEventKey:
        var key_event := event as InputEventKey
        if key_event.scancode == KEY_P and key_event.control and key_event.pressed:
            MessageDialog.alert(get_tree(), "ctrl+P pressed")
