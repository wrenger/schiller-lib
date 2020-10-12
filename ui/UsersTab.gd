extends MarginContainer

signal show_user_books(user)


func _on_show_books(user) -> void:
    emit_signal("show_user_books", user.account)
