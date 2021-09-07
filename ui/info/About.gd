extends VBoxContainer


func _ready() -> void:
    var about = Project.about()

    $Name.text = Util.trf(".info.about", [about.name, about.version])
    $Description.text = about.description
    $Repo/Url.text = about.repository
    $Box/License.text = Util.trf(".info.about.license", [about.license])
    $Devs.text = about.authors.join("\n")


func _on_repo_pressed() -> void:
    var result := OS.shell_open($Repo/Url.text)
    assert(result == OK)


func _on_license_godot_pressed() -> void:
    var result := OS.shell_open($Box/Godot.hint_tooltip)
    assert(result == OK)
