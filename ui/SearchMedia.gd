extends MarginContainer

onready var _project := get_node("/root/Project") as Project

onready var _media_list := $Split/Search/MediaList as Tree
onready var _medium_box := $Split/Panel/Box as Control


func _on_basic_search(new_text):
    var result: Dictionary = _project.medium_search(new_text)

    if result.has("Ok"):
        _media_list.fill(result["Ok"])
    else:
        MessageDialog.alert(get_tree(), tr(Util.error_msg(result["Err"])))
