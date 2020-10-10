extends Node

var savepath := "user://ui.json"

func _enter_tree():
    var file := File.new()
    if file.open(savepath, File.READ) == OK:
        var result := JSON.parse(file.get_as_text())
        file.close()

        if result.error:
            print("Error parsing ui settings: ", result.error_string)
        else:
            var settings := result.result as Dictionary
            if settings:
                for key in settings.keys():
                    var node := get_node(key)
                    if node and node.is_in_group("Persist"):
                        node.persist_load(settings[key])
                    else:
                        print("Error missing node: ", key)


func _exit_tree():
    var settings := {}
    var save_nodes := get_tree().get_nodes_in_group("Persist")
    for node in save_nodes:
        settings[String(node.get_path())] = node.persist_save()

    var file := File.new()
    var error := file.open(savepath, File.WRITE)
    assert(error == OK)
    file.store_string(JSON.print(settings, "  "))
    file.close()
