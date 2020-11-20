extends VBoxContainer
class_name MailView

export var subject := "" setget set_subject, get_subject
export var content := "" setget set_content, get_content

export var default_subject: String
export var default_content: String

onready var _subject: LineEdit = $Subject
onready var _content: TextEdit = $Content


func set_subject(val: String):
    _subject.text = val if val else tr(default_subject)


func get_subject() -> String:
    return _subject.text


func set_content(val: String):
    _content.text = val if val else tr(default_content)


func get_content() -> String:
    return _content.text
