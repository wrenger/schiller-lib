extends ConfirmationDialog
class_name SettingsDialog

onready var _project: Project = $"/root/Project"
onready var _window_content := $"../Content" as Control

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


var _is_only_dialog := false


static func open():
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("SettingsDialog")
    nodes.front()._open()


func _open():
    if not visible: popup_centered()


func _ready() -> void:
    var result := OK
    result = connect("about_to_show", self, "_about_to_show")
    assert(result == OK)
    result = connect("popup_hide", self, "_popup_hide")
    assert(result == OK)
    result = connect("confirmed", self, "save")
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


func _default_if_empty():
    if not _mail_info_subject.text: _mail_info_subject.text = tr(".mail.info.subject")
    if not _mail_info_content.text: _mail_info_content.text = tr(".mail.info.content")
    if not _mail_overdue_subject.text: _mail_overdue_subject.text = tr(".mail.overdue.subject")
    if not _mail_overdue_content.text: _mail_overdue_content.text = tr(".mail.overdue.content")
    if not _mail_overdue2_subject.text: _mail_overdue2_subject.text = tr(".mail.overdue2.subject")
    if not _mail_overdue2_content.text: _mail_overdue2_content.text = tr(".mail.overdue2.content")


func save() -> void:
    if _settings.empty(): return
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


func _popup_hide():
    if _is_only_dialog: _window_content.modulate.a = 1


func _about_to_show():
    _is_only_dialog = _window_content.modulate.a >= 1
    _window_content.modulate.a = 0.5
    reload()

