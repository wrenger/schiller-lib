extends Button

func _pressed() -> void:
    print(name, " _on_pressed")
    CategoryDialog.open()
