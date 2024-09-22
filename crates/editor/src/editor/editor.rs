use std::{collections::HashMap, sync::{Mutex, MutexGuard, OnceLock}};

use iced::{
    alignment, keyboard, 
    widget::{container, horizontal_space, pane_grid, row, text}, 
    Element, Length, Subscription, Task
};
use registers::{ Event, Register};

use ui::{button_with_icon, danger_button_with_icon, styles, Icon};
use crate::pane::Pane;

#[derive(Debug,Clone)]
pub struct KeyBinding {
    pub key: char,
    pub ctrl: bool,
    pub shift: bool,
    pub event: Event,
}

impl KeyBinding {
    pub fn new(key: char, ctrl: bool, shift: bool, event: Event) -> Self {
        Self {
            key,
            ctrl,
            shift,
            event,
        }
    }
}

fn get_global_hashmap() -> MutexGuard<'static, HashMap<char, Vec<KeyBinding>>> {
    static MAP_KEYS: OnceLock<Mutex<HashMap<char, Vec<KeyBinding>>>> = OnceLock::new();
    MAP_KEYS.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("Let's hope the lock isn't poisoned")
}

pub struct Editor {
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
}

impl Editor {
    pub fn new() -> Self {
        let (state, grid_panel) = pane_grid::State::new(Pane::new(0));
        Self {
            panes: state,
            focus: Some(grid_panel),
        }
    }

    pub fn new_from(t: &Self) -> Self {
        let this = Self {
            panes: t.panes.clone(),
            focus: t.focus,
        };
        this
    }

    pub fn get_key_bindings_map(&self) -> HashMap<char, Vec<KeyBinding>> {
        get_global_hashmap().clone()
    }
    
    pub fn add_key_binding(&mut self, key: char, ctrl: bool, shift: bool, event: Event) {
        let mut map = get_global_hashmap();
        map.entry(key)
            .or_insert(Vec::new())
            .push(KeyBinding::new(key, ctrl, shift, event.clone()));
    }
    
    pub fn add_ctrl_key_binding(&mut self, key: char, shift: bool, event: Event) {
        self.add_key_binding(key, true, shift, event.clone());
    }

    pub fn add_shift_key_binding(&mut self, key: char, event: Event) {
        self.add_key_binding(key, true, true, event.clone());
    }
    
    pub fn add_keys_bindings(&mut self, bindings: Vec<KeyBinding>) {
        for binding in bindings {
            self.add_key_binding(binding.key, binding.ctrl, binding.shift, binding.event);
        }
    }
    
}

impl Register for Editor {
    fn update(&mut self, _event: Event) -> Task<Event> {
        match _event {
            Event::PaneDragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.panes.drop(pane, target);
                Task::none()
            },
            Event::TogglePin(pane) => {
                if let Some(Pane { is_pinned, .. }) = self.panes.get_mut(pane) {
                    *is_pinned = !*is_pinned;
                }
                Task::none()
            },
            Event::Split(axis, pane) => {
                let state = Pane::new(self.panes.len());
                let result =
                    self.panes.split(axis, pane, state.clone());
                
                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
                Task::none()
            },
            Event::PaneClicked(pane) => {
                self.focus = Some(pane);
                Task::none()
            },
            Event::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
                Task::none()
            },
            Event::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
                Task::none()
            }
            _ => {
                if let Some(p) = self.focus {
                    let pane = self.panes.get_mut(p).expect("Unable to get pane");
                    return pane.core.update(_event.clone());
                }
                self.panes.iter_mut().fold(
                    Task::none(),
                    |task, (_, e)| {
                        Task::chain(task,e.core.update(_event.clone()))
                    }
                )
            },
        }
    }

    fn subscription(&self) -> Subscription<Event> {
        keyboard::on_key_release(|k,m| {
            let map = get_global_hashmap();
            
            if let keyboard::Key::Character(c) = k {
                // Get the first character of the key
                let key = &c.as_str().chars().next().unwrap();
                
                if let Some(bindings) = map.get(key) {
                    for binding in bindings {
                        if binding.ctrl == m.control() && binding.shift == m.shift() {
                            return Some(binding.event.clone());
                        }
                    }
                }
                
            }
            
            None
        })
    }
    
    fn view(&self) -> Element<'_, Event> {
        let total_panes = self.panes.len();
        let focus = self.focus;
        
        let grid = 
            pane_grid(&self.panes, |id, pane, _is_maximized| {
                let is_focused = Some(id) == focus;
                
                let button = 
                    |icon: Icon, label: &'static str, message: Event| {
                        button_with_icon(
                            icon,
                            label, message
                        )
                        .padding(4)
                        .style(styles::tooltip_style)
                    };
                
                let danger_button = 
                    |icon: Icon, label: &'static str, message: Event| {
                        danger_button_with_icon(
                            icon,
                            label, message
                        )
                        .padding(4)
                        .style(styles::tooltip_danger_style)
                    };
                
                let content = if pane.is_pinned { Icon::Pin } else { Icon::Unpin };
                let pin_button = 
                    button(
                        content,
                        "Pin/Unpin",
                        Event::TogglePin(id)
                    );
                
                let controls = row![
                        button(
                            Icon::HorizontalSplit,
                            "Split Horizontal",
                            Event::Split(pane_grid::Axis::Horizontal, id),
                        ),
                        button(
                            Icon::VerticalSplit,
                            "Split Vertical",
                            Event::Split(pane_grid::Axis::Vertical, id),
                        )
                    ]
                    .push_maybe(if total_panes > 1 && !pane.is_pinned {
                        Some(
                            danger_button(
                                Icon::Close,
                                "Close", 
                                Event::Close(id)
                            )
                        )
                    } else {
                        None
                    })
                    .spacing(5)
                        .padding(5);
                
                let title_bar = 
                    pane_grid::TitleBar::new(
                        row![
                            pin_button,
                            text("Editor".to_string()),
                            horizontal_space(),
                            controls
                        ].align_y(alignment::Vertical::Center)
                        .padding(4)
                        .spacing(5)
                    );
                
                pane_grid::Content::new(
                    pane.core.view()
                )
                .title_bar( title_bar )
                .style(if is_focused {
                    styles::pane_focused
                }else{
                    styles::pane_active
                }).into()
                
            }).width(Length::Fill)
            .height(Length::Fill)
            .on_click(Event::PaneClicked)
            .on_drag(Event::PaneDragged)
            .on_resize(5, Event::PaneResized)
            .spacing(5);
        
        container(
            grid
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(4)
        .into()
    }
}
