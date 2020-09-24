extends Tree


signal user_selected(user)

var results := {}


func _ready():
    set_column_titles_visible(true)
    set_column_title(0, tr(".user.account"))
    set_column_min_width(0, 150)
    set_column_title(1, tr(".user.forename"))
    set_column_min_width(1, 100)
    set_column_title(2, tr(".user.surname"))
    set_column_min_width(2, 100)
    set_column_title(3, tr(".user.role"))
    set_column_min_width(3, 100)


func fill(rows: Array):
    self.results.clear()
    clear()
    var root := create_item()
    for user in rows:
        self.results[user.account] = user

        var item := create_item(root)
        item.set_text(0, user.account)
        item.set_text(1, user.forename)
        item.set_text(2, user.surname)
        item.set_text(3, user.role)


func _on_item_selected():
    var selected := get_selected()
    var medium = results[selected.get_text(0)]
    emit_signal("medium_selected", medium)
