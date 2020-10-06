extends Control

signal search_results(results)

onready var _project: Project = $"/root/Project"


func _on_search(text: String):
    var result: Dictionary = _project.user_search(text)
    if result.has("Ok"):
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])
