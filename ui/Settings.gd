extends Control

onready var _project: Project = $"/root/Project"

onready var _settings_panel: Control = $Inner/Box
onready var _borrowing_duration: SpinBox = $Inner/Box/Scroll/Box/Borrowing/Margin/Duration/Value
onready var _user_delimiter: LineEdit = $Inner/Box/Scroll/Box/User/Margin/Box/Delimiter/Value
onready var _user_path: Button = $Inner/Box/Scroll/Box/User/Margin/Box/Box/File
onready var _request_token: LineEdit = $Inner/Box/Scroll/Box/Request/Margin/Token/Value

onready var _mail_host: LineEdit = $Inner/Box/Scroll/Box/MailAccount/Margin/Box/Host/Value
onready var _mail_from: LineEdit = $Inner/Box/Scroll/Box/MailAccount/Margin/Box/From/Value
onready var _mail_password: LineEdit = $Inner/Box/Scroll/Box/MailAccount/Margin/Box/Password/Value

onready var _mail_info_subject: LineEdit = $Inner/Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Info/Subject
onready var _mail_info_content: TextEdit = $Inner/Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Info/Content
onready var _mail_overdue_subject: LineEdit = $Inner/Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue/Subject
onready var _mail_overdue_content: TextEdit = $Inner/Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue/Content
onready var _mail_overdue2_subject: LineEdit = $Inner/Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue2/Subject
onready var _mail_overdue2_content: TextEdit = $Inner/Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue2/Content


var _settings: Dictionary = {}


func reload():
    var result: Dictionary = _project.settings_get()
    if result.has("Ok"):
        _settings = result["Ok"]
        _settings_panel.visible = true
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
        _settings_panel.visible = false


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
    var settings := {
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
    var result: Dictionary = _project.settings_update(settings)
    if result.has("Err"):
        MessageDialog.error_code(result["Err"])
