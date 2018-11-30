use errors::*;
use models::application::modes::SelectMode;
use scribe::Workspace;
use scribe::buffer::Range;
use presenters::current_buffer_status_line_data;
use view::{Colors, StatusLineData, Style, View};

pub fn display(workspace: &mut Workspace, mode: &SelectMode, view: &mut View) -> Result<()> {
    let buffer_status = current_buffer_status_line_data(workspace);

    if let Some(buf) = workspace.current_buffer() {
        let selected_range = Range::new(mode.anchor, *buf.cursor.clone());

        // Draw the visible set of tokens to the terminal.
        view.draw_buffer(buf, Some(&[selected_range]), None, None)?;

        // Draw the status line.
        view.draw_status_line(&[
            StatusLineData {
                content: " SELECT ".to_string(),
                style: Style::Default,
                colors: Colors::SelectMode,
            },
            buffer_status
        ]);
    } else {
        // Wipe the slate clean.
        view.clear();

        // There's no buffer; clear the cursor.
        view.set_cursor(None);
    }

    // Render the changes to the screen.
    view.present();

    Ok(())
}
