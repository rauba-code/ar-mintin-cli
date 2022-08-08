/*
 * main.rs -- Core application
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

extern crate ar_mintin;
extern crate clap;
extern crate crossterm;
extern crate ctrlc;

mod args;
mod cli;

use clap::Parser;
use std::io::prelude::*;

use ar_mintin::ent::{ProgressTable, TableEntry};
use ar_mintin::file;
use ar_mintin::sim;

fn warranty() {
    print!(
        "    AR-MINTIN v{} -- Įsiminimo programa / Memorising application
    Copyright (C) 2022 Arnoldas Rauba

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

    Press ENTER to begin
",
        env!("CARGO_PKG_VERSION")
    );
}

fn init() {
    ctrlc::set_handler(cli::interrupt).unwrap();

    print!(
        "    AR-MINTIN v{} -- Įsiminimo programa / Memorising application
    Copyright (C) 2022 Arnoldas Rauba

    This program is FREE SOFTWARE with ABSOLUTELY NO WARRANTY.
    Press 'L' for more information

    Press 'ENTER' to begin
",
        env!("CARGO_PKG_VERSION")
    );

    if cli::standby_ex() {
        warranty();
        cli::standby();
    }
}

fn main() {
    init();
    let args = args::Args::parse();
    cli::cls();
    let table: Vec<TableEntry> = file::load_table(&args.inpath);
    let ptable = if let Some(ppath) = args.progress.clone() {
        if match file::get_file_type(&ppath) {
            Some(pftype) => pftype.is_file(),
            None => false,
        } {
            ProgressTable::new_from_file(&table, &ppath)
        } else {
            ProgressTable::new(&table)
        }
    } else {
        ProgressTable::new(&table)
    };
    let interact = |msg: sim::UiMessage| {
        let lines = &mut std::io::stdin().lock().lines();
        match msg {
            sim::UiMessage::Assess(ent, ans) => {
                println!("    {}", ent.lhs);
                *ans = cli::readin(lines).unwrap();
            }
            sim::UiMessage::Display(ent) => {
                println!("    {}", ent.lhs);
                println!("    {}", ent.rhs);
                cli::standby();
            }
            sim::UiMessage::NotifyAssessment => {
                println!("=== SAVIKONTROLĖ ===");
                cli::standby();
            }
        }
    };
    sim::Simulation {
        pt: ptable,
        args: args.into(),
    }
    .simulate(&interact);
}
