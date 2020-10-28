extends Button

onready var _project: Project = $"/root/Project"


func _pressed():
    var result: Dictionary = _project.settings_get()
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var settings: Dictionary = result["Ok"]

    # TODO: flexible provider selection & configuration
    var provider = UserCSVProvider.new()
    provider.path = settings.user_path
    if len(settings.user_delimiter) != 1:
        return MessageDialog.error(Util.trf(".error.provider.config", [tr(".pref.user.delimiter")]))
    provider.delimiter = settings.user_delimiter

    # collect all user accounts
    result = _project.user_search("")
    if result.has("Err"): return MessageDialog.error_code(result["Err"])
    var user_accounts := []
    for user in result["Ok"]:
        user_accounts.push_back(user.account)

    # fetch new roles for all users
    result = provider.bulk_request(user_accounts)
    if result.has("Err"):
        return MessageDialog.error("Request failed: error code: " + String(result["Err"]))
    var user_roles := []
    for user in result["Ok"]:
        user_roles.push_back([user.account, user.role])

    # update user roles
    result = _project.user_update_roles(user_roles)
    if result.has("Err"): MessageDialog.error_code(result["Err"])
    else: MessageDialog.alert(tr(".alert.user.updated"))
