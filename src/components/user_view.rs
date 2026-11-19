use crate::{
    components::separator::{SeparatorOrientation, separator},
    t,
};

use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::db::User;

#[component]
pub async fn user_view(cx: &Cx, user: User) -> Result<impl View> {
    let role = if user.role.is_empty() {
        t!(action_empty).resolve(cx).into_owned()
    } else {
        user.role.clone()
    };

    Ok(view! {
        <div class="h-full w-full space-y-4 overflow-y-auto p-4">
            <div>
                <h2 class="text-xl font-semibold">(format!("{} {}", user.forename, user.surname))</h2>
                <p class="text-md">
                    <span class="text-sm text-muted-foreground">(t!(user_account))": "</span>
                    (user.account)
                </p>
            </div>
            separator(orientation: SeparatorOrientation::Horizontal)
            <div>
                <h3 class="text-sm text-muted-foreground">(t!(user_role))</h3>
                <p class="text-md whitespace-pre-line font-medium">(role)</p>
            </div>
            <div class="flex items-center space-x-2">
                <span class="text-md font-medium">(t!(user_may_borrow))":"</span>
                if user.may_borrow {
                    <span class="inline-flex items-center rounded-md bg-primary px-2.5 py-0.5 text-xs font-semibold text-primary-foreground">(t!(action_yes))</span>
                } else {
                    <span class="inline-flex items-center rounded-md bg-destructive px-2.5 py-0.5 text-xs font-semibold text-destructive-foreground">(t!(action_no))</span>
                }
            </div>
        </div>
    })
}
