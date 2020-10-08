extends MarginContainer

onready var _project: Project = $"/root/Project"

onready var _settings_panel: Control = $Box
onready var _borrowing_duration: SpinBox = $Box/Scroll/Box/Borrowing/Margin/Duration/Value
onready var _user_delimiter: LineEdit = $Box/Scroll/Box/User/Margin/Box/Delimiter/Value
onready var _user_path: Button = $Box/Scroll/Box/User/Margin/Box/Box/File
onready var _request_token: LineEdit = $Box/Scroll/Box/Request/Margin/Token/Value
onready var _mail_host: LineEdit = $Box/Scroll/Box/MailAccount/Margin/Box/Host/Value
onready var _mail_from: LineEdit = $Box/Scroll/Box/MailAccount/Margin/Box/From/Value
onready var _mail_password: LineEdit = $Box/Scroll/Box/MailAccount/Margin/Box/Password/Value

onready var _mail_info_subject: LineEdit = $Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Info/Subject
onready var _mail_info_content: TextEdit = $Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Info/Content
onready var _mail_overdue_subject: LineEdit = $Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue/Subject
onready var _mail_overdue_content: TextEdit = $Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue/Content
onready var _mail_overdue2_subject: LineEdit = $Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue2/Subject
onready var _mail_overdue2_content: TextEdit = $Box/Scroll/Box/MailTemplates/Margin/Box/TabContainer/Overdue2/Content


func reload():
    var result: Dictionary = _project.settings_get()
    var settings: Reference
    if result.has("Ok"):
        settings = result["Ok"]
        _settings_panel.visible = true
        _borrowing_duration.value = settings.borrowing_duration
        _user_delimiter.text = settings.user_delimiter
        _user_path.hint_tooltip = settings.user_path
        _request_token.text = settings.dnb_token
        _mail_host.text = settings.mail_host
        _mail_from.text = settings.mail_from
        _mail_password.text = settings.mail_password
        _mail_info_subject.text = settings.mail_info_subject
        _mail_info_content.text = settings.mail_info_content
        _mail_overdue_subject.text = settings.mail_overdue_subject
        _mail_overdue_content.text = settings.mail_overdue_content
        _mail_overdue2_subject.text = settings.mail_overdue2_subject
        _mail_overdue2_content.text = settings.mail_overdue2_content
        _default_if_empty()
    else:
        _settings_panel.visible = false


func _default_if_empty():
    if not _mail_info_subject.text: _mail_info_subject.text = tr(".mail.info.subject")
    if not _mail_info_content.text: _mail_info_content.text = tr(".mail.info.content")
    if not _mail_overdue_subject.text: _mail_overdue_subject.text = tr(".mail.overdue.subject")
    if not _mail_overdue_content.text: _mail_overdue_content.text = tr(".mail.overdue.content")
    if not _mail_overdue2_subject.text: _mail_overdue2_subject.text = tr(".mail.overdue2.subject")
    if not _mail_overdue2_content.text: _mail_overdue2_content.text = tr(".mail.overdue2.content")


func _on_save() -> void:
    print("TODO: Save")
    pass # Replace with function body.
