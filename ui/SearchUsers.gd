extends MarginContainer

onready var project := $"/root/Project" as Project

export var user_list : NodePath

onready var _user_list := get_node(user_list) as Tree


func _on_search(text: String) -> void:
    var result = project.search_user_basic(text)
    if result.has("Ok"):
        _user_list.fill(result["Ok"])
    else:
        MessageDialog.alert(get_tree(), "Search Error: " + result["Err"])


func _on_user_selected(user) -> void:
    pass # Replace with function body.
