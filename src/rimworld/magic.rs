use anyhow::Context;
use encoding_rs_io::DecodeReaderBytes;
use glob::glob;
use serde::de::value;
use std::{
    clone,
    collections::{BTreeMap, HashMap},
    convert,
    fs::File,
    io::BufReader,
    string,
};

use schema_analysis::{context::SequenceContext, Field, FieldStatus, InferredSchema, Schema};

use crate::{anydef::AnyDef, utils::{self, identity, to_map_by}};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
struct Defs {
    //#[serde(flatten)] defs: HashMap<String, Vec<Def>>,
    //#[serde(with = "serde_with::rust::tuple_list_as_map", flatten)]
    #[serde(rename = "$value", default)]
    defs: Vec<AnyDef>,
}

serde_conv!(
    BoolAsString,
    bool,
    |x: &bool| ::std::string::ToString::to_string(x),
    |x: ::std::string::String| x.to_ascii_lowercase().parse()
);

serde_conv!(
    StringAsLeaf,
    Option<String>,
    |x: &Option<String>| Leaf::new(x.clone()),
    |x: Leaf<String>| -> Result<_, std::convert::Infallible> { Ok(Some(x.value())) }
);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
enum Leaf<T> {
    Value(T),
    Leaf {
        #[serde(rename = "$value")]
        value: T,
    },
}

impl<T> Leaf<T> {
    fn new(value: T) -> Self {
        Self::Value(value)
    }

    fn value(self) -> T {
        match self {
            Leaf::Value(value) => value,
            Leaf::Leaf { value } => value,
        }
    }
}

//type Def = DumbValue;
#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Def {
    #[serde(default, rename = "Class")]
    pub class: Option<String>,

    ///The name of this Def. It is used as an identifier by the game code.
    #[serde(
        default,
        rename = "ParentName",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_name: Option<String>,

    ///A human-readable label used to identify this in game.
    #[serde(default, rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// A human-readable description given when the Def is inspected by players.
    #[serde(
        default,
        rename = "description",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        default,
        skip_serializing,
        with = "serde_with::rust::tuple_list_as_map",
        rename = "descriptionHyperlinks",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub description_hyperlinks: Vec<(String, String)>,

    // #[serde(
    //     //deserialize_with = "serde_with::rust::tuple_list_as_map::deserialize",
    //     rename = "$unflatten=descriptionHyperlinks",
    //     skip_serializing_if = "HashMap::is_empty"
    // )]
    // pub description_hyperlinks: HashMap<String, String>,
    /// Disables config error checking. Intended for mod use. (Be careful!)"
    #[serde(
        default,
        deserialize_with = "BoolAsString::deserialize",
        rename = "ignoreConfigErrors",
        skip_serializing_if = "std::ops::Not::not"
    )]
    pub ignore_config_errors: bool,

    #[serde(
        default,
        deserialize_with = "BoolAsString::deserialize",
        rename = "Abstract",
        skip_serializing_if = "std::ops::Not::not"
    )]
    pub is_abstract: bool,

    // #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    // /// the name
    // pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "StringAsLeaf::deserialize",
        rename = "Name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(
        default,
        deserialize_with = "StringAsLeaf::deserialize",
        rename = "defName",
        skip_serializing_if = "Option::is_none"
    )]
    pub def_name: Option<String>,

    // #[serde(default, alias = "defName", alias = "Name")]
    // pub name: String,

    //#[merge(strategy = merge_object)]
    #[serde(default, flatten, with = "serde_with::rust::tuple_list_as_map")]
    extra: Vec<(String, DumbValue)>,
}

