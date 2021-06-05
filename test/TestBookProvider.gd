extends Test

func test_simple():
    var dnb_provider := BookDNBProvider.new()
    dnb_provider.token = OS.get_environment("SBV_DNB_TOKEN")

    var result := dnb_provider.request("9783570303337")
    print(result)
    assert_eq(result,
        { "Ok": {
            "title": "Eragon - Das Vermächtnis der Drachenreiter",
            "authors": ["Christopher Paolini"],
            "publisher": "cbj",
            "costs": 9.95,
        }}
    )
