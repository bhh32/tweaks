use cosmic::{
    cosmic_config::{self, Config, CosmicConfigEntry},
    widget, Element, Task,
};
use cosmic_panel_config::CosmicPanelConfig;
use serde::{Deserialize, Serialize};

use crate::{
    app::TweakMessage,
    core::{
        cosmic_panel_button_config::{CosmicPanelButtonConfig, IndividualConfig, Override},
        icons,
    },
    fl,
};

#[derive(Debug)]
pub struct Panel {
    pub panel_helper: Option<Config>,
    pub panel_config: Option<CosmicPanelConfig>,
    pub padding: u32,
    pub spacing: u32,
    pub show_panel: bool,
    pub cosmic_panel_config: CosmicPanel,
    pub cosmic_panel_config_helper: Option<Config>,
    pub cosmic_panel_button_config: CosmicPanelButtonConfig,
    pub cosmic_panel_button_config_helper: Option<Config>,
    pub force_icons: bool,
}

#[derive(
    Debug,
    Clone,
    Default,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    cosmic_config::cosmic_config_derive::CosmicConfigEntry,
)]
pub struct CosmicPanel {
    pub entries: Vec<String>,
}

impl Default for Panel {
    fn default() -> Self {
        let panel_helper = CosmicPanelConfig::cosmic_config("Panel").ok();
        let panel_config = panel_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Panel").then_some(panel_config)
        });
        let (cosmic_panel_config_helper, cosmic_panel_config) =
            match cosmic_config::Config::new("com.system76.CosmicPanel", 1) {
                Ok(config_handler) => {
                    let config = match CosmicPanel::get_entry(&config_handler) {
                        Ok(ok) => ok,
                        Err((errs, config)) => {
                            log::error!("errors loading config: {:?}", errs);
                            config
                        }
                    };
                    (Some(config_handler), config)
                }
                Err(err) => {
                    log::error!("failed to create config handler: {}", err);
                    (None, CosmicPanel::default())
                }
            };

        let (cosmic_panel_button_config_helper, cosmic_panel_button_config) =
            match cosmic_config::Config::new("com.system76.CosmicPanelButton", 1) {
                Ok(config_handler) => {
                    let config = match CosmicPanelButtonConfig::get_entry(&config_handler) {
                        Ok(ok) => ok,
                        Err((errs, config)) => {
                            log::error!(
                                "errors loading config for cosmic panel button: {:?}",
                                errs
                            );
                            config
                        }
                    };
                    (Some(config_handler), config)
                }
                Err(err) => {
                    log::error!(
                        "failed to create config handler for cosmic panel button: {}",
                        err
                    );
                    (None, CosmicPanelButtonConfig::default())
                }
            };

        let padding = panel_config
            .clone()
            .map(|config| config.padding)
            .unwrap_or(0);
        let spacing = panel_config
            .clone()
            .map(|config| config.spacing)
            .unwrap_or(0);
        let show_panel = cosmic_panel_config.entries.iter().any(|e| e == "Panel");
        let force_icons = cosmic_panel_button_config
            .configs
            .iter()
            .find(|(e, _)| *e == "Panel")
            .map(|(_, conf)| {
                conf.force_presentation
                    .as_ref()
                    .is_some_and(|presentation| *presentation == Override::Icon)
            })
            .unwrap_or(false);
        Self {
            panel_helper,
            panel_config,
            padding,
            spacing,
            show_panel,
            cosmic_panel_config,
            cosmic_panel_config_helper,
            cosmic_panel_button_config,
            cosmic_panel_button_config_helper,
            force_icons,
        }
    }
}

impl Panel {
    pub fn view<'a>(&self) -> Element<'a, TweakMessage> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        widget::scrollable(
            widget::settings::section()
                .title("Panel")
                .add(
                    widget::settings::item::builder(fl!("show-panel"))
                        .toggler(self.show_panel, TweakMessage::ShowPanel),
                )
                .add(
                    widget::settings::item::builder(fl!("force-icon-buttons-in-panel"))
                        .toggler(self.force_icons, TweakMessage::ForceIcons),
                )
                .add(
                    widget::settings::item::builder(fl!("padding"))
                        .description(fl!("padding-description"))
                        .icon(icons::get_icon("resize-mode-symbolic", 18))
                        .control(
                            widget::row::with_children(vec![
                                widget::slider(0..=20, self.padding, TweakMessage::SetPanelPadding).into(),
                                widget::text::text(format!("{} px", self.padding)).into(),
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
                                widget::slider(0..=28, self.spacing, TweakMessage::SetPanelSpacing).into(),
                                widget::text::text(format!("{} px", self.spacing)).into(),
                            ])
                            .spacing(spacing.space_xxs),
                        ),
                ),
        )
        .into()
    }

    pub fn update(&mut self, message: TweakMessage) -> Task<crate::app::Message> {
        let Some(panel_helper) = &mut self.panel_helper else {
            return cosmic::Task::none();
        };
        let Some(panel_config) = &mut self.panel_config else {
            return cosmic::Task::none();
        };

        match message {
            TweakMessage::SetPanelPadding(padding) => {
                self.padding = padding;
                let update = panel_config.set_padding(panel_helper, self.padding);
                if let Err(err) = update {
                    log::error!("Error updating panel padding: {}", err);
                }
            }
            TweakMessage::SetPanelSpacing(spacing) => {
                self.spacing = spacing;
                let update = panel_config.set_spacing(panel_helper, self.spacing);
                if let Err(err) = update {
                    log::error!("Error updating panel spacing: {}", err);
                }
            }
            TweakMessage::ForceIcons(force) => {
                let mut configs = self.cosmic_panel_button_config.configs.clone();
                if let Some(inner_config) = configs.get_mut("Panel") {
                    inner_config.force_presentation =
                        if force { Some(Override::Icon) } else { None };
                } else {
                    configs.insert(
                        "Panel".to_owned(),
                        IndividualConfig {
                            force_presentation: if force { Some(Override::Icon) } else { None },
                        },
                    );
                }

                if let Some(helper) = &self.cosmic_panel_button_config_helper {
                    let update = self.cosmic_panel_button_config.set_configs(helper, configs);
                    if let Err(err) = update {
                        log::error!("Error updating cosmic panel button configs: {}", err);
                    } else {
                        self.force_icons = force;
                    }
                }
            }
            TweakMessage::ShowPanel(show) => {
                if show {
                    if !self
                        .cosmic_panel_config
                        .entries
                        .iter()
                        .any(|e| e == "Panel")
                    {
                        let mut entries = self.cosmic_panel_config.entries.clone();
                        entries.push("Panel".to_owned());
                        if let Some(helper) = &self.cosmic_panel_config_helper {
                            let update = self.cosmic_panel_config.set_entries(helper, entries);
                            if let Err(err) = update {
                                log::error!("Error updating cosmic panel entries: {}", err);
                            } else {
                                self.show_panel = false;
                            }
                        }
                    }
                } else if let Some(i) = self
                    .cosmic_panel_config
                    .entries
                    .iter()
                    .position(|e| e == "Panel")
                {
                    let mut entries = self.cosmic_panel_config.entries.clone();
                    entries.remove(i);
                    if let Some(helper) = &self.cosmic_panel_config_helper {
                        let update = self.cosmic_panel_config.set_entries(helper, entries);
                        if let Err(err) = update {
                            log::error!("Error updating cosmic panel entries: {}", err);
                        } else {
                            self.show_panel = true;
                        }
                    }
                }
            }
            // Ignore any message that isn't for the Panel
            _ => {}
        }
        Task::none()
    }
}
