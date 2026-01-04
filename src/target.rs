use crate::args::{MatchArgs, OpenArgs, Target};

use hyprland::{
    data::{Client, Clients},
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
        tracing::info!(?clients, "got clients");

        let filtered = clients
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

        tracing::info!(?filtered, "filtered them");

        Ok(filtered)
    }
}

impl Target {
    pub fn execute(&self) -> Result<(), Error> {
        tracing::info!(?self, "hello");

        let client_target = self.as_client_target();
        let clients = client_target.find()?;
        if !clients.is_empty() {
            // lets try to focus it
            todo!()
        } else {
            // no match, are we configured to open anything?

            if let Some(open_args) = self.as_open_args() {
                // TODO: try to open
            } else {
                return Err(Error::NoMatchFound);
            }
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
