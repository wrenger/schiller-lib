extends MarginContainer

onready var text_view = $Text


func _on_show_overdues() -> void:
    text_view.text = tr(".mail.overdue2.content")
