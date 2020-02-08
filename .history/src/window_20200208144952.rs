pub mod window {
  pub fn print_line(
    stream: &mut termion::raw::RawTerminal<std::io::Stdout>,
    left_pad: u16,
    terminal_line_nb: u16,
    file_line_nb: u16,
    content: &str,
    cursor: &CursorPosition,
  ) {
    let line_nb_displayed = render_line_nb(&left_pad, &file_line_nb);
    write!(
        stream,
        "{}{}{}.{} {}{}",
        termion::cursor::Goto(1, terminal_line_nb + 1),
        color::Fg(color::Blue),
        line_nb_displayed,
        style::Reset,
        content,
        termion::cursor::Goto(left_pad + cursor.x + 2, cursor.y + 1),
    )
  ).unwrap();
}
  pub fn print_first_line(stream: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    write!(
        stream,
        "{}{}{}{}Rustor{}: ESC to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        color::Fg(color::Red),
        style::Bold,
        style::Reset,
        termion::cursor::Goto(1, 2)
    )
        .unwrap();
}


}