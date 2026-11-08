action_add = Add
action_all = All
action_apply = Apply
action_cancel = Cancel
action_close = Close
action_copy = Copy
action_copy_long = Copy to Clipboard
action_cut = Cut
action_delete = Delete
action_edit = Edit
action_empty = Empty
action_finish = Finish
action_load = Loading
action_logout = Logout
action_new = New
action_next = Next
action_no = No
action_ok = OK
action_open = Open
action_paste = Paste
action_previous = Previous
action_redo = Redo
action_select = Select
action_selectAll = Select All
action_undo = Undo
action_yes = Yes
alert_confirm = Confirm
alert_create_project = Create new Project Database
alert_error = Error
alert_info = Info
alert_mail_overdue = Should the reminders for the { $arg0 } overdue book be sent out now?
alert_mail_send_cancel = Cancelled
alert_mail_send_content = Sending Mails
alert_mail_send_error = The mail transfer failed.
alert_mail_send_error_to = Mail transfer failed for:
alert_mail_send_success = Finished sending mails.
alert_mail_send_title = Mail
alert_open_project = Select Project Database
alert_overdue = Overdue Notifications
alert_overdue_none = There are currently no overdrafts.
alert_update = Welcome to Version { $arg0 }.
    The database has been updated successfully.
alert_user_updated = Updated user roles successfully
alert_userfile = Select User File
book = Book
book_authors = Authors
book_authors_def = Forename Surname, Other Author
book_available = Available
book_available_long = Not Borrowed or Reserved
book_borrowable = Borrowable
book_borrowed = Borrowed
book_overdues = Overdues
book_borrowed_by = Borrowed by { $arg0 } until { $arg1 }
book_borrowed_by_short = Borrowed by { $arg0 }
book_costs = Costs
book_delete_reservation = Delete Reservation
book_delete = Do you want to delete the book "{ $arg0 }"?
book_from_user = By { $arg0 } ({ $arg1 })
book_id = ID
book_id_action = Suggest
book_id_format = The Book ID has to consist of the category, author and a numeric id.
book_invalid_isbn = The given ISBN is not valid
book_invalid = Title, ID and Category are mandatory.
book_isbn = ISBN
book_lend = Lend
book_lend_period = Period
book_lend_to = Lend to { $arg0 }
book_lend_until = Borrowed until { $arg0 }
book_new = Add Book
book_not_borrowable = Not Borrowable
book_note = Notes
book_period = { $arg0 } ({ $arg1 ->
        [one] { $arg1 } day
       *[other] { $arg1 } days
    })
book_period_days = Days
book_period_date = Date
book_publisher = Publisher
book_renew = Renew
book_request = Fetch Book Infos
book_reserve = Reserve
book_reserved = Reserved
book_reserved_by = Reserved by { $arg0 }
book_revoke = Return
book_revoke_reminder = The book is reserved by "{ $arg0 }".
    Do you want to inform this person that the book is now available?
book_state = State
book_title = Title
book_unavailable = Borrowed or Reserved
book_year = Year
category = Category
category_edit = Edit Categories
category_empty_input = Empty inputs are not allowed.
category_id = ID
category_name = Name
category_not_empty_del = A category has to be empty to be removed. This category, however, is not empty.
category_section = Section
category_none = Uncategorized
category_t1_id = SCFI
category_t1_name = Science Fiction
category_t1_section = Novels
category_t2_id = THRL
category_t2_name = Thriller
category_t2_section = Novels
error_book_existing = The book id '{ $arg0 }' already exists.
error_book_none = No book found.
error_category_existing = The category id '{ $arg0 }' already exists.
error_config_default = Could not access the default preferences.
error_date = The date could not be loaded correctly.
    Please update this in the settings under the section "Last Time Reminded!"
error_db = The database could not be opened.
error_file_open = The file could not be opened
error_format = Invalid file format
error_input = Malformed User or Book Data.
error_lending_already_borrowed_by = The user borrows the book already.
error_lending_already_borrowed = The book is already borrowed.
error_lending_already_reserved = The book is already reserved.
error_lending_not_reserved = The book was not reserved.
error_lending_book = The book may not be borrowed.
error_lending_not_borrowed = The book may not be reserved because it is not borrowed yet.
error_lending_user = The user may not borrow books.
error_network = Could not connect to server
error_no_project = No project has been opened.
error_none = No results found
error_provider_none = No { $arg0 } plugin installed
error_provider_config = Invalid value for { $arg0 }
error_user_existing = The user account '{ $arg0 }' already exists.
error_user_none = No user found.
error_version = The current Project Version is unsupported.
    Please contanct a developer.
