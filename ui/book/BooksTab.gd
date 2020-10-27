extends MarginContainer


func show_user_books(account: String):
    get_parent().current_tab = get_index()
    $Split/Left/Search.show_user_books(account)


func show_book(id: String):
    get_parent().current_tab = get_index()
    $Split/Left/Search.show_book(id)
