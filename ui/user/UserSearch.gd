extends LineEdit

signal search_results(results)


func _on_search(_t = null):
    var result: Dictionary = Project.user_search(text)
    if result.has("Ok"):
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])
