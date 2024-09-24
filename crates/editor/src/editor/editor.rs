use std::collections::HashMap;

use iced::{
    keyboard, 
    widget::{container, pane_grid, row}, 
    Element, Length, Subscription, Task
};
use registers::{ Event, Register};

use ui::{button_with_icon, danger_button_with_icon, styles, Icon};
use crate::{pane::Pane, key_bindings::*};


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
    
    pub fn add_key_binding(&mut self, kb: KeyBinding) {
        let mut map = get_global_hashmap();
        map.entry(kb.key)
            .or_insert(Vec::new())
            .push(kb);
    }
    
    pub fn make_key_binding(&mut self, key: char, ctrl: bool, shift: bool, alt: bool, event: Event) {
        let mut map = get_global_hashmap();
        map.entry(key)
            .or_insert(Vec::new())
            .push(KeyBinding::new(key, ctrl, shift, alt, event.clone()));
    }
    
    pub fn add_keys_bindings(&mut self, bindings: Vec<KeyBinding>) {
        bindings.iter().for_each(move |kb| self.add_key_binding(kb.clone()));
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
            Event::PaneDragged(_) => { Task::none() }
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
                        if 
                                binding.ctrl == (m.control() || m.command())
                            &&  binding.shift == m.shift()
                            &&  binding.alt == m.alt()
                        {
                            println!("Binding {binding:?} matched");
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
                
                let (content, label) = if pane.is_pinned { (Icon::Pin, "Unpin") } else { (Icon::Unpin, "Pin") };
                
                let controls = row![
                    button(
                        content,
                        label,
                        Event::TogglePin(id)
                    ),
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
                ].push_maybe(if total_panes > 1 && !pane.is_pinned {
                        Some(
                            danger_button(
                                Icon::Close,
                                "Close", 
                                Event::Close(id)
                            )
                        )
                    } else {
                        None
                    }).padding([8,2]).spacing(8);
                
                let title_bar = 
                    pane_grid::TitleBar::new(
                        controls
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
            .on_resize(10, Event::PaneResized)
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
