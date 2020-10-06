extends MarginContainer

onready var text_view = $Text


func _ready():
    if visible: _on_visibility_changed()


func _on_visibility_changed():
    if visible:
        text_view.text = tr(".mail.overdue2.content")
