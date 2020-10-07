extends Test

func test_trf():
    var start := OS.get_ticks_usec()

    assert_eq("", Util.trf(""))
    assert_eq("", Util.trf("", [1, 2]))
    assert_eq("{0}", Util.trf("{0}"))
    assert_eq("1 2 {9}", Util.trf("{0} {1} {9}", [1, 2]))
    assert_eq("1 2 10 11", Util.trf("{0} {1} {9} {10}", [1, 2, 3, 4, 5, 6, 7, 8, 9, "10", 11]))
    assert_eq("Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            Util.trf("Lorem ipsum dolor sit amet, consectetur adipiscing elit."))
    assert_eq("Lorem {} dolor sit amet, {consectetur} adipiscing elit.",
            Util.trf("Lorem {} dolor sit amet, {consectetur} adipiscing elit."))
    assert_eq("Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            Util.trf("Lorem {0} dolor sit amet, consectetur adipiscing elit.", ["ipsum"]))
    assert_eq("Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            Util.trf("Lorem {1} dolor sit amet, {0} adipiscing elit.", ["consectetur", "ipsum"]))
    assert_eq("Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            Util.trf("Lorem {2} dolor sit amet, consectetur {1} elit.", [1, "adipiscing", "ipsum"]))
    assert_eq("1.1 Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            Util.trf("{0} Lorem {2} dolor sit amet, consectetur {1} elit.", [1.1, "adipiscing", "ipsum"]))
    assert_eq("{100}", Util.trf("{100}", [0]))

    assert_eq("1 Tag", Util.trf("{0} Tag{0:|e}", [1]))
    assert_eq("Tag{1:|e} 1 Tag", Util.trf("Tag{1:|e} {0} Tag{0:|e}", [1]))
    assert_eq("10 Tage 1 Buch", Util.trf("{0} Tag{0:|e} {1} {1:Buch|Bücher}", [10, 1]))
    assert_eq("10 Tage 2 Bücher", Util.trf("{0} Tag{0:|e} {1} {1:Buch|Bücher}", [10, 2]))
    assert_eq("0 Bücher null Tage", Util.trf("{1} {1:Buch|Bücher} {0} Tag{0:|e}", ["null", 0]))

    print("Test script time: ", OS.get_ticks_usec() - start)
