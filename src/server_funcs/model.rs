use std::str::FromStr;

use anyhow::anyhow;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use surrealdb::types::SurrealValue;

// at some point we may move to a more dynamic permissions system
// with plugins in the future, but for right now this is enough.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "server", derive(SurrealValue))]
pub struct AccountPermissions(u8);

bitflags! {
    impl AccountPermissions: u8 {
        /// The ability to view and respond to support tickets.
        const ANSWER_TICKETS = 1 << 0;
        
        /// The ability to close/archive support tickets.
        /// In most cases, this requires [`ANSWER_TICKETS`][Self::ANSWER_TICKETS] to function properly.
        const CLOSE_TICKETS = 1 << 1;

        /// The ability to manage others' forum threads.
        /// This includes locking 
        const MANAGE_FORUM_THREADS = 1 << 2;

        /// The ability to see roles that should normally be hidden.
        /// This effectively disables pruning of hidden roles from server -> client.
        const SEE_HIDDEN_ROLES = 1 << 3;

        /// The ability to manage global settings, i.e. configure
        /// the entire server. This perm can be used to obtain any
        /// other perms, so be careful about assigning it.
        const MANAGE_GLOBAL_SETTINGS = 1 << 4;
    }
}

impl FromStr for AccountPermissions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "Token::AnswerTickets" => Ok(Self::ANSWER_TICKETS),
            "Token::CloseTickets" => Ok(Self::CLOSE_TICKETS),
            "Token::ManageForumThreads" => Ok(Self::MANAGE_FORUM_THREADS),
            "Token::SeeHiddenRoles" => Ok(Self::SEE_HIDDEN_ROLES),
            "Token::ManageGlobalSettings" => Ok(Self::MANAGE_GLOBAL_SETTINGS),
            _ => Err(anyhow!("Invalid permission token '{s}'"))
        }
    }
}

/// The visibility of a role as a badge on accounts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(SurrealValue))]
#[serde(tag = "vis")]
pub enum RoleVisibility {
    /// The role shows up as a badge on accounts.
    Visible {
        /// The name shown on the badge
        display_name: String,

        /// The color of the badge and the user's name.
        display_color: String,
    },

    /// A hidden role that will not be visible to frontend users.
    #[default]
    Hidden,
}

impl RoleVisibility {
    pub fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden)
    }
}

/// A global role in the server.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SurrealValue))]
pub struct Role {
    /// The identifier of the role. This will be a name with specific naming contraints.
    pub id: String,

    /// The visibility of the role.
    #[serde(flatten)]
    pub vis: RoleVisibility,

    /// The permissions associated with the role.
    pub perms: AccountPermissions,

    /// Represents whether this role can be modified by server admins.
    pub readonly: bool,
}

/// A newtype wrapping a collection of roles,
/// ordered by visibility priority (lower index = higher priority)
#[derive(Debug)]
#[cfg_attr(feature = "server", derive(SurrealValue))]
pub struct Roles(pub Vec<Role>);

impl Roles {
    /// The total permissions inherited by a user with these roles.
    pub fn total_permissions(&self) -> AccountPermissions {
        self.0.iter().fold(AccountPermissions::empty(), |acc, r| acc | r.perms)
    }

    /// The highest-priority visibility flag, or [`Hidden`][RoleVisibility::Hidden] if no roles are visible.
    pub fn priority_display(&self) -> RoleVisibility {
        self.0.iter()
            .find(|r| !r.vis.is_hidden())
            .map(|r| r.vis.clone())
            .unwrap_or_default()
    }
}

