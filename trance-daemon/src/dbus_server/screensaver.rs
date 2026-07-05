// SPDX-License-Identifier: MIT

use std::sync::Arc;
use crate::controller::{DaemonCommand, DaemonController};

pub struct ScreenSaverService {
    pub controller: Arc<DaemonController>,
}

#[zbus::interface(name = "org.freedesktop.ScreenSaver")]
impl ScreenSaverService {
    async fn inhibit(
        &self,
        application_name: &str,
        reason_for_inhibit: &str,
        #[zbus(header)] header: zbus::message::Header<'_>,
    ) -> zbus::fdo::Result<u32> {
        let sender = header.sender().ok_or_else(|| {
            zbus::fdo::Error::Failed("inhibit request missing D-Bus sender".into())
        })?;
        tracing::info!(
            "ScreenSaver: Inhibit requested by {} ({}): {}",
            sender,
            application_name,
            reason_for_inhibit
        );
        let cookie = self.controller.inhibitors.add(
            application_name.to_string(),
            reason_for_inhibit.to_string(),
            sender.to_owned(),
        );
        let _ = self
            .controller
            .command_tx
            .send(DaemonCommand::StopPresentation);
        self.controller.mark_dirty();
        Ok(cookie)
    }

    async fn un_inhibit(
        &self,
        cookie: u32,
        #[zbus(header)] header: zbus::message::Header<'_>,
    ) -> zbus::fdo::Result<()> {
        let sender = header.sender().ok_or_else(|| {
            zbus::fdo::Error::Failed("un_inhibit request missing D-Bus sender".into())
        })?;
        tracing::info!(
            "ScreenSaver: UnInhibit requested by {} for cookie {}",
            sender,
            cookie
        );
        if !self.controller.inhibitors.remove_for_client(cookie, sender) {
            return Err(zbus::fdo::Error::Failed(format!(
                "unknown inhibit cookie for caller: {cookie}"
            )));
        }
        self.controller.mark_dirty();
        Ok(())
    }

    async fn simulate_user_activity(&self) {
        tracing::info!("ScreenSaver: SimulateUserActivity requested");
        let _ = self
            .controller
            .command_tx
            .send(DaemonCommand::StopPresentation);
    }

    async fn get_active(&self) -> bool {
        let active = self.controller.status.lock().unwrap().presentation_active;
        tracing::debug!("ScreenSaver: GetActive requested: {}", active);
        active
    }

    async fn set_active(&self, active: bool) {
        tracing::info!("ScreenSaver: SetActive requested: {}", active);
        if active {
            let saver = self
                .controller
                .config
                .lock()
                .unwrap()
                .active_saver
                .clone()
                .unwrap_or_else(|| "beams".to_string());
            let _ = self
                .controller
                .command_tx
                .send(DaemonCommand::Preview(saver));
        } else {
            let _ = self
                .controller
                .command_tx
                .send(DaemonCommand::StopPresentation);
        }
        self.controller.mark_dirty();
    }

    async fn lock(&self) {
        tracing::info!("ScreenSaver: Lock requested");
        let _ = self
            .controller
            .command_tx
            .send(DaemonCommand::StopPresentation);
    }
}
