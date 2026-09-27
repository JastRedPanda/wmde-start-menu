use std::process;

#[derive(Clone, Debug, PartialEq)]
pub struct SystemTool {
    exec: &'static str,
    args: &'static [&'static str],
}

impl SystemTool {
    // WMDE: the applet's own page in Settings. The standalone settings window is gone; the
    // page is built from res/applet-settings/fun.wmde.start-menu.ron.
    pub const APPLET_SETTINGS: SystemTool = Self {
        exec: "wmde-settings",
        args: &["applet", "fun.wmde.start-menu"],
    };
    pub const SYSTEM_SETTINGS: SystemTool = Self {
        exec: "wmde-settings",
        args: &[],
    };
    pub const SYSTEM_MONITOR: SystemTool = Self {
        exec: "wmde-taskmgr",
        args: &[],
    };
    pub const DISK_MANAGEMENT: SystemTool = Self {
        exec: "gnome-disks",
        args: &[],
    };

    pub fn perform(&self) {
        self.handle_generic_tool();
    }

    fn handle_generic_tool(&self) {
        let is_flatpak = std::env::var("FLATPAK_ID").is_ok();

        // Logic to determine the final command and arguments, centralizing Flatpak handling
        let (main_exec, args) = if is_flatpak {
            // For Flatpak, use `flatpak-spawn` with the `--host` argument
            let command = std::iter::once(self.exec)
                .chain(self.args.iter().copied())
                .collect::<Vec<_>>()
                .join(" ");
            (
                "flatpak-spawn",
                vec![
                    "--host".to_owned(),
                    "/bin/sh".to_owned(),
                    "-l".to_owned(),
                    "-c".to_owned(),
                    command,
                ],
            )
        } else {
            // For native, use the direct executable name
            (self.exec, self.args.iter().map(|arg| (*arg).to_owned()).collect())
        };

        // Execute the command and provide better error reporting
        if let Err(e) = process::Command::new(main_exec).args(args).spawn() {
            log::error!("Error launching tool '{}': {}", main_exec, e);
        }
    }
}
