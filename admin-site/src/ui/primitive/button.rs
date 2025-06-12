use leptos::{attr::any_attribute::AnyAttribute, prelude::*};
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "flex rounded-md")]
pub struct ButtonVariant {
    pub size: ButtonSize,
    pub look: ButtonLook,
}

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2 has-[>svg]:px-3")]
    Default,
    #[tw(class = "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5")]
    Sm,
    #[tw(class = "h-10 rounded-md px-6 has-[>svg]:px-4")]
    Lg,
    #[tw(class = "size-9")]
    Icon,
}

#[derive(TwVariant)]
pub enum ButtonLook {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80")]
    Secondary,
    #[tw(
        class = "bg-destructive text-destructive-foreground shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60"
    )]
    Destructive,
    #[tw(
        class = "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50"
    )]
    Outline,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[component]
pub fn Button(
    #[prop(optional, into)] look: Signal<ButtonLook>,
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
    #[prop(optional)] attr: Vec<AnyAttribute>,
) -> impl IntoView {
    let class = ArcMemo::new(move |_| {
        let look = look.get();
        let size = size.get();
        let button = ButtonVariant { look, size };
        button.with_class(class.get())
    });

    view! {
        <button {..attr} class=class>
            {children()}
        </button>
    }
}
