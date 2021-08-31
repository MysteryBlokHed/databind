/*
 * Databind - Expand the functionality of Minecraft Datapacks.
 * Copyright (C) 2021  Adam Thompson-Sharpe
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
 */
//! Contains type definitions used throughout the project
use super::compiler;
use std::collections::HashMap;

/// Maps tags to a vector of functions with that tag
///
/// ```rust
/// use databind::types::TagMap;
/// let tag_map: TagMap = TagMap::new();
/// ```
pub type TagMap = HashMap<String, Vec<String>>;
/// Maps names of macros to an instance of the macro struct
///
/// ```rust
/// use databind::types::GlobalMacros;
/// let global_macros: GlobalMacros = GlobalMacros::new();
/// ```
pub type GlobalMacros = HashMap<String, compiler::Macro>;
