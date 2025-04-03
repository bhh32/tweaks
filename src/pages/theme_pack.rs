use cosmic::{
    cosmic_config::{
        Config, 
        CosmicConfigEntry}, 
        iced::{alignment::Horizontal, Size}, 
        iced_widget::Scrollable, 
        theme, 
        widget::{self, 
            button, 
            column, 
            container, 
            flex_row, 
            horizontal_space,
            responsive,
            row, scrollable, 
            settings::{
                item::builder, 
                section
            },
            slider, 
            spin_button, 
            text::{self, text}, 
            tooltip, 
            Column, 
            Row
    }, Apply, Element, Task, Theme
};
use cosmic_panel_config::CosmicPanelConfig;

use crate::{
    app::TweakMessage, 
    core::icons, 
    pages::dock::Dock,
    fl
};

use super::{color_schemes::{config::ColorScheme, preview, ColorSchemes}, layouts::{config::Layout, Layouts}, panel::Panel};

#[derive(Debug)]
pub struct ThemePack {
    pub color_schemes: ColorSchemes,
    pub dock: Dock,
    pub panel: Panel,
    pub layouts: Layouts,
}

impl Default for ThemePack {
    fn default() -> Self {
        let dock = Dock::default();
        let panel = Panel::default();
        let color_schemes = ColorSchemes::default();
        let layouts = Layouts::default();

        Self {
            color_schemes,
            dock,
            panel,
            layouts,
        }
    }
}

impl ThemePack {
    pub fn view<'a>(&self) -> Element<TweakMessage> {
        let spacing = theme::active().cosmic().spacing;

        // Theme Packs Section
        let theme_packs_section: Column<TweakMessage> = column::with_children(vec![
            row::with_children(vec![
                text::title3(format!("Theme Packs")).into(),
                horizontal_space().into(),
                tooltip::tooltip(
                    icons::get_handle("insert-object-symbolic", 16)
                        .apply(button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::ApplyThemePack(
                            (
                                self.layouts.selected_layout
                                    .clone()
                                    .unwrap_or(Layout::Cosmic), 
                                self.color_schemes.clone())
                            )
                        )
                        .class(cosmic::style::Button::Standard),
                    text(format!("Apply Selected Theme Pack")),
                    tooltip::Position::Bottom,
                )
                .into(),
                tooltip::tooltip(
                    icons::get_handle("document-save-as-symbolic", 16)
                        .apply(button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::SaveThemePack)
                        .class(cosmic::style::Button::Standard),
                    text(format!("Save Current Theme Pack")),
                    tooltip::Position::Bottom,
                )
                .into(),
                tooltip::tooltip(
                    icons::get_handle("edit-delete-symbolic", 16)
                        .apply(button::icon)
                        .padding(spacing.space_xxs)
                        .on_press(TweakMessage::DeleteThemePack)
                        .class(cosmic::style::Button::Standard),
                    text(format!("Delete Selected Theme Pack")),
                    tooltip::Position::Bottom,
                )
                .into(),
            ])
            .spacing(spacing.space_xxs)
            .into(),
        ])
        .spacing(spacing.space_xxs)
        .into();

        // Color Scheme Section
        let color_scheme_section: Column<TweakMessage> = column::with_children(vec![
            row::with_children(vec![
                text::title4(fl!("color-schemes")).into(),
                horizontal_space().into(),                
            ])
            .spacing(spacing.space_xxs)
            .into(),
            row::with_children(vec![
                section()
                .title(fl!("installed"))
                .add(
                    column().push(row::with_children(vec![
                        tooltip::tooltip(
                            icons::get_handle("arrow-into-box-symbolic", 16)
                                .apply(button::icon)
                                .padding(spacing.space_xxs)
                                .on_press(TweakMessage::OpenAvailableThemes)
                                .class(cosmic::style::Button::Standard),
                            text(fl!("save-current-color-scheme")),
                            tooltip::Position::Bottom,
                        )
                        .into(),
                        tooltip::tooltip(
                            icons::get_handle("document-save-symbolic", 16)
                                .apply(button::icon)
                                .padding(spacing.space_xxs)
                                .on_press(TweakMessage::OpenAvailableThemes)
                                .class(cosmic::style::Button::Standard),
                            text(fl!("import-color-scheme")),
                            tooltip::Position::Bottom,
                        )
                        .into(),
                        tooltip::tooltip(
                            icons::get_handle("search-global-symbolic", 16)
                                .apply(button::icon)
                                .padding(spacing.space_xxs)
                                .on_press(TweakMessage::OpenAvailableThemes)
                                .class(cosmic::style::Button::Standard),
                            text(fl!("find-color-schemes")),
                            tooltip::Position::Bottom,
                        )
                        .into(),
                    ])
                    .spacing(spacing.space_xs),
                    )
                    .align_x(Horizontal::Right),
                )
                .add({
                    let themes: Vec<Element<TweakMessage>> = self.color_schemes
                        .installed
                        .iter()
                        .map(|color_scheme| preview::installed(color_scheme, &self.color_schemes.selected))
                        .collect();

                    flex_row(themes)
                        .row_spacing(spacing.space_xs)
                        .column_spacing(spacing.space_xs)
                        .apply(widget::container)
                        .padding([0, spacing.space_xxs])
                }).into(),
        ]).into(),
        ])
        .spacing(spacing.space_xxs);