use serde_with::OneOrMany;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum DumbValue {
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(#[serde(deserialize_with = "BoolAsString::deserialize")] bool),

    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Integer(#[serde(deserialize_with = "serde_with::rust::display_fromstr::deserialize")] isize),
    Float(#[serde(deserialize_with = "serde_with::rust::display_fromstr::deserialize")] f64),

    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),

    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<DumbValue>),

    // Li {
    //     #[serde_as(deserialize_as = "OneOrMany<_>")]
    //     #[serde(rename = "$unflatten=li")]
    //     li: Vec<Value>,
    // },
    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(#[serde(with = "serde_with::rust::tuple_list_as_map")] Vec<(String, DumbValue)>),
}

fn remove_inherit(value: DumbValue) -> DumbValue {
    if let DumbValue::Object(values) = value {
        DumbValue::Object(values.into_iter().filter(|(name, _)| name != "Inherit" && name != "MayRequire").collect())
    } else {
        value
    }
}

fn de_li(value: DumbValue) -> DumbValue {
    if let DumbValue::Object(values) = value {
        if values.iter().all(|(name, _value)| name == "li") {
            let values = values.into_iter().map(|(_name, value)| value);
            DumbValue::Array(values.collect())
        } else {
            DumbValue::Object(values)
        }
    } else {
        value
    }
}

fn raise_value(value: DumbValue) -> DumbValue {
    if let DumbValue::Object(mut values) = value {
        if values.len() == 1 && values.iter().all(|(name, _)| name == "$value") {
            let (_, value) = values.remove(0);
            value
        } else {
            DumbValue::Object(values)
        }
    } else {
        value
    }
}

fn simplfiy_value(value: DumbValue) -> DumbValue {
    let value = remove_inherit(value);
    let value = de_li(value);
    let value = raise_value(value);
    match value {
        DumbValue::Array(values) => {
            DumbValue::Array(values.into_iter().map(simplfiy_value).collect())
        }
        DumbValue::Object(values) => DumbValue::Object(simplfiy_values(values)),
        _ => value,
    }
}

fn simplfiy_values(values: Vec<(String, DumbValue)>) -> Vec<(String, DumbValue)> {
    values
        .into_iter()
        .map(|(name, value)| (name, simplfiy_value(value)))
        .collect()
}

fn load_rimworld_defs() -> anyhow::Result<Defs> {
    let mut output = Defs::default();
    let files = glob("/mnt/rimworld/Defs/**/*.xml").context("cant find rimworld def files")?
    //.skip(1).take(1)
    ;
    for entry in files {
        let context = format!("loading def {:?}", entry);
        //println!("loading def {:?}", entry);
        let path = entry?;
        let file = File::open(&path)?;
        let reader = DecodeReaderBytes::new(file);
        let reader = BufReader::new(reader);

        let defs: Defs = quick_xml::de::from_reader(reader).context(context)?;
        //println!("{} {}",defs.len(), serde_json::to_string_pretty(&defs)?);
        let mut defs = defs
            .defs
            .into_iter()
            .filter(AnyDef::is_known)
            .map(|mut anydef| {
                let class = anydef.class().to_string();
                let def = anydef.def_mut();
                def.class = Some(class);
                def.extra = simplfiy_values(def.extra.clone());
                // if def.name.is_some() && def.def_name.is_some(){
                //     assert_eq!(def.name, def.def_name);
                // }
                // if def.name.is_none() {
                //     def.name = def.def_name;
                // }
                // def.def_name = None;

                // let extra: HashMap<_, _> = def.extra.iter().map(clone::Clone::clone).collect();
                // assert!(!extra.contains_key("defName"));
                //(class, def)
                anydef
            })
            .collect();
        output.defs.append(&mut defs);
    }
    println!("finished loading");

    Ok(output)
}

fn handle_arrays(schema: Schema) -> Schema {
    if let Schema::Struct { fields, context } = schema {
        let fields = fields
            .into_iter()
            .map(|(name, field)| {
                let field = if field.status.may_be_duplicate || name == "li" {
                    let context = SequenceContext::default();
                    let status = FieldStatus {
                        may_be_duplicate: false,
                        may_be_missing: true,
                        ..field.status
                    };
                    let schema = Some(Schema::Sequence {
                        field: Box::new(field),
                        context,
                    });

                    Field { schema, status }
                } else {
                    field
                };
                (name, field)
            })
            .collect();
        Schema::Struct { fields, context }
    } else {
        schema
    }
}

fn simplifiy_schema(schema: Schema) -> Schema {
    //let input = schema.clone();
    let schema = handle_arrays(schema);
    match schema {
        Schema::Sequence { mut field, context } => {
            field.schema = field.schema.map(simplifiy_schema);
            // let field = Field {
            //     schema: field.schema.map(simplifiy_schema),
            //     status: field.status,
            // };
            Schema::Sequence { field, context }
        }
        Schema::Struct {
            mut fields,
            context,
        } => {
            if fields.keys().eq(["$value"]) {
                let ouput = fields
                    .remove("$value")
                    .expect("i just saw you")
                    .schema
                    .expect("seriously?");
                //println!("doing it {:?} => {:?}", input, ouput);
                simplifiy_schema(ouput)
            } else {
                // if fields.keys().len() == 1 {
                //     println!("{:#?}", fields.keys());
                // }
                let fields = fields
                    .into_iter()
                    .map(|(name, field)| {
                        (
                            name,
                            Field {
                                schema: field.schema.map(simplifiy_schema),
                                status: field.status,
                            },
                        )
                    })
                    .collect();
                Schema::Struct { fields, context }
            }
        }
        Schema::Union { variants } => Schema::Union {
            variants: variants.into_iter().map(simplifiy_schema).collect(),
        },
        _ => schema,
    }
}

#[test]
fn infer_schema() -> anyhow::Result<()> {
    let defs = load_rimworld_defs()?;

    //let value = serde_value::to_value(defs.clone())?;
    //let defs: Vec<Value> = defs.defs.into_iter().map(|(_kind, value)| value).collect();
    //let defs = utils::to_map_by(defs.defs, Clone::clone);
    //let data = include_str!("/mnt/rimworld/Defs/SkillDefs/Skills.xml").trim_start_matches("\u{feff}");

    // let file = File::open("/mnt/rimworld/Defs/SkillDefs/Skills.xml")?;
    //let reader = BufReader::new(file);
    //println!("{:#?}", defs.keys());

    //let map = &to_map_by(defs.defs, identity)["ThingDef"];
    let map = defs;

    let json = serde_json::to_string_pretty(&map)?;

    let xml = quick_xml::se::to_string(&map)?;
    let inferred: InferredSchema = serde_json::from_str(&json)?;

    //let dumb: DumbValue = quick_xml::de::from_str(&xml)?;

    std::fs::write("defs.json", json)?;

    std::fs::write("defs.xml", xml)?;
    let schema: Schema = simplifiy_schema(inferred.schema);

    let serialized_schema: String = serde_json::to_string_pretty(&schema)?;
    std::fs::write("schema.json", serialized_schema)?;

    //let jtg = schema.process_with_json_typegen(json_typegen::OutputMode::Rust)?;
    //std::fs::write("defs.rs", jtg);

    let schema = schema.to_json_schema_with_schemars()?;
    std::fs::write("json-schema.json", schema)?;

    Ok(())
}
