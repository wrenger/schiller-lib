extends ConfirmationDialog
class_name SettingsDialog

onready var _borrowing_duration: SpinBox = $Scroll/Box/Borrowing/Margin/Duration/Value
onready var _user_delimiter: LineEdit = $Scroll/Box/User/Margin/Box/Delimiter/Value
onready var _user_path: Button = $Scroll/Box/User/Margin/Box/Box/File
onready var _request_token: LineEdit = $Scroll/Box/Request/Margin/Token/Value

onready var _mail_host: LineEdit = $Scroll/Box/MailAccount/Margin/Box/Host/Value
onready var _mail_from: LineEdit = $Scroll/Box/MailAccount/Margin/Box/From/Value
onready var _mail_password: LineEdit = $Scroll/Box/MailAccount/Margin/Box/Password/Value

onready var _mail_info: MailView = $Scroll/Box/MailTemplates/Margin/Box/Tabs/Info
onready var _mail_overdue: MailView = $Scroll/Box/MailTemplates/Margin/Box/Tabs/Overdue
onready var _mail_overdue2: MailView = $Scroll/Box/MailTemplates/Margin/Box/Tabs/Overdue2

var _settings: Dictionary = {}


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


func reload():
    if not get_parent().is_visible_in_tree(): return

    var result: Dictionary = Project.settings_get()
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
        _mail_info.subject = _settings.mail_info_subject
        _mail_info.content = _settings.mail_info_content
        _mail_overdue.subject = _settings.mail_overdue_subject
        _mail_overdue.content = _settings.mail_overdue_content
        _mail_overdue2.subject = _settings.mail_overdue2_subject
        _mail_overdue2.content = _settings.mail_overdue2_content
    else:
        _settings = {}
        visible = false


func save() -> void:
    if _settings.empty(): return
    var settings := get_settings()
    var result: Dictionary = Project.settings_update(settings)
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
        mail_info_subject = _mail_info.subject,
        mail_info_content = _mail_info.content,
        mail_overdue_subject = _mail_overdue.subject,
        mail_overdue_content = _mail_overdue.content,
        mail_overdue2_subject = _mail_overdue2.subject,
        mail_overdue2_content = _mail_overdue2.content,
    }


func _about_to_show():
    reload()

