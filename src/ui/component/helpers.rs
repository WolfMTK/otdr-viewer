use leptos::ev;
use leptos::prelude::*;
use leptos_use::{use_event_listener, use_window};

pub fn close_on_escape(open: RwSignal<bool>) {
    let _ = use_event_listener(use_window(), ev::keydown, move |event| {
        if event.key() == "Escape" && open.get_untracked() {
            open.set(false);
        }
    });
}

pub fn toggle_class(base: &str, active: &str, condition: bool) -> String {
    if condition {
        format!("{base} {active}")
    } else {
        base.to_string()
    }
}

pub fn filter_by_name<T>(
    items: impl Fn() -> Vec<T> + Send + Sync + 'static,
    query: RwSignal<String>,
    name: fn(&T) -> &str,
) -> Memo<Vec<T>>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    Memo::new(move |_| {
        let q = query.get().to_lowercase();
        items()
            .into_iter()
            .filter(|item| q.is_empty() || name(item).to_lowercase().contains(&q))
            .collect()
    })
}

#[component]
pub fn SearchBox(query: RwSignal<String>, placeholder: &'static str, class: &'static str) -> impl IntoView {
    view! {
        <div class=class>
            <img src="public/search.svg" alt="search" draggable="false" />
            <input
                type="text"
                placeholder=placeholder
                prop:value=move || query.get()
                on:input=move |e| query.set(event_target_value(&e))
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::ui::component::helpers::toggle_class;

    #[rstest]
    #[case(true, "base active")]
    #[case(false, "base")]
    fn toggle_class_cases(#[case] condition: bool, #[case] expected: &str) {
        assert_eq!(toggle_class("base", "active", condition), expected);
    }
}
