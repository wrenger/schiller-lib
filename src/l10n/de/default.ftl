action_add = Hinzufügen
action_all = Alle
action_apply = Übernehmen
action_cancel = Abbrechen
action_close = Schließen
action_copy = Kopieren
action_copy_long = In Zwischenablage Kopieren
action_cut = Ausschneiden
action_delete = Löschen
action_edit = Bearbeiten
action_empty = Leer
action_finish = Fertigstellen
action_load = Lädt
action_logout = Abmelden
action_new = Neu
action_next = Weiter
action_no = Nein
action_ok = OK
action_open = Öffnen
action_paste = Einfügen
action_previous = Zurück
action_redo = Wiederholen
action_select = Auswählen
action_selectAll = Alles auswählen
action_undo = Rückgängig machen
action_yes = Ja
alert_confirm = Bestätigung
alert_create_project = Neue Projekt-Datenbank erstellen
alert_error = Fehler
alert_info = Info
alert_mail_overdue = Sollen die Erinnerungen für die { $arg0 } überzogenen Bücher jetzt versendet werden?
alert_mail_send_cancel = Abgebrochen
alert_mail_send_content = Versende Emails
alert_mail_send_error = Fehler beim senden der Emails.
alert_mail_send_error_to = Fehler beim senden der Emails an:
alert_mail_send_success = Senden der Mahnungen erfolgreich.
alert_mail_send_title = Emailversand
alert_open_project = Projekt-Datenbank auswählen
alert_overdue = Benachrichtige Überziehungen
alert_overdue_none = Momentan gibt es keine Überziehungen.
alert_update = Willkommen zu Version { $arg0 }.
    Die Datenbank wurde erfolgreich aktualisiert.
alert_user_updated = Gruppen erfolgreich aktualisiert
alert_userfile = Accountdatei auswählen
book = Buch
book_authors = Autoren
book_authors_def = Vorname Nachname, Anderer Autor
book_available = Verfügbar
book_available_long = Nicht Ausgeliehen oder Vorgemerkt
book_borrowable = Ausleihbar
book_borrowed = Ausgeliehen
book_overdues = Überziehungen
book_borrowed_by = Ausgeliehen von { $arg0 } bis zum { $arg1 }
book_borrowed_by_short = Ausgeliehen von { $arg0 }
book_costs = Preis
book_delete_reservation = Vormerkung Löschen
book_delete = Möchtest du das Buch "{ $arg0 }" wirklich löschen?
book_from_user = Von { $arg0 } ({ $arg1 })
book_id = Signatur
book_id_action = Vorschlagen
book_id_format = Die Signatur muss aus der Kategorie, dem Autor und einer ID bestehen.
book_invalid_isbn = Die angegebene ISBN is fehlerhaft
book_invalid = Titel, Signatur und Kategorie dürfen nicht leer bleiben.
book_isbn = ISBN
book_lend = Ausleihen
book_lend_period = Dauer
book_lend_to = Ausleihen an { $arg0 }
book_lend_until = Ausgeliehen bis zum { $arg0 }
book_new = Neues Buch
book_not_borrowable = Nicht ausleihbar
book_note = Notizen
book_period = { $arg0 } ({ $arg1 ->
        [one] { $arg1 } Tag
       *[other] { $arg1 } Tage
    })
book_period_days = Tage
book_period_date = Datum
book_publisher = Verlag
book_renew = Verlängern
book_request = Buchinfos laden
book_reserve = Vormerken
book_reserved = Vorgemerkt
book_reserved_by = Vorgemerkt von { $arg0 }
book_revoke = Zurückgeben
book_revoke_reminder = Das Buch wurde von "{ $arg0 }" vorgemerkt.
    
    Möchtest du die Person per Email über die Rückgabe informieren?
