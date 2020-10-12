extends Control

onready var _project: Project = $"/root/Project"

onready var _borrowing_duration: SpinBox = $Scroll/Box/Borrowing/Margin/Duration/Value
onready var _user_delimiter: LineEdit = $Scroll/Box/User/Margin/Box/Delimiter/Value
onready var _user_path: Button = $Scroll/Box/User/Margin/Box/Box/File
onready var _request_token: LineEdit = $Scroll/Box/Request/Margin/Token/Value

onready var _mail_host: LineEdit = $Scroll/Box/MailAccount/Margin/Box/Host/Value
onready var _mail_from: LineEdit = $Scroll/Box/MailAccount/Margin/Box/From/Value
onready var _mail_password: LineEdit = $Scroll/Box/MailAccount/Margin/Box/Password/Value

onready var _mail_info_subject: LineEdit = $Scroll/Box/MailTemplates/Margin/Box/TabContainer/Info/Subject
onready var _mail_info_content: TextEdit = $Scroll/Box/MailTemplates/Margin/Box/TabContainer/Info/Content
onready var _mail_overdue_subject: LineEdit = $Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue/Subject
onready var _mail_overdue_content: TextEdit = $Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue/Content
onready var _mail_overdue2_subject: LineEdit = $Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue2/Subject
onready var _mail_overdue2_content: TextEdit = $Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue2/Content


var _settings: Dictionary = {}


func _ready() -> void:
    # Reload if the project changes and the tab focused
    for node in get_tree().get_nodes_in_group("ProjectChanger"):
        var result: int = node.connect("project_changed", self, "reload")
        assert(result == OK)
    var result := get_parent().connect("visibility_changed", self, "_on_visibility_changed")
    assert(result == OK)


func reload():
    if not get_parent().is_visible_in_tree(): return

    var result: Dictionary = _project.settings_get()
    if result.has("Ok"):
        _settings = result["Ok"]
        visible = true
        _borrowing_duration.value = _settings.borrowing_duration
        _user_delimiter.text = _settings.user_delimiter
        _user_path.hint_tooltip = _settings.user_path
        _request_token.text = _settings.dnb_token
        _mail_host.text = _settings.mail_host
        _mail_from.text = _settings.mail_from
        _mail_password.text = _settings.mail_password
        _mail_info_subject.text = _settings.mail_info_subject
        _mail_info_content.text = _settings.mail_info_content
        _mail_overdue_subject.text = _settings.mail_overdue_subject
        _mail_overdue_content.text = _settings.mail_overdue_content
        _mail_overdue2_subject.text = _settings.mail_overdue2_subject
        _mail_overdue2_content.text = _settings.mail_overdue2_content
        _default_if_empty()
    else:
        _settings = {}
        visible = false


func _on_visibility_changed():
    if get_parent().is_visible_in_tree(): reload()


func _default_if_empty():
    if not _mail_info_subject.text: _mail_info_subject.text = tr(".mail.info.subject")
    if not _mail_info_content.text: _mail_info_content.text = tr(".mail.info.content")
    if not _mail_overdue_subject.text: _mail_overdue_subject.text = tr(".mail.overdue.subject")
    if not _mail_overdue_content.text: _mail_overdue_content.text = tr(".mail.overdue.content")
    if not _mail_overdue2_subject.text: _mail_overdue2_subject.text = tr(".mail.overdue2.subject")
    if not _mail_overdue2_content.text: _mail_overdue2_content.text = tr(".mail.overdue2.content")


func _on_save() -> void:
    if _settings.empty():
        return
    var settings := get_settings()
    var result: Dictionary = _project.settings_update(settings)
    if result.has("Err"):
        MessageDialog.error_code(result["Err"])


func get_settings() -> Dictionary:
    return {
        borrowing_duration = int(_borrowing_duration.value),
        user_delimiter = _user_delimiter.text,
        user_path = _user_path.hint_tooltip,
        dnb_token = _request_token.text,
        mail_last_reminder = _settings.mail_last_reminder,
        mail_host = _mail_host.text,
        mail_from = _mail_from.text,
        mail_password = _mail_password.text,
        mail_info_subject = _mail_info_subject.text,
        mail_info_content = _mail_info_content.text,
        mail_overdue_subject = _mail_overdue_subject.text,
        mail_overdue_content = _mail_overdue_content.text,
        mail_overdue2_subject = _mail_overdue2_subject.text,
        mail_overdue2_content = _mail_overdue2_content.text,
    }
