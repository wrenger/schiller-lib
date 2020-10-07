extends Control

signal search_results(results)

onready var _project: Project = $"/root/Project"

onready var _tabs: TabContainer = $Tabs
onready var _search: LineEdit = $Tabs/Basic

onready var _id: LineEdit = $Tabs/Advanced/Grid/ID
onready var _isbn: LineEdit = $Tabs/Advanced/Grid/ISBN
onready var _title: LineEdit = $Tabs/Advanced/Grid/Title
onready var _publisher: LineEdit = $Tabs/Advanced/Grid/Publisher
onready var _authors: LineEdit = $Tabs/Advanced/Grid/Authors
onready var _year: LineEdit = $Tabs/Advanced/Grid/Year
onready var _category: OptionButton = $Tabs/Advanced/Grid/Category
onready var _note: LineEdit = $Tabs/Advanced/Grid/Note
onready var _user: LineEdit = $Tabs/Advanced/Grid/User
onready var _state: OptionButton = $Tabs/Advanced/Grid/State


func _on_search(_t = null):
    var result: Dictionary = _project.medium_search(_search.text)
    if result.has("Ok"):
        print("search results: ", len(result["Ok"]))
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_advanced_search(_t = null):
    print("Advanced search")
    var id := _id.text
    var isbn := _isbn.text
    var title := _title.text
    var publisher: String = _publisher.text
    var authors: String = _authors.text
    var year: String = _year.text
    var category: String = ""
    if _category.selected >= 1:
        var text: String = _category.get_item_text(_category.selected)
        category = text.split(" - ", true, 1)[0]
    var note: String = _note.text
    var user: String = _user.text
    var state: int = _state.get_selected_id()

    var result: Dictionary = _project.medium_search_advanced(
            id, isbn, title, publisher, authors,
            year, category, note, user, state)

    if result.has("Ok"):
        print("search results: ", len(result["Ok"]))
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func clear_advanced():
    _id.text = ""
    _isbn.text = ""
    _title.text = ""
    _publisher.text = ""
    _authors.text = ""
    _year.text = ""
    _category.select(0)
    _note.text = ""
    _user.text = ""
    _state.select(0)


func search_user(account: String):
    _tabs.current_tab = 1
    clear_advanced()
    _user.text = account
    _on_advanced_search()