book_state = Status
book_title = Titel
book_unavailable = Ausgeliehen oder Vorgemerkt
book_year = Anschaffungsjahr
category = Kategorie
category_edit = Kategorien bearbeiten
category_empty_input = Leere Eingaben sind nicht erlaubt.
category_id = Abkürzung
category_name = Name
category_not_empty_del = Es können nur leere Kategorien gelöscht werden. Diese Kategorie ist allerdings nicht leer.
category_section = Bereich
category_none = Unkategorisiert
category_t1_id = SCFI
category_t1_name = Science-Fiction
category_t1_section = Romane
category_t2_id = THRL
category_t2_name = Thriller
category_t2_section = Romane
error_book_existing = Es gibt bereits ein anderes Buch mit der Signatur { $arg0 }.
error_book_none = Kein Buch gefunden.
error_category_existing = Die Abkürzung '{ $arg0 }' existiert bereits.
error_config_default = Die Voreinstellungen konnten nicht geladen werden.
error_date = Das Datum konnte nicht richtig geladen werden.
    Aktualiseren Sie diese in den Einstellungen unter dem Punkt "Letztes Mal Erinnert"!
error_db = Die Datenbank konnte nicht geöffnet werden.
error_file_open = Die Datei konnte nicht geöffnet werden.
error_format = Invalides Dateiformat
error_input = Fehlerhafte Account- oder Buchdaten.
error_lending_already_borrowed_by = Die Person leiht das Buch bereits aus.
error_lending_already_borrowed = Das Buch ist bereits ausgeliehen.
error_lending_already_reserved = Das Buch wurde bereits reserviert.
error_lending_not_reserved = Das Buch wurde nicht reserviert.
error_lending_book = Das Buch darf nicht ausgeliehen werden.
error_lending_not_borrowed = Das Buch kann nicht reserviert werden, da es noch nicht ausgeliehen wurde.
error_lending_user = Die Person darf keine Bücher ausleihen.
error_network = Verbindung zum Server fehlgeschlagen
error_no_project = Es wurde noch kein Projekt geöffnet.
error_none = Keine Ergebnisse gefunden
error_provider_none = Kein { $arg0 } Plugin installiert
error_provider_config = Invalider Wert für { $arg0 }
error_user_existing = Der Account '{ $arg0 }' existiert bereits.
error_user_none = Keine Person gefunden.
error_version = Die aktuelle Projektversion wird nicht unterstützt.
    Bitte kontaktieren Sie einen Entwickler.
error_unknown = Ein unbekannter Fehler ist aufgetreten.
    Versuchen sie die Seite neuzuladen oder sich nochmal ab- und wieder anzumelden!
info_category_freq = Anzahl der Bücher in den Kategorien:
    
    { $arg0 }
info_about = Über { $arg0 } - Version { $arg1 }
info_about_repo = Repository:
info_about_license = Lizenz: { $arg0 }
info_about_devs = Entwickler:
info_about_name = Name: { $arg0 }
info_books = { $arg0 } { $arg0 ->
        [one] Buch
       *[other] Bücher
    }
info_users = { $arg0 } Accounts
info_categories = { $arg0 } Kategorie{ $arg0 ->
        [one] { "" }
       *[other] n
    }
info_borrows = { $arg0 } Ausleihung{ $arg0 ->
        [one] { "" }
       *[other] en
    }
info_reservations = { $arg0 } Vormerkung{ $arg0 ->
        [one] { "" }
       *[other] en
    }
info_overdues = { $arg0 } Überziehung{ $arg0 ->
        [one] { "" }
       *[other] en
    }
info_costs = Gesamtpreis aller Medien: { $arg0 }
    (Summe aller eingetragenen Preise)
