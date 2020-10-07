extends MarginContainer


func show_user_media(account: String):
    get_parent().current_tab = get_index()
    $Split/Left/Search.search_user(account)
