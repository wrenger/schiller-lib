extends Control

signal search_results(results)

onready var _project := $"/root/Project" as Project


func _on_search(text: String):
    var result: Dictionary = _project.user_search(text)
    if result.has("Ok"):
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))