info_stats = Statistiken
info_session = Session
info_session_id = ID: { $arg0 }
info_session_username = Benutzername: { $arg0 }
lang_title = Sprache
lang_en = Englisch
lang_de = Deutsch
mail_info = { "Sowohl im Titel und auch im Inhalt können Schlüsselwörter verwendet werden. Diese werden beim Senden der Email durch den entsprechenden Werte ersetzt.\u000A\u000A{username}: Benutzername, {booktitle}: Buchtitel" }
mail_info_content = { "Hallo {username},\u000ADu hast dich für das Buch '{booktitle}' interessiert.\u000ADas Buch war ausgeliehen und liegt nun wieder in der Schülerbücherei.\u000AWenn du willst, kannst du es jetzt ausleihen.\u000A\u000AViele Grüße vom Schülerbüchereiteam." }
mail_info_subject = { "'{booktitle}' ist nun verfügbar!" }
mail_info_title = Info
mail_label_content = Inhalt
mail_label_title = Titel
mail_overdue_content = { "Hallo {username},\u000ADie Ausleihfrist für das Buch '{booktitle}' ist abgelaufen.\u000ABitte gib das Buch schnellstmöglich wieder in der Schülerbücherei ab.\u000A\u000AViele Grüße vom Schülerbüchereiteam." }
mail_overdue_subject = { "'{booktitle}' wird nun überzogen!" }
mail_overdue_title = Überziehungsnachricht
mail_overdue2_content = { "Hallo {username},\u000ADu hat das Buch '{booktitle}' schon seit über 2 Wochen\u000Atrotz täglicher Mahnungen überzogen.\u000AFür jede weitere Woche fallen ab jetzt 1€ Mahngebühren an.\u000A\u000AViele Grüße vom Schülerbüchereiteam." }
mail_overdue2_subject = { "Mahngebühren aufgrund der Überziehung von '{booktitle}'." }
mail_overdue2_title = Mahnung
mail_reset = Reset
main_info = Informationen
main_title = Schiller Büchereiverwaltung
menu_file = Datei
pref_appearance_title = Design
pref_appearance_dark = Dunkles Design
pref_appearance_light = Helles Design
pref_borrowing_duration = Ausleihdauer
pref_borrowing_header = Ausleihen
pref_cred = Email Zugangsdaten
pref_database_header = Database
pref_database_warning = Zur Datenbankwartung werden SQL-Kenntnisse benötigt. Bitte wende dich an einen Informatiklehrer oder Entwickler.
pref_global = Global
pref_local = Lokal
pref_mail_last_reminder = Letztes Mal Erinnert
pref_mail_account_from = Absender
pref_mail_account_header = Email Account
pref_mail_account_host = Host
pref_mail_account_password = Passwort
pref_mail_templates_header = Email Vorlagen
pref_overdue_warning_delay = Überziehungszeit bist zur Mahnung
pref_project = Projekt
pref_project_label = Die globalen Projekteinstellungen betreffen die ganze Bibliothek
pref_request_header = Anfrage von Mediendaten
pref_request_token = DNB Token
pref_title = Einstellungen
pref_user_delimiter = Trennzeichen
pref_user_header = Accounts
pref_user_path = Accountdatei
pref_user_update = Accounts aktualisieren
pref_user_update_info = Lade und aktualisiere die Benutzergruppen (z.B. Klassen oder Jahrgangsstufen).
search_action = Suchen
search_advanced = Erweitert
search_base = Basic
search_book = Bücher
search_book_entry = Suchbegriff (z.B. Autor, Titel, Signatur oder Account)
search_results = { $arg0 } Ergebniss{ $arg0 ->
        [one] { "" }
       *[other] e
    }
search_user = Accounts
search_user_entry = Suchbegriff (z.B. Name, Account oder Gruppe)
sql_title = Datenbank warten
user = Accounts
user_account = Account
user_books = Ausgeliehene oder Vorgemerkte Bücher
user_delete = Möchtest du den Account "{ $arg0 }" wirklich löschen?
user_forename = Vorname
user_invalid = Account, Vorname und Nachname dürfen nicht leer bleiben.
user_may_borrow = Darf ausleihen
user_may_not_borrow = Darf nicht ausleihen
user_name = Name
user_new = Neuer Account
user_no_books = Keine Bücher ausgeliehen oder vorgemerkt.
user_permission = Ausleihberechtigung
user_referenced_del = Die Person hat noch ausgeliehene oder reservierte Bücher und kann noch nicht gelöscht werden.
user_request = Accountinfos laden
user_role = Gruppe
user_role_entry = Klasse / Jahrgang
user_surname = Nachname
