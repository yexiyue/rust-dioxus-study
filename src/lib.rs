#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_elements::input_data::keyboard_types::Key;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};


pub mod platform;
use platform::{Store,get_store};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FilterState {
    All,
    Active,
    Completed,
}



#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Todos {
    items: HashMap<u32, TodoItem>,
    next_id:u32,
}
impl Default for Todos {
    fn default() -> Self {
        let store = get_store();
        store.get()
    }
}
//可以使用Deref 和 DerefMut来简化读取，不用在了
impl Deref for Todos {
    type Target = HashMap<u32, TodoItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Todos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl Todos {
    pub fn save(&self) {
        let store = get_store();
        store
            .set( &self)
    }
}

pub fn App(cx: Scope<()>) -> Element {
    let todos = use_state(cx, Todos::default);
    let filter = use_state(cx, FilterState::default);
    let draft = use_state(cx, || "".to_string());
    let todo_id=todos.get().next_id;
    // Filter the todos based on the filter state
    let mut filtered_todos = todos
        .iter()
        .filter(|(_, item)| match **filter {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
    filtered_todos.sort_unstable();

    let active_todo_count = todos.values().filter(|item| !item.checked).count();
    let active_todo_text = match active_todo_count {
        1 => "item",
        _ => "items",
    };

    let show_clear_completed = todos.values().any(|todo| todo.checked);

    let selected = |state| {
        if *filter == state {
            "selected"
        } else {
            "false"
        }
    };

    cx.render(rsx! {
        section { class: "todoapp",
            style { include_str!("./asserts/tomvc.css") }
            header { class: "header",
                h1 {"todos"}
                input {
                    class: "new-todo",
                    placeholder: "What needs to be done?",
                    value: "{draft}",
                    autofocus: "true",
                    oninput: move |evt| {
                        draft.set(evt.value.clone());
                    },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter && !draft.is_empty() {
                            todos.make_mut().insert(
                                todo_id,
                                TodoItem {
                                    id: todo_id,
                                    checked: false,
                                    contents: draft.to_string(),
                                },
                            );
                            todos.make_mut().next_id+=1;
                            todos.save();
                            draft.set("".to_string());
                        }
                    }
                }
            }
            section {
                class: "main",
                if !todos.is_empty() {
                    rsx! {
                        input {
                            id: "toggle-all",
                            class: "toggle-all",
                            r#type: "checkbox",
                            onchange: move |_| {
                                let check = active_todo_count != 0;
                                for (_, item) in todos.make_mut().iter_mut() {
                                    item.checked = check;
                                }
                                todos.save();
                            },
                            checked: if active_todo_count == 0 { "true" } else { "false" },
                        }
                        label { r#for: "toggle-all" }
                    }
                }
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| rsx!(TodoEntry {
                        key: "{id}",
                        id: *id,
                        todos: todos,
                    }))
                }
                (!todos.is_empty()).then(|| rsx!(
                    footer { class: "footer",
                        span { class: "todo-count",
                            strong {"{active_todo_count} "}
                            span {"{active_todo_text} left"}
                        }
                        ul { class: "filters",
                            for (state, state_text, url) in [
                                (FilterState::All, "All", "#/"),
                                (FilterState::Active, "Active", "#/active"),
                                (FilterState::Completed, "Completed", "#/completed"),
                            ] {
                                li {
                                    a {
                                        href: url,
                                        class: selected(state),
                                        onclick: move |_| filter.set(state),
                                        prevent_default: "onclick",
                                        state_text
                                    }
                                }
                            }
                        }
                        show_clear_completed.then(|| rsx!(
                            button {
                                class: "clear-completed",
                                onclick: move |_| {
                                    todos.make_mut().retain(|_, todo| !todo.checked);
                                    todos.save();
                                },
                                "Clear completed"
                            }
                        ))
                    }
                ))
            }
        }
        footer { class: "info",
            p { "Double-click to edit a todo" }
            p { "Created by ", a { href: "http://github.com/jkelleyrtp/", "jkelleyrtp" }}
            p { "Part of ", a { href: "http://todomvc.com", "TodoMVC" }}
        }
    })
}

#[derive(Props)]
pub struct TodoEntryProps<'a> {
    todos: &'a UseState<Todos>,
    id: u32,
}

pub fn TodoEntry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    let is_editing = use_state(cx, || false);

    let todos = cx.props.todos.get();
    let todo = &todos[&cx.props.id];
    let completed = if todo.checked { "completed" } else { "" };
    let editing = if **is_editing { "editing" } else { "" };

    cx.render(rsx!{
        li {
            class: "{completed} {editing}",
            div { class: "view",
                input {
                    class: "toggle",
                    r#type: "checkbox",
                    id: "cbg-{todo.id}",
                    checked: "{todo.checked}",
                    oninput: move |evt| {
                        cx.props.todos.make_mut().get_mut(&cx.props.id).unwrap().checked = evt.value.parse().unwrap();
                        todos.save();
                    }
                }
                label {
                    r#for: "cbg-{todo.id}",
                    ondblclick: move |_| is_editing.set(true),
                    prevent_default: "onclick",
                    "{todo.contents}"
                }
                button {
                    class: "destroy",
                    onclick: move |_| { cx.props.todos.make_mut().remove(&todo.id);todos.save(); },
                    prevent_default: "onclick",
                }
            }
            is_editing.then(|| rsx!{
                input {
                    class: "edit",
                    value: "{todo.contents}",
                    oninput: move |evt|{ 
                            cx.props.todos.make_mut().get_mut(&cx.props.id).unwrap().contents = evt.value.clone();
                            todos.save(); 
                        },
                    autofocus: "true",
                    onfocusout: move |_| is_editing.set(false),

                    onkeydown: move |evt| {
                        match evt.key() {
                            Key::Enter | Key::Escape | Key::Tab => is_editing.set(false),
                            _ => {}
                        }
                    },
                }
            })
        }
    })
}

