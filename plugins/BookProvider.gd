extends Reference

# Private implementation
var _pimpl := BookDNBProvider.new()

# Adopt properties of private implementation
func _get_property_list() -> Array:
    return _pimpl.get_property_list()
func _get(property: String):
    return _pimpl.get(property)
func _set(property: String, value) -> bool:
    _pimpl.set(property, value)
    return true


# Public exported methods
func request(isbn: String) -> Dictionary:
    return _pimpl.request(isbn)
