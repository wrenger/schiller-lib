extends WindowDialog
class_name MailDialog

onready var _window_content: Control = $"../Content"
onready var _project: Project = $"/root/Project"

var _is_only_dialog := false


static func info(user: Dictionary, booktitle: String):
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("MailDialog")
    if nodes: nodes.front()._info(user, booktitle)


static func overdues():
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("MailDialog")
    if nodes: nodes.front()._overdues()


func _ready() -> void:
    var result := OK
    result = connect("popup_hide", self, "_popup_hide")
    assert(result == OK)
    result = connect("about_to_show", self, "_about_to_show")
    assert(result == OK)
    for node in get_tree().get_nodes_in_group("ProjectChanger"):
        result = node.connect("project_changed", self, "_project_changed")
        assert(result == OK)


func _project_changed():
    var result: Dictionary = _project.settings_get()
    if result.has("Ok"):
        var settings: Dictionary = result["Ok"]
        var today = Date.new()
        prints("overdues", settings.mail_last_reminder, today.get_iso())
        if settings.mail_last_reminder != today.get_iso():
            var confirmed = yield(ConfirmDialog.open(tr(".alert.mail.overdue")), "response")
            if confirmed: _overdues()


func _info(user: Dictionary, booktitle: String):
    popup_centered()
    yield(get_tree().create_timer(0.1), "timeout")

    var result: Dictionary = _project.settings_get()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var settings: Dictionary = result["Ok"]

    var mailer = Mailer.new()
    mailer.host = settings.mail_host
    mailer.password = settings.mail_password
    mailer.from = settings.mail_from

    var subject: String = settings.mail_info_subject
    var body: String = settings.mail_info_content
    var username: String = user.forename + " " + user.surname
    subject = subject.replace("{username}", username).replace("{booktitle}", booktitle)
    body = body.replace("{username}", username).replace("{booktitle}", booktitle)

    result = mailer.send(user.account, subject, body)

    hide()
    if result.has("Ok"):
        MessageDialog.alert(tr(".alert.mail.send.success"))
    else:
        MessageDialog.error(tr(".alert.mail.send.error") + "\n\n" + Util.error_msg(result["Err"]))


func _overdues():
    popup_centered()
    yield(get_tree().create_timer(0.1), "timeout")

    var result: Dictionary = _project.settings_get()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var settings: Dictionary = result["Ok"]

    var mailer = Mailer.new()
    mailer.host = settings.mail_host
    mailer.password = settings.mail_password
    mailer.from = settings.mail_from

    var errors := PoolStringArray([])

    result = _project.lending_overdues()
    if result.has("Ok"):
        for period in result["Ok"]:
            var book: Dictionary = period[0]
            var user: Dictionary = period[1]
            var username: String = user.forename + " " + user.surname
            var date := Date.new()
            result = date.set_iso(book.deadline)
            if result.has("Err"):
                errors.append(user.account + " - " + Util.error_msg(result["Err"]))
                continue

            var subject := ""
            var body := ""
            if date.days_until_today() < 14:
                subject = settings.mail_overdue_subject
                body = settings.mail_overdue_content
            else:
                subject = settings.mail_overdue2_subject
                body = settings.mail_overdue2_content
            subject = subject.replace("{username}", username).replace("{booktitle}", book.title)
            body = body.replace("{username}", username).replace("{booktitle}", book.title)

            result = mailer.send(user.account, subject, body)
            if result.has("Err"):
                errors.append(user.account + " - " + Util.error_msg(result["Err"]))

            yield(get_tree().create_timer(0.2), "timeout")

    hide()
    if not errors:
        settings.mail_last_reminder = Date.new().get_iso()
        result = _project.settings_update(settings)
        if result.has("Ok"):
            MessageDialog.alert(tr(".alert.mail.send.success"))
        else:
            MessageDialog.error_code(result["Err"])
    else:
        MessageDialog.error(tr(".alert.mail.send.error.to") + "\n\n" + errors.join("\n"))


func _popup_hide():
    if _is_only_dialog: _window_content.modulate.a = 1


func _about_to_show():
    _is_only_dialog = _window_content.modulate.a >= 1
    _window_content.modulate.a = 0.5