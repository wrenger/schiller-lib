extends MarginContainer

signal show_user_media(user)


func _on_show_media(user) -> void:
    emit_signal("show_user_media", user.account)
