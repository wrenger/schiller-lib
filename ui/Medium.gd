extends GridContainer

export var editable := false setget set_editable

var medium: Object = null setget set_medium, get_medium

var _borrower := ""
var _deadline := ""
var _reservation := ""

func _ready():
    set_editable(editable)


func set_medium(m: Object):
    if m != null:
        $ID.text = m.id
        $ISBN.text = m.isbn
        $Title.text = m.title
        $Publisher.text = m.publisher
        $Price.value = m.costs
        $Year.value = m.year
        $Authors.clear()
        var root := $Authors.create_item() as TreeItem
        for author in m.authors:
            var item := $Authors.create_item(root) as TreeItem
            item.set_text(0, author)
        $Category.clear()
        $Category.add_item(m.category)
        $Notes.text = m.note
        $Borrowable.pressed = m.borrowable
    else:
        $ID.clear()
        $ISBN.clear()
        $Title.clear()
        $Publisher.clear()
        $Price.value = 0
        $Year.value = 2000
        $Authors.clear()
        $Category.clear()
        $Notes.text = ""
        $Borrowable.pressed = true
        _borrower = ""
        _deadline = ""
        _reservation = ""


func get_medium() -> Object:
    var medium := Medium.new()
    medium.id = $ID.text
    medium.isbn = $ISBN.text
    medium.title = $Title.text
    medium.publisher = $Publisher.text
    medium.costs = $Price.value
    medium.year = $Year.value as int
    if $Authors.get_root():
        var child := $Authors.get_root().get_children() as TreeItem
        while child:
            medium.authors.append(child.get_text(0))
            child = child.get_next()
    medium.category = $Category.get_item_text($Category.selected)
    medium.note = $Notes.text
    medium.borrowable = $Borrowable.pressed
    medium.borrower = _borrower
    medium.deadline = _deadline
    medium.reservation = _reservation
    return medium


func set_editable(e: bool):
    editable = e
    $ID.editable = e
    $ISBN.editable = e
    $Title.editable = e
    $Publisher.editable = e
    $Price.editable = e
    $Year.editable = e
    $Category.disabled = not e
    $Notes.readonly = not e
    $Borrowable.disabled = not e
    if $Authors.get_root():
        var child := $Authors.get_root().get_children() as TreeItem
        while child:
            child.set_editable(0, e)
            child = child.get_next()
