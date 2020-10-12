extends MarginContainer

signal show_book(id)

func _on_show_book(id):
    emit_signal("show_book", id)
