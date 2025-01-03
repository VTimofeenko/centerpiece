use crate::plugin::utils::Plugin;

pub struct TempPlugin {
    components: sysinfo::Components,
    settings: crate::settings::Settings,
    entries: Vec<crate::model::Entry>,
}

impl Plugin for TempPlugin {
    fn new() -> Self {
        Self {
            components: sysinfo::Components::new(),
            settings: crate::settings::Settings::new(),
            entries: vec![],
        }
    }

    fn id() -> &'static str {
        "temp"
    }

    fn priority() -> u32 {
        40 // TODO: lower. Higher prio shows this plugin first
    }

    fn title() -> &'static str {
        "° Temperature"
    }

    fn update_timeout() -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(1))
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        log::warn!(target: "temperature", "Refreshing the temp list");
        self.components.refresh_list();
        self.entries.clear();

        let _ = &self
            .components
            .iter()
            // If one needs to find the component name, it will be in the logs
            .inspect(|component| {
                log::warn!(target: "temperature",
                    "Component {}: {}",
                    component.label(),
                    component.temperature())
            })
            // Filter only the ones allowed by settings
            .filter_map(|p| {
                self.settings
                    .plugin
                    .temperature
                    .components
                    .iter()
                    .find(|c| c.sysinfo_label == p.label())
                    .map(|c| (c, p))
            })
            // Create an entry by using the display_label from settings
            .for_each(
                |(f, c): (&crate::settings::TemperatureEntry, &sysinfo::Component)| {
                    let title = format!("{}: {} ℃ ", f.display_label, c.temperature());
                    self.entries.push(crate::model::Entry {
                        id: f.sysinfo_label.clone(),
                        title,
                        action: String::from(""),
                        meta: String::from("Resource Monitor Temp"),
                        command: None,
                    })
                },
            );
        Ok(())
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }
}
