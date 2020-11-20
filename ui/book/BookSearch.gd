extends Control

signal search_results(results)

onready var _tabs: TabContainer = $Tabs
onready var _search: LineEdit = $Tabs/Basic

# advanced book search
onready var _id: LineEdit = $Tabs/Advanced/Grid/ID
onready var _isbn: LineEdit = $Tabs/Advanced/Grid/ISBN
onready var _title: LineEdit = $Tabs/Advanced/Grid/Title
onready var _publisher: LineEdit = $Tabs/Advanced/Grid/Publisher
onready var _authors: LineEdit = $Tabs/Advanced/Grid/Authors
onready var _year: LineEdit = $Tabs/Advanced/Grid/Year
onready var _category: CategorySelect = $Tabs/Advanced/Grid/Category
onready var _note: LineEdit = $Tabs/Advanced/Grid/Note
onready var _user: LineEdit = $Tabs/Advanced/Grid/User
onready var _state: OptionButton = $Tabs/Advanced/Grid/State


func _on_search(_t = null):
    var result: Dictionary = Project.book_search(_search.text)
    if result.has("Ok"):
        print("search results: ", len(result["Ok"]))
        emit_signal("search_results", result["Ok"])
    else:
        MessageDialog.error_code(result["Err"])


func _on_advanced_search(_t = null):
    var result: Dictionary = Project.book_search_advanced({
        id = _id.text,
        isbn = _isbn.text,
        title = _title.text,
        publisher = _publisher.text,
        authors = _authors.text,
        year = _year.text,
        category = _category.get_selected_category_id(),
        note = _note.text,
        user = _user.text,
        state = _state.get_selected_id(),
    })

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


func show_user_books(account: String):
    _tabs.current_tab = 1
    clear_advanced()
    _user.text = account
    _on_advanced_search()


func show_book(id: String):
    _tabs.current_tab = 1
    clear_advanced()
    _id.text = id
    _on_advanced_search()
