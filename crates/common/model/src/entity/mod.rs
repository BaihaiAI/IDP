// Copyright 2022 BaihaiAI, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod cell;
pub mod notebook;

#[cfg(test)]
mod tests {

    use crate::entity::cell::Cell;
    use crate::entity::notebook::Notebook;

    #[test]
    fn serialize_and_deserialize_cell() {
        let cell = Cell::default();
        // dbg!(cell.clone());
        let cell_json = serde_json::to_string(&cell).unwrap();
        println!("{}", serde_json::to_string_pretty(&cell).unwrap());
        let cell = serde_json::from_str::<Cell>(&cell_json).unwrap();
        dbg!(cell);
    }
    #[test]
    fn serialize_and_deserialize_notebook() {
        let notebook = Notebook::default();
        // dbg!(notebook.clone());
        let notebook_json = serde_json::to_string_pretty(&notebook).unwrap();
        println!("{}", notebook_json);
        let notebook = serde_json::from_str::<Notebook>(&notebook_json).unwrap();
        dbg!(notebook);
    }
    #[test]
    fn notebook_get_set_field() {
        let mut notebook = Notebook::default();
        assert_eq!(notebook.path(), None);
        notebook.set_path("/store/test/path");
        dbg!(notebook.clone());
        assert_eq!(notebook.path(), Some("/store/test/path"))
    }
    #[test]
    fn test_uuid_to_string_cell_id() {
        // let cell_id = Uuid::new_v4();
        // dbg!(cell_id.to_string());
        let cell = Cell::default();
        let option = cell.id();
        dbg!(option);
    }
}
