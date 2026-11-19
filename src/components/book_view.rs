use crate::{
    components::separator::{SeparatorOrientation, separator},
    t,
};

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    view::{View, component, view},
};

use crate::db::{Book, Category};

#[component]
pub async fn book_view(
    cx: &Cx,
    book: Book,
    #[default] category: Option<Category>,
) -> Result<impl View> {
    let category = category.map_or_else(
        || book.category.clone(),
        |category| format!("{} - {} - {}", category.id, category.name, category.section),
    );
    let fields = [
        (t!(book_isbn), book.isbn.clone()),
        (t!(book_authors), book.authors.clone()),
        (t!(book_publisher), book.publisher.clone()),
        (
            t!(book_costs),
            if book.costs == 0.0 {
                String::new()
            } else {
                book.costs.to_string()
            },
        ),
        (
            t!(book_year),
            if book.year == 0 {
                String::new()
            } else {
                book.year.to_string()
            },
        ),
        (t!(category), category),
        (t!(book_note), book.note.clone()),
    ];
    let state = if let Some(borrower) = &book.borrower {
        t!(
            book_borrowed_by,
            arg0 = borrower.user.as_str(),
            arg1 = borrower.deadline.to_string()
        )
        .resolve(cx)
        .into_owned()
    } else if let Some(reservation) = &book.reservation {
        t!(book_reserved_by, arg0 = reservation.as_str())
            .resolve(cx)
            .into_owned()
    } else if book.borrowable {
        t!(component_book_available).resolve(cx).into_owned()
    } else {
        t!(component_book_not_borrowable).resolve(cx).into_owned()
    };

    Ok(view! {
        <div class="h-full w-full space-y-4 overflow-y-auto p-4">
            <div>
                <h2 class="text-xl font-semibold">(book.title)</h2>
                <p class="text-md">
                    <span class="text-sm text-muted-foreground">(t!(book_id))": "</span>
                    (if book.id.is_empty() { t!(action_empty).resolve(cx).into_owned() } else { book.id })
                </p>
            </div>
            separator(orientation: SeparatorOrientation::Horizontal)
            for (label, value) in fields {
                <div>
                    <h3 class="text-sm text-muted-foreground">(label)</h3>
                    <p class="text-md whitespace-pre-line font-medium">
                        (if value.is_empty() { t!(action_empty).resolve(cx).into_owned() } else { value })
                    </p>
                </div>
            }
            <div class="flex items-center space-x-2">
                <span class="text-md font-medium">(t!(book_borrowable))":"</span>
                if book.borrowable {
                    <span class="inline-flex items-center rounded-md bg-primary px-2.5 py-0.5 text-xs font-semibold text-primary-foreground">(t!(action_yes))</span>
                } else {
                    <span class="inline-flex items-center rounded-md bg-destructive px-2.5 py-0.5 text-xs font-semibold text-destructive-foreground">(t!(action_no))</span>
                }
            </div>
            <div role="alert" class="relative flex gap-3 rounded-lg border p-4 text-sm">
                icon(data: iconify_icon!("lucide:book-dashed"))
                <div>
                    <h3 class="mb-1 font-medium">(t!(book_state))</h3>
                    <div>
                        (state)
                        if book.borrower.is_some() {
                            if let Some(reservation) = book.reservation {
                                <div class="pt-1">(t!(book_reserved_by, arg0 = reservation.as_str()).resolve(cx).into_owned())</div>
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    })
}
