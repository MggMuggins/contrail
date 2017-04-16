use ansi_term::Color;
use config::Config;
use clap::Shell;

use utils::{ConvertError, FormatResult};

use modules;

pub fn format_prompt(c: &Config,
                     exit_code: u8,
                     next_bg: Option<Color>,
                     shell: Shell)
                     -> Result<FormatResult, ConvertError> {
    let mut options = modules::read_options("prompt", c)?;

    let style_success = modules::read_style("modules.prompt.style_success", c)?;
    let style_error = modules::read_style("modules.prompt.style_error", c)?;

    // A command exited successfully if and only if the exit code is 0
    if exit_code == 0 {
        options.style = style_success;
    } else {
        options.style = style_error;
    }

    let format_result = FormatResult {
        output: Some(modules::format_for_module("$", &options, next_bg, shell)),
        next_bg: options.style.background,
    };

    Ok(format_result)
}
