extends Reference

# Private implementation
var _pimpl := UserCSVProvider.new()

# Adopt properties of private implementation
func _get_property_list() -> Array:
    return _pimpl.get_property_list()
func _get(property: String):
    return _pimpl.get(property)
func _set(property: String, value) -> bool:
    _pimpl.set(property, value)
    return true


# Public exported methods
func request(account: String) -> Dictionary:
    return _pimpl.request(account)
func bulk_request(accounts: PoolStringArray) -> Dictionary:
    return _pimpl.bulk_request(accounts)