error_unknown = An unknown error has occurred.
    Try refreshing the page or logging out and logging in again!
info_category_freq = Number of Books per Category:
    
    { $arg0 }
info_about = About { $arg0 } - Version { $arg1 }
info_about_repo = Repository:
info_about_license = License: { $arg0 }
info_about_devs = Developers:
info_about_name = Name: { $arg0 }
info_books = { $arg0 } book{ $arg0 ->
        [one] { "" }
       *[other] s
    }
info_users = { $arg0 } user{ $arg0 ->
        [one] { "" }
       *[other] s
    }
info_categories = { $arg0 } categor{ $arg0 ->
        [one] y
       *[other] ies
    }
info_borrows = { $arg0 } borrowed
info_reservations = { $arg0 } reservation{ $arg0 ->
        [one] { "" }
       *[other] s
    }
info_overdues = { $arg0 } overdue{ $arg0 ->
        [one] { "" }
       *[other] s
    }
info_costs = Total costs of all books: { $arg0 }
    (sum of all entered costs)
info_stats = Statistics
info_session = Session
info_session_id = ID: { $arg0 }
info_session_username = Username: { $arg0 }
lang_title = Language
lang_en = English
lang_de = German
mail_info = { "You can use keywords in the subject and content. They are replace with the respective values before the mail is send.\u000A\u000A{username}: Username, {booktitle}: Book Title" }
mail_info_content = { "Hallo {username},\u000AYou have shown interest in the book '{booktitle}.'\u000ANow it is back in the library, reserved for you to borrow.\u000A\u000ARegards,\u000AThe Library Team." }
mail_info_subject = { "'{booktitle}' is back in the library" }
mail_info_title = Info
mail_label_content = Content
mail_label_title = Subject
mail_overdue_content = { "Hello {username},\u000AThe borrowing period for the book '{booktitle}' has expired.\u000APlease bring it back to the library.\u000A\u000ARegards,\u000AThe Library Team." }
mail_overdue_subject = { "'{booktitle}' is overdrawn" }
mail_overdue_title = Overdraw Notice
mail_overdue2_content = { "Hello {username},\u000AThe borrowing period for the book '{booktitle}' has expired two weeks ago.\u000AFrom now on we have to charge you 1€ for every additional week.\u000A\u000ARegards,\u000AThe Library Team." }
mail_overdue2_subject = { "Overdraw fines for '{booktitle}'!" }
mail_overdue2_title = Overdraw Warning
mail_reset = Reset
main_info = Information
main_title = Schiller Library Software
menu_file = File
pref_appearance_title = Theme
pref_appearance_dark = Dark Theme
pref_appearance_light = Light Theme
pref_borrowing_duration = Borrowing Duration
pref_borrowing_header = Borrowing
pref_cred = Mail Credentials
pref_database_header = Database
pref_database_warning = Advanced SQL knowledge is needed for maintaining the database. Please ask a teacher or developer if you want to proceed.
pref_global = Global
pref_local = Local
pref_mail_last_reminder = Last Time Reminded
pref_mail_account_from = Sender
pref_mail_account_header = Mail Account
pref_mail_account_host = Hostname
pref_mail_account_password = Password
pref_mail_templates_header = Mail Templates
pref_project = Project
pref_project_label = These settings are global to the whole library
pref_overdue_warning_delay = Overdraft time until warning
pref_request_header = Requesting Media Information
pref_request_token = DNB Token
pref_title = Settings
pref_user_delimiter = Delimiter
pref_user_header = User
pref_user_path = User file
pref_user_update = Update Users
pref_user_update_info = Load and update the user groups (e.g., classes or year groups).
search_action = Search
search_advanced = Advanced
search_base = Basic
search_book = Books
search_book_entry = Keyword (e.g. Author, Title, ID, ISBN or User)
search_results = { $arg0 } Result{ $arg0 ->
        [one] { "" }
       *[other] s
    }
search_user = Users
search_user_entry = Keyword (e.g. Name, Account or Role)
sql_title = Maintain Database
user = User
user_account = Account
user_books = Borrowed or Reserved Books
user_delete = Do you really want to delete user "{ $arg0 }"?
user_forename = Forename
user_invalid = Account, Forename and Surname are mandatory.
user_may_borrow = May Borrow
user_may_not_borrow = May not Borrow
user_name = Name
user_new = Add User
user_no_books = No book borrowed or reserved.
user_permission = Borrowing Permission
user_referenced_del = The use still borrows or reserves books and cannot be deleted.
user_request = Fetch User Info
user_role = Role
user_role_entry = Class / Grade
user_surname = Surname
