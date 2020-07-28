pub mod diff_ignore_whitespace_setting;
pub mod diff_show_whitespace_setting;
pub mod git_config;
pub mod key_bindings;
pub mod theme;
mod utils;

use crate::config::diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting;
use crate::config::diff_show_whitespace_setting::DiffShowWhitespaceSetting;
use crate::config::git_config::GitConfig;
use crate::config::key_bindings::KeyBindings;
use crate::config::theme::Theme;
use crate::config::utils::{
	get_bool,
	get_diff_ignore_whitespace,
	get_diff_show_whitespace,
	get_string,
	get_unsigned_integer,
	open_git_config,
};

#[derive(Clone, Debug)]
pub struct Config {
	pub(crate) auto_select_next: bool,
	pub(crate) diff_ignore_whitespace: DiffIgnoreWhitespaceSetting,
	pub(crate) diff_show_whitespace: DiffShowWhitespaceSetting,
	pub(crate) diff_tab_width: u32,
	pub(crate) diff_tab_symbol: String,
	pub(crate) diff_space_symbol: String,
	pub(crate) git: GitConfig,
	pub(crate) key_bindings: KeyBindings,
	pub(crate) theme: Theme,
}

impl Config {
	pub(crate) fn new() -> Result<Self, String> {
		let git_config = open_git_config()?;

		Ok(Config {
			auto_select_next: get_bool(&git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			diff_ignore_whitespace: get_diff_ignore_whitespace(&git_config)?,
			diff_show_whitespace: get_diff_show_whitespace(&git_config)?,
			diff_tab_width: get_unsigned_integer(&git_config, "interactive-rebase-tool.diffTabWidth", 4)?,
			diff_tab_symbol: get_string(&git_config, "interactive-rebase-tool.diffTabSymbol", "→")?,
			diff_space_symbol: get_string(&git_config, "interactive-rebase-tool.diffSpaceSymbol", "·")?,
			git: GitConfig::new(&git_config)?,
			key_bindings: KeyBindings::new(&git_config)?,
			theme: Theme::new(&git_config)?,
		})
	}
}
