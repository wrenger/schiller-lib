extends WindowDialog
class_name MailDialog


static func info(user: Dictionary, booktitle: String):
    var scene: SceneTree = Engine.get_main_loop()
    var nodes = scene.get_nodes_in_group("MailDialog")
    if nodes: nodes.front()._info(user, booktitle)


func _ready() -> void:
    for node in get_tree().get_nodes_in_group("ProjectChanger"):
        var result = node.connect("project_changed", self, "_project_changed")
        assert(result == OK)


func _info(user: Dictionary, booktitle: String):
    popup_centered()
    yield(get_tree().create_timer(0.1), "timeout")

    var result: Dictionary = Project.settings_get()
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
    if result.has("Ok"):
        result = yield(mailer, "done")

    hide()
    if result.has("Ok"):
        MessageDialog.alert(tr(".alert.mail.send.success"))
    else:
        MessageDialog.error(tr(".alert.mail.send.error") + "\n\n" +  Util.error_msg(result["Err"]))


func _project_changed():
    var result: Dictionary = Project.settings_get()
    if result.has("Ok"):
        var settings: Dictionary = result["Ok"]
        var today = Date.new()
        prints("overdues", settings.mail_last_reminder, today.get_iso())
        if settings.mail_last_reminder != today.get_iso():
            result = Project.lending_overdues()
            if not result.has("Ok"): return MessageDialog.error_code(result["Err"])

            if result["Ok"]:
                var confirmed = yield(ConfirmDialog.open(tr(".alert.mail.overdue")), "response")
                if confirmed: _overdues(result["Ok"])
            else:
                settings.mail_last_reminder = Date.new().get_iso()
                result = Project.settings_update(settings)


func _overdues(overdues: Array):
    assert(overdues != null and len(overdues) > 0)
    popup_centered()
    yield(get_tree().create_timer(0.1), "timeout")

    var result: Dictionary = Project.settings_get()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var settings: Dictionary = result["Ok"]

    var mailer = Mailer.new()
    mailer.host = settings.mail_host
    mailer.password = settings.mail_password
    mailer.from = settings.mail_from

    var errors := PoolStringArray([])
    for period in overdues:
        var book: Dictionary = period[0]
        var user: Dictionary = period[1]
        var username: String = user.forename + " " + user.surname

        if not visible:
            errors.append(user.account + " (" + book.id + ") - " + tr(".alert.mail.send.cancel"))
            continue

        var date := Date.new()
        result = date.set_iso(book.deadline)
        if result.has("Err"):
            errors.append(user.account + " (" + book.id + ") - " + Util.error_msg(result["Err"]))
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
        if result.has("Ok"):
            result = yield(mailer, "done")
        if result.has("Err"):
            errors.append(user.account + " (" + book.id + ") - " + Util.error_msg(result["Err"]))

    hide()
    if not errors:
        settings.mail_last_reminder = Date.new().get_iso()
        result = Project.settings_update(settings)
        if result.has("Ok"):
            MessageDialog.alert(tr(".alert.mail.send.success"))
        else:
            MessageDialog.error_code(result["Err"])
    else:
        MessageDialog.error(tr(".alert.mail.send.error.to") + "\n\n" + errors.join("\n"))
