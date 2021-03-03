extends Test

func test_single():
    var csv_provider = UserCSVProvider.new()
    csv_provider.path = "test/data/csv/users.csv"

    assert_eq(
        csv_provider.request("callen.lawson"),
        {"Ok": {"account": "callen.lawson", "forename": "Callen", "surname": "Lawson", "role": "Person"}}
    );

    assert_eq(
        csv_provider.request("charlotte.penn"),
        {"Ok": {"account": "charlotte.penn", "forename": "Charlotte", "surname": "Penn", "role": "Person"}}
    );
    assert_eq(
        csv_provider.request("safah.scott"),
        {"Ok": {"account": "safah.scott", "forename": "Safah", "surname": "Scott", "role": "Person"}}
    );


func test_multiple():
    var csv_provider = UserCSVProvider.new()
    csv_provider.path = "test/data/csv/users.csv"

    var result = csv_provider.bulk_request(PoolStringArray(["charlotte.penn", "callen.lawson", "safah.scott"]));
    assert_eq(
        result,
        {"Ok":[
            {"account": "callen.lawson", "forename": "Callen", "surname": "Lawson", "role": "Person"},
            {"account": "charlotte.penn", "forename": "Charlotte", "surname": "Penn", "role": "Person"},
            {"account": "safah.scott", "forename": "Safah", "surname": "Scott", "role": "Person"},
        ]}
    )
