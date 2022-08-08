/*
 * cli.rs -- Utility functions for manipulating the command line
 * Copyright (C) 2022 Arnoldas Rauba
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{stdout, Lines, StdinLock};
pub fn cls() {
    stdout()
        .lock()
        .execute(terminal::Clear(terminal::ClearType::FromCursorUp))
        .unwrap()
        .execute(cursor::MoveTo(0, 1))
        .unwrap();
}

pub fn standby() {
    use crossterm::event::{self, Event, KeyCode, KeyModifiers};
    use std::io::prelude::*;
    stdout().lock().execute(cursor::Hide).unwrap();
    stdout().flush().unwrap();
    terminal::enable_raw_mode().unwrap();

    loop {
        if let Some(lic) = if let Event::Key(event) = event::read().unwrap() {
            match (event.code, event.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    interrupt();
                    unreachable!()
                }
                (KeyCode::Enter, KeyModifiers::NONE) => break,
                (_, _) => None,
            }
        } else {
            None
        } {
            return lic;
        }
    }
    terminal::disable_raw_mode().unwrap();
    stdout().lock().execute(cursor::Show).unwrap();
    cls();
}

pub fn interrupt() {
    terminal::disable_raw_mode().unwrap();
    std::io::stdout().lock().execute(cursor::Show).unwrap();
    println!();
    println!("Viso gero!");
    std::process::exit(0);
}

pub fn standby_ex() -> bool {
    use crossterm::event::{self, Event, KeyCode, KeyModifiers};
    use std::io::prelude::*;
    stdout().lock().execute(cursor::Hide).unwrap();
    stdout().flush().unwrap();
    terminal::enable_raw_mode().unwrap();

    let a = || loop {
        if let Some(lic) = if let Event::Key(event) = event::read().unwrap() {
            match (event.code, event.modifiers) {
                (KeyCode::Char('l'), KeyModifiers::NONE) => Some(true),
                (KeyCode::Enter, KeyModifiers::NONE) => Some(false),
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    interrupt();
                    unreachable!()
                }
                (_, _) => None,
            }
        } else {
            None
        } {
            return lic;
        }
    };
    let lic = a();
    terminal::disable_raw_mode().unwrap();
    stdout().lock().execute(cursor::Show).unwrap();
    cls();
    lic
}

pub fn readin(lines: &mut Lines<StdinLock>) -> Option<String> {
    std::io::stdout()
        .lock()
        .execute(cursor::MoveRight(4))
        .unwrap();
    let r = lines.filter_map(|x| x.ok()).find(|y| !y.is_empty());
    cls();
    r
}
