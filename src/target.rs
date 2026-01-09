use crate::args::{MatchArgs, OpenArgs, Target};

use hyprland::{
    data::{Client, Clients},
    dispatch::Dispatch,
    error::HyprError,
    shared::{HyprData as _, HyprDataVec as _},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("hyprland error")]
    Hyprland(#[from] HyprError),

    #[error("no match found")]
    NoMatchFound,
}

enum ClientTarget {
    InitialClass(MatchArgs),
    Class(MatchArgs),
    InitialTitle(MatchArgs),
    Title(MatchArgs),
}

impl MatchArgs {
    fn matches(&self, content: &str) -> bool {
        let content = match self.mode {
            crate::args::MatchMode::CaseSensitive => content,
            crate::args::MatchMode::Insensitive => &content.to_lowercase(),
        };

        let matcher = match self.mode {
            crate::args::MatchMode::CaseSensitive => &self.matcher,
            crate::args::MatchMode::Insensitive => &self.matcher.to_lowercase(),
        };

        match self.area {
            crate::args::MatchArea::Exact => content == matcher,
            crate::args::MatchArea::Contains => content.contains(matcher),
            crate::args::MatchArea::StartsWith => content.starts_with(matcher),
            crate::args::MatchArea::EndsWith => content.ends_with(matcher),
        }
    }
}

impl ClientTarget {
    fn find(&self) -> Result<Vec<Client>, HyprError> {
        let clients = Clients::get()?.to_vec();
        tracing::debug!(?clients, "got clients");

        let mut filtered: Vec<_> = clients
            .into_iter()
            .filter(|client| {
                let (match_args, content) = match self {
                    ClientTarget::InitialClass(match_args) => (match_args, &client.initial_class),
                    ClientTarget::Class(match_args) => (match_args, &client.class),
                    ClientTarget::InitialTitle(match_args) => (match_args, &client.initial_title),
                    ClientTarget::Title(match_args) => (match_args, &client.title),
                };
                match_args.matches(content)
            })
            .collect();

        filtered.sort_by_key(|c| -c.focus_history_id);

        Ok(filtered)
    }
}

impl OpenArgs {
    fn exec(&self) -> Result<(), HyprError> {
        let command = match self {
            OpenArgs::Start(command_args) => &command_args.command,
        };
        Dispatch::call(hyprland::dispatch::DispatchType::Exec(command))
    }
}

fn focus_client(client: Client) -> Result<(), HyprError> {
    Dispatch::call(hyprland::dispatch::DispatchType::FocusWindow(
        hyprland::dispatch::WindowIdentifier::Address(client.address),
    ))
}

impl Target {
    pub fn execute(&self) -> Result<(), Error> {
        let client_target = self.as_client_target();
        let clients = client_target.find()?;
        if let Some(next_client) = clients.into_iter().next() {
            // lets try to focus it
            if next_client.focus_history_id == 0 {
                tracing::info!(
                    ?next_client,
                    "app only has one client, and it is already focused"
                );
            } else {
                tracing::info!(?next_client, "focusing client");
                focus_client(next_client)?;
            }
        } else {
            // no match, are we configured to open anything?
            let open_args = self.as_open_args().ok_or(Error::NoMatchFound)?;

            tracing::info!(?open_args, "opening app");

            open_args.exec()?;
        }
        Ok(())
    }

    fn as_client_target(&self) -> ClientTarget {
        match self {
            Target::InitialClass(target_args) => {
                ClientTarget::InitialClass(target_args.matcher.clone())
            }
            Target::Class(target_args) => ClientTarget::Class(target_args.matcher.clone()),
            Target::InitialTitle(target_args) => {
                ClientTarget::InitialTitle(target_args.matcher.clone())
            }
            Target::Title(target_args) => ClientTarget::Title(target_args.matcher.clone()),
        }
    }

    fn as_open_args(&self) -> Option<&OpenArgs> {
        match self {
            Target::InitialClass(target_args) => target_args.starter.as_ref(),
            Target::Class(target_args) => target_args.starter.as_ref(),
            Target::InitialTitle(target_args) => target_args.starter.as_ref(),
            Target::Title(target_args) => target_args.starter.as_ref(),
        }
    }
}
