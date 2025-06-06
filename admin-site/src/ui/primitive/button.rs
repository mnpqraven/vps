use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "flex rounded-md")]
pub struct ButtonVariant {
    pub size: ButtonSize,
    pub look: ButtonLook,
}

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-8 px-3")]
    Sm,
    #[tw(class = "h-10 px-8")]
    Lg,
}

#[derive(TwVariant)]
pub enum ButtonLook {
    // TODO: global variables with dark mode
    #[tw(default, class = "bg-blue-500 text-blue-100")]
    Default,
    #[tw(class = "border")]
    Outline,
}

#[component]
pub fn Button(
    #[prop(optional)] look: Signal<ButtonLook>,
    #[prop(optional)] size: Signal<ButtonSize>,
    #[prop(optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = ArcMemo::new(move |_| {
        let look = look.get();
        let size = size.get();
        let button = ButtonVariant { look, size };
        button.with_class(class.get())
    });

    view! {
        <button class=class>
            {children()}
        </button>
    }
}