        // Dock Section
        let dock_section: Column<TweakMessage> = column::with_children(vec![
            text::title4(fl!("dock")).into(),
            scrollable(
                widget::settings::section()
                    .add(
                        widget::settings::item::builder(fl!("padding"))
                            .description(fl!("padding-description"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control(
                                widget::row::with_children(vec![
                                    widget::slider(0..=28, self.dock.padding, TweakMessage::SetDockPadding).into(),
                                    widget::text::text(format!("{} px", self.dock.padding)).into(),
                                ])
                                .spacing(spacing.space_xxs),
                            ),
                    )
                    .add(
                        widget::settings::item::builder(fl!("spacing"))
                            .description(fl!("spacing-description"))
                            .icon(icons::get_icon("size-horizontally-symbolic", 18))
                            .control(
                                widget::row::with_children(vec![
                                    widget::slider(0..=28, self.dock.spacing, TweakMessage::SetDockSpacing).into(),
                                    widget::text::text(format!("{} px", self.dock.spacing)).into(),
                                ])
                                .spacing(spacing.space_xxs),
                            ),
                    ),
            )
            .into(),
        ])
        .spacing(spacing.space_xxs);

        // Panel Section
        let panel_section: Column<TweakMessage> = column::with_children(vec![
            text::title4(fl!("panel")).into(),
            scrollable(
                widget::settings::section()
                    .add(
                        widget::settings::item::builder(fl!("show-panel"))
                            .toggler(self.panel.show_panel, TweakMessage::ShowPanel),
                    )
                    .add(
                        widget::settings::item::builder(fl!("force-icon-buttons-in-panel"))
                            .toggler(self.panel.force_icons, TweakMessage::ForceIcons),
                    )
                    .add(
                        widget::settings::item::builder(fl!("padding"))
                            .description(fl!("padding-description"))
                            .icon(icons::get_icon("resize-mode-symbolic", 18))
                            .control(
                                widget::row::with_children(vec![
                                    widget::slider(0..=20, self.panel.padding, TweakMessage::SetPanelPadding).into(),
                                    widget::text::text(format!("{} px", self.panel.padding)).into(),
                                ])
                                .spacing(spacing.space_xxs),
                            ),
                    )
                    .add(
                        widget::settings::item::builder(fl!("spacing"))
                            .description(fl!("spacing-description"))
                            .icon(icons::get_icon("size-horizontally-symbolic", 18))
                            .control(
                                widget::row::with_children(vec![
                                    widget::slider(0..=28, self.panel.spacing, TweakMessage::SetPanelSpacing).into(),
                                    widget::text::text(format!("{} px", self.panel.spacing)).into(),
                                ])
                                .spacing(spacing.space_xxs),
                            ),
                    ),
            )
            .into(),
        ])
        .spacing(spacing.space_xxs);

        // Layouts Section
        let layouts = self.layouts
            .config
            .layouts
            .iter()
            .map(|layout| {
                widget::column()
                    .push(layout.preview())
                    .push(widget::text(layout.name()))
                    .spacing(spacing.space_xs)
                    .align_x(Horizontal::Center)
                    .into()
            })
            .collect::<Vec<Element<TweakMessage>>>();

        let layouts_section = widget::column::with_children(vec![
                widget::row::with_children(vec![
                    widget::text::title4(fl!("layouts")).into(),
                    widget::horizontal_space().into(),
                    widget::tooltip::tooltip(
                        icons::get_handle("arrow-into-box-symbolic", 16)
                            .apply(widget::button::icon)
                            .padding(spacing.space_xxs)
                            .on_press(TweakMessage::OpenSaveDialog)
                            .class(cosmic::style::Button::Standard),
                        widget::text(fl!("save-current-layout")),
                        widget::tooltip::Position::Bottom,
                    )
                    .into(),
                ])
                .spacing(spacing.space_xxs)
                .into(),
                widget::settings::section()
                    .add(
                        widget::flex_row(layouts)
                            .row_spacing(spacing.space_s)
                            .column_spacing(spacing.space_s)
                            .apply(widget::container)
                            .padding([0, spacing.space_xxs]),
                    )
                    .apply(scrollable)
                    .into(),
            ])
            .spacing(spacing.space_s)
            .height(400.);

        // Combine all sections
        let row_width = 600.;
        container(
            column::with_children(vec![
                theme_packs_section.max_width(row_width).into(),
                horizontal_space().height(spacing.space_xs).into(),
                color_scheme_section.max_width(row_width).into(),
                horizontal_space().height(spacing.space_xs).into(),
                dock_section.max_width(row_width).into(),
                horizontal_space().height(spacing.space_xs).into(),
                panel_section.max_width(row_width).into(),
                horizontal_space().height(spacing.space_xs).into(),
                container(layouts_section).max_width(row_width).into(),
            ])
        )
        .apply(scrollable)
        .into()
    }

    pub fn update(&mut self, message: TweakMessage) -> Task<crate::app::Message> {
        match message {
            TweakMessage::ApplyThemePack((layout, color_schemes)) => {
                println!("Applying Theme Pack!");
                self.color_schemes = color_schemes;
                // Update layout logic here
            }
            TweakMessage::SelectThemePack((layout, color_schemes)) => {
                println!("Selecting Theme Pack!");
                self.color_schemes = color_schemes;
                // Update layout logic here
            }
            TweakMessage::SaveThemePack => println!("Saving Theme Pack!"),
            TweakMessage::DeleteThemePack => println!("Deleting Theme Pack!"),
            TweakMessage::SetDockPadding(padding) => self.dock.padding = padding,
            TweakMessage::SetDockSpacing(spacing) => self.dock.spacing = spacing,
            TweakMessage::SetPanelPadding(padding) => self.panel.padding = padding,
            TweakMessage::SetPanelSpacing(spacing) => self.panel.spacing = spacing,
            TweakMessage::SelectLayout(_layout) => println!("Selecting Layout!"),
            _ => {}
        }

        Task::none()
    }
}