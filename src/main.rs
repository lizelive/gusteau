#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_with;

extern crate json_patch;

use data::{Def, Thing};
use json_patch::patch;

mod rimworld;

mod utils;

mod csxml;

use utils::*;
mod data;
mod num_string;

use std::{collections::{hash_set, HashMap, HashSet}, convert::{self, TryInto}, fs::File, hash::Hash, io::BufReader, iter::FromIterator, path::{Path, PathBuf}};

use anyhow::{bail, Context};
use glob::glob;
use serde_json::Value;
use xmltojson::to_json;

fn get_def_kinds(value: &serde_json::Value) -> Option<Vec<&String>> {
    let obj = value.as_object()?;
    let defs = obj.get("Defs")?.as_object()?;
    Some(defs.keys().collect())
}

// fn handle_def(defs: &mut HashMap<String, (String, Value)>, file: &Value) -> Option<Vec<Value>> {
//     let obj = file.as_object()?;
//     let mut out = Vec::new();
//     let file_defs = obj.get("Defs")?.as_object()?;
//     for (def_kind, kind_defs) in file_defs {
//         let kind_defs = match kind_defs {
//             Value::Array(kind_defs) => kind_defs.clone(),
//             Value::Object(def) => vec![Value::Object(def.clone())],
//             _ => return None,
//         };
//         for def in kind_defs {
//             let value = def.as_object_mut()?;
//             value.insert("@Class".to_string(), Value::String(def_kind.clone()));
//             let def_name = def.as_object()?.get("defName")?.as_str()?;
//             let def = (def_kind.clone(), def.clone());

//             defs.insert(def_name.to_string(), def);
//         }
//     }
//     Some(out);
// }

fn get_defs(file: &Value) -> anyhow::Result<HashMap<String, Vec<Value>>> {
    let mut out = Default::default();
    let defs = file
        .as_object()
        .context("file is not an object")?
        .get("Defs")
        .context("defs is missing")?;
    if let Some(defs) = defs.as_object() {
        for (def_kind, kind_defs) in defs {
            match kind_defs {
                Value::Array(defs) => {
                    for def in defs {
                        map_push(&mut out, def_kind.clone(), def.clone());
                    }
                }
                obj @ Value::Object(_) => {
                    map_push(&mut out, def_kind.clone(), obj.clone());
                }
                _ => assert_eq!(def_kind, "#text"),
            };
        }
    }
    Ok(out)
}



use merge::Merge;

use crate::{data::{Labor, Product, Recipe}};

fn overwrite<T>(left: &mut T, right: T) {
    *left = right;
}


fn main() -> anyhow::Result<()> {
    Ok(())
}

// fn convert_stuff() -> anyhow::Result<()> {
//     //do_dump()?;
//     let defs = flaten_defs()?;

//     // let json = serde_json::to_string_pretty(&defs)?;
//     // std::fs::write("full_defs.json", json)?;

//     let mut things = Vec::new();
//     let mut recipes = Vec::new();
//     let mut benches = Vec::new();
//     for thing_def in defs.get("ThingDef").unwrap_or(&Vec::new()) {
//         //let is_abstract = thing_def.is_abstract.as_deref().map(serde_json::from_str).unwrap_or(Ok(false));
//         //let is_abstract:bool = is_abstract?;
//         let is_abstract = thing_def.is_abstract == Some("True".to_string());

//         if !is_abstract {
//             let value = serde_json::to_value(thing_def)?;

//             let thing_def: ThingDef = serde_json::from_value(value)
//                 .with_context(|| format!("{:#?} can't be parsed", thing_def))?;
            
//             let name = thing_def
//                 .base
//                 .def_name
//                 .context("missing def_name aka title")?;

//             //let value = thing_def.stats.value.and_then(|v| v.as_f64());

//             let thing = Thing {
//                 def: Def {
//                     description: thing_def.base.description,
//                     name: name.clone(),
//                     title: thing_def.base.label,
//                 },
//             };


//             if let Some(recipe_maker) = thing_def.recipe_maker {
//                 println!("make recipe for {:#?} {:#?}", thing_def.stats, recipe_maker);

//                 let work = thing_def
//                     .stats
//                     .work_to_make
//                     .context(name)
//                     .context("using recipe_maker requires WorkToMake")?
//                     .as_f64()
//                     .expect("bad number");

//                 let recipe = Recipe {
//                     def: Def {
//                         name: format!("Make_{}", thing.name()),
//                         description: Some(format!("Craft {}.", thing.title())),
//                         title: Some(format!("make {}", thing.title())),
//                     },
//                     labor: Labor {
//                         amount: work,
//                         skill_stat: recipe_maker.skill.context("missing skill")?, //GeneralLaborSpeed
//                         speed_stat: recipe_maker.speed_stat.context("missing speed_stat")?,
//                     },
//                     products: vec![Product {
//                         amount: 1.0,
//                         thing: thing.name().clone(),
//                     }],
//                     ingredient: vec![],
//                 };
//                 if let Some(recipe_users) = recipe_maker.recipe_users {}
//                 recipes.push(recipe);
//             }

//             things.push(thing);
//         }
//     }
//     let world = data::World {
//         things,
//         recipes,
//         benches,
//     };

//     let json = serde_json::to_string_pretty(&world)?;
//     std::fs::write("data.json", json)?;

//     Ok(())
// }

// fn flaten_defs() -> Result<HashMap<String, Vec<GenericDef>>, anyhow::Error> {
//     let file = File::open("defs.json")?;
//     let rdr = BufReader::new(file);
//     let mut defs: HashMap<String, Vec<GenericDef>> = serde_json::from_reader(rdr)?;
//     for defs in defs.values_mut() {
//         let mut defs_by_parent = to_map_by(defs, |def| (def.parent_name.clone(), def.clone()));
//         // println!("defs_by_parent {:#?}", defs_by_parent.keys());
//         let mut to_propegate: Vec<Option<&GenericDef>> = vec![None];

//         while let Some(parent_completed) = to_propegate.pop() {
//             let parent_name = parent_completed.and_then(|parent| parent.name.clone());
//             //println!("parent_completed {:?}", &parent_name);
//             if let Some(to_update) = defs_by_parent.remove(&parent_name) {
//                 if let Some(parent) = parent_completed {
//                     for def in to_update {
//                         let has_name = def.name.is_some();
//                         let mut out = parent.clone();
//                         out.merge(def.clone());

//                         if has_name {
//                             println!("apply {:?} + {:?} = {:?}", parent_name, def, out);
//                         }

//                         *def = out;

//                         if has_name {
//                             //println!("prop {:?} -> {:?}", parent_name, &def.name);
//                             to_propegate.push(Some(def))
//                         }
//                     }
//                 } else {
//                     // no changes are needed, but need to propegate
//                     for def in to_update {
//                         //println!("apply {:?} + {:?} = {:?}", parent_name, def.name, def.extra.get("defName"));
//                         let has_name = def.name.is_some();
//                         if has_name {
//                             println!("apply {:?} + {:?} = {:?}", parent_name, def, def);
//                         }
//                         if has_name {
//                             //println!("prop {:?} -> {:?}", parent_name, &def.name);
//                             to_propegate.push(Some(def))
//                         }
//                     }
//                 }
//             }
//         }
//         // println!("orphans {:?}", defs_by_parent.keys());
//     }
//     Ok(defs)
// }
