extends Reference
class_name BookDNBProvider

# DNB Requests
#
# See Also
# https://www.dnb.de/EN/Professionell/Metadatendienste/Datenbezug/SRU/sru_node.html

var token := ""

const HOST := "services.dnb.de"
const URL := "/sru/accessToken~{token}/dnb?version=1.1&operation=searchRetrieve&recordSchema=MARC21-xml&query=NUM%3D{isbn}"
const TIMEOUT := 3000 # ms

func request(isbn: String) -> Dictionary:
    prints("request", token, isbn)

    var result := request_get(isbn)
    if result.has("Err"): return result;

    var parser = Marc21.new()
    return parser.parse(isbn, result["Ok"])


func request_get(isbn: String) -> Dictionary:
    var start = OS.get_ticks_msec()
    var client := HTTPClient.new()

    var error := OK
    error = client.connect_to_host(HOST, 443, true)
    if error != OK: return {"Err": Util.SbvError.Network}

    # wait until connected
    while client.get_status() == HTTPClient.STATUS_CONNECTING or client.get_status() == HTTPClient.STATUS_RESOLVING:
        error = client.poll()
        if error != OK or OS.get_ticks_msec() - start > TIMEOUT:
            return {"Err": Util.SbvError.Network}
        OS.delay_msec(100)

    var url := URL.format({"token": token, "isbn": isbn})

    error = client.request(HTTPClient.METHOD_GET, url, [])
    if error != OK: return {"Err": Util.SbvError.Network}

    while client.get_status() == HTTPClient.STATUS_REQUESTING:
        error = client.poll()
        if error != OK or OS.get_ticks_msec() - start > TIMEOUT:
            return {"Err": Util.SbvError.Network}
        OS.delay_msec(100)


    # Make sure request finished well.
    if client.get_response_code() != HTTPClient.RESPONSE_OK or \
        (client.get_status() != HTTPClient.STATUS_BODY and client.get_status() != HTTPClient.STATUS_CONNECTED) or \
        not client.has_response():
        return {"Err": Util.SbvError.Network}

    var buffer = PoolByteArray()

    while client.get_status() == HTTPClient.STATUS_BODY:
        # While there is body left to be read
        error = client.poll()
        if error != OK or OS.get_ticks_msec() - start > TIMEOUT:
            return {"Err": Util.SbvError.Network}

        # Get a chunk.
        var chunk = client.read_response_body_chunk()
        if chunk.size() == 0:
            # Got nothing, wait for buffers to fill a bit.
            OS.delay_usec(1000)
        else:
            buffer = buffer + chunk

    var text = buffer.get_string_from_utf8()
    return {"Ok": text}
