extends Label

func _ready() -> void:
    text = Util.trf(".info.title", [Project.version()])
