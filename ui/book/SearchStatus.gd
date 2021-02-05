extends Label

func _ready() -> void:
    for node in get_tree().get_nodes_in_group("ProjectChanger"):
        node.connect("project_changed", self, "_on_search_results", [[]])

func _on_search_results(results) -> void:
    text = Util.trf(".search.results", [len(results)])
