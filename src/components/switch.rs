use dioxus::prelude::*;
use dioxus_components::switch::{Switch, SwitchThumb};

#[component]
pub fn SwitchUse() -> Element {
    let mut checked = use_signal(|| false);

    // 根据 checked 切换背景色
    let switch_style = if checked() {
        "all: unset;
         position: relative;
         width: 2rem;
         height: 1.15rem;
         border-radius: 9999px;
         background-color: var(--secondary-color-2);
         cursor: pointer;
         transition: background-color 150ms;"
    } else {
        "all: unset;
         position: relative;
         width: 2rem;
         height: 1.15rem;
         border-radius: 9999px;
         background-color: var(--primary-color-6);
         cursor: pointer;
         transition: background-color 150ms;"
    };

    // 根据 checked 切换 thumb 样式
    let thumb_style = if checked() {
        "display: block;
         width: calc(1.15rem - 2px);
         height: calc(1.15rem - 2px);
         border-radius: 9999px;
         background-color: var(--light, var(--primary-color)) var(--dark, var(--primary-color-3));
         transform: translateX(calc(2rem - 1px - (1.15rem - 2px)));
         transition: transform 150ms;
         will-change: transform;"
    } else {
        "display: block;
         width: calc(1.15rem - 2px);
         height: calc(1.15rem - 2px);
         border-radius: 9999px;
         background-color: var(--light, var(--primary-color)) var(--dark, var(--secondary-color-2));
         transform: translateX(1px);
         transition: transform 150ms;
         will-change: transform;"
    };

    rsx! {
        div {
            "style": "display: flex; align-items: center; padding: 20px; gap: 15px;",
            Switch {
                "style": "{switch_style}",
                checked: checked(),
                aria_label: "Switch Demo",
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                },
                SwitchThumb {
                    "style": "{thumb_style}",
                }
            }
        }
    }
}
