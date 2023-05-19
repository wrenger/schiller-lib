extends Reference
class_name UserCSVProvider

var path := ""
var delimiter := ","
var has_headers := false
var column_account := 0
var column_forename := 1
var column_surname := 2
var column_role := 3


func request(account: String) -> Dictionary:
    account = account.strip_edges().to_lower()

    var max_col: int = [column_account, column_forename, column_surname, column_role].max()

    var similar := []

    var file := File.new()
    if file.open(path, File.READ) == OK:
        while not file.eof_reached():
            var line := file.get_csv_line(delimiter)
            if len(line) > max_col:
                var account_n: String = line[column_account].strip_edges().to_lower()
                var rank := account_n.similarity(account)

                if rank > 0.7:
                    var user := {
                        "account": account_n,
                        "forename": line[column_forename].strip_edges(),
                        "surname": line[column_surname].strip_edges(),
                        "role": line[column_role].strip_edges(),
                    }

                    if rank >= 1.0:
                        file.close()
                        return {"Ok": user}

                    similar.append({"rank": rank, "user": user})
        file.close()

        var best = null
        for user in similar:
            if best == null or best["rank"] < user["rank"]:
                best = user
        if best != null:
            return {"Ok": best["user"]}

        return {"Err": Util.SbvError.NothingFound}
    else:
        return {"Err": Util.SbvError.FileOpen}


func bulk_request(accounts: PoolStringArray) -> Dictionary:
    var file := File.new()

    var max_col := max(column_account, max(column_forename, max(column_surname, column_role)))

    var account_set := {}
    for account in accounts:
        account_set[account] = true

    if file.open(path, File.READ) == OK:
        var results = []
        while not file.eof_reached():
            var line := file.get_csv_line(delimiter)
            if len(line) > max_col:
                var account := line[column_account].strip_edges()
                if account_set.has(account):
                     results.append({
                        "account": account,
                        "forename": line[column_forename].strip_edges(),
                        "surname": line[column_surname].strip_edges(),
                        "role": line[column_role].strip_edges(),
                    })
        file.close()
        return {"Ok": results}
    else:
        return {"Err": Util.SbvError.FileOpen}
