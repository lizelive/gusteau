#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(default)]
pub struct Defs {
    #[serde(rename = "AbilityDef")]
    pub ability_defs: List<AbilityDef>,
    #[serde(rename = "ApparelLayerDef")]
    pub apparel_layer_defs: List<ApparelLayerDef>,
    #[serde(rename = "BeardDef")]
    pub beard_defs: List<BeardDef>,
    #[serde(rename = "BillRepeatModeDef")]
    pub bill_repeat_mode_defs: List<BillRepeatModeDef>,
    #[serde(rename = "BillStoreModeDef")]
    pub bill_store_mode_defs: List<BillStoreModeDef>,
    #[serde(rename = "BiomeDef")]
    pub biome_defs: List<BiomeDef>,
    #[serde(rename = "BodyDef")]
    pub body_defs: List<BodyDef>,
    #[serde(rename = "BodyPartDef")]
    pub body_part_defs: List<BodyPartDef>,
    #[serde(rename = "BodyPartGroupDef")]
    pub body_part_group_defs: List<BodyPartGroupDef>,
    #[serde(rename = "BodyPartTagDef")]
    pub body_part_tag_defs: List<BodyPartTagDef>,
    #[serde(rename = "BodyTypeDef")]
    pub body_type_defs: List<BodyTypeDef>,
    #[serde(rename = "ChemicalDef")]
    pub chemical_defs: List<ChemicalDef>,
    #[serde(rename = "ClamorDef")]
    pub clamor_defs: List<ClamorDef>,
    #[serde(rename = "ConceptDef")]
    pub concept_defs: List<ConceptDef>,
    #[serde(rename = "CultureDef")]
    pub culture_defs: List<CultureDef>,
    #[serde(rename = "DamageArmorCategoryDef")]
    pub damage_armor_category_defs: List<DamageArmorCategoryDef>,
    #[serde(rename = "DamageDef")]
    pub damage_defs: List<DamageDef>,
    #[serde(rename = "DesignationCategoryDef")]
    pub designation_category_defs: List<DesignationCategoryDef>,
    #[serde(rename = "DesignationDef")]
    pub designation_defs: List<DesignationDef>,
    #[serde(rename = "DesignatorDropdownGroupDef")]
    pub designator_dropdown_group_defs: List<DesignatorDropdownGroupDef>,
    #[serde(rename = "DifficultyDef")]
    pub difficulty_defs: List<DifficultyDef>,
    #[serde(rename = "DrugPolicyDef")]
    pub drug_policy_defs: List<DrugPolicyDef>,
    #[serde(rename = "DutyDef")]
    pub duty_defs: List<DutyDef>,
    #[serde(rename = "EffecterDef")]
    pub effecter_defs: List<EffecterDef>,
    #[serde(rename = "ExpansionDef")]
    pub expansion_defs: List<ExpansionDef>,
    #[serde(rename = "ExpectationDef")]
    pub expectation_defs: List<ExpectationDef>,
    #[serde(rename = "FactionDef")]
    pub faction_defs: List<FactionDef>,
    #[serde(rename = "FeatureDef")]
    pub feature_defs: List<FeatureDef>,
    #[serde(rename = "FleckDef")]
    pub fleck_defs: List<FleckDef>,
    #[serde(rename = "FleshTypeDef")]
    pub flesh_type_defs: List<FleshTypeDef>,
    #[serde(rename = "GameConditionDef")]
    pub game_condition_defs: List<GameConditionDef>,
    #[serde(rename = "GatheringDef")]
    pub gathering_defs: List<GatheringDef>,
    #[serde(rename = "GenStepDef")]
    pub gen_step_defs: List<GenStepDef>,
    #[serde(rename = "GoodwillSituationDef")]
    pub goodwill_situation_defs: List<GoodwillSituationDef>,
    #[serde(rename = "HairDef")]
    pub hair_defs: List<HairDef>,
    #[serde(rename = "HediffDef")]
    pub hediff_defs: List<HediffDef>,
    #[serde(rename = "HediffGiverSetDef")]
    pub hediff_giver_set_defs: List<HediffGiverSetDef>,
    #[serde(rename = "HibernatableStateDef")]
    pub hibernatable_state_defs: List<HibernatableStateDef>,
    #[serde(rename = "HistoryAutoRecorderDef")]
    pub history_auto_recorder_defs: List<HistoryAutoRecorderDef>,
    #[serde(rename = "HistoryAutoRecorderGroupDef")]
    pub history_auto_recorder_group_defs: List<HistoryAutoRecorderGroupDef>,
    #[serde(rename = "HistoryEventDef")]
    pub history_event_defs: List<HistoryEventDef>,
    #[serde(rename = "ImpactSoundTypeDef")]
    pub impact_sound_type_defs: List<ImpactSoundTypeDef>,
    #[serde(rename = "ImplementOwnerTypeDef")]
    pub implement_owner_type_defs: List<ImplementOwnerTypeDef>,
    #[serde(rename = "IncidentCategoryDef")]
    pub incident_category_defs: List<IncidentCategoryDef>,
    #[serde(rename = "IncidentDef")]
    pub incident_defs: List<IncidentDef>,
    #[serde(rename = "IncidentTargetTagDef")]
    pub incident_target_tag_defs: List<IncidentTargetTagDef>,
    #[serde(rename = "InspirationDef")]
    pub inspiration_defs: List<InspirationDef>,
    #[serde(rename = "InstructionDef")]
    pub instruction_defs: List<InstructionDef>,
    #[serde(rename = "InteractionDef")]
    pub interaction_defs: List<InteractionDef>,
    #[serde(rename = "InventoryStockGroupDef")]
    pub inventory_stock_group_defs: List<InventoryStockGroupDef>,
    #[serde(rename = "IssueDef")]
    pub issue_defs: List<IssueDef>,
    #[serde(rename = "JobDef")]
    pub job_defs: List<JobDef>,
    #[serde(rename = "JoyGiverDef")]
    pub joy_giver_defs: List<JoyGiverDef>,
    #[serde(rename = "JoyKindDef")]
    pub joy_kind_defs: List<JoyKindDef>,
    #[serde(rename = "KeyBindingCategoryDef")]
    pub key_binding_category_defs: List<KeyBindingCategoryDef>,
    #[serde(rename = "KeyBindingDef")]
    pub key_binding_defs: List<KeyBindingDef>,
    #[serde(rename = "LetterDef")]
    pub letter_defs: List<LetterDef>,
    #[serde(rename = "LifeStageDef")]
    pub life_stage_defs: List<LifeStageDef>,
    #[serde(rename = "LogEntryDef")]
    pub log_entry_defs: List<LogEntryDef>,
    #[serde(rename = "MainButtonDef")]
    pub main_button_defs: List<MainButtonDef>,
    #[serde(rename = "ManeuverDef")]
    pub maneuver_defs: List<ManeuverDef>,
    #[serde(rename = "MapGeneratorDef")]
    pub map_generator_defs: List<MapGeneratorDef>,
    #[serde(rename = "MeditationFocusDef")]
    pub meditation_focus_defs: List<MeditationFocusDef>,
    #[serde(rename = "MentalBreakDef")]
    pub mental_break_defs: List<MentalBreakDef>,
    #[serde(rename = "MentalStateDef")]
    pub mental_state_defs: List<MentalStateDef>,
    #[serde(rename = "MessageTypeDef")]
    pub message_type_defs: List<MessageTypeDef>,
    #[serde(rename = "NeedDef")]
    pub need_defs: List<NeedDef>,
    #[serde(rename = "OrderedTakeGroupDef")]
    pub ordered_take_group_defs: List<OrderedTakeGroupDef>,
    #[serde(rename = "PawnCapacityDef")]
    pub pawn_capacity_defs: List<PawnCapacityDef>,
    #[serde(rename = "PawnColumnDef")]
    pub pawn_column_defs: List<PawnColumnDef>,
    #[serde(rename = "PawnGroupKindDef")]
    pub pawn_group_kind_defs: List<PawnGroupKindDef>,
    #[serde(rename = "PawnKindDef")]
    pub pawn_kind_defs: List<PawnKindDef>,
    #[serde(rename = "PawnRelationDef")]
    pub pawn_relation_defs: List<PawnRelationDef>,
    #[serde(rename = "PawnTableDef")]
    pub pawn_table_defs: List<PawnTableDef>,
    #[serde(rename = "PawnsArrivalModeDef")]
    pub pawns_arrival_mode_defs: List<PawnsArrivalModeDef>,
    #[serde(rename = "PreceptDef")]
    pub precept_defs: List<PreceptDef>,
    #[serde(rename = "PrisonerInteractionModeDef")]
    pub prisoner_interaction_mode_defs: List<PrisonerInteractionModeDef>,
    #[serde(rename = "QuestScriptDef")]
    pub quest_script_defs: List<QuestScriptDef>,
    #[serde(rename = "RaidStrategyDef")]
    pub raid_strategy_defs: List<RaidStrategyDef>,
    #[serde(rename = "RecipeDef")]
    pub recipe_defs: List<RecipeDef>,
    #[serde(rename = "RecordDef")]
    pub record_defs: List<RecordDef>,
    #[serde(rename = "ResearchProjectDef")]
    pub research_project_defs: List<ResearchProjectDef>,
    #[serde(rename = "ResearchProjectTagDef")]
    pub research_project_tag_defs: List<ResearchProjectTagDef>,
    #[serde(rename = "ResearchTabDef")]
    pub research_tab_defs: List<ResearchTabDef>,
    #[serde(rename = "ReservationLayerDef")]
    pub reservation_layer_defs: List<ReservationLayerDef>,
    #[serde(rename = "RitualOutcomeEffectDef")]
    pub ritual_outcome_effect_defs: List<RitualOutcomeEffectDef>,
    #[serde(rename = "RitualPatternDef")]
    pub ritual_pattern_defs: List<RitualPatternDef>,
    #[serde(rename = "RitualVisualEffectDef")]
    pub ritual_visual_effect_defs: List<RitualVisualEffectDef>,
    #[serde(rename = "RiverDef")]
    pub river_defs: List<RiverDef>,
    #[serde(rename = "RoadDef")]
    pub road_defs: List<RoadDef>,
    #[serde(rename = "RoadPathingDef")]
    pub road_pathing_defs: List<RoadPathingDef>,
    #[serde(rename = "RoadWorldLayerDef")]
    pub road_world_layer_defs: List<RoadWorldLayerDef>,
    #[serde(rename = "RoofDef")]
    pub roof_defs: List<RoofDef>,
    #[serde(rename = "RoomRoleDef")]
    pub room_role_defs: List<RoomRoleDef>,
    #[serde(rename = "RoomStatDef")]
    pub room_stat_defs: List<RoomStatDef>,
    #[serde(rename = "RuleDef")]
    pub rule_defs: List<RuleDef>,
    #[serde(rename = "RulePackDef")]
    pub rule_pack_defs: List<RulePackDef>,
    #[serde(rename = "ScatterableDef")]
    pub scatterable_defs: List<ScatterableDef>,
    #[serde(rename = "ScenPartDef")]
    pub scen_part_defs: List<ScenPartDef>,
    #[serde(rename = "ScenarioDef")]
    pub scenario_defs: List<ScenarioDef>,
    #[serde(rename = "ShaderTypeDef")]
    pub shader_type_defs: List<ShaderTypeDef>,
    #[serde(rename = "SiteCoreDef")]
    pub site_core_defs: List<SiteCoreDef>,
    #[serde(rename = "SitePartDef")]
    pub site_part_defs: List<SitePartDef>,
    #[serde(rename = "SketchResolverDef")]
    pub sketch_resolver_defs: List<SketchResolverDef>,
    #[serde(rename = "SkillDef")]
    pub skill_defs: List<SkillDef>,
    #[serde(rename = "SongDef")]
    pub song_defs: List<SongDef>,
    #[serde(rename = "SoundDef")]
    pub sound_defs: List<SoundDef>,
    #[serde(rename = "SpecialThingFilterDef")]
    pub special_thing_filter_defs: List<SpecialThingFilterDef>,
    #[serde(rename = "StatCategoryDef")]
    pub stat_category_defs: List<StatCategoryDef>,
    #[serde(rename = "StatDef")]
    pub stat_defs: List<StatDef>,
    #[serde(rename = "StorytellerDef")]
    pub storyteller_defs: List<StorytellerDef>,
    #[serde(rename = "StuffAppearanceDef")]
    pub stuff_appearance_defs: List<StuffAppearanceDef>,
    #[serde(rename = "StuffCategoryDef")]
    pub stuff_category_defs: List<StuffCategoryDef>,
    #[serde(rename = "StyleItemCategoryDef")]
    pub style_item_category_defs: List<StyleItemCategoryDef>,
    #[serde(rename = "SubcameraDef")]
    pub subcamera_defs: List<SubcameraDef>,
    #[serde(rename = "TaleDef")]
    pub tale_defs: List<TaleDef>,
    #[serde(rename = "TattooDef")]
    pub tattoo_defs: List<TattooDef>,
    #[serde(rename = "TerrainAffordanceDef")]
    pub terrain_affordance_defs: List<TerrainAffordanceDef>,
    #[serde(rename = "TerrainDef")]
    pub terrain_defs: List<TerrainDef>,
    #[serde(rename = "ThingCategoryDef")]
    pub thing_category_defs: List<ThingCategoryDef>,
    #[serde(rename = "ThingDef")]
    pub thing_defs: List<ThingDef>,
    #[serde(rename = "ThingSetMakerDef")]
    pub thing_set_maker_defs: List<ThingSetMakerDef>,
    #[serde(rename = "ThinkTreeDef")]
    pub think_tree_defs: List<ThinkTreeDef>,
    #[serde(rename = "ThoughtDef")]
    pub thought_defs: List<ThoughtDef>,
    #[serde(rename = "TimeAssignmentDef")]
    pub time_assignment_defs: List<TimeAssignmentDef>,
    #[serde(rename = "TipSetDef")]
    pub tip_set_defs: List<TipSetDef>,
    #[serde(rename = "ToolCapacityDef")]
    pub tool_capacity_defs: List<ToolCapacityDef>,
    #[serde(rename = "TraderKindDef")]
    pub trader_kind_defs: List<TraderKindDef>,
    #[serde(rename = "TrainabilityDef")]
    pub trainability_defs: List<TrainabilityDef>,
    #[serde(rename = "TrainableDef")]
    pub trainable_defs: List<TrainableDef>,
    #[serde(rename = "TraitDef")]
    pub trait_defs: List<TraitDef>,
    #[serde(rename = "TransferableSorterDef")]
    pub transferable_sorter_defs: List<TransferableSorterDef>,
    #[serde(rename = "WeaponClassDef")]
    pub weapon_class_defs: List<WeaponClassDef>,
    #[serde(rename = "WeatherDef")]
    pub weather_defs: List<WeatherDef>,
    #[serde(rename = "WorkGiverDef")]
    pub work_giver_defs: List<WorkGiverDef>,
    #[serde(rename = "WorkGiverEquivalenceGroupDef")]
    pub work_giver_equivalence_group_defs: List<WorkGiverEquivalenceGroupDef>,
    #[serde(rename = "WorkTypeDef")]
    pub work_type_defs: List<WorkTypeDef>,
    #[serde(rename = "WorldGenStepDef")]
    pub world_gen_step_defs: List<WorldGenStepDef>,
    #[serde(rename = "WorldObjectDef")]
    pub world_object_defs: List<WorldObjectDef>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AbilityDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: bool,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "abilityClass")]
    pub ability_class: Option<String>,
    pub category: Option<String>,
    #[serde(default)]
    pub comps: Vec<Comp>,
    #[serde(rename = "disableGizmoWhileUndrafted")]
    pub disable_gizmo_while_undrafted: Option<bool>,
    #[serde(rename = "displayGizmoWhileUndrafted")]
    pub display_gizmo_while_undrafted: Option<bool>,
    #[serde(rename = "gizmoClass")]
    pub gizmo_class: Option<String>,
    #[serde(rename = "hotKey")]
    pub hot_key: Option<String>,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    #[serde(rename = "statBases")]
    pub stat_bases: Option<StatBases>,
    #[serde(rename = "targetRequired")]
    pub target_required: Option<bool>,
    #[serde(rename = "verbProperties")]
    pub verb_properties: VerbProperties,
    #[serde(rename = "warmupSound")]
    pub warmup_sound: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp {
    #[serde(rename = "Class")]
    pub class: String,
    pub capacity: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatBases {
    #[serde(rename = "Ability_Duration")]
    pub ability_duration: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct VerbProperties {
    pub range: ::serde_json::Value,
    #[serde(rename = "requireLineOfSight")]
    pub require_line_of_sight: Option<bool>,
    #[serde(rename = "targetParams")]
    pub target_params: Option<TargetParams>,
    #[serde(rename = "verbClass")]
    pub verb_class: String,
    #[serde(rename = "warmupTime")]
    pub warmup_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TargetParams {
    #[serde(rename = "canTargetAnimals")]
    pub can_target_animals: bool,
    #[serde(rename = "canTargetBuildings")]
    pub can_target_buildings: bool,
    #[serde(rename = "mapObjectTargetsMustBeAutoAttackable")]
    pub map_object_targets_must_be_auto_attackable: bool,
    #[serde(rename = "thingCategory")]
    pub thing_category: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApparelLayerDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "drawOrder")]
    pub draw_order: i64,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BeardDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub category: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    pub label: String,
    #[serde(rename = "offsetNarrowEast")]
    pub offset_narrow_east: Option<String>,
    #[serde(rename = "offsetNarrowSouth")]
    pub offset_narrow_south: Option<String>,
    #[serde(rename = "styleGender")]
    pub style_gender: Option<String>,
    #[serde(rename = "styleTags")]
    pub style_tags: Vec<String>,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BillRepeatModeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "listOrder")]
    pub list_order: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BillStoreModeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "listOrder")]
    pub list_order: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiomeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowFarmingCamps")]
    pub allow_farming_camps: Option<bool>,
    #[serde(rename = "allowRivers")]
    pub allow_rivers: Option<bool>,
    #[serde(rename = "allowRoads")]
    pub allow_roads: Option<bool>,
    #[serde(rename = "allowedPackAnimals")]
    #[serde(default)]
    pub allowed_pack_animals: Vec<String>,
    #[serde(rename = "animalDensity")]
    pub animal_density: Option<f64>,
    #[serde(rename = "baseWeatherCommonalities")]
    pub base_weather_commonalities: Option<BaseWeatherCommonalities>,
    #[serde(rename = "campSelectionWeight")]
    pub camp_selection_weight: Option<f64>,
    #[serde(rename = "canAutoChoose")]
    pub can_auto_choose: Option<bool>,
    #[serde(rename = "canBuildBase")]
    pub can_build_base: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "diseaseMtbDays")]
    pub disease_mtb_days: Option<i64>,
    #[serde(default)]
    pub diseases: Vec<Disease>,
    pub forageability: Option<::serde_json::Value>,
    #[serde(rename = "foragedFood")]
    pub foraged_food: Option<String>,
    #[serde(rename = "hasBedrock")]
    pub has_bedrock: Option<bool>,
    #[serde(rename = "hasVirtualPlants")]
    pub has_virtual_plants: Option<bool>,
    pub impassable: Option<bool>,
    #[serde(rename = "isExtremeBiome")]
    pub is_extreme_biome: Option<bool>,
    pub label: String,
    #[serde(rename = "movementDifficulty")]
    pub movement_difficulty: Option<::serde_json::Value>,
    #[serde(rename = "plantDensity")]
    pub plant_density: Option<::serde_json::Value>,
    #[serde(rename = "settlementSelectionWeight")]
    pub settlement_selection_weight: Option<::serde_json::Value>,
    #[serde(rename = "soundsAmbient")]
    #[serde(default)]
    pub sounds_ambient: Vec<String>,
    #[serde(rename = "terrainPatchMakers")]
    #[serde(default)]
    pub terrain_patch_makers: Vec<TerrainPatchMaker>,
    #[serde(rename = "terrainsByFertility")]
    #[serde(default)]
    pub terrains_by_fertility: Vec<TerrainsByFertility>,
    pub texture: String,
    #[serde(rename = "wildAnimals")]
    pub wild_animals: Option<WildAnimals>,
    #[serde(rename = "wildPlantRegrowDays")]
    pub wild_plant_regrow_days: Option<i64>,
    #[serde(rename = "wildPlants")]
    pub wild_plants: Option<WildPlants>,
    #[serde(rename = "wildPlantsCareAboutLocalFertility")]
    pub wild_plants_care_about_local_fertility: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BaseWeatherCommonalities {
    #[serde(rename = "Clear")]
    pub clear: i64,
    #[serde(rename = "DryThunderstorm")]
    pub dry_thunderstorm: ::serde_json::Value,
    #[serde(rename = "Fog")]
    pub fog: Option<i64>,
    #[serde(rename = "FoggyRain")]
    pub foggy_rain: Option<::serde_json::Value>,
    #[serde(rename = "Rain")]
    pub rain: i64,
    #[serde(rename = "RainyThunderstorm")]
    pub rainy_thunderstorm: ::serde_json::Value,
    #[serde(rename = "SnowGentle")]
    pub snow_gentle: ::serde_json::Value,
    #[serde(rename = "SnowHard")]
    pub snow_hard: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Disease {
    pub commonality: i64,
    #[serde(rename = "diseaseInc")]
    pub disease_inc: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TerrainPatchMaker {
    #[serde(rename = "maxFertility")]
    pub max_fertility: Option<f64>,
    #[serde(rename = "minSize")]
    pub min_size: Option<i64>,
    #[serde(rename = "perlinFrequency")]
    pub perlin_frequency: f64,
    pub thresholds: Vec<Threshold>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Threshold {
    pub max: ::serde_json::Value,
    pub min: f64,
    pub terrain: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TerrainsByFertility {
    pub max: ::serde_json::Value,
    pub min: ::serde_json::Value,
    pub terrain: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WildAnimals {
    #[serde(rename = "Alpaca")]
    pub alpaca: Option<f64>,
    #[serde(rename = "Bear_Grizzly")]
    pub bear_grizzly: Option<f64>,
    #[serde(rename = "Bear_Polar")]
    pub bear_polar: Option<f64>,
    #[serde(rename = "Bison")]
    pub bison: Option<f64>,
    #[serde(rename = "Boomalope")]
    pub boomalope: Option<f64>,
    #[serde(rename = "Boomrat")]
    pub boomrat: Option<f64>,
    #[serde(rename = "Capybara")]
    pub capybara: Option<f64>,
    #[serde(rename = "Caribou")]
    pub caribou: Option<::serde_json::Value>,
    #[serde(rename = "Cassowary")]
    pub cassowary: Option<f64>,
    #[serde(rename = "Chinchilla")]
    pub chinchilla: Option<f64>,
    #[serde(rename = "Cobra")]
    pub cobra: Option<f64>,
    #[serde(rename = "Cougar")]
    pub cougar: Option<f64>,
    #[serde(rename = "Deer")]
    pub deer: Option<f64>,
    #[serde(rename = "Donkey")]
    pub donkey: Option<f64>,
    #[serde(rename = "Dromedary")]
    pub dromedary: Option<f64>,
    #[serde(rename = "Elephant")]
    pub elephant: Option<f64>,
    #[serde(rename = "Elk")]
    pub elk: Option<::serde_json::Value>,
    #[serde(rename = "Emu")]
    pub emu: Option<f64>,
    #[serde(rename = "Fox_Arctic")]
    pub fox_arctic: Option<f64>,
    #[serde(rename = "Fox_Fennec")]
    pub fox_fennec: Option<f64>,
    #[serde(rename = "Fox_Red")]
    pub fox_red: Option<f64>,
    #[serde(rename = "Gazelle")]
    pub gazelle: Option<f64>,
    #[serde(rename = "GuineaPig")]
    pub guinea_pig: Option<f64>,
    #[serde(rename = "Hare")]
    pub hare: Option<::serde_json::Value>,
    #[serde(rename = "Horse")]
    pub horse: Option<f64>,
    #[serde(rename = "Ibex")]
    pub ibex: Option<::serde_json::Value>,
    #[serde(rename = "Iguana")]
    pub iguana: Option<f64>,
    #[serde(rename = "Lynx")]
    pub lynx: Option<f64>,
    #[serde(rename = "Megasloth")]
    pub megasloth: Option<f64>,
    #[serde(rename = "Monkey")]
    pub monkey: Option<i64>,
    #[serde(rename = "Muffalo")]
    pub muffalo: Option<::serde_json::Value>,
    #[serde(rename = "Ostrich")]
    pub ostrich: Option<f64>,
    #[serde(rename = "Panther")]
    pub panther: Option<f64>,
    #[serde(rename = "Raccoon")]
    pub raccoon: Option<f64>,
    #[serde(rename = "Rat")]
    pub rat: Option<::serde_json::Value>,
    #[serde(rename = "Rhinoceros")]
    pub rhinoceros: Option<f64>,
    #[serde(rename = "Snowhare")]
    pub snowhare: Option<i64>,
    #[serde(rename = "Squirrel")]
    pub squirrel: Option<::serde_json::Value>,
    #[serde(rename = "Tortoise")]
    pub tortoise: Option<f64>,
    #[serde(rename = "Turkey")]
    pub turkey: Option<f64>,
    #[serde(rename = "Warg")]
    pub warg: Option<f64>,
    #[serde(rename = "WildBoar")]
    pub wild_boar: Option<f64>,
    #[serde(rename = "Wolf_Arctic")]
    pub wolf_arctic: Option<f64>,
    #[serde(rename = "Wolf_Timber")]
    pub wolf_timber: Option<f64>,
    #[serde(rename = "Yak")]
    pub yak: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WildPlants {
    #[serde(rename = "Plant_Agave")]
    pub plant_agave: Option<f64>,
    #[serde(rename = "Plant_Alocasia")]
    pub plant_alocasia: Option<f64>,
    #[serde(rename = "Plant_Astragalus")]
    pub plant_astragalus: Option<f64>,
    #[serde(rename = "Plant_Berry")]
    pub plant_berry: Option<f64>,
    #[serde(rename = "Plant_Brambles")]
    pub plant_brambles: Option<f64>,
    #[serde(rename = "Plant_Bush")]
    pub plant_bush: Option<f64>,
    #[serde(rename = "Plant_Chokevine")]
    pub plant_chokevine: Option<f64>,
    #[serde(rename = "Plant_Clivia")]
    pub plant_clivia: Option<f64>,
    #[serde(rename = "Plant_Dandelion")]
    pub plant_dandelion: Option<f64>,
    #[serde(rename = "Plant_Grass")]
    pub plant_grass: Option<::serde_json::Value>,
    #[serde(rename = "Plant_HealrootWild")]
    pub plant_healroot_wild: Option<f64>,
    #[serde(rename = "Plant_Moss")]
    pub plant_moss: Option<f64>,
    #[serde(rename = "Plant_PincushionCactus")]
    pub plant_pincushion_cactus: Option<::serde_json::Value>,
    #[serde(rename = "Plant_Rafflesia")]
    pub plant_rafflesia: Option<f64>,
    #[serde(rename = "Plant_SaguaroCactus")]
    pub plant_saguaro_cactus: Option<::serde_json::Value>,
    #[serde(rename = "Plant_ShrubLow")]
    pub plant_shrub_low: Option<f64>,
    #[serde(rename = "Plant_TallGrass")]
    pub plant_tall_grass: Option<f64>,
    #[serde(rename = "Plant_TreeBamboo")]
    pub plant_tree_bamboo: Option<f64>,
    #[serde(rename = "Plant_TreeBirch")]
    pub plant_tree_birch: Option<f64>,
    #[serde(rename = "Plant_TreeCecropia")]
    pub plant_tree_cecropia: Option<f64>,
    #[serde(rename = "Plant_TreeCypress")]
    pub plant_tree_cypress: Option<f64>,
    #[serde(rename = "Plant_TreeDrago")]
    pub plant_tree_drago: Option<f64>,
    #[serde(rename = "Plant_TreeMaple")]
    pub plant_tree_maple: Option<f64>,
    #[serde(rename = "Plant_TreeOak")]
    pub plant_tree_oak: Option<f64>,
    #[serde(rename = "Plant_TreePalm")]
    pub plant_tree_palm: Option<f64>,
    #[serde(rename = "Plant_TreePine")]
    pub plant_tree_pine: Option<f64>,
    #[serde(rename = "Plant_TreePoplar")]
    pub plant_tree_poplar: Option<f64>,
    #[serde(rename = "Plant_TreeTeak")]
    pub plant_tree_teak: Option<f64>,
    #[serde(rename = "Plant_TreeWillow")]
    pub plant_tree_willow: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BodyDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "corePart")]
    pub core_part: CorePart,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CorePart {
    pub def: String,
    pub depth: String,
    #[serde(default)]
    pub groups: Vec<String>,
    pub height: String,
    pub parts: Vec<Part>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Part {
    pub coverage: ::serde_json::Value,
    #[serde(rename = "customLabel")]
    pub custom_label: Option<String>,
    pub def: String,
    pub depth: Option<String>,
    #[serde(default)]
    pub groups: Vec<String>,
    pub height: Option<String>,
    #[serde(default)]
    pub parts: Vec<Part2>,
    #[serde(rename = "woundAnchorTag")]
    pub wound_anchor_tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Part2 {
    pub coverage: f64,
    #[serde(rename = "customLabel")]
    pub custom_label: Option<String>,
    pub def: String,
    pub depth: Option<String>,
    #[serde(default)]
    pub groups: Vec<String>,
    pub height: Option<String>,
    #[serde(default)]
    pub parts: Vec<Part3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Part3 {
    pub coverage: f64,
    #[serde(rename = "customLabel")]
    pub custom_label: Option<String>,
    pub def: String,
    pub depth: Option<String>,
    #[serde(default)]
    pub groups: Vec<String>,
    pub height: Option<String>,
    #[serde(default)]
    pub parts: Vec<Part4>,
    #[serde(rename = "woundAnchorTag")]
    pub wound_anchor_tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Part4 {
    pub coverage: ::serde_json::Value,
    #[serde(rename = "customLabel")]
    pub custom_label: Option<String>,
    pub def: String,
    pub depth: Option<String>,
    #[serde(default)]
    pub groups: Vec<String>,
    #[serde(default)]
    pub parts: Vec<Part5>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Part5 {
    pub coverage: f64,
    pub def: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BodyPartDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    pub alive: Option<bool>,
    #[serde(rename = "beautyRelated")]
    pub beauty_related: Option<bool>,
    #[serde(rename = "bleedRate")]
    pub bleed_rate: Option<::serde_json::Value>,
    #[serde(rename = "canScarify")]
    pub can_scarify: Option<bool>,
    #[serde(rename = "canSuggestAmputation")]
    pub can_suggest_amputation: Option<bool>,
    pub conceptual: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    pub delicate: Option<bool>,
    #[serde(rename = "destroyableByDamage")]
    pub destroyable_by_damage: Option<bool>,
    #[serde(rename = "forceAlwaysRemovable")]
    pub force_always_removable: Option<bool>,
    #[serde(rename = "frostbiteVulnerability")]
    pub frostbite_vulnerability: Option<::serde_json::Value>,
    #[serde(rename = "hitChanceFactors")]
    #[serde(default)]
    pub hit_chance_factors: Vec<HitChanceFactor>,
    #[serde(rename = "hitPoints")]
    pub hit_points: Option<i64>,
    pub label: Option<String>,
    #[serde(rename = "labelShort")]
    pub label_short: Option<String>,
    #[serde(rename = "pawnGeneratorCanAmputate")]
    pub pawn_generator_can_amputate: Option<bool>,
    #[serde(rename = "permanentInjuryChanceFactor")]
    pub permanent_injury_chance_factor: Option<i64>,
    #[serde(rename = "removeRecipeLabelOverride")]
    pub remove_recipe_label_override: Option<String>,
    #[serde(rename = "skinCovered")]
    pub skin_covered: Option<bool>,
    pub socketed: Option<bool>,
    pub solid: Option<bool>,
    #[serde(rename = "spawnThingOnRemoved")]
    pub spawn_thing_on_removed: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HitChanceFactor {
    pub key: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BodyPartGroupDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "labelShort")]
    pub label_short: Option<String>,
    #[serde(rename = "listOrder")]
    pub list_order: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BodyPartTagDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub vital: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BodyTypeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "bodyDessicatedGraphicPath")]
    pub body_dessicated_graphic_path: String,
    #[serde(rename = "bodyNakedGraphicPath")]
    pub body_naked_graphic_path: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "headOffset")]
    pub head_offset: String,
    #[serde(rename = "woundAnchors")]
    pub wound_anchors: Vec<WoundAnchor>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WoundAnchor {
    #[serde(rename = "canMirror")]
    pub can_mirror: Option<bool>,
    #[serde(rename = "crownType")]
    pub crown_type: Option<String>,
    #[serde(rename = "debugColor")]
    pub debug_color: String,
    pub group: Option<String>,
    pub layer: Option<String>,
    pub offset: String,
    pub range: ::serde_json::Value,
    pub rotation: String,
    pub tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ChemicalDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "addictionHediff")]
    pub addiction_hediff: String,
    #[serde(rename = "canBinge")]
    pub can_binge: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "onGeneratedAddictedEvents")]
    #[serde(default)]
    pub on_generated_addicted_events: Vec<OnGeneratedAddictedEvent>,
    #[serde(rename = "onGeneratedAddictedToleranceChance")]
    pub on_generated_addicted_tolerance_chance: Option<f64>,
    #[serde(rename = "toleranceHediff")]
    pub tolerance_hediff: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OnGeneratedAddictedEvent {
    pub chance: f64,
    pub hediff: String,
    #[serde(rename = "partsToAffect")]
    pub parts_to_affect: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ClamorDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ConceptDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "gameMode")]
    pub game_mode: Option<String>,
    #[serde(rename = "helpText")]
    pub help_text: Option<String>,
    #[serde(rename = "highlightTags")]
    #[serde(default)]
    pub highlight_tags: Vec<String>,
    pub label: Option<String>,
    #[serde(rename = "needsOpportunity")]
    pub needs_opportunity: Option<bool>,
    #[serde(rename = "opportunityDecays")]
    pub opportunity_decays: Option<bool>,
    pub priority: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CultureDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowedPlaceTags")]
    pub allowed_place_tags: Vec<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "deityNameMaker")]
    pub deity_name_maker: DeityNameMaker,
    #[serde(rename = "deityTypeMaker")]
    pub deity_type_maker: DeityTypeMaker,
    pub description: String,
    #[serde(rename = "festivalNameMaker")]
    pub festival_name_maker: FestivalNameMaker,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "ideoNameMaker")]
    pub ideo_name_maker: IdeoNameMaker,
    pub label: String,
    #[serde(rename = "leaderTitleMaker")]
    pub leader_title_maker: LeaderTitleMaker,
    #[serde(rename = "pawnNameMaker")]
    pub pawn_name_maker: Option<String>,
    #[serde(rename = "preferredWeaponClasses")]
    pub preferred_weapon_classes: Option<PreferredWeaponClasses>,
    #[serde(rename = "styleItemTags")]
    pub style_item_tags: Vec<StyleItemTag>,
    #[serde(rename = "thingStyleCategories")]
    #[serde(default)]
    pub thing_style_categories: Vec<ThingStyleCategory>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DeityNameMaker {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DeityTypeMaker {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FestivalNameMaker {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IdeoNameMaker {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LeaderTitleMaker {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PreferredWeaponClasses {
    pub despised: String,
    pub noble: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StyleItemTag {
    #[serde(rename = "baseWeight")]
    pub base_weight: ::serde_json::Value,
    pub tag: String,
    #[serde(rename = "weightFactor")]
    pub weight_factor: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingStyleCategory {
    #[serde(rename = "MayRequire")]
    pub may_require: String,
    pub category: String,
    pub priority: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DamageArmorCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "armorRatingStat")]
    pub armor_rating_stat: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "multStat")]
    pub mult_stat: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DamageDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "additionalHediffs")]
    #[serde(default)]
    pub additional_hediffs: Vec<AdditionalHediff>,
    #[serde(rename = "applyAdditionalHediffsIfHuntingForFood")]
    pub apply_additional_hediffs_if_hunting_for_food: Option<bool>,
    #[serde(rename = "armorCategory")]
    pub armor_category: Option<::serde_json::Value>,
    #[serde(rename = "bluntInnerHitChance")]
    pub blunt_inner_hit_chance: Option<f64>,
    #[serde(rename = "bluntInnerHitDamageFractionToAdd")]
    pub blunt_inner_hit_damage_fraction_to_add: Option<String>,
    #[serde(rename = "bluntInnerHitDamageFractionToConvert")]
    pub blunt_inner_hit_damage_fraction_to_convert: Option<String>,
    #[serde(rename = "bluntStunChancePerDamagePctOfCorePartToBodyCurve")]
    pub blunt_stun_chance_per_damage_pct_of_core_part_to_body_curve: Option<BluntStunChancePerDamagePctOfCorePartToBodyCurve>,
    #[serde(rename = "bluntStunChancePerDamagePctOfCorePartToHeadCurve")]
    pub blunt_stun_chance_per_damage_pct_of_core_part_to_head_curve: Option<BluntStunChancePerDamagePctOfCorePartToHeadCurve>,
    #[serde(rename = "bluntStunDuration")]
    pub blunt_stun_duration: Option<f64>,
    #[serde(rename = "buildingDamageFactor")]
    pub building_damage_factor: Option<::serde_json::Value>,
    #[serde(rename = "buildingDamageFactorImpassable")]
    pub building_damage_factor_impassable: Option<i64>,
    #[serde(rename = "buildingDamageFactorPassable")]
    pub building_damage_factor_passable: Option<::serde_json::Value>,
    #[serde(rename = "canInterruptJobs")]
    pub can_interrupt_jobs: Option<bool>,
    #[serde(rename = "canUseDeflectMetalEffect")]
    pub can_use_deflect_metal_effect: Option<bool>,
    #[serde(rename = "combatLogRules")]
    pub combat_log_rules: Option<String>,
    #[serde(rename = "cutCleaveBonus")]
    pub cut_cleave_bonus: Option<f64>,
    #[serde(rename = "cutExtraTargetsCurve")]
    pub cut_extra_targets_curve: Option<CutExtraTargetsCurve>,
    #[serde(rename = "damageEffecter")]
    pub damage_effecter: Option<String>,
    #[serde(rename = "deathMessage")]
    pub death_message: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultArmorPenetration")]
    pub default_armor_penetration: Option<::serde_json::Value>,
    #[serde(rename = "defaultDamage")]
    pub default_damage: Option<i64>,
    #[serde(rename = "defaultStoppingPower")]
    pub default_stopping_power: Option<f64>,
    pub execution: Option<bool>,
    #[serde(rename = "explosionAffectOutsidePartsOnly")]
    pub explosion_affect_outside_parts_only: Option<bool>,
    #[serde(rename = "explosionCellFleck")]
    pub explosion_cell_fleck: Option<String>,
    #[serde(rename = "explosionColorCenter")]
    pub explosion_color_center: Option<String>,
    #[serde(rename = "explosionColorEdge")]
    pub explosion_color_edge: Option<String>,
    #[serde(rename = "explosionHeatEnergyPerCell")]
    pub explosion_heat_energy_per_cell: Option<i64>,
    #[serde(rename = "explosionInteriorFleck")]
    pub explosion_interior_fleck: Option<String>,
    #[serde(rename = "explosionSnowMeltAmount")]
    pub explosion_snow_melt_amount: Option<i64>,
    #[serde(rename = "externalViolence")]
    pub external_violence: Option<bool>,
    #[serde(rename = "externalViolenceForMechanoids")]
    pub external_violence_for_mechanoids: Option<bool>,
    #[serde(rename = "harmAllLayersUntilOutside")]
    pub harm_all_layers_until_outside: Option<bool>,
    #[serde(rename = "harmsHealth")]
    pub harms_health: Option<bool>,
    #[serde(rename = "hasForcefulImpact")]
    pub has_forceful_impact: Option<bool>,
    pub hediff: Option<String>,
    #[serde(rename = "hediffSkin")]
    pub hediff_skin: Option<String>,
    #[serde(rename = "hediffSolid")]
    pub hediff_solid: Option<String>,
    #[serde(rename = "impactSoundType")]
    pub impact_sound_type: Option<::serde_json::Value>,
    #[serde(rename = "isExplosive")]
    pub is_explosive: Option<bool>,
    #[serde(rename = "isRanged")]
    pub is_ranged: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "makesAnimalsFlee")]
    pub makes_animals_flee: Option<bool>,
    #[serde(rename = "makesBlood")]
    pub makes_blood: Option<bool>,
    #[serde(rename = "minDamageToFragment")]
    pub min_damage_to_fragment: Option<i64>,
    #[serde(rename = "overkillPctToDestroyPart")]
    pub overkill_pct_to_destroy_part: Option<String>,
    #[serde(rename = "plantDamageFactor")]
    pub plant_damage_factor: Option<i64>,
    #[serde(rename = "scratchSplitPercentage")]
    pub scratch_split_percentage: Option<f64>,
    #[serde(rename = "soundExplosion")]
    pub sound_explosion: Option<String>,
    #[serde(rename = "stabChanceOfForcedInternal")]
    pub stab_chance_of_forced_internal: Option<f64>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AdditionalHediff {
    pub hediff: String,
    #[serde(rename = "severityPerDamageDealt")]
    pub severity_per_damage_dealt: f64,
    #[serde(rename = "victimSeverityScaling")]
    pub victim_severity_scaling: String,
    #[serde(rename = "victimSeverityScalingByInvBodySize")]
    pub victim_severity_scaling_by_inv_body_size: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BluntStunChancePerDamagePctOfCorePartToBodyCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BluntStunChancePerDamagePctOfCorePartToHeadCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CutExtraTargetsCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DesignationCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    pub order: i64,
    #[serde(rename = "showPowerGrid")]
    pub show_power_grid: Option<bool>,
    #[serde(rename = "specialDesignatorClasses")]
    pub special_designator_classes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DesignationDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "designateCancelable")]
    pub designate_cancelable: Option<bool>,
    #[serde(rename = "removeIfBuildingDespawned")]
    pub remove_if_building_despawned: Option<bool>,
    #[serde(rename = "targetType")]
    pub target_type: String,
    #[serde(rename = "texturePath")]
    pub texture_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DesignatorDropdownGroupDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DifficultyDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "adaptationEffectFactor")]
    pub adaptation_effect_factor: Option<f64>,
    #[serde(rename = "adaptationGrowthRateFactorOverZero")]
    pub adaptation_growth_rate_factor_over_zero: Option<f64>,
    #[serde(rename = "allowBigThreats")]
    pub allow_big_threats: Option<bool>,
    #[serde(rename = "allowCaveHives")]
    pub allow_cave_hives: Option<bool>,
    #[serde(rename = "allowExtremeWeatherIncidents")]
    pub allow_extreme_weather_incidents: Option<bool>,
    #[serde(rename = "allowIntroThreats")]
    pub allow_intro_threats: Option<bool>,
    #[serde(rename = "allowViolentQuests")]
    pub allow_violent_quests: Option<bool>,
    #[serde(rename = "butcherYieldFactor")]
    pub butcher_yield_factor: Option<f64>,
    #[serde(rename = "colonistMoodOffset")]
    pub colonist_mood_offset: Option<i64>,
    #[serde(rename = "cropYieldFactor")]
    pub crop_yield_factor: Option<f64>,
    #[serde(rename = "deepDrillInfestationChanceFactor")]
    pub deep_drill_infestation_chance_factor: Option<f64>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    pub difficulty: Option<i64>,
    #[serde(rename = "diseaseIntervalFactor")]
    pub disease_interval_factor: Option<f64>,
    #[serde(rename = "enemyDeathOnDownedChanceFactor")]
    pub enemy_death_on_downed_chance_factor: Option<f64>,
    #[serde(rename = "enemyReproductionRateFactor")]
    pub enemy_reproduction_rate_factor: Option<f64>,
    #[serde(rename = "foodPoisonChanceFactor")]
    pub food_poison_chance_factor: Option<f64>,
    #[serde(rename = "isCustom")]
    pub is_custom: Option<bool>,
    pub label: String,
    #[serde(rename = "maintenanceCostFactor")]
    pub maintenance_cost_factor: Option<f64>,
    #[serde(rename = "manhunterChanceOnDamageFactor")]
    pub manhunter_chance_on_damage_factor: Option<f64>,
    #[serde(rename = "mineYieldFactor")]
    pub mine_yield_factor: Option<f64>,
    #[serde(rename = "peacefulTemples")]
    pub peaceful_temples: Option<bool>,
    #[serde(rename = "playerPawnInfectionChanceFactor")]
    pub player_pawn_infection_chance_factor: Option<f64>,
    #[serde(rename = "predatorsHuntHumanlikes")]
    pub predators_hunt_humanlikes: Option<bool>,
    #[serde(rename = "researchSpeedFactor")]
    pub research_speed_factor: Option<f64>,
    #[serde(rename = "scariaRotChance")]
    pub scaria_rot_chance: Option<f64>,
    #[serde(rename = "threatScale")]
    pub threat_scale: Option<f64>,
    #[serde(rename = "tradePriceFactorLoss")]
    pub trade_price_factor_loss: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DrugPolicyDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowPleasureDrugs")]
    pub allow_pleasure_drugs: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(default)]
    pub entries: Vec<Entry>,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Entry {
    #[serde(rename = "allowScheduled")]
    pub allow_scheduled: Option<bool>,
    #[serde(rename = "allowedForJoy")]
    pub allowed_for_joy: bool,
    #[serde(rename = "daysFrequency")]
    pub days_frequency: Option<i64>,
    pub drug: String,
    #[serde(rename = "takeToInventory")]
    pub take_to_inventory: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DutyDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "alwaysShowWeapon")]
    pub always_show_weapon: Option<bool>,
    #[serde(rename = "constantThinkNode")]
    pub constant_think_node: Option<ConstantThinkNode>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub hook: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "socialModeMax")]
    pub social_mode_max: Option<String>,
    #[serde(rename = "thinkNode")]
    pub think_node: ThinkNode,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ConstantThinkNode {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "subNodes")]
    pub sub_nodes: Vec<SubNode>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "pickUpUtilityItems")]
    pub pick_up_utility_items: Option<bool>,
    #[serde(rename = "preferBuildingDestroyers")]
    pub prefer_building_destroyers: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThinkNode {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defaultLocomotion")]
    pub default_locomotion: Option<String>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode2>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode2 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "attackAllInert")]
    pub attack_all_inert: Option<bool>,
    #[serde(rename = "canMineNonMineables")]
    pub can_mine_non_mineables: Option<bool>,
    #[serde(rename = "chaseTarget")]
    pub chase_target: Option<bool>,
    #[serde(rename = "defaultLocomotion")]
    pub default_locomotion: Option<String>,
    #[serde(rename = "exactCell")]
    pub exact_cell: Option<bool>,
    #[serde(rename = "expiryInterval")]
    pub expiry_interval: Option<i64>,
    #[serde(rename = "forceCanDig")]
    pub force_can_dig: Option<bool>,
    #[serde(rename = "forceCanDigIfAnyHostileActiveThreat")]
    pub force_can_dig_if_any_hostile_active_threat: Option<bool>,
    #[serde(rename = "forceCanDigIfCantReachMapEdge")]
    pub force_can_dig_if_cant_reach_map_edge: Option<bool>,
    pub invert: Option<bool>,
    #[serde(rename = "jobMaxDuration")]
    pub job_max_duration: Option<i64>,
    #[serde(rename = "locomotionUrgency")]
    pub locomotion_urgency: Option<String>,
    #[serde(rename = "locomotionUrgencyOutsideRadius")]
    pub locomotion_urgency_outside_radius: Option<String>,
    #[serde(rename = "maxDanger")]
    pub max_danger: Option<String>,
    #[serde(rename = "maxDistFromPoint")]
    pub max_dist_from_point: Option<i64>,
    #[serde(rename = "maxDistToDutyTarget")]
    pub max_dist_to_duty_target: Option<i64>,
    #[serde(rename = "maxDistToSquadFlag")]
    pub max_dist_to_squad_flag: Option<i64>,
    pub min: Option<f64>,
    #[serde(rename = "mtbHours")]
    pub mtb_hours: Option<f64>,
    #[serde(rename = "needLOSToAcquireNonPawnTargets")]
    pub need_losto_acquire_non_pawn_targets: Option<bool>,
    #[serde(rename = "onlyIfDamagingState")]
    pub only_if_damaging_state: Option<bool>,
    #[serde(rename = "onlyIfInDanger")]
    pub only_if_in_danger: Option<bool>,
    #[serde(rename = "ritualTagOnArrival")]
    pub ritual_tag_on_arrival: Option<String>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode3>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
    #[serde(rename = "targetAcquireRadius")]
    pub target_acquire_radius: Option<i64>,
    #[serde(rename = "targetKeepRadius")]
    pub target_keep_radius: Option<i64>,
    #[serde(rename = "thresholdTicks")]
    pub threshold_ticks: Option<i64>,
    #[serde(rename = "ticksBetweenWandersRange")]
    pub ticks_between_wanders_range: Option<String>,
    #[serde(rename = "treeDef")]
    pub tree_def: Option<String>,
    #[serde(rename = "wanderRadius")]
    pub wander_radius: Option<i64>,
    #[serde(rename = "workTags")]
    #[serde(default)]
    pub work_tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode3 {
    #[serde(rename = "Class")]
    pub class: String,
    pub chance: Option<f64>,
    #[serde(rename = "expiryInterval")]
    pub expiry_interval: Option<i64>,
    #[serde(rename = "locomotionUrgency")]
    pub locomotion_urgency: Option<String>,
    #[serde(rename = "minCategory")]
    pub min_category: Option<String>,
    #[serde(rename = "needLOSToAcquireNonPawnTargets")]
    pub need_losto_acquire_non_pawn_targets: Option<bool>,
    #[serde(rename = "soundDefFemale")]
    pub sound_def_female: Option<String>,
    #[serde(rename = "soundDefMale")]
    pub sound_def_male: Option<String>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode4>,
    #[serde(rename = "targetAcquireRadius")]
    pub target_acquire_radius: Option<i64>,
    #[serde(rename = "targetKeepRadius")]
    pub target_keep_radius: Option<i64>,
    pub ticks: Option<i64>,
    #[serde(rename = "ticksBetweenWandersRange")]
    pub ticks_between_wanders_range: Option<String>,
    #[serde(rename = "treeDef")]
    pub tree_def: Option<String>,
    #[serde(rename = "wanderRadius")]
    pub wander_radius: Option<i64>,
    #[serde(rename = "workTags")]
    #[serde(default)]
    pub work_tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode4 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "needLOSToAcquireNonPawnTargets")]
    pub need_losto_acquire_non_pawn_targets: Option<bool>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode5>,
    #[serde(rename = "targetAcquireRadius")]
    pub target_acquire_radius: Option<i64>,
    #[serde(rename = "targetKeepRadius")]
    pub target_keep_radius: Option<i64>,
    #[serde(rename = "ticksRange")]
    pub ticks_range: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode5 {
    #[serde(rename = "Class")]
    pub class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EffecterDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    pub children: Vec<Children>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "offsetTowardsTarget")]
    pub offset_towards_target: Option<String>,
    #[serde(rename = "positionRadius")]
    pub position_radius: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Children {
    #[serde(rename = "absoluteAngle")]
    pub absolute_angle: Option<bool>,
    #[serde(rename = "airTime")]
    pub air_time: Option<String>,
    pub angle: Option<String>,
    #[serde(rename = "burstCount")]
    pub burst_count: Option<::serde_json::Value>,
    #[serde(rename = "chancePerTick")]
    pub chance_per_tick: Option<f64>,
    pub color: Option<String>,
    #[serde(rename = "fleckDef")]
    pub fleck_def: Option<String>,
    #[serde(rename = "fleckUsesAngleForVelocity")]
    pub fleck_uses_angle_for_velocity: Option<bool>,
    #[serde(rename = "intermittentSoundInterval")]
    pub intermittent_sound_interval: Option<IntermittentSoundInterval>,
    #[serde(rename = "moteDef")]
    pub mote_def: Option<String>,
    #[serde(rename = "positionLerpFactor")]
    pub position_lerp_factor: Option<::serde_json::Value>,
    #[serde(rename = "positionRadius")]
    pub position_radius: Option<f64>,
    #[serde(rename = "rotationRate")]
    pub rotation_rate: Option<String>,
    pub scale: Option<::serde_json::Value>,
    #[serde(rename = "soundDef")]
    pub sound_def: Option<String>,
    #[serde(rename = "spawnLocType")]
    pub spawn_loc_type: Option<String>,
    pub speed: Option<::serde_json::Value>,
    #[serde(rename = "subEffecterClass")]
    pub sub_effecter_class: String,
    #[serde(rename = "ticksBeforeSustainerStart")]
    pub ticks_before_sustainer_start: Option<i64>,
    #[serde(rename = "ticksBetweenMotes")]
    pub ticks_between_motes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IntermittentSoundInterval {
    pub max: i64,
    pub min: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ExpansionDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "backgroundPath")]
    pub background_path: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "isCore")]
    pub is_core: Option<bool>,
    pub label: String,
    #[serde(rename = "linkedMod")]
    pub linked_mod: String,
    #[serde(rename = "notOwnedIconPath")]
    pub not_owned_icon_path: Option<String>,
    #[serde(rename = "previewImagesFolderPath")]
    pub preview_images_folder_path: Option<String>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "steamUrl")]
    pub steam_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ExpectationDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "joyKindsNeeded")]
    pub joy_kinds_needed: i64,
    #[serde(rename = "joyToleranceDropPerDay")]
    pub joy_tolerance_drop_per_day: f64,
    pub label: String,
    #[serde(rename = "maxMapWealth")]
    pub max_map_wealth: Option<i64>,
    pub order: i64,
    #[serde(rename = "ritualQualityOffset")]
    pub ritual_quality_offset: Option<f64>,
    #[serde(rename = "thoughtStage")]
    pub thought_stage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FactionDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "allowedArrivalTemperatureRange")]
    pub allowed_arrival_temperature_range: Option<String>,
    #[serde(rename = "allowedCultures")]
    #[serde(default)]
    pub allowed_cultures: Vec<String>,
    #[serde(rename = "allowedMemes")]
    #[serde(default)]
    pub allowed_memes: Vec<AllowedMeme>,
    #[serde(rename = "apparelStuffFilter")]
    pub apparel_stuff_filter: Option<ApparelStuffFilter>,
    #[serde(rename = "autoFlee")]
    pub auto_flee: Option<bool>,
    #[serde(rename = "backstoryFilters")]
    #[serde(default)]
    pub backstory_filters: Vec<BackstoryFilter>,
    #[serde(rename = "baseTraderKinds")]
    #[serde(default)]
    pub base_trader_kinds: Vec<String>,
    #[serde(rename = "basicMemberKind")]
    pub basic_member_kind: Option<String>,
    #[serde(rename = "canMakeRandomly")]
    pub can_make_randomly: Option<bool>,
    #[serde(rename = "canSiege")]
    pub can_siege: Option<bool>,
    #[serde(rename = "canStageAttacks")]
    pub can_stage_attacks: Option<bool>,
    #[serde(rename = "canUseAvoidGrid")]
    pub can_use_avoid_grid: Option<bool>,
    #[serde(rename = "caravanTraderKinds")]
    #[serde(default)]
    pub caravan_trader_kinds: Vec<String>,
    #[serde(rename = "categoryTag")]
    pub category_tag: Option<String>,
    #[serde(rename = "classicIdeo")]
    pub classic_ideo: Option<bool>,
    #[serde(rename = "colorSpectrum")]
    #[serde(default)]
    pub color_spectrum: Vec<String>,
    #[serde(rename = "configurationListOrderPriority")]
    pub configuration_list_order_priority: Option<i64>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "disallowedMemes")]
    #[serde(default)]
    pub disallowed_memes: Vec<DisallowedMeme>,
    #[serde(rename = "disallowedPrecepts")]
    #[serde(default)]
    pub disallowed_precepts: Vec<DisallowedPrecept>,
    #[serde(rename = "earliestRaidDays")]
    pub earliest_raid_days: Option<i64>,
    #[serde(rename = "factionIconPath")]
    pub faction_icon_path: Option<String>,
    #[serde(rename = "factionNameMaker")]
    pub faction_name_maker: Option<String>,
    #[serde(rename = "fixedName")]
    pub fixed_name: Option<String>,
    #[serde(rename = "forageabilityFactor")]
    pub forageability_factor: Option<f64>,
    #[serde(rename = "geneticVariance")]
    pub genetic_variance: Option<::serde_json::Value>,
    pub hidden: Option<bool>,
    #[serde(rename = "hostileToFactionlessHumanlikes")]
    pub hostile_to_factionless_humanlikes: Option<bool>,
    #[serde(rename = "humanlikeFaction")]
    pub humanlike_faction: Option<bool>,
    #[serde(rename = "isPlayer")]
    pub is_player: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "leaderTitle")]
    pub leader_title: Option<String>,
    #[serde(rename = "listOrderPriority")]
    pub list_order_priority: Option<i64>,
    #[serde(rename = "maxConfigurableAtWorldCreation")]
    pub max_configurable_at_world_creation: Option<i64>,
    #[serde(rename = "maxCountAtGameStart")]
    pub max_count_at_game_start: Option<i64>,
    #[serde(rename = "maxPawnCostPerTotalPointsCurve")]
    pub max_pawn_cost_per_total_points_curve: Option<MaxPawnCostPerTotalPointsCurve>,
    #[serde(rename = "naturalEnemy")]
    pub natural_enemy: Option<bool>,
    #[serde(rename = "pawnGroupMakers")]
    #[serde(default)]
    pub pawn_group_makers: Vec<PawnGroupMaker>,
    #[serde(rename = "pawnSingular")]
    pub pawn_singular: Option<String>,
    #[serde(rename = "pawnsPlural")]
    pub pawns_plural: Option<String>,
    #[serde(rename = "permanentEnemy")]
    pub permanent_enemy: Option<bool>,
    #[serde(rename = "playerInitialSettlementNameMaker")]
    pub player_initial_settlement_name_maker: Option<String>,
    #[serde(rename = "raidCommonalityFromPointsCurve")]
    pub raid_commonality_from_points_curve: Option<RaidCommonalityFromPointsCurve>,
    #[serde(rename = "raidLootMaker")]
    pub raid_loot_maker: Option<String>,
    #[serde(rename = "raidLootValueFromPointsCurve")]
    pub raid_loot_value_from_points_curve: Option<RaidLootValueFromPointsCurve>,
    #[serde(rename = "recipePrerequisiteTags")]
    #[serde(default)]
    pub recipe_prerequisite_tags: Vec<String>,
    #[serde(rename = "requiredCountAtGameStart")]
    pub required_count_at_game_start: Option<i64>,
    #[serde(rename = "requiredMemes")]
    #[serde(default)]
    pub required_memes: Vec<RequiredMeme>,
    #[serde(rename = "rescueesCanJoin")]
    pub rescuees_can_join: Option<bool>,
    #[serde(rename = "settlementGenerationWeight")]
    pub settlement_generation_weight: Option<i64>,
    #[serde(rename = "settlementNameMaker")]
    pub settlement_name_maker: Option<String>,
    #[serde(rename = "settlementTexturePath")]
    pub settlement_texture_path: Option<String>,
    #[serde(rename = "startingResearchTags")]
    #[serde(default)]
    pub starting_research_tags: Vec<String>,
    #[serde(rename = "startingTechprintsResearchTags")]
    #[serde(default)]
    pub starting_techprints_research_tags: Vec<String>,
    #[serde(rename = "structureMemeWeights")]
    pub structure_meme_weights: Option<StructureMemeWeights>,
    #[serde(rename = "techLevel")]
    pub tech_level: Option<String>,
    #[serde(rename = "visitorTraderKinds")]
    #[serde(default)]
    pub visitor_trader_kinds: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AllowedMeme {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApparelStuffFilter {
    #[serde(rename = "thingDefs")]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BackstoryFilter {
    pub categories: Vec<String>,
    pub commonality: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DisallowedMeme {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DisallowedPrecept {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MaxPawnCostPerTotalPointsCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnGroupMaker {
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    pub carriers: Option<Carriers>,
    pub commonality: Option<::serde_json::Value>,
    #[serde(rename = "disallowedStrategies")]
    #[serde(default)]
    pub disallowed_strategies: Vec<String>,
    pub guards: Option<Guards>,
    #[serde(rename = "kindDef")]
    pub kind_def: String,
    #[serde(rename = "maxTotalPoints")]
    pub max_total_points: Option<i64>,
    pub options: Option<Options>,
    pub traders: Option<Traders>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Carriers {
    #[serde(rename = "Alpaca")]
    pub alpaca: ::serde_json::Value,
    #[serde(rename = "Dromedary")]
    pub dromedary: i64,
    #[serde(rename = "Elephant")]
    pub elephant: i64,
    #[serde(rename = "Muffalo")]
    pub muffalo: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Guards {
    #[serde(rename = "Grenadier_Destructive")]
    pub grenadier_destructive: Option<f64>,
    #[serde(rename = "Mercenary_Elite")]
    pub mercenary_elite: Option<i64>,
    #[serde(rename = "Mercenary_Gunner")]
    pub mercenary_gunner: Option<i64>,
    #[serde(rename = "Mercenary_Slasher")]
    pub mercenary_slasher: Option<i64>,
    #[serde(rename = "Town_Guard")]
    pub town_guard: Option<i64>,
    #[serde(rename = "Tribal_Archer")]
    pub tribal_archer: Option<i64>,
    #[serde(rename = "Tribal_Berserker")]
    pub tribal_berserker: Option<i64>,
    #[serde(rename = "Tribal_HeavyArcher")]
    pub tribal_heavy_archer: Option<i64>,
    #[serde(rename = "Tribal_Hunter")]
    pub tribal_hunter: Option<i64>,
    #[serde(rename = "Tribal_Warrior")]
    pub tribal_warrior: Option<i64>,
    #[serde(rename = "Villager")]
    pub villager: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Options {
    #[serde(rename = "Drifter")]
    pub drifter: Option<i64>,
    #[serde(rename = "Farmer")]
    pub farmer: Option<i64>,
    #[serde(rename = "Grenadier_Destructive")]
    pub grenadier_destructive: Option<::serde_json::Value>,
    #[serde(rename = "Grenadier_EMP")]
    pub grenadier_emp: Option<f64>,
    #[serde(rename = "Grenadier_Smoke")]
    pub grenadier_smoke: Option<f64>,
    #[serde(rename = "Hunter")]
    pub hunter: Option<i64>,
    #[serde(rename = "Logger")]
    pub logger: Option<i64>,
    #[serde(rename = "Mech_Centipede")]
    pub mech_centipede: Option<i64>,
    #[serde(rename = "Mech_Lancer")]
    pub mech_lancer: Option<i64>,
    #[serde(rename = "Mech_Pikeman")]
    pub mech_pikeman: Option<i64>,
    #[serde(rename = "Mech_Scyther")]
    pub mech_scyther: Option<i64>,
    #[serde(rename = "Mech_Termite_Breach")]
    pub mech_termite_breach: Option<i64>,
    #[serde(rename = "Mercenary_Elite")]
    pub mercenary_elite: Option<i64>,
    #[serde(rename = "Mercenary_Gunner")]
    pub mercenary_gunner: Option<i64>,
    #[serde(rename = "Mercenary_Heavy")]
    pub mercenary_heavy: Option<i64>,
    #[serde(rename = "Mercenary_Slasher")]
    pub mercenary_slasher: Option<i64>,
    #[serde(rename = "Mercenary_Sniper")]
    pub mercenary_sniper: Option<i64>,
    #[serde(rename = "Miner")]
    pub miner: Option<i64>,
    #[serde(rename = "Pirate")]
    pub pirate: Option<i64>,
    #[serde(rename = "PirateBoss")]
    pub pirate_boss: Option<i64>,
    #[serde(rename = "Scavenger")]
    pub scavenger: Option<i64>,
    #[serde(rename = "Thrasher")]
    pub thrasher: Option<i64>,
    #[serde(rename = "Town_Councilman")]
    pub town_councilman: Option<i64>,
    #[serde(rename = "Town_Guard")]
    pub town_guard: Option<i64>,
    #[serde(rename = "Tribal_Archer")]
    pub tribal_archer: Option<i64>,
    #[serde(rename = "Tribal_Berserker")]
    pub tribal_berserker: Option<i64>,
    #[serde(rename = "Tribal_Breacher")]
    pub tribal_breacher: Option<i64>,
    #[serde(rename = "Tribal_ChiefMelee")]
    pub tribal_chief_melee: Option<::serde_json::Value>,
    #[serde(rename = "Tribal_ChiefRanged")]
    pub tribal_chief_ranged: Option<i64>,
    #[serde(rename = "Tribal_Farmer")]
    pub tribal_farmer: Option<i64>,
    #[serde(rename = "Tribal_HeavyArcher")]
    pub tribal_heavy_archer: Option<i64>,
    #[serde(rename = "Tribal_Hunter")]
    pub tribal_hunter: Option<i64>,
    #[serde(rename = "Tribal_Logger")]
    pub tribal_logger: Option<i64>,
    #[serde(rename = "Tribal_Miner")]
    pub tribal_miner: Option<i64>,
    #[serde(rename = "Tribal_Penitent")]
    pub tribal_penitent: Option<i64>,
    #[serde(rename = "Tribal_Warrior")]
    pub tribal_warrior: Option<i64>,
    #[serde(rename = "Villager")]
    pub villager: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Traders {
    #[serde(rename = "Town_Trader")]
    pub town_trader: Option<i64>,
    #[serde(rename = "Tribal_Trader")]
    pub tribal_trader: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RaidCommonalityFromPointsCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RaidLootValueFromPointsCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RequiredMeme {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureMemeWeights {
    #[serde(rename = "Structure_Animist")]
    pub structure_animist: Option<StructureAnimist>,
    #[serde(rename = "Structure_Archist")]
    pub structure_archist: Option<StructureArchist>,
    #[serde(rename = "Structure_Ideological")]
    pub structure_ideological: Option<StructureIdeological>,
    #[serde(rename = "Structure_OriginBuddhist")]
    pub structure_origin_buddhist: Option<StructureOriginBuddhist>,
    #[serde(rename = "Structure_OriginChristian")]
    pub structure_origin_christian: Option<StructureOriginChristian>,
    #[serde(rename = "Structure_OriginHindu")]
    pub structure_origin_hindu: Option<StructureOriginHindu>,
    #[serde(rename = "Structure_OriginIslamic")]
    pub structure_origin_islamic: Option<StructureOriginIslamic>,
    #[serde(rename = "Structure_TheistAbstract")]
    pub structure_theist_abstract: Option<StructureTheistAbstract>,
    #[serde(rename = "Structure_TheistEmbodied")]
    pub structure_theist_embodied: Option<StructureTheistEmbodied>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureAnimist {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureArchist {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureIdeological {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureOriginBuddhist {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureOriginChristian {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureOriginHindu {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureOriginIslamic {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureTheistAbstract {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StructureTheistEmbodied {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FeatureDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "acceptableBiomes")]
    #[serde(default)]
    pub acceptable_biomes: Vec<String>,
    #[serde(rename = "canTouchWorldEdge")]
    pub can_touch_world_edge: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "maxPassageWidth")]
    pub max_passage_width: Option<i64>,
    #[serde(rename = "maxPctOfWholeArea")]
    pub max_pct_of_whole_area: Option<f64>,
    #[serde(rename = "maxRootGroupSize")]
    pub max_root_group_size: Option<i64>,
    #[serde(rename = "maxSize")]
    pub max_size: Option<i64>,
    #[serde(rename = "maxSpaceBetweenRootGroups")]
    pub max_space_between_root_groups: Option<i64>,
    #[serde(rename = "minRootGroupSize")]
    pub min_root_group_size: Option<i64>,
    #[serde(rename = "minRootGroupsInCluster")]
    pub min_root_groups_in_cluster: Option<i64>,
    #[serde(rename = "minSize")]
    pub min_size: Option<i64>,
    #[serde(rename = "nameMaker")]
    pub name_maker: String,
    pub order: i64,
    #[serde(rename = "rootBiomes")]
    #[serde(default)]
    pub root_biomes: Vec<String>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FleckDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "altitudeLayer")]
    pub altitude_layer: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "fadeInTime")]
    pub fade_in_time: Option<::serde_json::Value>,
    #[serde(rename = "fadeOutTime")]
    pub fade_out_time: Option<::serde_json::Value>,
    #[serde(rename = "fleckSystemClass")]
    pub fleck_system_class: Option<String>,
    #[serde(rename = "graphicData")]
    pub graphic_data: Option<GraphicData>,
    #[serde(rename = "growthRate")]
    pub growth_rate: Option<::serde_json::Value>,
    #[serde(rename = "landSound")]
    pub land_sound: Option<String>,
    #[serde(rename = "realTime")]
    pub real_time: Option<bool>,
    #[serde(rename = "solidTime")]
    pub solid_time: Option<::serde_json::Value>,
    #[serde(rename = "speedPerTime")]
    pub speed_per_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GraphicData {
    pub color: Option<String>,
    #[serde(rename = "drawSize")]
    pub draw_size: Option<::serde_json::Value>,
    #[serde(rename = "graphicClass")]
    pub graphic_class: Option<String>,
    #[serde(rename = "renderInstanced")]
    pub render_instanced: Option<bool>,
    #[serde(rename = "shaderParameters")]
    pub shader_parameters: Option<ShaderParameters>,
    #[serde(rename = "shaderType")]
    pub shader_type: Option<String>,
    #[serde(rename = "texPath")]
    pub tex_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShaderParameters {
    #[serde(rename = "_DistortionTex")]
    pub distortion_tex: Option<String>,
    #[serde(rename = "_brightnessMultiplier")]
    pub brightness_multiplier: Option<f64>,
    #[serde(rename = "_distortionIntensity")]
    pub distortion_intensity: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FleshTypeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "bandagedWounds")]
    #[serde(default)]
    pub bandaged_wounds: Vec<BandagedWound>,
    #[serde(rename = "corpseCategory")]
    pub corpse_category: String,
    #[serde(rename = "damageEffecter")]
    pub damage_effecter: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "genericWounds")]
    pub generic_wounds: Vec<GenericWound>,
    #[serde(rename = "hediffWounds")]
    #[serde(default)]
    pub hediff_wounds: Vec<HediffWound>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BandagedWound {
    pub texture: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GenericWound {
    pub color: Option<String>,
    pub texture: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HediffWound {
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    pub hediff: String,
    pub wounds: Vec<Wound>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Wound {
    #[serde(rename = "displayOverApparel")]
    pub display_over_apparel: Option<bool>,
    #[serde(rename = "displayPermanent")]
    pub display_permanent: Option<bool>,
    #[serde(rename = "flipOnRotation")]
    pub flip_on_rotation: Option<String>,
    #[serde(rename = "flipOnWoundAnchorTag")]
    pub flip_on_wound_anchor_tag: Option<String>,
    #[serde(rename = "flipWest")]
    pub flip_west: Option<bool>,
    #[serde(rename = "missingBodyPartFresh")]
    pub missing_body_part_fresh: Option<bool>,
    #[serde(rename = "onlyOnPart")]
    pub only_on_part: Option<String>,
    pub scale: Option<f64>,
    pub texture: Option<String>,
    #[serde(rename = "textureEast")]
    pub texture_east: Option<String>,
    #[serde(rename = "textureNorth")]
    pub texture_north: Option<String>,
    #[serde(rename = "textureSouth")]
    pub texture_south: Option<String>,
    #[serde(rename = "textureWest")]
    pub texture_west: Option<String>,
    #[serde(rename = "tintWithSkinColor")]
    pub tint_with_skin_color: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GameConditionDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "canBePermanent")]
    pub can_be_permanent: Option<bool>,
    #[serde(rename = "conditionClass")]
    pub condition_class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "defaultDroneLevel")]
    pub default_drone_level: Option<String>,
    pub description: String,
    #[serde(rename = "descriptionFuture")]
    pub description_future: Option<String>,
    #[serde(rename = "endMessage")]
    pub end_message: Option<String>,
    #[serde(rename = "exclusiveConditions")]
    #[serde(default)]
    pub exclusive_conditions: Vec<String>,
    #[serde(rename = "jumpToSourceKey")]
    pub jump_to_source_key: Option<String>,
    pub label: String,
    #[serde(rename = "letterDef")]
    pub letter_def: Option<String>,
    #[serde(rename = "letterHyperlinks")]
    #[serde(default)]
    pub letter_hyperlinks: Vec<String>,
    #[serde(rename = "letterText")]
    pub letter_text: Option<String>,
    #[serde(rename = "preventRain")]
    pub prevent_rain: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GatheringDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "calledOffMessage")]
    pub called_off_message: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub duty: Option<String>,
    #[serde(rename = "finishedMessage")]
    pub finished_message: Option<String>,
    #[serde(rename = "gatherSpotDefs")]
    #[serde(default)]
    pub gather_spot_defs: Vec<String>,
    pub label: String,
    #[serde(rename = "letterText")]
    pub letter_text: Option<String>,
    #[serde(rename = "letterTitle")]
    pub letter_title: Option<String>,
    #[serde(rename = "randomSelectionWeight")]
    pub random_selection_weight: Option<i64>,
    #[serde(rename = "respectTimetable")]
    pub respect_timetable: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GenStepDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "genStep")]
    pub gen_step: GenStep,
    #[serde(rename = "linkWithSite")]
    pub link_with_site: Option<String>,
    pub order: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GenStep {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowInWaterBiome")]
    pub allow_in_water_biome: Option<bool>,
    #[serde(rename = "clearSpaceSize")]
    pub clear_space_size: Option<i64>,
    pub count: Option<i64>,
    #[serde(rename = "countPer10kCellsRange")]
    pub count_per10_k_cells_range: Option<::serde_json::Value>,
    #[serde(rename = "extraNoBuildEdgeDist")]
    pub extra_no_build_edge_dist: Option<i64>,
    #[serde(rename = "guardsCountRange")]
    pub guards_count_range: Option<GuardsCountRange>,
    #[serde(rename = "maxMineableValue")]
    pub max_mineable_value: Option<i64>,
    #[serde(rename = "minSpacing")]
    pub min_spacing: Option<i64>,
    #[serde(default)]
    pub mineables: Vec<String>,
    #[serde(rename = "nearMapCenter")]
    pub near_map_center: Option<bool>,
    #[serde(rename = "terrainValidationDisallowed")]
    #[serde(default)]
    pub terrain_validation_disallowed: Vec<String>,
    #[serde(rename = "terrainValidationRadius")]
    pub terrain_validation_radius: Option<i64>,
    #[serde(rename = "thingDef")]
    pub thing_def: Option<String>,
    #[serde(rename = "totalValueRange")]
    pub total_value_range: Option<TotalValueRange>,
    #[serde(default)]
    pub validators: Vec<Validator>,
    #[serde(rename = "widthRange")]
    pub width_range: Option<WidthRange>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GuardsCountRange {
    pub max: i64,
    pub min: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TotalValueRange {
    pub max: i64,
    pub min: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Validator {
    #[serde(rename = "Class")]
    pub class: String,
    pub affordance: Option<String>,
    pub radius: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WidthRange {
    pub max: i64,
    pub min: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GoodwillSituationDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HairDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub category: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "styleGender")]
    pub style_gender: String,
    #[serde(rename = "styleTags")]
    pub style_tags: Vec<String>,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HediffDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "addedPartProps")]
    pub added_part_props: Option<AddedPartProps>,
    #[serde(rename = "battleStateLabel")]
    pub battle_state_label: Option<String>,
    #[serde(rename = "causesNeed")]
    pub causes_need: Option<String>,
    #[serde(rename = "chanceToCauseNoPain")]
    pub chance_to_cause_no_pain: Option<f64>,
    pub chronic: Option<bool>,
    #[serde(default)]
    pub comps: Vec<Comp2>,
    #[serde(rename = "countsAsAddedPartOrImplant")]
    pub counts_as_added_part_or_implant: Option<bool>,
    #[serde(rename = "cureAllAtOnceIfCuredByItem")]
    pub cure_all_at_once_if_cured_by_item: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultLabelColor")]
    pub default_label_color: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "displayWound")]
    pub display_wound: Option<bool>,
    #[serde(rename = "everCurableByItem")]
    pub ever_curable_by_item: Option<bool>,
    #[serde(rename = "hediffClass")]
    pub hediff_class: Option<String>,
    #[serde(rename = "hediffGivers")]
    #[serde(default)]
    pub hediff_givers: Vec<HediffGiver>,
    #[serde(rename = "initialSeverity")]
    pub initial_severity: Option<::serde_json::Value>,
    #[serde(rename = "injuryProps")]
    pub injury_props: Option<InjuryProps>,
    #[serde(rename = "isBad")]
    pub is_bad: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "labelNoun")]
    pub label_noun: Option<String>,
    #[serde(rename = "labelNounPretty")]
    pub label_noun_pretty: Option<String>,
    #[serde(rename = "lethalSeverity")]
    pub lethal_severity: Option<::serde_json::Value>,
    #[serde(rename = "makesAlert")]
    pub makes_alert: Option<bool>,
    #[serde(rename = "makesSickThought")]
    pub makes_sick_thought: Option<bool>,
    #[serde(rename = "maxSeverity")]
    pub max_severity: Option<::serde_json::Value>,
    #[serde(rename = "minSeverity")]
    pub min_severity: Option<f64>,
    #[serde(rename = "priceImpact")]
    pub price_impact: Option<bool>,
    #[serde(rename = "removeOnQuestLodgers")]
    pub remove_on_quest_lodgers: Option<bool>,
    #[serde(rename = "removeOnRedressChanceByDaysCurve")]
    pub remove_on_redress_chance_by_days_curve: Option<RemoveOnRedressChanceByDaysCurve>,
    #[serde(rename = "scenarioCanAdd")]
    pub scenario_can_add: Option<bool>,
    #[serde(rename = "spawnThingOnRemoved")]
    pub spawn_thing_on_removed: Option<String>,
    #[serde(default)]
    pub stages: Vec<::serde_json::Value>,
    #[serde(rename = "taleOnVisible")]
    pub tale_on_visible: Option<String>,
    pub tendable: Option<bool>,
    #[serde(rename = "woundAnchorRange")]
    pub wound_anchor_range: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AddedPartProps {
    #[serde(rename = "betterThanNatural")]
    pub better_than_natural: Option<bool>,
    #[serde(rename = "isGoodWeapon")]
    pub is_good_weapon: Option<bool>,
    #[serde(rename = "partEfficiency")]
    pub part_efficiency: Option<f64>,
    pub solid: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp2 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "animalMentalState")]
    pub animal_mental_state: Option<String>,
    #[serde(rename = "animalMentalStateAlias")]
    pub animal_mental_state_alias: Option<String>,
    #[serde(rename = "baseTendDurationHours")]
    pub base_tend_duration_hours: Option<i64>,
    pub chemical: Option<String>,
    pub days: Option<i64>,
    #[serde(rename = "decayPerDayPercentageLevelCurve")]
    pub decay_per_day_percentage_level_curve: Option<DecayPerDayPercentageLevelCurve>,
    #[serde(rename = "disappearsAfterTicks")]
    pub disappears_after_ticks: Option<::serde_json::Value>,
    #[serde(rename = "disappearsAtTotalTendQuality")]
    pub disappears_at_total_tend_quality: Option<i64>,
    #[serde(rename = "discoverLetterLabel")]
    pub discover_letter_label: Option<String>,
    #[serde(rename = "discoverLetterText")]
    pub discover_letter_text: Option<String>,
    pub filth: Option<String>,
    pub fleck: Option<String>,
    #[serde(rename = "healAmount")]
    pub heal_amount: Option<f64>,
    #[serde(rename = "humanMentalState")]
    pub human_mental_state: Option<String>,
    #[serde(rename = "immunityPerDayNotSick")]
    pub immunity_per_day_not_sick: Option<f64>,
    #[serde(rename = "immunityPerDaySick")]
    pub immunity_per_day_sick: Option<f64>,
    #[serde(rename = "infectionChance")]
    pub infection_chance: Option<f64>,
    #[serde(rename = "injuryCount")]
    pub injury_count: Option<String>,
    #[serde(rename = "injuryCreatedOnDeath")]
    pub injury_created_on_death: Option<String>,
    #[serde(rename = "instantlyPermanentLabel")]
    pub instantly_permanent_label: Option<String>,
    #[serde(rename = "labelSolidTendedWell")]
    pub label_solid_tended_well: Option<String>,
    #[serde(rename = "labelTendedWell")]
    pub label_tended_well: Option<String>,
    #[serde(rename = "labelTendedWellInner")]
    pub label_tended_well_inner: Option<String>,
    #[serde(rename = "letterDef")]
    pub letter_def: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "messageType")]
    pub message_type: Option<String>,
    #[serde(rename = "moteCount")]
    pub mote_count: Option<i64>,
    #[serde(rename = "moteOffsetRange")]
    pub mote_offset_range: Option<String>,
    #[serde(rename = "mtbDaysToCauseMentalState")]
    pub mtb_days_to_cause_mental_state: Option<i64>,
    #[serde(rename = "permanentLabel")]
    pub permanent_label: Option<String>,
    #[serde(rename = "sendLetterWhenDiscovered")]
    pub send_letter_when_discovered: Option<bool>,
    #[serde(rename = "severityIndices")]
    pub severity_indices: Option<String>,
    #[serde(rename = "severityPerDay")]
    pub severity_per_day: Option<::serde_json::Value>,
    #[serde(rename = "severityPerDayGrowing")]
    pub severity_per_day_growing: Option<f64>,
    #[serde(rename = "severityPerDayGrowingRandomFactor")]
    pub severity_per_day_growing_random_factor: Option<String>,
    #[serde(rename = "severityPerDayImmune")]
    pub severity_per_day_immune: Option<f64>,
    #[serde(rename = "severityPerDayNotImmune")]
    pub severity_per_day_not_immune: Option<f64>,
    #[serde(rename = "severityPerDayNotImmuneRandomFactor")]
    pub severity_per_day_not_immune_random_factor: Option<String>,
    #[serde(rename = "severityPerDayRemission")]
    pub severity_per_day_remission: Option<f64>,
    #[serde(rename = "severityPerDayRemissionRandomFactor")]
    pub severity_per_day_remission_random_factor: Option<String>,
    #[serde(rename = "severityPerDayTended")]
    pub severity_per_day_tended: Option<f64>,
    #[serde(rename = "showDaysToRecover")]
    pub show_days_to_recover: Option<bool>,
    #[serde(rename = "showHoursToRecover")]
    pub show_hours_to_recover: Option<bool>,
    #[serde(rename = "showTendQuality")]
    pub show_tend_quality: Option<bool>,
    pub sound: Option<String>,
    #[serde(rename = "stateEffecter")]
    pub state_effecter: Option<String>,
    #[serde(rename = "tendAllAtOnce")]
    pub tend_all_at_once: Option<bool>,
    pub thought: Option<String>,
    pub ticks: Option<i64>,
    #[serde(default)]
    pub tools: Vec<Tool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DecayPerDayPercentageLevelCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tool {
    #[serde(rename = "alwaysTreatAsWeapon")]
    pub always_treat_as_weapon: Option<bool>,
    pub capacities: Vec<String>,
    #[serde(rename = "cooldownTime")]
    pub cooldown_time: i64,
    pub label: String,
    pub power: ::serde_json::Value,
    #[serde(rename = "soundMeleeHit")]
    pub sound_melee_hit: Option<String>,
    #[serde(rename = "soundMeleeMiss")]
    pub sound_melee_miss: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HediffGiver {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "countToAffect")]
    pub count_to_affect: Option<i64>,
    pub hediff: String,
    #[serde(rename = "partsToAffect")]
    pub parts_to_affect: Vec<String>,
    #[serde(rename = "severityToMtbDaysCurve")]
    pub severity_to_mtb_days_curve: SeverityToMtbDaysCurve,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SeverityToMtbDaysCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct InjuryProps {
    #[serde(rename = "averagePainPerSeverityPermanent")]
    pub average_pain_per_severity_permanent: Option<f64>,
    #[serde(rename = "bleedRate")]
    pub bleed_rate: Option<f64>,
    #[serde(rename = "canMerge")]
    pub can_merge: Option<bool>,
    #[serde(rename = "destroyedLabel")]
    pub destroyed_label: Option<String>,
    #[serde(rename = "destroyedOutLabel")]
    pub destroyed_out_label: Option<String>,
    #[serde(rename = "painPerSeverity")]
    pub pain_per_severity: Option<f64>,
    #[serde(rename = "useRemovedLabel")]
    pub use_removed_label: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RemoveOnRedressChanceByDaysCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HediffGiverSetDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "hediffGivers")]
    pub hediff_givers: Vec<HediffGiver2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HediffGiver2 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "ageFractionChanceCurve")]
    pub age_fraction_chance_curve: Option<AgeFractionChanceCurve>,
    #[serde(rename = "ageFractionMtbDaysCurve")]
    pub age_fraction_mtb_days_curve: Option<AgeFractionMtbDaysCurve>,
    #[serde(rename = "allowOnBeggars")]
    pub allow_on_beggars: Option<bool>,
    #[serde(rename = "allowOnLodgers")]
    pub allow_on_lodgers: Option<bool>,
    #[serde(rename = "allowOnQuestReservedPawns")]
    pub allow_on_quest_reserved_pawns: Option<bool>,
    #[serde(rename = "allowOnQuestRewardPawns")]
    pub allow_on_quest_reward_pawns: Option<bool>,
    #[serde(rename = "averageSeverityPerDayBeforeGeneration")]
    pub average_severity_per_day_before_generation: Option<f64>,
    #[serde(rename = "canAffectAnyLivePart")]
    pub can_affect_any_live_part: Option<bool>,
    #[serde(rename = "chancePerDamagePct")]
    pub chance_per_damage_pct: Option<f64>,
    #[serde(rename = "countToAffect")]
    pub count_to_affect: Option<i64>,
    pub hediff: String,
    #[serde(rename = "hediffInsectoid")]
    pub hediff_insectoid: Option<String>,
    pub letter: Option<String>,
    #[serde(rename = "letterLabel")]
    pub letter_label: Option<String>,
    #[serde(rename = "minPlayerPopulation")]
    pub min_player_population: Option<i64>,
    #[serde(rename = "partsToAffect")]
    #[serde(default)]
    pub parts_to_affect: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AgeFractionChanceCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AgeFractionMtbDaysCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HibernatableStateDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HistoryAutoRecorderDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "graphColor")]
    pub graph_color: String,
    pub label: String,
    #[serde(rename = "recordTicksFrequency")]
    pub record_ticks_frequency: i64,
    #[serde(rename = "valueFormat")]
    pub value_format: String,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HistoryAutoRecorderGroupDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "devModeOnly")]
    pub dev_mode_only: Option<bool>,
    #[serde(rename = "fixedScale")]
    pub fixed_scale: Option<String>,
    #[serde(rename = "historyAutoRecorderDefs")]
    pub history_auto_recorder_defs: Vec<String>,
    #[serde(rename = "integersOnly")]
    pub integers_only: Option<bool>,
    pub label: String,
    #[serde(rename = "onlyPositiveValues")]
    pub only_positive_values: Option<bool>,
    #[serde(rename = "useFixedScale")]
    pub use_fixed_scale: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HistoryEventDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ImpactSoundTypeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "playOnlyIfHitPawn")]
    pub play_only_if_hit_pawn: Option<bool>,
    #[serde(rename = "soundDef")]
    pub sound_def: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ImplementOwnerTypeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "implementOwnerRuleName")]
    pub implement_owner_rule_name: Option<String>,
    #[serde(rename = "implementOwnerTypeValue")]
    pub implement_owner_type_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IncidentCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "needsParmsPoints")]
    pub needs_parms_points: Option<bool>,
    pub tale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IncidentDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "allowedBiomes")]
    #[serde(default)]
    pub allowed_biomes: Vec<String>,
    #[serde(rename = "baseChance")]
    pub base_chance: Option<::serde_json::Value>,
    #[serde(rename = "baseChanceWithRoyalty")]
    pub base_chance_with_royalty: Option<f64>,
    pub category: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "disabledWhen")]
    pub disabled_when: Option<DisabledWhen>,
    #[serde(rename = "diseaseIncident")]
    pub disease_incident: Option<String>,
    #[serde(rename = "diseasePartsToAffect")]
    #[serde(default)]
    pub disease_parts_to_affect: Vec<String>,
    #[serde(rename = "diseaseVictimFractionRange")]
    pub disease_victim_fraction_range: Option<String>,
    #[serde(rename = "durationDays")]
    pub duration_days: Option<String>,
    #[serde(rename = "earliestDay")]
    pub earliest_day: Option<i64>,
    #[serde(rename = "gameCondition")]
    pub game_condition: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "letterDef")]
    pub letter_def: Option<String>,
    #[serde(rename = "letterHyperlinkHediffDefs")]
    #[serde(default)]
    pub letter_hyperlink_hediff_defs: Vec<String>,
    #[serde(rename = "letterLabel")]
    pub letter_label: Option<String>,
    #[serde(rename = "letterText")]
    pub letter_text: Option<String>,
    #[serde(rename = "mechClusterBuilding")]
    pub mech_cluster_building: Option<String>,
    #[serde(rename = "minGreatestPopulation")]
    pub min_greatest_population: Option<i64>,
    #[serde(rename = "minRefireDays")]
    pub min_refire_days: Option<::serde_json::Value>,
    #[serde(rename = "minThreatPoints")]
    pub min_threat_points: Option<i64>,
    #[serde(rename = "mtbDaysByBiome")]
    pub mtb_days_by_biome: Option<MtbDaysByBiome>,
    #[serde(rename = "pawnFixedGender")]
    pub pawn_fixed_gender: Option<String>,
    #[serde(rename = "pawnKind")]
    pub pawn_kind: Option<String>,
    #[serde(rename = "pawnMustBeCapableOfViolence")]
    pub pawn_must_be_capable_of_violence: Option<bool>,
    #[serde(rename = "pointsScaleable")]
    pub points_scaleable: Option<bool>,
    #[serde(rename = "populationEffect")]
    pub population_effect: Option<String>,
    #[serde(rename = "questScriptDef")]
    pub quest_script_def: Option<String>,
    #[serde(rename = "refireCheckTags")]
    #[serde(default)]
    pub refire_check_tags: Vec<String>,
    #[serde(rename = "requireColonistsPresent")]
    pub require_colonists_present: Option<bool>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub tale: Option<String>,
    #[serde(rename = "targetTags")]
    #[serde(default)]
    pub target_tags: Vec<String>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DisabledWhen {
    #[serde(rename = "extremeWeatherIncidentsDisabled")]
    pub extreme_weather_incidents_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MtbDaysByBiome {
    #[serde(rename = "AridShrubland")]
    pub arid_shrubland: i64,
    #[serde(rename = "BorealForest")]
    pub boreal_forest: i64,
    #[serde(rename = "ColdBog")]
    pub cold_bog: i64,
    #[serde(rename = "Desert")]
    pub desert: i64,
    #[serde(rename = "ExtremeDesert")]
    pub extreme_desert: i64,
    #[serde(rename = "IceSheet")]
    pub ice_sheet: i64,
    #[serde(rename = "SeaIce")]
    pub sea_ice: i64,
    #[serde(rename = "TemperateForest")]
    pub temperate_forest: i64,
    #[serde(rename = "TemperateSwamp")]
    pub temperate_swamp: i64,
    #[serde(rename = "TropicalRainforest")]
    pub tropical_rainforest: i64,
    #[serde(rename = "TropicalSwamp")]
    pub tropical_swamp: i64,
    #[serde(rename = "Tundra")]
    pub tundra: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IncidentTargetTagDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct InspirationDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowedOnDownedPawns")]
    pub allowed_on_downed_pawns: Option<bool>,
    #[serde(rename = "associatedSkills")]
    #[serde(default)]
    pub associated_skills: Vec<String>,
    #[serde(rename = "baseDurationDays")]
    pub base_duration_days: i64,
    #[serde(rename = "baseInspectLine")]
    pub base_inspect_line: String,
    #[serde(rename = "beginLetter")]
    pub begin_letter: String,
    #[serde(rename = "beginLetterDef")]
    pub begin_letter_def: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "endMessage")]
    pub end_message: String,
    pub label: String,
    #[serde(rename = "requiredAnyNonDisabledWorkType")]
    #[serde(default)]
    pub required_any_non_disabled_work_type: Vec<String>,
    #[serde(rename = "requiredAnySkill")]
    pub required_any_skill: Option<RequiredAnySkill>,
    #[serde(rename = "requiredCapacities")]
    #[serde(default)]
    pub required_capacities: Vec<String>,
    #[serde(rename = "requiredNonDisabledStats")]
    #[serde(default)]
    pub required_non_disabled_stats: Vec<String>,
    #[serde(rename = "requiredNonDisabledWorkTags")]
    pub required_non_disabled_work_tags: Option<String>,
    #[serde(rename = "requiredNonDisabledWorkTypes")]
    #[serde(default)]
    pub required_non_disabled_work_types: Vec<String>,
    #[serde(rename = "requiredSkills")]
    pub required_skills: Option<RequiredSkills>,
    #[serde(rename = "statFactors")]
    pub stat_factors: Option<StatFactors>,
    #[serde(rename = "statOffsets")]
    pub stat_offsets: Option<StatOffsets>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RequiredAnySkill {
    #[serde(rename = "Artistic")]
    pub artistic: i64,
    #[serde(rename = "Construction")]
    pub construction: i64,
    #[serde(rename = "Crafting")]
    pub crafting: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RequiredSkills {
    #[serde(rename = "Animals")]
    pub animals: Option<i64>,
    #[serde(rename = "Medicine")]
    pub medicine: Option<i64>,
    #[serde(rename = "Social")]
    pub social: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatFactors {
    #[serde(rename = "MoveSpeed")]
    pub move_speed: Option<f64>,
    #[serde(rename = "WorkSpeedGlobal")]
    pub work_speed_global: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatOffsets {
    #[serde(rename = "ShootingAccuracyPawn")]
    pub shooting_accuracy_pawn: Option<i64>,
    #[serde(rename = "TradePriceImprovement")]
    pub trade_price_improvement: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct InstructionDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "actionTagsAllowed")]
    #[serde(default)]
    pub action_tags_allowed: Vec<String>,
    pub concept: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "endTutorial")]
    pub end_tutorial: Option<bool>,
    #[serde(rename = "eventTagInitiate")]
    pub event_tag_initiate: String,
    #[serde(rename = "eventTagInitiateSource")]
    pub event_tag_initiate_source: Option<String>,
    #[serde(rename = "eventTagsEnd")]
    #[serde(default)]
    pub event_tags_end: Vec<String>,
    #[serde(rename = "giveOnActivateCount")]
    pub give_on_activate_count: Option<i64>,
    #[serde(rename = "giveOnActivateDef")]
    pub give_on_activate_def: Option<String>,
    #[serde(rename = "highlightTags")]
    #[serde(default)]
    pub highlight_tags: Vec<String>,
    #[serde(rename = "instructionClass")]
    pub instruction_class: Option<String>,
    #[serde(rename = "onMapInstruction")]
    pub on_map_instruction: Option<String>,
    #[serde(rename = "recipeDef")]
    pub recipe_def: Option<String>,
    #[serde(rename = "recipeTargetCount")]
    pub recipe_target_count: Option<i64>,
    #[serde(rename = "rejectInputMessage")]
    pub reject_input_message: Option<String>,
    #[serde(rename = "resetBuildDesignatorStuffs")]
    pub reset_build_designator_stuffs: Option<bool>,
    #[serde(rename = "startCentered")]
    pub start_centered: Option<bool>,
    #[serde(rename = "targetCount")]
    pub target_count: Option<i64>,
    pub text: String,
    #[serde(rename = "thingDef")]
    pub thing_def: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct InteractionDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "initiatorThought")]
    pub initiator_thought: Option<String>,
    #[serde(rename = "initiatorXpGainAmount")]
    pub initiator_xp_gain_amount: Option<i64>,
    #[serde(rename = "initiatorXpGainSkill")]
    pub initiator_xp_gain_skill: Option<String>,
    #[serde(rename = "interactionMote")]
    pub interaction_mote: Option<String>,
    pub label: String,
    #[serde(rename = "logRulesInitiator")]
    pub log_rules_initiator: LogRulesInitiator,
    #[serde(rename = "recipientThought")]
    pub recipient_thought: Option<String>,
    #[serde(rename = "socialFightBaseChance")]
    pub social_fight_base_chance: Option<f64>,
    pub symbol: Option<String>,
    #[serde(rename = "symbolSource")]
    pub symbol_source: Option<String>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LogRulesInitiator {
    #[serde(rename = "rulesStrings")]
    pub rules_strings: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct InventoryStockGroupDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "defaultThingDef")]
    pub default_thing_def: String,
    #[serde(rename = "thingDefs")]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IssueDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowMultiplePrecepts")]
    pub allow_multiple_precepts: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<::serde_json::Value>,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JobDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowOpportunisticPrefix")]
    pub allow_opportunistic_prefix: Option<bool>,
    #[serde(rename = "alwaysShowWeapon")]
    pub always_show_weapon: Option<bool>,
    #[serde(rename = "carryThingAfterJob")]
    pub carry_thing_after_job: Option<bool>,
    #[serde(rename = "casualInterruptible")]
    pub casual_interruptible: Option<bool>,
    #[serde(rename = "checkOverrideOnDamage")]
    pub check_override_on_damage: Option<String>,
    #[serde(rename = "collideWithPawns")]
    pub collide_with_pawns: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "driverClass")]
    pub driver_class: String,
    #[serde(rename = "dropThingBeforeJob")]
    pub drop_thing_before_job: Option<bool>,
    #[serde(rename = "isIdle")]
    pub is_idle: Option<bool>,
    #[serde(rename = "joyDuration")]
    pub joy_duration: Option<i64>,
    #[serde(rename = "joyKind")]
    pub joy_kind: Option<String>,
    #[serde(rename = "joyMaxParticipants")]
    pub joy_max_participants: Option<i64>,
    #[serde(rename = "joySkill")]
    pub joy_skill: Option<String>,
    #[serde(rename = "joyXpPerTick")]
    pub joy_xp_per_tick: Option<f64>,
    #[serde(rename = "makeTargetPrisoner")]
    pub make_target_prisoner: Option<bool>,
    #[serde(rename = "neverFleeFromEnemies")]
    pub never_flee_from_enemies: Option<bool>,
    #[serde(rename = "neverShowWeapon")]
    pub never_show_weapon: Option<bool>,
    #[serde(rename = "playerInterruptible")]
    pub player_interruptible: Option<bool>,
    #[serde(rename = "reportString")]
    pub report_string: Option<String>,
    pub suspendable: Option<bool>,
    #[serde(rename = "taleOnCompletion")]
    pub tale_on_completion: Option<String>,
    #[serde(rename = "waitAfterArriving")]
    pub wait_after_arriving: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JoyGiverDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "baseChance")]
    pub base_chance: ::serde_json::Value,
    #[serde(rename = "canDoWhileInBed")]
    pub can_do_while_in_bed: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "desireSit")]
    pub desire_sit: Option<bool>,
    #[serde(rename = "giverClass")]
    pub giver_class: String,
    #[serde(rename = "jobDef")]
    pub job_def: Option<String>,
    #[serde(rename = "joyKind")]
    pub joy_kind: String,
    #[serde(rename = "pctPawnsEverDo")]
    pub pct_pawns_ever_do: Option<f64>,
    #[serde(rename = "requireChair")]
    pub require_chair: Option<bool>,
    #[serde(rename = "requiredCapacities")]
    #[serde(default)]
    pub required_capacities: Vec<String>,
    #[serde(rename = "requiresEnjoyOutdoors")]
    pub requires_enjoy_outdoors: Option<bool>,
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<String>,
    #[serde(rename = "unroofedOnly")]
    pub unroofed_only: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JoyKindDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "needsThing")]
    pub needs_thing: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct KeyBindingCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "checkForConflicts")]
    pub check_for_conflicts: Vec<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "isGameUniversal")]
    pub is_game_universal: Option<bool>,
    pub label: String,
    #[serde(rename = "selfConflicting")]
    pub self_conflicting: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct KeyBindingDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    pub category: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultKeyCodeA")]
    pub default_key_code_a: Option<String>,
    #[serde(rename = "defaultKeyCodeB")]
    pub default_key_code_b: Option<String>,
    #[serde(rename = "devModeOnly")]
    pub dev_mode_only: Option<bool>,
    #[serde(rename = "extraConflictTags")]
    #[serde(default)]
    pub extra_conflict_tags: Vec<String>,
    pub label: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LetterDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "arriveSound")]
    pub arrive_sound: Option<String>,
    pub bounce: Option<bool>,
    pub color: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "flashColor")]
    pub flash_color: Option<String>,
    #[serde(rename = "flashInterval")]
    pub flash_interval: Option<i64>,
    #[serde(rename = "letterClass")]
    pub letter_class: Option<String>,
    #[serde(rename = "pauseMode")]
    pub pause_mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LifeStageDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    pub adjective: Option<String>,
    #[serde(rename = "bodySizeFactor")]
    pub body_size_factor: Option<f64>,
    #[serde(rename = "caravanRideable")]
    pub caravan_rideable: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "foodMaxFactor")]
    pub food_max_factor: Option<::serde_json::Value>,
    #[serde(rename = "healthScaleFactor")]
    pub health_scale_factor: Option<f64>,
    #[serde(rename = "hungerRateFactor")]
    pub hunger_rate_factor: Option<f64>,
    pub label: Option<String>,
    #[serde(rename = "marketValueFactor")]
    pub market_value_factor: Option<f64>,
    #[serde(rename = "meleeDamageFactor")]
    pub melee_damage_factor: Option<f64>,
    pub milkable: Option<bool>,
    pub reproductive: Option<bool>,
    pub shearable: Option<bool>,
    #[serde(rename = "statFactors")]
    pub stat_factors: Option<StatFactors2>,
    pub visible: Option<bool>,
    #[serde(rename = "voxPitch")]
    pub vox_pitch: Option<f64>,
    #[serde(rename = "voxVolume")]
    pub vox_volume: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatFactors2 {
    #[serde(rename = "MoveSpeed")]
    pub move_speed: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LogEntryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "iconDamaged")]
    pub icon_damaged: String,
    #[serde(rename = "iconDamagedFromInstigator")]
    pub icon_damaged_from_instigator: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MainButtonDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "buttonVisible")]
    pub button_visible: Option<bool>,
    #[serde(rename = "canBeTutorDenied")]
    pub can_be_tutor_denied: Option<bool>,
    #[serde(rename = "closesWorldView")]
    pub closes_world_view: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "defaultHotKey")]
    pub default_hot_key: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    pub label: String,
    pub minimized: Option<bool>,
    pub order: Option<i64>,
    #[serde(rename = "tabWindowClass")]
    pub tab_window_class: Option<String>,
    #[serde(rename = "validWithoutMap")]
    pub valid_without_map: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ManeuverDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "combatLogRulesDeflect")]
    pub combat_log_rules_deflect: String,
    #[serde(rename = "combatLogRulesDodge")]
    pub combat_log_rules_dodge: String,
    #[serde(rename = "combatLogRulesHit")]
    pub combat_log_rules_hit: String,
    #[serde(rename = "combatLogRulesMiss")]
    pub combat_log_rules_miss: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "logEntryDef")]
    pub log_entry_def: String,
    #[serde(rename = "requiredCapacity")]
    pub required_capacity: String,
    pub verb: Verb,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Verb {
    #[serde(rename = "ai_IsBuildingDestroyer")]
    pub ai_is_building_destroyer: Option<bool>,
    #[serde(rename = "bodypartTagTarget")]
    pub bodypart_tag_target: Option<String>,
    #[serde(rename = "impactFleck")]
    pub impact_fleck: Option<String>,
    #[serde(rename = "linkedBodyPartsGroup")]
    pub linked_body_parts_group: Option<String>,
    #[serde(rename = "meleeDamageDef")]
    pub melee_damage_def: Option<String>,
    #[serde(rename = "minIntelligence")]
    pub min_intelligence: Option<String>,
    #[serde(rename = "verbClass")]
    pub verb_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MapGeneratorDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "genSteps")]
    pub gen_steps: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MeditationFocusDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "incompatibleBackstoriesAny")]
    #[serde(default)]
    pub incompatible_backstories_any: Vec<IncompatibleBackstoriesAny>,
    pub label: String,
    #[serde(rename = "requiredBackstoriesAny")]
    #[serde(default)]
    pub required_backstories_any: Vec<RequiredBackstoriesAny>,
    #[serde(rename = "requiresRoyalTitle")]
    pub requires_royal_title: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IncompatibleBackstoriesAny {
    #[serde(rename = "categoryName")]
    pub category_name: String,
    pub slot: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RequiredBackstoriesAny {
    #[serde(rename = "categoryName")]
    pub category_name: String,
    pub slot: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MentalBreakDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "baseCommonality")]
    pub base_commonality: Option<::serde_json::Value>,
    #[serde(rename = "commonalityFactorPerPopulationCurve")]
    pub commonality_factor_per_population_curve: Option<CommonalityFactorPerPopulationCurve>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub intensity: String,
    pub label: Option<String>,
    #[serde(rename = "mentalState")]
    pub mental_state: Option<String>,
    #[serde(rename = "requiredTrait")]
    pub required_trait: Option<String>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonalityFactorPerPopulationCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MentalStateDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "allowBeatfire")]
    pub allow_beatfire: Option<bool>,
    #[serde(rename = "baseInspectLine")]
    pub base_inspect_line: Option<String>,
    #[serde(rename = "beginLetter")]
    pub begin_letter: Option<String>,
    #[serde(rename = "beginLetterDef")]
    pub begin_letter_def: Option<String>,
    #[serde(rename = "beginLetterLabel")]
    pub begin_letter_label: Option<String>,
    #[serde(rename = "blockInteractionInitiationExcept")]
    #[serde(default)]
    pub block_interaction_initiation_except: Vec<Option<String>>,
    #[serde(rename = "blockInteractionRecipientExcept")]
    #[serde(default)]
    pub block_interaction_recipient_except: Vec<Option<String>>,
    #[serde(rename = "blockNormalThoughts")]
    pub block_normal_thoughts: Option<bool>,
    #[serde(rename = "blockRandomInteraction")]
    pub block_random_interaction: Option<bool>,
    pub category: Option<String>,
    #[serde(rename = "colonistsOnly")]
    pub colonists_only: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "drugCategory")]
    pub drug_category: Option<String>,
    #[serde(rename = "escapingPrisonersIgnore")]
    pub escaping_prisoners_ignore: Option<bool>,
    #[serde(rename = "ignoreDrugPolicy")]
    pub ignore_drug_policy: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "maxTicksBeforeRecovery")]
    pub max_ticks_before_recovery: Option<i64>,
    #[serde(rename = "minTicksBeforeRecovery")]
    pub min_ticks_before_recovery: Option<i64>,
    #[serde(rename = "moodRecoveryThought")]
    pub mood_recovery_thought: Option<String>,
    #[serde(rename = "nameColor")]
    pub name_color: Option<String>,
    #[serde(rename = "prisonersCanDo")]
    pub prisoners_can_do: Option<bool>,
    #[serde(rename = "recoverFromCollapsingExhausted")]
    pub recover_from_collapsing_exhausted: Option<bool>,
    #[serde(rename = "recoverFromDowned")]
    pub recover_from_downed: Option<bool>,
    #[serde(rename = "recoverFromSleep")]
    pub recover_from_sleep: Option<bool>,
    #[serde(rename = "recoveryMessage")]
    pub recovery_message: Option<::serde_json::Value>,
    #[serde(rename = "recoveryMtbDays")]
    pub recovery_mtb_days: Option<f64>,
    #[serde(rename = "requiredCapacities")]
    #[serde(default)]
    pub required_capacities: Vec<String>,
    #[serde(rename = "slavesCanDo")]
    pub slaves_can_do: Option<bool>,
    #[serde(rename = "stateClass")]
    pub state_class: Option<String>,
    #[serde(rename = "stateEffecter")]
    pub state_effecter: Option<String>,
    pub tale: Option<String>,
    #[serde(rename = "unspawnedCanDo")]
    pub unspawned_can_do: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MessageTypeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub sound: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NeedDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "baseLevel")]
    pub base_level: Option<f64>,
    #[serde(rename = "colonistAndPrisonersOnly")]
    pub colonist_and_prisoners_only: Option<bool>,
    #[serde(rename = "colonistsOnly")]
    pub colonists_only: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "fallPerDay")]
    pub fall_per_day: Option<f64>,
    #[serde(rename = "freezeInMentalState")]
    pub freeze_in_mental_state: Option<bool>,
    #[serde(rename = "freezeWhileSleeping")]
    pub freeze_while_sleeping: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "listPriority")]
    pub list_priority: Option<i64>,
    pub major: Option<bool>,
    #[serde(rename = "minIntelligence")]
    pub min_intelligence: Option<String>,
    #[serde(rename = "needClass")]
    pub need_class: Option<String>,
    #[serde(rename = "neverOnPrisoner")]
    pub never_on_prisoner: Option<bool>,
    #[serde(rename = "neverOnSlave")]
    pub never_on_slave: Option<bool>,
    #[serde(rename = "nullifyingPrecepts")]
    #[serde(default)]
    pub nullifying_precepts: Vec<NullifyingPrecept>,
    #[serde(rename = "onlyIfCausedByHediff")]
    pub only_if_caused_by_hediff: Option<bool>,
    #[serde(rename = "scaleBar")]
    pub scale_bar: Option<bool>,
    #[serde(rename = "seekerFallPerHour")]
    pub seeker_fall_per_hour: Option<f64>,
    #[serde(rename = "seekerRisePerHour")]
    pub seeker_rise_per_hour: Option<f64>,
    #[serde(rename = "showForCaravanMembers")]
    pub show_for_caravan_members: Option<bool>,
    #[serde(rename = "showOnNeedList")]
    pub show_on_need_list: Option<bool>,
    #[serde(rename = "tutorHighlightTag")]
    pub tutor_highlight_tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NullifyingPrecept {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OrderedTakeGroupDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnCapacityDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "labelMechanoids")]
    pub label_mechanoids: Option<String>,
    #[serde(rename = "lethalFlesh")]
    pub lethal_flesh: Option<bool>,
    #[serde(rename = "lethalMechanoids")]
    pub lethal_mechanoids: Option<bool>,
    #[serde(rename = "listOrder")]
    pub list_order: i64,
    #[serde(rename = "minForCapable")]
    pub min_for_capable: Option<f64>,
    #[serde(rename = "minValue")]
    pub min_value: Option<f64>,
    #[serde(rename = "showOnAnimals")]
    pub show_on_animals: Option<bool>,
    #[serde(rename = "showOnCaravanHealthTab")]
    pub show_on_caravan_health_tab: Option<bool>,
    #[serde(rename = "showOnMechanoids")]
    pub show_on_mechanoids: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
    #[serde(rename = "zeroIfCannotBeAwake")]
    pub zero_if_cannot_be_awake: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnColumnDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub gap: Option<i64>,
    #[serde(rename = "headerIcon")]
    pub header_icon: Option<String>,
    #[serde(rename = "headerTip")]
    pub header_tip: Option<String>,
    #[serde(rename = "ignoreWhenCalculatingOptimalTableSize")]
    pub ignore_when_calculating_optimal_table_size: Option<bool>,
    pub label: Option<String>,
    pub paintable: Option<bool>,
    pub sortable: Option<bool>,
    pub width: Option<i64>,
    #[serde(rename = "widthPriority")]
    pub width_priority: Option<i64>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnGroupKindDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnKindDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "acceptArrestChanceFactor")]
    pub accept_arrest_chance_factor: Option<f64>,
    #[serde(rename = "aiAvoidCover")]
    pub ai_avoid_cover: Option<bool>,
    #[serde(rename = "alternateGraphicChance")]
    pub alternate_graphic_chance: Option<f64>,
    #[serde(rename = "alternateGraphics")]
    #[serde(default)]
    pub alternate_graphics: Vec<AlternateGraphic>,
    #[serde(rename = "apparelAllowHeadgearChance")]
    pub apparel_allow_headgear_chance: Option<::serde_json::Value>,
    #[serde(rename = "apparelColor")]
    pub apparel_color: Option<String>,
    #[serde(rename = "apparelIgnoreSeasons")]
    pub apparel_ignore_seasons: Option<bool>,
    #[serde(rename = "apparelMoney")]
    pub apparel_money: Option<String>,
    #[serde(rename = "apparelRequired")]
    #[serde(default)]
    pub apparel_required: Vec<String>,
    #[serde(rename = "apparelTags")]
    #[serde(default)]
    pub apparel_tags: Vec<String>,
    #[serde(rename = "backstoryCryptosleepCommonality")]
    pub backstory_cryptosleep_commonality: Option<::serde_json::Value>,
    #[serde(rename = "backstoryFilters")]
    #[serde(default)]
    pub backstory_filters: Vec<BackstoryFilter2>,
    #[serde(rename = "backstoryFiltersOverride")]
    #[serde(default)]
    pub backstory_filters_override: Vec<BackstoryFiltersOverride>,
    #[serde(rename = "baseRecruitDifficulty")]
    pub base_recruit_difficulty: Option<f64>,
    #[serde(rename = "biocodeWeaponChance")]
    pub biocode_weapon_chance: Option<f64>,
    #[serde(rename = "canArriveManhunter")]
    pub can_arrive_manhunter: Option<bool>,
    #[serde(rename = "canBeSapper")]
    pub can_be_sapper: Option<bool>,
    #[serde(rename = "chemicalAddictionChance")]
    pub chemical_addiction_chance: Option<f64>,
    #[serde(rename = "combatEnhancingDrugsChance")]
    pub combat_enhancing_drugs_chance: Option<f64>,
    #[serde(rename = "combatEnhancingDrugsCount")]
    pub combat_enhancing_drugs_count: Option<String>,
    #[serde(rename = "combatPower")]
    pub combat_power: Option<i64>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultFactionType")]
    pub default_faction_type: Option<String>,
    #[serde(rename = "defendPointRadius")]
    pub defend_point_radius: Option<i64>,
    #[serde(rename = "destroyGearOnDrop")]
    pub destroy_gear_on_drop: Option<bool>,
    #[serde(rename = "disallowedTraits")]
    #[serde(default)]
    pub disallowed_traits: Vec<String>,
    #[serde(rename = "ecoSystemWeight")]
    pub eco_system_weight: Option<::serde_json::Value>,
    #[serde(rename = "factionLeader")]
    pub faction_leader: Option<bool>,
    #[serde(rename = "forceNormalGearQuality")]
    pub force_normal_gear_quality: Option<bool>,
    #[serde(rename = "gearHealthRange")]
    pub gear_health_range: Option<String>,
    #[serde(rename = "initialResistanceRange")]
    pub initial_resistance_range: Option<String>,
    #[serde(rename = "initialWillRange")]
    pub initial_will_range: Option<::serde_json::Value>,
    #[serde(rename = "invFoodDef")]
    pub inv_food_def: Option<String>,
    #[serde(rename = "invNutrition")]
    pub inv_nutrition: Option<::serde_json::Value>,
    #[serde(rename = "inventoryOptions")]
    pub inventory_options: Option<InventoryOptions>,
    #[serde(rename = "isFighter")]
    pub is_fighter: Option<bool>,
    #[serde(rename = "isGoodBreacher")]
    pub is_good_breacher: Option<bool>,
    #[serde(rename = "itemQuality")]
    pub item_quality: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "labelFemale")]
    pub label_female: Option<String>,
    #[serde(rename = "labelFemalePlural")]
    pub label_female_plural: Option<String>,
    #[serde(rename = "labelMale")]
    pub label_male: Option<String>,
    #[serde(rename = "labelPlural")]
    pub label_plural: Option<String>,
    #[serde(rename = "lifeStages")]
    #[serde(default)]
    pub life_stages: Vec<LifeStage>,
    #[serde(rename = "maxGenerationAge")]
    pub max_generation_age: Option<i64>,
    #[serde(rename = "minGenerationAge")]
    pub min_generation_age: Option<i64>,
    pub race: Option<String>,
    #[serde(rename = "requiredWorkTags")]
    #[serde(default)]
    pub required_work_tags: Vec<String>,
    #[serde(default)]
    pub skills: Vec<Skill>,
    #[serde(rename = "techHediffsChance")]
    pub tech_hediffs_chance: Option<f64>,
    #[serde(rename = "techHediffsMoney")]
    pub tech_hediffs_money: Option<String>,
    #[serde(rename = "techHediffsRequired")]
    #[serde(default)]
    pub tech_hediffs_required: Vec<String>,
    #[serde(rename = "techHediffsTags")]
    #[serde(default)]
    pub tech_hediffs_tags: Vec<String>,
    pub trader: Option<bool>,
    #[serde(rename = "weaponMoney")]
    pub weapon_money: Option<String>,
    #[serde(rename = "weaponTags")]
    #[serde(default)]
    pub weapon_tags: Vec<Option<String>>,
    #[serde(rename = "wildGroupSize")]
    pub wild_group_size: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AlternateGraphic {
    pub color: Option<String>,
    #[serde(rename = "texPath")]
    pub tex_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BackstoryFilter2 {
    pub categories: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BackstoryFiltersOverride {
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(rename = "categoriesAdulthood")]
    #[serde(default)]
    pub categories_adulthood: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct InventoryOptions {
    #[serde(rename = "skipChance")]
    pub skip_chance: Option<f64>,
    #[serde(rename = "subOptionsChooseOne")]
    pub sub_options_choose_one: Vec<SubOptionsChooseOne>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubOptionsChooseOne {
    #[serde(rename = "countRange")]
    pub count_range: i64,
    #[serde(rename = "thingDef")]
    pub thing_def: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LifeStage {
    #[serde(rename = "bodyGraphicData")]
    pub body_graphic_data: BodyGraphicData,
    #[serde(rename = "butcherBodyPart")]
    pub butcher_body_part: Option<ButcherBodyPart>,
    #[serde(rename = "dessicatedBodyGraphicData")]
    pub dessicated_body_graphic_data: Option<DessicatedBodyGraphicData>,
    #[serde(rename = "femaleDessicatedBodyGraphicData")]
    pub female_dessicated_body_graphic_data: Option<FemaleDessicatedBodyGraphicData>,
    #[serde(rename = "femaleGraphicData")]
    pub female_graphic_data: Option<FemaleGraphicData>,
    pub label: Option<String>,
    #[serde(rename = "labelMale")]
    pub label_male: Option<String>,
    #[serde(rename = "labelPlural")]
    pub label_plural: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BodyGraphicData {
    pub color: Option<String>,
    #[serde(rename = "drawSize")]
    pub draw_size: ::serde_json::Value,
    #[serde(rename = "graphicClass")]
    pub graphic_class: Option<String>,
    #[serde(rename = "shaderType")]
    pub shader_type: Option<String>,
    #[serde(rename = "shadowData")]
    pub shadow_data: Option<ShadowData>,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShadowData {
    pub offset: Option<String>,
    pub volume: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ButcherBodyPart {
    #[serde(rename = "allowFemale")]
    pub allow_female: Option<bool>,
    #[serde(rename = "bodyPartGroup")]
    pub body_part_group: String,
    pub thing: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DessicatedBodyGraphicData {
    #[serde(rename = "drawSize")]
    pub draw_size: ::serde_json::Value,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FemaleDessicatedBodyGraphicData {
    #[serde(rename = "drawSize")]
    pub draw_size: f64,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FemaleGraphicData {
    pub color: Option<String>,
    #[serde(rename = "drawSize")]
    pub draw_size: f64,
    #[serde(rename = "shaderType")]
    pub shader_type: Option<String>,
    #[serde(rename = "shadowData")]
    pub shadow_data: Option<ShadowData2>,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShadowData2 {
    pub offset: String,
    pub volume: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Skill {
    pub range: String,
    pub skill: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnRelationDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "diedThought")]
    pub died_thought: Option<String>,
    #[serde(rename = "diedThoughtFemale")]
    pub died_thought_female: Option<String>,
    #[serde(rename = "familyByBloodRelation")]
    pub family_by_blood_relation: Option<bool>,
    #[serde(rename = "generationChanceFactor")]
    pub generation_chance_factor: Option<f64>,
    pub implied: bool,
    pub importance: i64,
    #[serde(rename = "incestOpinionOffset")]
    pub incest_opinion_offset: Option<i64>,
    #[serde(rename = "killedThought")]
    pub killed_thought: Option<String>,
    #[serde(rename = "killedThoughtFemale")]
    pub killed_thought_female: Option<String>,
    pub label: String,
    #[serde(rename = "labelFemale")]
    pub label_female: Option<String>,
    #[serde(rename = "lostThought")]
    pub lost_thought: Option<String>,
    #[serde(rename = "lostThoughtFemale")]
    pub lost_thought_female: Option<String>,
    #[serde(rename = "opinionOffset")]
    pub opinion_offset: i64,
    pub reflexive: Option<bool>,
    #[serde(rename = "romanceChanceFactor")]
    pub romance_chance_factor: Option<f64>,
    #[serde(rename = "soldThoughts")]
    #[serde(default)]
    pub sold_thoughts: Vec<String>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnTableDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub columns: Vec<::serde_json::Value>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "minWidth")]
    pub min_width: Option<i64>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PawnsArrivalModeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "minTechLevel")]
    pub min_tech_level: Option<String>,
    #[serde(rename = "pointsFactorCurve")]
    pub points_factor_curve: Option<PointsFactorCurve>,
    #[serde(rename = "selectionWeightCurve")]
    pub selection_weight_curve: SelectionWeightCurve,
    #[serde(rename = "textEnemy")]
    pub text_enemy: String,
    #[serde(rename = "textFriendly")]
    pub text_friendly: Option<String>,
    #[serde(rename = "textWillArrive")]
    pub text_will_arrive: String,
    #[serde(rename = "walkIn")]
    pub walk_in: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PointsFactorCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SelectionWeightCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PreceptDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "approvesOfSlavery")]
    pub approves_of_slavery: Option<bool>,
    #[serde(rename = "associatedMemes")]
    #[serde(default)]
    pub associated_memes: Vec<AssociatedMeme>,
    pub classic: Option<bool>,
    #[serde(default)]
    pub comps: Vec<Comp3>,
    #[serde(rename = "countsTowardsPreceptLimit")]
    pub counts_towards_precept_limit: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultSelectionWeight")]
    pub default_selection_weight: Option<i64>,
    pub description: Option<String>,
    #[serde(rename = "displayOrderInImpact")]
    pub display_order_in_impact: Option<i64>,
    #[serde(rename = "displayOrderInIssue")]
    pub display_order_in_issue: Option<i64>,
    pub impact: Option<String>,
    pub issue: String,
    pub label: Option<String>,
    #[serde(rename = "preceptClass")]
    pub precept_class: Option<String>,
    #[serde(rename = "selectionWeight")]
    pub selection_weight: Option<f64>,
    pub visible: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AssociatedMeme {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp3 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    pub chance: Option<f64>,
    pub description: Option<String>,
    #[serde(rename = "displayDescription")]
    pub display_description: Option<bool>,
    #[serde(rename = "eventDef")]
    pub event_def: Option<String>,
    pub gender: Option<String>,
    pub thought: Option<String>,
    #[serde(rename = "tooltipShowMoodRange")]
    pub tooltip_show_mood_range: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PrisonerInteractionModeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowOnWildMan")]
    pub allow_on_wild_man: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    pub label: String,
    #[serde(rename = "listOrder")]
    pub list_order: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct QuestScriptDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "autoAccept")]
    pub auto_accept: Option<bool>,
    #[serde(rename = "canGiveRoyalFavor")]
    pub can_give_royal_favor: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "defaultChallengeRating")]
    pub default_challenge_rating: Option<i64>,
    #[serde(rename = "defaultCharity")]
    pub default_charity: Option<bool>,
    #[serde(rename = "defaultHidden")]
    pub default_hidden: Option<bool>,
    #[serde(rename = "expireDaysRange")]
    pub expire_days_range: Option<::serde_json::Value>,
    #[serde(rename = "failedOrExpiredHistoryEvent")]
    pub failed_or_expired_history_event: Option<FailedOrExpiredHistoryEvent>,
    #[serde(rename = "isRootSpecial")]
    pub is_root_special: Option<bool>,
    #[serde(rename = "questDescriptionRules")]
    pub quest_description_rules: Option<QuestDescriptionRules>,
    #[serde(rename = "questNameRules")]
    pub quest_name_rules: Option<QuestNameRules>,
    pub root: Root2,
    #[serde(rename = "rootIncreasesPopulation")]
    pub root_increases_population: Option<bool>,
    #[serde(rename = "rootMinPoints")]
    pub root_min_points: Option<i64>,
    #[serde(rename = "rootMinProgressScore")]
    pub root_min_progress_score: Option<i64>,
    #[serde(rename = "rootSelectionWeight")]
    pub root_selection_weight: Option<::serde_json::Value>,
    #[serde(rename = "successHistoryEvent")]
    pub success_history_event: Option<SuccessHistoryEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FailedOrExpiredHistoryEvent {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct QuestDescriptionRules {
    #[serde(rename = "rulesStrings")]
    pub rules_strings: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct QuestNameRules {
    #[serde(rename = "rulesStrings")]
    pub rules_strings: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root2 {
    #[serde(rename = "Class")]
    pub class: String,
    pub curve: Option<Curve>,
    pub faction: Option<String>,
    #[serde(rename = "hiddenSitePartsPossible")]
    pub hidden_site_parts_possible: Option<String>,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(rename = "singleSitePartRules")]
    pub single_site_part_rules: Option<SingleSitePartRules>,
    #[serde(rename = "sitePartsParams")]
    pub site_parts_params: Option<String>,
    #[serde(rename = "storeAs")]
    pub store_as: Option<String>,
    pub tile: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Curve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    #[serde(rename = "allowActiveTradeRequest")]
    pub allow_active_trade_request: Option<bool>,
    #[serde(rename = "allowEnemy")]
    pub allow_enemy: Option<bool>,
    #[serde(rename = "allowPermanentEnemy")]
    pub allow_permanent_enemy: Option<bool>,
    #[serde(rename = "allowPermanentEnemyFaction")]
    pub allow_permanent_enemy_faction: Option<bool>,
    #[serde(rename = "arrivalMode")]
    pub arrival_mode: Option<String>,
    #[serde(rename = "canTimeoutOrFlee")]
    pub can_timeout_or_flee: Option<String>,
    #[serde(rename = "challengeRating")]
    pub challenge_rating: Option<String>,
    pub contents: Option<String>,
    pub curve: Option<Curve2>,
    #[serde(rename = "customLetterLabel")]
    pub custom_letter_label: Option<String>,
    #[serde(rename = "customLetterLabelRules")]
    pub custom_letter_label_rules: Option<String>,
    #[serde(rename = "customLetterText")]
    pub custom_letter_text: Option<String>,
    #[serde(rename = "customLetterTextRules")]
    pub custom_letter_text_rules: Option<String>,
    pub def: Option<String>,
    #[serde(rename = "defsToExcludeFromHyperlinks")]
    pub defs_to_exclude_from_hyperlinks: Option<String>,
    #[serde(rename = "delayTicks")]
    pub delay_ticks: Option<String>,
    #[serde(rename = "dontRequest")]
    #[serde(default)]
    pub dont_request: Vec<String>,
    pub duration: Option<String>,
    #[serde(rename = "elseNode")]
    pub else_node: Option<ElseNode>,
    pub faction: Option<String>,
    #[serde(rename = "factionOf")]
    pub faction_of: Option<String>,
    #[serde(rename = "historyDef")]
    pub history_def: Option<String>,
    #[serde(rename = "hostileWeight")]
    pub hostile_weight: Option<f64>,
    #[serde(rename = "inSignal")]
    pub in_signal: Option<String>,
    #[serde(rename = "inSignalChoiceUsed")]
    pub in_signal_choice_used: Option<String>,
    #[serde(rename = "inSignalDisable")]
    pub in_signal_disable: Option<String>,
    #[serde(rename = "inSignalLeave")]
    pub in_signal_leave: Option<String>,
    #[serde(rename = "isQuestTimeout")]
    pub is_quest_timeout: Option<bool>,
    #[serde(rename = "isSingleReward")]
    pub is_single_reward: Option<bool>,
    pub items: Option<String>,
    #[serde(rename = "joinPlayer")]
    pub join_player: Option<bool>,
    #[serde(rename = "kindDef")]
    pub kind_def: Option<String>,
    pub label: Option<Label>,
    #[serde(rename = "leaderMustBeSafe")]
    pub leader_must_be_safe: Option<bool>,
    #[serde(rename = "loopCount")]
    pub loop_count: Option<String>,
    pub map: Option<String>,
    #[serde(rename = "maxTileDistance")]
    pub max_tile_distance: Option<i64>,
    #[serde(rename = "mustBeFactionLeader")]
    pub must_be_faction_leader: Option<bool>,
    #[serde(rename = "mustBeHostileToFactionOf")]
    pub must_be_hostile_to_faction_of: Option<String>,
    #[serde(rename = "mustBePermanentEnemy")]
    pub must_be_permanent_enemy: Option<bool>,
    #[serde(rename = "mustHaveGoodwillRewardsEnabled")]
    pub must_have_goodwill_rewards_enabled: Option<bool>,
    pub name: Option<String>,
    pub node: Option<Node4>,
    #[serde(rename = "nodeIfChosenPawnSignalUsed")]
    pub node_if_chosen_pawn_signal_used: Option<NodeIfChosenPawnSignalUsed2>,
    #[serde(default)]
    pub nodes: Vec<Node9>,
    #[serde(rename = "outSignalComplete")]
    pub out_signal_complete: Option<String>,
    pub outcome: Option<String>,
    pub parms: Option<Parms4>,
    pub pawn: Option<String>,
    pub pawns: Option<String>,
    #[serde(rename = "peaceTalksCantExist")]
    pub peace_talks_cant_exist: Option<bool>,
    #[serde(rename = "playerCantBeAttackingCurrently")]
    pub player_cant_be_attacking_currently: Option<bool>,
    #[serde(rename = "preferCloserTiles")]
    pub prefer_closer_tiles: Option<bool>,
    #[serde(rename = "qualityGenerator")]
    pub quality_generator: Option<String>,
    #[serde(rename = "raidPawnKind")]
    pub raid_pawn_kind: Option<String>,
    pub range: Option<String>,
    #[serde(rename = "requestedThingCount")]
    pub requested_thing_count: Option<String>,
    #[serde(rename = "requestedThingDef")]
    pub requested_thing_def: Option<String>,
    #[serde(rename = "rewardDetailsHidden")]
    pub reward_details_hidden: Option<bool>,
    #[serde(rename = "sendStandardLetter")]
    pub send_standard_letter: Option<bool>,
    pub settlement: Option<String>,
    #[serde(rename = "sitePartDefs")]
    pub site_part_defs: Option<String>,
    #[serde(rename = "sitePartsParams")]
    pub site_parts_params: Option<String>,
    #[serde(rename = "sitePartsTags")]
    #[serde(default)]
    pub site_parts_tags: Vec<SitePartsTag>,
    #[serde(rename = "storeAs")]
    pub store_as: Option<String>,
    #[serde(rename = "storeEstimatedTravelTimeAs")]
    pub store_estimated_travel_time_as: Option<String>,
    #[serde(rename = "storeFactionAs")]
    pub store_faction_as: Option<String>,
    #[serde(rename = "storeFactionLeaderAs")]
    pub store_faction_leader_as: Option<String>,
    #[serde(rename = "storeHasQualityAs")]
    pub store_has_quality_as: Option<String>,
    #[serde(rename = "storeLoopCounterAs")]
    pub store_loop_counter_as: Option<String>,
    #[serde(rename = "storeMarketValueAs")]
    pub store_market_value_as: Option<String>,
    #[serde(rename = "storeSitePartsParamsAs")]
    pub store_site_parts_params_as: Option<String>,
    #[serde(rename = "storeThingAs")]
    pub store_thing_as: Option<String>,
    #[serde(rename = "storeThingCountAs")]
    pub store_thing_count_as: Option<String>,
    pub tag: Option<String>,
    pub text: Option<Text4>,
    pub thing: Option<String>,
    #[serde(rename = "thingSetMaker")]
    pub thing_set_maker: Option<String>,
    pub things: Option<String>,
    pub tile: Option<String>,
    #[serde(rename = "totalMarketValueRange")]
    pub total_market_value_range: Option<String>,
    #[serde(rename = "useDifficultyFactor")]
    pub use_difficulty_factor: Option<bool>,
    pub value: Option<String>,
    pub value1: Option<String>,
    pub value2: Option<::serde_json::Value>,
    #[serde(rename = "worldObject")]
    pub world_object: Option<String>,
    #[serde(rename = "worldObjects")]
    pub world_objects: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Curve2 {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ElseNode {
    #[serde(rename = "Class")]
    pub class: String,
    pub name: Option<String>,
    pub node: Option<Node2>,
    #[serde(default)]
    pub nodes: Vec<Node3>,
    pub value: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node2 {
    #[serde(rename = "Class")]
    pub class: String,
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node3 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "hostileWeight")]
    pub hostile_weight: Option<f64>,
    #[serde(rename = "mustBeFactionLeader")]
    pub must_be_faction_leader: Option<bool>,
    #[serde(rename = "mustBeNonHostileToPlayer")]
    pub must_be_non_hostile_to_player: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "selectionWeight")]
    pub selection_weight: f64,
    #[serde(rename = "storeAs")]
    pub store_as: Option<String>,
    pub value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Label {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node4 {
    #[serde(rename = "Class")]
    pub class: String,
    pub def: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub nodes: Vec<Node5>,
    pub outcome: Option<String>,
    pub parms: Option<Parms3>,
    pub prefix: Option<String>,
    #[serde(rename = "sendStandardLetter")]
    pub send_standard_letter: Option<bool>,
    #[serde(rename = "storeAs")]
    pub store_as: Option<String>,
    pub value: Option<::serde_json::Value>,
    pub value1: Option<String>,
    pub value2: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node5 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "addCampLootReward")]
    pub add_camp_loot_reward: Option<bool>,
    #[serde(rename = "customLetterLabel")]
    pub custom_letter_label: Option<CustomLetterLabel>,
    #[serde(rename = "customLetterText")]
    pub custom_letter_text: Option<CustomLetterText>,
    pub def: Option<String>,
    #[serde(rename = "delayTicks")]
    pub delay_ticks: Option<String>,
    #[serde(rename = "elseNode")]
    pub else_node: Option<ElseNode2>,
    #[serde(rename = "getRaidersFromMapParent")]
    pub get_raiders_from_map_parent: Option<String>,
    pub label: Option<Label2>,
    pub node: Option<Node6>,
    #[serde(rename = "nodeIfChosenPawnSignalUsed")]
    pub node_if_chosen_pawn_signal_used: Option<NodeIfChosenPawnSignalUsed>,
    #[serde(rename = "outSignals")]
    pub out_signals: Option<String>,
    pub outcome: Option<String>,
    pub parms: Option<Parms2>,
    #[serde(rename = "storeAs")]
    pub store_as: Option<String>,
    pub text: Option<Text2>,
    pub value: Option<String>,
    pub value1: Option<String>,
    pub value2: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CustomLetterLabel {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CustomLetterText {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ElseNode2 {
    #[serde(rename = "Class")]
    pub class: String,
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Label2 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node6 {
    #[serde(rename = "Class")]
    pub class: String,
    pub nodes: Vec<Node7>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node7 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "compareAs")]
    pub compare_as: Option<String>,
    pub def: Option<String>,
    pub node: Option<Node8>,
    pub parms: Option<Parms>,
    pub prefix: Option<String>,
    pub value1: Option<String>,
    pub value2: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node8 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "outSignals")]
    pub out_signals: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Parms {
    #[serde(rename = "customLetterLabel")]
    pub custom_letter_label: String,
    #[serde(rename = "customLetterLabelRules")]
    pub custom_letter_label_rules: String,
    #[serde(rename = "customLetterText")]
    pub custom_letter_text: String,
    #[serde(rename = "customLetterTextRules")]
    pub custom_letter_text_rules: String,
    #[serde(rename = "enemyFaction")]
    pub enemy_faction: String,
    #[serde(rename = "inSignal")]
    pub in_signal: String,
    pub map: String,
    pub points: String,
    pub tag: String,
    #[serde(rename = "walkInSpot")]
    pub walk_in_spot: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NodeIfChosenPawnSignalUsed {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "chosenPawnSignal")]
    pub chosen_pawn_signal: String,
    pub label: Label3,
    #[serde(rename = "letterDef")]
    pub letter_def: String,
    pub text: Text,
    #[serde(rename = "useColonistsOnMap")]
    pub use_colonists_on_map: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Label3 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Parms2 {
    #[serde(rename = "allowGoodwill")]
    pub allow_goodwill: bool,
    #[serde(rename = "allowRoyalFavor")]
    pub allow_royal_favor: bool,
    #[serde(rename = "chosenPawnSignal")]
    pub chosen_pawn_signal: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text2 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Parms3 {
    #[serde(rename = "customLetterLabel")]
    pub custom_letter_label: CustomLetterLabel2,
    #[serde(rename = "customLetterText")]
    pub custom_letter_text: CustomLetterText2,
    #[serde(rename = "enemyFaction")]
    pub enemy_faction: String,
    #[serde(rename = "inSignal")]
    pub in_signal: String,
    pub map: String,
    pub points: String,
    #[serde(rename = "walkInSpot")]
    pub walk_in_spot: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CustomLetterLabel2 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CustomLetterText2 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NodeIfChosenPawnSignalUsed2 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "chosenPawnSignal")]
    pub chosen_pawn_signal: String,
    pub label: Label4,
    #[serde(rename = "letterDef")]
    pub letter_def: String,
    pub text: Text3,
    #[serde(rename = "useColonistsFromCaravanArg")]
    pub use_colonists_from_caravan_arg: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Label4 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text3 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node9 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "hostileWeight")]
    pub hostile_weight: Option<f64>,
    #[serde(rename = "mustBeFactionLeader")]
    pub must_be_faction_leader: Option<bool>,
    #[serde(rename = "mustBeNonHostileToPlayer")]
    pub must_be_non_hostile_to_player: Option<bool>,
    pub name: Option<String>,
    #[serde(default)]
    pub nodes: Vec<Node10>,
    #[serde(rename = "selectionWeight")]
    pub selection_weight: ::serde_json::Value,
    #[serde(rename = "storeAs")]
    pub store_as: Option<String>,
    pub value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Node10 {
    #[serde(rename = "Class")]
    pub class: String,
    pub name: String,
    pub value: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Parms4 {
    #[serde(rename = "allowGoodwill")]
    pub allow_goodwill: Option<bool>,
    #[serde(rename = "allowRoyalFavor")]
    pub allow_royal_favor: Option<bool>,
    #[serde(rename = "chosenPawnSignal")]
    pub chosen_pawn_signal: Option<String>,
    #[serde(rename = "disallowedThingDefs")]
    #[serde(default)]
    pub disallowed_thing_defs: Vec<String>,
    #[serde(rename = "giveToCaravan")]
    pub give_to_caravan: Option<bool>,
    #[serde(rename = "hiddenSitePartsPossible")]
    pub hidden_site_parts_possible: Option<bool>,
    pub points: Option<String>,
    #[serde(rename = "pointsFactorThreeStar")]
    pub points_factor_three_star: Option<i64>,
    #[serde(rename = "pointsFactorTwoStar")]
    pub points_factor_two_star: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SitePartsTag {
    pub chance: Option<String>,
    pub tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Text4 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SingleSitePartRules {
    #[serde(rename = "rulesStrings")]
    pub rules_strings: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SuccessHistoryEvent {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RaidStrategyDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "arrivalTextEnemy")]
    pub arrival_text_enemy: Option<String>,
    #[serde(rename = "arrivalTextFriendly")]
    pub arrival_text_friendly: Option<String>,
    #[serde(rename = "arriveModes")]
    #[serde(default)]
    pub arrive_modes: Vec<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "letterLabelEnemy")]
    pub letter_label_enemy: Option<String>,
    #[serde(rename = "letterLabelFriendly")]
    pub letter_label_friendly: Option<String>,
    #[serde(rename = "minPawns")]
    pub min_pawns: Option<i64>,
    #[serde(rename = "pawnsCanBringFood")]
    pub pawns_can_bring_food: Option<bool>,
    #[serde(rename = "pointsFactorCurve")]
    pub points_factor_curve: Option<PointsFactorCurve2>,
    #[serde(rename = "raidLootValueFactor")]
    pub raid_loot_value_factor: Option<i64>,
    #[serde(rename = "selectionWeightPerPointsCurve")]
    pub selection_weight_per_points_curve: Option<SelectionWeightPerPointsCurve>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PointsFactorCurve2 {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SelectionWeightPerPointsCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RecipeDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "addsHediff")]
    pub adds_hediff: Option<String>,
    #[serde(rename = "allowMixingIngredients")]
    pub allow_mixing_ingredients: Option<bool>,
    pub anesthetize: Option<bool>,
    #[serde(rename = "appliedOnFixedBodyParts")]
    #[serde(default)]
    pub applied_on_fixed_body_parts: Vec<String>,
    #[serde(rename = "autoStripCorpses")]
    pub auto_strip_corpses: Option<bool>,
    #[serde(rename = "conceptLearned")]
    pub concept_learned: Option<String>,
    #[serde(rename = "deathOnFailedSurgeryChance")]
    pub death_on_failed_surgery_chance: Option<f64>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultIngredientFilter")]
    pub default_ingredient_filter: Option<DefaultIngredientFilter>,
    pub description: Option<String>,
    #[serde(rename = "dontShowIfAnyIngredientMissing")]
    pub dont_show_if_any_ingredient_missing: Option<bool>,
    #[serde(rename = "effectWorking")]
    pub effect_working: Option<String>,
    #[serde(rename = "efficiencyStat")]
    pub efficiency_stat: Option<String>,
    #[serde(rename = "fixedIngredientFilter")]
    pub fixed_ingredient_filter: Option<FixedIngredientFilter>,
    #[serde(rename = "forceHiddenSpecialFilters")]
    #[serde(default)]
    pub force_hidden_special_filters: Vec<String>,
    #[serde(rename = "hideBodyPartNames")]
    pub hide_body_part_names: Option<bool>,
    #[serde(rename = "ignoreIngredientCountTakeEntireStacks")]
    pub ignore_ingredient_count_take_entire_stacks: Option<bool>,
    #[serde(rename = "ingredientValueGetterClass")]
    pub ingredient_value_getter_class: Option<String>,
    #[serde(default)]
    pub ingredients: Vec<Ingredient>,
    #[serde(rename = "interruptIfIngredientIsRotting")]
    pub interrupt_if_ingredient_is_rotting: Option<bool>,
    #[serde(rename = "isViolation")]
    pub is_violation: Option<bool>,
    #[serde(rename = "jobString")]
    pub job_string: Option<String>,
    pub label: Option<String>,
    pub products: Option<Products>,
    #[serde(rename = "recipeUsers")]
    #[serde(default)]
    pub recipe_users: Vec<String>,
    #[serde(rename = "removesHediff")]
    pub removes_hediff: Option<String>,
    #[serde(rename = "requiredGiverWorkType")]
    pub required_giver_work_type: Option<String>,
    #[serde(rename = "researchPrerequisite")]
    pub research_prerequisite: Option<String>,
    #[serde(rename = "skillRequirements")]
    pub skill_requirements: Option<SkillRequirements>,
    #[serde(rename = "soundWorking")]
    pub sound_working: Option<String>,
    #[serde(rename = "specialProducts")]
    #[serde(default)]
    pub special_products: Vec<String>,
    #[serde(rename = "successfullyRemovedHediffMessage")]
    pub successfully_removed_hediff_message: Option<String>,
    #[serde(rename = "surgeryIgnoreEnvironment")]
    pub surgery_ignore_environment: Option<bool>,
    #[serde(rename = "surgerySuccessChanceFactor")]
    pub surgery_success_chance_factor: Option<::serde_json::Value>,
    #[serde(rename = "targetCountAdjustment")]
    pub target_count_adjustment: Option<i64>,
    #[serde(rename = "targetsBodyPart")]
    pub targets_body_part: Option<bool>,
    #[serde(rename = "uiIconThing")]
    pub ui_icon_thing: Option<String>,
    #[serde(rename = "unfinishedThingDef")]
    pub unfinished_thing_def: Option<String>,
    #[serde(rename = "workAmount")]
    pub work_amount: Option<i64>,
    #[serde(rename = "workSkill")]
    pub work_skill: Option<String>,
    #[serde(rename = "workSkillLearnFactor")]
    pub work_skill_learn_factor: Option<i64>,
    #[serde(rename = "workSpeedStat")]
    pub work_speed_stat: Option<String>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
    #[serde(rename = "workerCounterClass")]
    pub worker_counter_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DefaultIngredientFilter {
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(rename = "disallowedCategories")]
    #[serde(default)]
    pub disallowed_categories: Vec<String>,
    #[serde(rename = "disallowedThingDefs")]
    #[serde(default)]
    pub disallowed_thing_defs: Vec<String>,
    #[serde(rename = "specialFiltersToAllow")]
    #[serde(default)]
    pub special_filters_to_allow: Vec<::serde_json::Value>,
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedIngredientFilter {
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(rename = "disallowedCategories")]
    #[serde(default)]
    pub disallowed_categories: Vec<String>,
    #[serde(rename = "disallowedThingDefs")]
    #[serde(default)]
    pub disallowed_thing_defs: Vec<String>,
    #[serde(rename = "specialFiltersToDisallow")]
    #[serde(default)]
    pub special_filters_to_disallow: Vec<String>,
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Ingredient {
    pub count: ::serde_json::Value,
    pub filter: Filter,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter {
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(rename = "disallowedCategories")]
    #[serde(default)]
    pub disallowed_categories: Vec<String>,
    #[serde(rename = "disallowedThingDefs")]
    #[serde(default)]
    pub disallowed_thing_defs: Vec<String>,
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Products {
    #[serde(rename = "BlocksGranite")]
    pub blocks_granite: Option<i64>,
    #[serde(rename = "BlocksLimestone")]
    pub blocks_limestone: Option<i64>,
    #[serde(rename = "BlocksMarble")]
    pub blocks_marble: Option<i64>,
    #[serde(rename = "BlocksSandstone")]
    pub blocks_sandstone: Option<i64>,
    #[serde(rename = "BlocksSlate")]
    pub blocks_slate: Option<i64>,
    #[serde(rename = "Chemfuel")]
    pub chemfuel: Option<i64>,
    #[serde(rename = "ComponentIndustrial")]
    pub component_industrial: Option<i64>,
    #[serde(rename = "ComponentSpacer")]
    pub component_spacer: Option<i64>,
    #[serde(rename = "Kibble")]
    pub kibble: Option<i64>,
    #[serde(rename = "Leather_Patch")]
    pub leather_patch: Option<i64>,
    #[serde(rename = "MealFine")]
    pub meal_fine: Option<i64>,
    #[serde(rename = "MealFine_Meat")]
    pub meal_fine_meat: Option<i64>,
    #[serde(rename = "MealFine_Veg")]
    pub meal_fine_veg: Option<i64>,
    #[serde(rename = "MealLavish")]
    pub meal_lavish: Option<i64>,
    #[serde(rename = "MealLavish_Meat")]
    pub meal_lavish_meat: Option<i64>,
    #[serde(rename = "MealLavish_Veg")]
    pub meal_lavish_veg: Option<i64>,
    #[serde(rename = "MealSimple")]
    pub meal_simple: Option<i64>,
    #[serde(rename = "MealSurvivalPack")]
    pub meal_survival_pack: Option<i64>,
    #[serde(rename = "Pemmican")]
    pub pemmican: Option<i64>,
    #[serde(rename = "Steel")]
    pub steel: Option<i64>,
    #[serde(rename = "Wort")]
    pub wort: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkillRequirements {
    #[serde(rename = "Cooking")]
    pub cooking: Option<i64>,
    #[serde(rename = "Crafting")]
    pub crafting: Option<i64>,
    #[serde(rename = "Medicine")]
    pub medicine: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RecordDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    pub label: String,
    #[serde(rename = "measuredTimeJobs")]
    #[serde(default)]
    pub measured_time_jobs: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResearchProjectDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "baseCost")]
    pub base_cost: Option<i64>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "discoveredLetterDisabledWhen")]
    pub discovered_letter_disabled_when: Option<DiscoveredLetterDisabledWhen>,
    #[serde(rename = "discoveredLetterText")]
    pub discovered_letter_text: Option<String>,
    #[serde(rename = "discoveredLetterTitle")]
    pub discovered_letter_title: Option<String>,
    #[serde(rename = "hiddenPrerequisites")]
    #[serde(default)]
    pub hidden_prerequisites: Vec<String>,
    #[serde(rename = "hideWhen")]
    pub hide_when: Option<HideWhen>,
    pub label: Option<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(rename = "requiredResearchBuilding")]
    pub required_research_building: Option<String>,
    #[serde(rename = "requiredResearchFacilities")]
    #[serde(default)]
    pub required_research_facilities: Vec<String>,
    #[serde(rename = "researchViewX")]
    pub research_view_x: Option<f64>,
    #[serde(rename = "researchViewY")]
    pub research_view_y: Option<f64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "techLevel")]
    pub tech_level: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DiscoveredLetterDisabledWhen {
    #[serde(rename = "bigThreatsDisabled")]
    pub big_threats_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HideWhen {
    #[serde(rename = "mortarsDisabled")]
    pub mortars_disabled: Option<bool>,
    #[serde(rename = "trapsDisabled")]
    pub traps_disabled: Option<bool>,
    #[serde(rename = "turretsDisabled")]
    pub turrets_disabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResearchProjectTagDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResearchTabDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReservationLayerDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RitualOutcomeEffectDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub comps: Vec<Comp4>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "outcomeChances")]
    pub outcome_chances: Vec<OutcomeChance>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp4 {
    #[serde(rename = "Class")]
    pub class: String,
    pub curve: Curve3,
    pub label: String,
    #[serde(rename = "labelAbstract")]
    pub label_abstract: Option<String>,
    #[serde(rename = "roleId")]
    pub role_id: Option<String>,
    #[serde(rename = "statDef")]
    pub stat_def: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Curve3 {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OutcomeChance {
    pub chance: f64,
    pub label: String,
    pub memory: String,
    #[serde(rename = "positivityIndex")]
    pub positivity_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RitualPatternDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: bool,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ritualOutcomeEffect")]
    pub ritual_outcome_effect: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RitualVisualEffectDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub comps: Vec<Comp5>,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp5 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "compClass")]
    pub comp_class: String,
    #[serde(rename = "eastRotationOffset")]
    pub east_rotation_offset: String,
    #[serde(rename = "fleckDef")]
    pub fleck_def: String,
    #[serde(rename = "northRotationOffset")]
    pub north_rotation_offset: String,
    #[serde(rename = "requiredTag")]
    pub required_tag: String,
    #[serde(rename = "spawnIntervalTicks")]
    pub spawn_interval_ticks: i64,
    #[serde(rename = "westRotationOffset")]
    pub west_rotation_offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RiverDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(default)]
    pub branches: Vec<Branch>,
    #[serde(rename = "debugOpacity")]
    pub debug_opacity: f64,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "degradeChild")]
    pub degrade_child: Option<String>,
    #[serde(rename = "degradeThreshold")]
    pub degrade_threshold: i64,
    pub label: String,
    #[serde(rename = "spawnChance")]
    pub spawn_chance: Option<::serde_json::Value>,
    #[serde(rename = "spawnFlowThreshold")]
    pub spawn_flow_threshold: Option<i64>,
    #[serde(rename = "widthOnMap")]
    pub width_on_map: i64,
    #[serde(rename = "widthOnWorld")]
    pub width_on_world: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Branch {
    pub chance: f64,
    pub child: String,
    #[serde(rename = "minFlow")]
    pub min_flow: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoadDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "ancientOnly")]
    pub ancient_only: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "distortionFrequency")]
    pub distortion_frequency: Option<f64>,
    #[serde(rename = "distortionIntensity")]
    pub distortion_intensity: Option<f64>,
    pub label: String,
    #[serde(rename = "movementCostMultiplier")]
    pub movement_cost_multiplier: f64,
    #[serde(rename = "pathingMode")]
    pub pathing_mode: String,
    pub priority: i64,
    #[serde(rename = "roadGenSteps")]
    pub road_gen_steps: Vec<RoadGenStep>,
    #[serde(rename = "tilesPerSegment")]
    pub tiles_per_segment: i64,
    #[serde(rename = "worldRenderSteps")]
    pub world_render_steps: Vec<WorldRenderStep>,
    #[serde(rename = "worldTransitionGroup")]
    pub world_transition_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoadGenStep {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "antialiasingMultiplier")]
    pub antialiasing_multiplier: Option<i64>,
    #[serde(rename = "chancePerPositionCurve")]
    pub chance_per_position_curve: ChancePerPositionCurve,
    #[serde(rename = "onlyIfOriginAllows")]
    pub only_if_origin_allows: Option<bool>,
    #[serde(rename = "periodicSpacing")]
    pub periodic_spacing: Option<i64>,
    pub place: Option<String>,
    #[serde(rename = "proximitySpacing")]
    pub proximity_spacing: Option<i64>,
    #[serde(rename = "suppressOnTerrainTag")]
    pub suppress_on_terrain_tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ChancePerPositionCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WorldRenderStep {
    pub layer: String,
    pub width: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoadPathingDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoadWorldLayerDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub color: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub order: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoofDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "collapseLeavingThingDef")]
    pub collapse_leaving_thing_def: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "filthLeaving")]
    pub filth_leaving: String,
    #[serde(rename = "isNatural")]
    pub is_natural: Option<bool>,
    #[serde(rename = "isThickRoof")]
    pub is_thick_roof: bool,
    pub label: String,
    #[serde(rename = "soundPunchThrough")]
    pub sound_punch_through: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoomRoleDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    #[serde(rename = "relatedStats")]
    #[serde(default)]
    pub related_stats: Vec<String>,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoomStatDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub curve: Option<Curve4>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "displayRounded")]
    pub display_rounded: Option<bool>,
    #[serde(rename = "inputStat")]
    pub input_stat: Option<String>,
    #[serde(rename = "isHidden")]
    pub is_hidden: Option<bool>,
    pub label: String,
    #[serde(rename = "roomlessScore")]
    pub roomless_score: ::serde_json::Value,
    #[serde(rename = "scoreStages")]
    #[serde(default)]
    pub score_stages: Vec<ScoreStage>,
    #[serde(rename = "updatePriority")]
    pub update_priority: i64,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Curve4 {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ScoreStage {
    pub label: String,
    #[serde(rename = "minScore")]
    pub min_score: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RuleDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub resolvers: Vec<Resolver>,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Resolver {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    #[serde(rename = "bladelinkChance")]
    pub bladelink_chance: Option<f64>,
    pub interior: Option<String>,
    #[serde(rename = "minRectSize")]
    pub min_rect_size: Option<String>,
    #[serde(rename = "psychicChance")]
    pub psychic_chance: Option<f64>,
    #[serde(rename = "selectionWeight")]
    pub selection_weight: Option<f64>,
    #[serde(rename = "techprintChance")]
    pub techprint_chance: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RulePackDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(rename = "rulePack")]
    pub rule_pack: Option<RulePack>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RulePack {
    #[serde(rename = "rulesFiles")]
    #[serde(default)]
    pub rules_files: Vec<String>,
    #[serde(rename = "rulesRaw")]
    #[serde(default)]
    pub rules_raw: Vec<RulesRaw>,
    #[serde(rename = "rulesStrings")]
    #[serde(default)]
    pub rules_strings: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RulesRaw {
    #[serde(rename = "Class")]
    pub class: String,
    pub gender: Option<String>,
    pub keyword: String,
    pub range: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ScatterableDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "maxSize")]
    pub max_size: Option<f64>,
    #[serde(rename = "minSize")]
    pub min_size: Option<f64>,
    #[serde(rename = "scatterType")]
    pub scatter_type: Option<String>,
    #[serde(rename = "selectionWeight")]
    pub selection_weight: Option<i64>,
    #[serde(rename = "texturePath")]
    pub texture_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ScenPartDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    pub category: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "designatorType")]
    pub designator_type: Option<String>,
    #[serde(rename = "durationRandomRange")]
    pub duration_random_range: Option<String>,
    #[serde(rename = "gameCondition")]
    pub game_condition: Option<String>,
    #[serde(rename = "gameConditionTargetsWorld")]
    pub game_condition_targets_world: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "maxUses")]
    pub max_uses: Option<i64>,
    #[serde(rename = "pageClass")]
    pub page_class: Option<String>,
    #[serde(rename = "scenPartClass")]
    pub scen_part_class: Option<String>,
    #[serde(rename = "selectionWeight")]
    pub selection_weight: Option<f64>,
    #[serde(rename = "summaryPriority")]
    pub summary_priority: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ScenarioDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: Option<String>,
    pub label: String,
    pub scenario: Scenario,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Scenario {
    pub parts: Vec<Part6>,
    #[serde(rename = "playerFaction")]
    pub player_faction: Option<PlayerFaction>,
    #[serde(rename = "showInUI")]
    pub show_in_ui: Option<bool>,
    pub summary: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Part6 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowRoofed")]
    pub allow_roofed: Option<bool>,
    #[serde(rename = "bondToRandomPlayerPawnChance")]
    pub bond_to_random_player_pawn_chance: Option<f64>,
    pub chance: Option<f64>,
    #[serde(rename = "closeSound")]
    pub close_sound: Option<String>,
    pub context: Option<String>,
    pub count: Option<i64>,
    pub def: String,
    pub hediff: Option<String>,
    #[serde(rename = "hideOffMap")]
    pub hide_off_map: Option<bool>,
    #[serde(rename = "levelRange")]
    pub level_range: Option<String>,
    pub method: Option<String>,
    pub need: Option<String>,
    #[serde(rename = "pawnChoiceCount")]
    pub pawn_choice_count: Option<i64>,
    #[serde(rename = "pawnCount")]
    pub pawn_count: Option<i64>,
    pub project: Option<String>,
    #[serde(rename = "severityRange")]
    pub severity_range: Option<String>,
    pub stuff: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "textKey")]
    pub text_key: Option<String>,
    #[serde(rename = "thingDef")]
    pub thing_def: Option<String>,
    pub visible: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PlayerFaction {
    pub def: String,
    #[serde(rename = "factionDef")]
    pub faction_def: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShaderTypeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "shaderPath")]
    pub shader_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SiteCoreDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "expandingIconTexture")]
    pub expanding_icon_texture: String,
    pub label: String,
    #[serde(rename = "siteTexture")]
    pub site_texture: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SitePartDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "applyFactionColorToSiteTexture")]
    pub apply_faction_color_to_site_texture: Option<bool>,
    #[serde(rename = "approachOrderString")]
    pub approach_order_string: Option<String>,
    #[serde(rename = "approachingReportString")]
    pub approaching_report_string: Option<String>,
    #[serde(rename = "arrivedLetter")]
    pub arrived_letter: Option<String>,
    #[serde(rename = "arrivedLetterDef")]
    pub arrived_letter_def: Option<String>,
    #[serde(rename = "arrivedLetterHediffHyperlinks")]
    #[serde(default)]
    pub arrived_letter_hediff_hyperlinks: Vec<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "defaultHidden")]
    pub default_hidden: Option<bool>,
    pub description: String,
    #[serde(rename = "excludesTags")]
    #[serde(default)]
    pub excludes_tags: Vec<String>,
    #[serde(rename = "expandingIconTexture")]
    pub expanding_icon_texture: Option<String>,
    #[serde(rename = "forceExitAndRemoveMapCountdownDurationDays")]
    pub force_exit_and_remove_map_countdown_duration_days: Option<i64>,
    #[serde(rename = "handlesWorldObjectTimeoutInspectString")]
    pub handles_world_object_timeout_inspect_string: Option<bool>,
    #[serde(rename = "increasesPopulation")]
    pub increases_population: Option<bool>,
    pub label: String,
    #[serde(rename = "mainPartAllThreatsLabel")]
    pub main_part_all_threats_label: Option<String>,
    #[serde(rename = "minFactionTechLevel")]
    pub min_faction_tech_level: Option<String>,
    #[serde(rename = "minThreatPoints")]
    pub min_threat_points: Option<i64>,
    #[serde(rename = "requiresFaction")]
    pub requires_faction: Option<bool>,
    #[serde(rename = "showFactionInInspectString")]
    pub show_faction_in_inspect_string: Option<bool>,
    #[serde(rename = "siteTexture")]
    pub site_texture: Option<String>,
    pub tags: Vec<String>,
    #[serde(rename = "wantsThreatPoints")]
    pub wants_threat_points: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SketchResolverDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "isRoot")]
    pub is_root: Option<bool>,
    pub resolver: Resolver2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Resolver2 {
    #[serde(rename = "Class")]
    pub class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkillDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "disablingWorkTags")]
    pub disabling_work_tags: Option<String>,
    #[serde(rename = "listOrder")]
    pub list_order: i64,
    #[serde(rename = "neverDisabledBasedOnWorkTypes")]
    pub never_disabled_based_on_work_types: Option<bool>,
    #[serde(rename = "pawnCreatorSummaryVisible")]
    pub pawn_creator_summary_visible: Option<bool>,
    #[serde(rename = "skillLabel")]
    pub skill_label: String,
    #[serde(rename = "usuallyDefinedInBackstories")]
    pub usually_defined_in_backstories: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SongDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowedSeasons")]
    #[serde(default)]
    pub allowed_seasons: Vec<String>,
    #[serde(rename = "allowedTimeOfDay")]
    pub allowed_time_of_day: Option<String>,
    #[serde(rename = "clipPath")]
    pub clip_path: String,
    pub commonality: Option<f64>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "playOnMap")]
    pub play_on_map: Option<bool>,
    pub tense: Option<bool>,
    pub volume: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SoundDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    pub context: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "maxSimultaneous")]
    pub max_simultaneous: Option<i64>,
    #[serde(rename = "maxVoices")]
    pub max_voices: Option<i64>,
    #[serde(rename = "priorityMode")]
    pub priority_mode: Option<String>,
    pub slot: Option<String>,
    #[serde(rename = "subSounds")]
    #[serde(default)]
    pub sub_sounds: Vec<Option<SubSound>>,
    pub sustain: Option<bool>,
    #[serde(rename = "sustainFadeoutTime")]
    pub sustain_fadeout_time: Option<::serde_json::Value>,
    #[serde(rename = "sustainStartSound")]
    pub sustain_start_sound: Option<String>,
    #[serde(rename = "sustainStopSound")]
    pub sustain_stop_sound: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubSound {
    #[serde(rename = "distRange")]
    pub dist_range: Option<::serde_json::Value>,
    #[serde(default)]
    pub filters: Vec<Filter2>,
    #[serde(rename = "gameSpeedRange")]
    pub game_speed_range: Option<String>,
    pub grains: Vec<Grain>,
    #[serde(rename = "muteWhenPaused")]
    pub mute_when_paused: Option<bool>,
    pub name: Option<::serde_json::Value>,
    #[serde(rename = "onCamera")]
    pub on_camera: Option<bool>,
    #[serde(rename = "paramMappings")]
    #[serde(default)]
    pub param_mappings: Vec<ParamMapping>,
    #[serde(rename = "pitchRange")]
    pub pitch_range: Option<::serde_json::Value>,
    #[serde(rename = "repeatMode")]
    pub repeat_mode: Option<String>,
    #[serde(rename = "startDelayRange")]
    pub start_delay_range: Option<::serde_json::Value>,
    #[serde(rename = "sustainAttack")]
    pub sustain_attack: Option<::serde_json::Value>,
    #[serde(rename = "sustainIntervalRange")]
    pub sustain_interval_range: Option<::serde_json::Value>,
    #[serde(rename = "sustainLoop")]
    pub sustain_loop: Option<bool>,
    #[serde(rename = "sustainLoopDurationRange")]
    pub sustain_loop_duration_range: Option<String>,
    #[serde(rename = "sustainRelease")]
    pub sustain_release: Option<::serde_json::Value>,
    #[serde(rename = "sustainSkipFirstAttack")]
    pub sustain_skip_first_attack: Option<bool>,
    #[serde(rename = "tempoAffectedByGameSpeed")]
    pub tempo_affected_by_game_speed: Option<bool>,
    #[serde(rename = "volumeRange")]
    pub volume_range: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter2 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "baseSetup")]
    pub base_setup: Option<BaseSetup>,
    #[serde(rename = "cutoffFrequency")]
    pub cutoff_frequency: Option<::serde_json::Value>,
    #[serde(rename = "decayRatio")]
    pub decay_ratio: Option<f64>,
    pub delay: Option<f64>,
    #[serde(rename = "dryMix")]
    pub dry_mix: Option<i64>,
    #[serde(rename = "lowpassResonaceQ")]
    pub lowpass_resonace_q: Option<i64>,
    #[serde(rename = "wetMix")]
    pub wet_mix: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BaseSetup {
    #[serde(rename = "decayHFRatio")]
    pub decay_hfratio: f64,
    #[serde(rename = "decayTime")]
    pub decay_time: i64,
    pub density: i64,
    pub diffusion: i64,
    #[serde(rename = "dryLevel")]
    pub dry_level: i64,
    #[serde(rename = "hfReference")]
    pub hf_reference: i64,
    #[serde(rename = "lfReference")]
    pub lf_reference: i64,
    #[serde(rename = "reflectionsDelay")]
    pub reflections_delay: i64,
    #[serde(rename = "reflectionsLevel")]
    pub reflections_level: i64,
    #[serde(rename = "reverbDelay")]
    pub reverb_delay: f64,
    #[serde(rename = "reverbLevel")]
    pub reverb_level: i64,
    pub room: i64,
    #[serde(rename = "roomHF")]
    pub room_hf: i64,
    #[serde(rename = "roomLF")]
    pub room_lf: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Grain {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "clipFolderPath")]
    pub clip_folder_path: Option<String>,
    #[serde(rename = "clipPath")]
    pub clip_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ParamMapping {
    pub curve: Option<Curve5>,
    #[serde(rename = "inParam")]
    pub in_param: InParam,
    #[serde(rename = "outParam")]
    pub out_param: OutParam,
    #[serde(rename = "paramUpdateMode")]
    pub param_update_mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Curve5 {
    pub points: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct InParam {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<i64>,
    #[serde(rename = "inParamName")]
    pub in_param_name: Option<String>,
    #[serde(rename = "timeType")]
    pub time_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OutParam {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "baseSetup")]
    pub base_setup: Option<BaseSetup2>,
    #[serde(rename = "filterProperty")]
    pub filter_property: Option<String>,
    #[serde(rename = "pitchType")]
    pub pitch_type: Option<String>,
    #[serde(rename = "targetSetup")]
    pub target_setup: Option<TargetSetup>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BaseSetup2 {
    #[serde(rename = "decayHFRatio")]
    pub decay_hfratio: f64,
    #[serde(rename = "decayTime")]
    pub decay_time: f64,
    pub density: i64,
    pub diffusion: i64,
    #[serde(rename = "dryLevel")]
    pub dry_level: i64,
    #[serde(rename = "hfReference")]
    pub hf_reference: i64,
    #[serde(rename = "lfReference")]
    pub lf_reference: i64,
    #[serde(rename = "reflectionsDelay")]
    pub reflections_delay: i64,
    #[serde(rename = "reflectionsLevel")]
    pub reflections_level: i64,
    #[serde(rename = "reverbDelay")]
    pub reverb_delay: f64,
    #[serde(rename = "reverbLevel")]
    pub reverb_level: i64,
    pub room: i64,
    #[serde(rename = "roomHF")]
    pub room_hf: i64,
    #[serde(rename = "roomLF")]
    pub room_lf: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TargetSetup {
    #[serde(rename = "decayHFRatio")]
    pub decay_hfratio: f64,
    #[serde(rename = "decayTime")]
    pub decay_time: f64,
    pub density: i64,
    pub diffusion: i64,
    #[serde(rename = "dryLevel")]
    pub dry_level: i64,
    #[serde(rename = "hfReference")]
    pub hf_reference: i64,
    #[serde(rename = "lfReference")]
    pub lf_reference: i64,
    #[serde(rename = "reflectionsDelay")]
    pub reflections_delay: i64,
    #[serde(rename = "reflectionsLevel")]
    pub reflections_level: i64,
    #[serde(rename = "reverbDelay")]
    pub reverb_delay: f64,
    #[serde(rename = "reverbLevel")]
    pub reverb_level: i64,
    pub room: i64,
    #[serde(rename = "roomHF")]
    pub room_hf: i64,
    #[serde(rename = "roomLF")]
    pub room_lf: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SpecialThingFilterDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowedByDefault")]
    pub allowed_by_default: bool,
    pub configurable: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: Option<String>,
    pub label: String,
    #[serde(rename = "parentCategory")]
    pub parent_category: String,
    #[serde(rename = "saveKey")]
    pub save_key: String,
    #[serde(rename = "workerClass")]
    pub worker_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "displayAllByDefault")]
    pub display_all_by_default: Option<bool>,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "alwaysHide")]
    pub always_hide: Option<bool>,
    #[serde(rename = "applyFactorsIfNegative")]
    pub apply_factors_if_negative: Option<bool>,
    #[serde(rename = "capacityFactors")]
    #[serde(default)]
    pub capacity_factors: Vec<CapacityFactor>,
    #[serde(rename = "capacityOffsets")]
    #[serde(default)]
    pub capacity_offsets: Vec<CapacityOffset>,
    pub category: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultBaseValue")]
    pub default_base_value: Option<::serde_json::Value>,
    pub description: Option<String>,
    #[serde(rename = "disableIfSkillDisabled")]
    pub disable_if_skill_disabled: Option<String>,
    #[serde(rename = "displayPriorityInCategory")]
    pub display_priority_in_category: Option<i64>,
    #[serde(rename = "finalizeEquippedStatOffset")]
    pub finalize_equipped_stat_offset: Option<bool>,
    #[serde(rename = "forInformationOnly")]
    pub for_information_only: Option<bool>,
    #[serde(rename = "formatString")]
    pub format_string: Option<String>,
    #[serde(rename = "formatStringUnfinalized")]
    pub format_string_unfinalized: Option<String>,
    #[serde(rename = "hideAtValue")]
    pub hide_at_value: Option<::serde_json::Value>,
    pub label: Option<String>,
    #[serde(rename = "labelForFullStatList")]
    pub label_for_full_stat_list: Option<String>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<::serde_json::Value>,
    #[serde(rename = "minValue")]
    pub min_value: Option<::serde_json::Value>,
    #[serde(rename = "minifiedThingInherits")]
    pub minified_thing_inherits: Option<bool>,
    #[serde(rename = "neverDisabled")]
    pub never_disabled: Option<bool>,
    #[serde(rename = "noSkillOffset")]
    pub no_skill_offset: Option<i64>,
    #[serde(default)]
    pub parts: Vec<Part7>,
    #[serde(rename = "postProcessCurve")]
    pub post_process_curve: Option<PostProcessCurve>,
    #[serde(rename = "postProcessStatFactors")]
    #[serde(default)]
    pub post_process_stat_factors: Vec<String>,
    #[serde(rename = "roundToFiveOver")]
    pub round_to_five_over: Option<i64>,
    #[serde(rename = "roundValue")]
    pub round_value: Option<bool>,
    #[serde(rename = "scenarioRandomizable")]
    pub scenario_randomizable: Option<bool>,
    #[serde(rename = "showIfModsLoaded")]
    #[serde(default)]
    pub show_if_mods_loaded: Vec<String>,
    #[serde(rename = "showIfUndefined")]
    pub show_if_undefined: Option<bool>,
    #[serde(rename = "showNonAbstract")]
    pub show_non_abstract: Option<bool>,
    #[serde(rename = "showOnAnimals")]
    pub show_on_animals: Option<bool>,
    #[serde(rename = "showOnDefaultValue")]
    pub show_on_default_value: Option<bool>,
    #[serde(rename = "showOnMechanoids")]
    pub show_on_mechanoids: Option<bool>,
    #[serde(rename = "showOnNonWildManHumanlikes")]
    pub show_on_non_wild_man_humanlikes: Option<bool>,
    #[serde(rename = "showOnNonWorkTables")]
    pub show_on_non_work_tables: Option<bool>,
    #[serde(rename = "showOnPawns")]
    pub show_on_pawns: Option<bool>,
    #[serde(rename = "showOnUnhaulables")]
    pub show_on_unhaulables: Option<bool>,
    #[serde(rename = "showOnUntradeables")]
    pub show_on_untradeables: Option<bool>,
    #[serde(rename = "showZeroBaseValue")]
    pub show_zero_base_value: Option<bool>,
    #[serde(rename = "skillNeedFactors")]
    #[serde(default)]
    pub skill_need_factors: Vec<SkillNeedFactor>,
    #[serde(rename = "skillNeedOffsets")]
    #[serde(default)]
    pub skill_need_offsets: Vec<SkillNeedOffset>,
    #[serde(rename = "statFactors")]
    #[serde(default)]
    pub stat_factors: Vec<String>,
    #[serde(rename = "supressDisabledError")]
    pub supress_disabled_error: Option<bool>,
    #[serde(rename = "toStringStyle")]
    pub to_string_style: Option<String>,
    #[serde(rename = "toStringStyleUnfinalized")]
    pub to_string_style_unfinalized: Option<String>,
    #[serde(rename = "valueIfMissing")]
    pub value_if_missing: Option<f64>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CapacityFactor {
    #[serde(rename = "allowedDefect")]
    pub allowed_defect: Option<f64>,
    pub capacity: String,
    pub max: Option<::serde_json::Value>,
    pub weight: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CapacityOffset {
    pub capacity: String,
    pub max: Option<::serde_json::Value>,
    pub scale: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Part7 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    #[serde(rename = "apparelStat")]
    pub apparel_stat: Option<String>,
    #[serde(rename = "applyToNegativeValues")]
    pub apply_to_negative_values: Option<bool>,
    pub curve: Option<Curve6>,
    #[serde(rename = "customLabel")]
    pub custom_label: Option<String>,
    pub factor: Option<f64>,
    #[serde(rename = "factorAwful")]
    pub factor_awful: Option<::serde_json::Value>,
    #[serde(rename = "factorExcellent")]
    pub factor_excellent: Option<::serde_json::Value>,
    #[serde(rename = "factorExhausted")]
    pub factor_exhausted: Option<f64>,
    #[serde(rename = "factorFromGlowCurve")]
    pub factor_from_glow_curve: Option<FactorFromGlowCurve>,
    #[serde(rename = "factorGood")]
    pub factor_good: Option<::serde_json::Value>,
    #[serde(rename = "factorIndoors")]
    pub factor_indoors: Option<::serde_json::Value>,
    #[serde(rename = "factorLegendary")]
    pub factor_legendary: Option<::serde_json::Value>,
    #[serde(rename = "factorMasterwork")]
    pub factor_masterwork: Option<::serde_json::Value>,
    #[serde(rename = "factorNormal")]
    pub factor_normal: Option<::serde_json::Value>,
    #[serde(rename = "factorOutdoors")]
    pub factor_outdoors: Option<f64>,
    #[serde(rename = "factorPoor")]
    pub factor_poor: Option<::serde_json::Value>,
    #[serde(rename = "factorStarving")]
    pub factor_starving: Option<f64>,
    #[serde(rename = "factorTired")]
    pub factor_tired: Option<f64>,
    #[serde(rename = "factorUrgentlyHungry")]
    pub factor_urgently_hungry: Option<f64>,
    #[serde(rename = "factorVeryTired")]
    pub factor_very_tired: Option<f64>,
    #[serde(rename = "humanlikeOnly")]
    pub humanlike_only: Option<bool>,
    #[serde(rename = "ignoreIfIncapableOfSight")]
    pub ignore_if_incapable_of_sight: Option<bool>,
    #[serde(rename = "ignoreIfPrefersDarkness")]
    pub ignore_if_prefers_darkness: Option<bool>,
    #[serde(rename = "includeWeapon")]
    pub include_weapon: Option<bool>,
    #[serde(rename = "maxGainExcellent")]
    pub max_gain_excellent: Option<i64>,
    #[serde(rename = "maxGainGood")]
    pub max_gain_good: Option<i64>,
    #[serde(rename = "maxGainLegendary")]
    pub max_gain_legendary: Option<i64>,
    #[serde(rename = "maxGainMasterwork")]
    pub max_gain_masterwork: Option<i64>,
    #[serde(rename = "multiplierStat")]
    pub multiplier_stat: Option<String>,
    pub offset: Option<::serde_json::Value>,
    pub priority: Option<i64>,
    #[serde(rename = "roomStat")]
    pub room_stat: Option<String>,
    pub stat: Option<String>,
    #[serde(rename = "stuffPowerStat")]
    pub stuff_power_stat: Option<String>,
    pub subtract: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Curve6 {
    pub points: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FactorFromGlowCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PostProcessCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkillNeedFactor {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "baseValue")]
    pub base_value: Option<::serde_json::Value>,
    #[serde(rename = "bonusPerLevel")]
    pub bonus_per_level: Option<f64>,
    pub skill: String,
    #[serde(rename = "valuesPerLevel")]
    #[serde(default)]
    pub values_per_level: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkillNeedOffset {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "baseValue")]
    pub base_value: i64,
    #[serde(rename = "bonusPerLevel")]
    pub bonus_per_level: ::serde_json::Value,
    pub skill: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StorytellerDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "adaptDaysGameStartGraceDays")]
    pub adapt_days_game_start_grace_days: Option<i64>,
    #[serde(rename = "adaptDaysGrowthRateCurve")]
    pub adapt_days_growth_rate_curve: Option<AdaptDaysGrowthRateCurve>,
    #[serde(rename = "adaptDaysLossFromColonistLostByPostPopulation")]
    pub adapt_days_loss_from_colonist_lost_by_post_population: Option<AdaptDaysLossFromColonistLostByPostPopulation>,
    #[serde(rename = "adaptDaysLossFromColonistViolentlyDownedByPopulation")]
    pub adapt_days_loss_from_colonist_violently_downed_by_population: Option<AdaptDaysLossFromColonistViolentlyDownedByPopulation>,
    #[serde(rename = "adaptDaysMax")]
    pub adapt_days_max: Option<i64>,
    #[serde(rename = "adaptDaysMin")]
    pub adapt_days_min: Option<i64>,
    pub comps: Vec<Option<Comp6>>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "disableAdaptiveTraining")]
    pub disable_adaptive_training: Option<bool>,
    #[serde(rename = "disableAlerts")]
    pub disable_alerts: Option<bool>,
    #[serde(rename = "disablePermadeath")]
    pub disable_permadeath: Option<bool>,
    #[serde(rename = "forcedDifficulty")]
    pub forced_difficulty: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "listOrder")]
    pub list_order: Option<i64>,
    #[serde(rename = "listVisible")]
    pub list_visible: Option<bool>,
    #[serde(rename = "pointsFactorFromAdaptDays")]
    pub points_factor_from_adapt_days: Option<PointsFactorFromAdaptDays>,
    #[serde(rename = "pointsFactorFromDaysPassed")]
    pub points_factor_from_days_passed: Option<PointsFactorFromDaysPassed>,
    #[serde(rename = "populationIntentFactorFromPopAdaptDaysCurve")]
    pub population_intent_factor_from_pop_adapt_days_curve: Option<PopulationIntentFactorFromPopAdaptDaysCurve>,
    #[serde(rename = "populationIntentFactorFromPopCurve")]
    pub population_intent_factor_from_pop_curve: Option<PopulationIntentFactorFromPopCurve>,
    #[serde(rename = "portraitLarge")]
    pub portrait_large: Option<String>,
    #[serde(rename = "portraitTiny")]
    pub portrait_tiny: Option<String>,
    #[serde(rename = "tutorialMode")]
    pub tutorial_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AdaptDaysGrowthRateCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AdaptDaysLossFromColonistLostByPostPopulation {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AdaptDaysLossFromColonistViolentlyDownedByPopulation {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp6 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    #[serde(rename = "acceptFractionByDaysPassedCurve")]
    pub accept_fraction_by_days_passed_curve: Option<AcceptFractionByDaysPassedCurve>,
    #[serde(rename = "acceptPercentFactorPerThreatPointsCurve")]
    pub accept_percent_factor_per_threat_points_curve: Option<AcceptPercentFactorPerThreatPointsCurve>,
    #[serde(rename = "allowedTargetTags")]
    #[serde(default)]
    pub allowed_target_tags: Vec<String>,
    #[serde(rename = "applyCaravanVisibility")]
    pub apply_caravan_visibility: Option<bool>,
    #[serde(rename = "baseIncidentsPerYear")]
    pub base_incidents_per_year: Option<i64>,
    #[serde(rename = "baseMtbDaysPerDrill")]
    pub base_mtb_days_per_drill: Option<i64>,
    pub category: Option<String>,
    #[serde(rename = "categoryWeights")]
    pub category_weights: Option<CategoryWeights>,
    #[serde(rename = "delayTicks")]
    pub delay_ticks: Option<i64>,
    #[serde(rename = "disableIfAnyModActive")]
    #[serde(default)]
    pub disable_if_any_mod_active: Vec<String>,
    #[serde(rename = "disallowedTargetTags")]
    #[serde(default)]
    pub disallowed_target_tags: Vec<String>,
    #[serde(rename = "enableIfAnyModActive")]
    #[serde(default)]
    pub enable_if_any_mod_active: Vec<String>,
    #[serde(rename = "fireAfterDaysPassed")]
    pub fire_after_days_passed: Option<i64>,
    #[serde(rename = "forceRaidEnemyBeforeDaysPassed")]
    pub force_raid_enemy_before_days_passed: Option<i64>,
    #[serde(rename = "fullAlliesOnly")]
    pub full_allies_only: Option<bool>,
    pub incident: Option<::serde_json::Value>,
    #[serde(rename = "maxThreatBigIntervalDays")]
    pub max_threat_big_interval_days: Option<i64>,
    #[serde(rename = "minColonistCount")]
    pub min_colonist_count: Option<i64>,
    #[serde(rename = "minColonyWealth")]
    pub min_colony_wealth: Option<i64>,
    #[serde(rename = "minDanger")]
    pub min_danger: Option<String>,
    #[serde(rename = "minDaysPassed")]
    pub min_days_passed: Option<::serde_json::Value>,
    #[serde(rename = "minIncChancePopulationIntentFactor")]
    pub min_inc_chance_population_intent_factor: Option<f64>,
    #[serde(rename = "minSpacingDays")]
    pub min_spacing_days: Option<::serde_json::Value>,
    #[serde(rename = "minWealth")]
    pub min_wealth: Option<i64>,
    #[serde(rename = "mtbDays")]
    pub mtb_days: Option<::serde_json::Value>,
    #[serde(rename = "numIncidentsRange")]
    pub num_incidents_range: Option<::serde_json::Value>,
    #[serde(rename = "offDays")]
    pub off_days: Option<::serde_json::Value>,
    #[serde(rename = "offDaysNoTreeConnectors")]
    pub off_days_no_tree_connectors: Option<i64>,
    #[serde(rename = "onDays")]
    pub on_days: Option<::serde_json::Value>,
    #[serde(rename = "onDaysNoTreeConnectors")]
    pub on_days_no_tree_connectors: Option<i64>,
    pub parms: Option<Parms5>,
    #[serde(rename = "randomPointsFactorRange")]
    pub random_points_factor_range: Option<String>,
    #[serde(rename = "refireEveryDays")]
    pub refire_every_days: Option<i64>,
    #[serde(rename = "skipIfColonistHasMinTitle")]
    pub skip_if_colonist_has_min_title: Option<SkipIfColonistHasMinTitle>,
    #[serde(rename = "skipIfOnExtremeBiome")]
    pub skip_if_on_extreme_biome: Option<bool>,
    #[serde(rename = "skipThreatBigIfRaidBeacon")]
    pub skip_threat_big_if_raid_beacon: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AcceptFractionByDaysPassedCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AcceptPercentFactorPerThreatPointsCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CategoryWeights {
    #[serde(rename = "FactionArrival")]
    pub faction_arrival: f64,
    #[serde(rename = "Misc")]
    pub misc: f64,
    #[serde(rename = "OrbitalVisitor")]
    pub orbital_visitor: f64,
    #[serde(rename = "ShipChunkDrop")]
    pub ship_chunk_drop: f64,
    #[serde(rename = "ThreatBig")]
    pub threat_big: f64,
    #[serde(rename = "ThreatSmall")]
    pub threat_small: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Parms5 {
    #[serde(rename = "allowedThreats")]
    pub allowed_threats: String,
    #[serde(rename = "minSpacingDays")]
    pub min_spacing_days: f64,
    #[serde(rename = "minThreatPoints")]
    pub min_threat_points: i64,
    #[serde(rename = "numIncidentsRange")]
    pub num_incidents_range: String,
    #[serde(rename = "offDays")]
    pub off_days: f64,
    #[serde(rename = "onDays")]
    pub on_days: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkipIfColonistHasMinTitle {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PointsFactorFromAdaptDays {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PointsFactorFromDaysPassed {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PopulationIntentFactorFromPopAdaptDaysCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PopulationIntentFactorFromPopCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StuffAppearanceDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StuffCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "destroySoundLarge")]
    pub destroy_sound_large: Option<String>,
    #[serde(rename = "destroySoundMedium")]
    pub destroy_sound_medium: String,
    #[serde(rename = "destroySoundSmall")]
    pub destroy_sound_small: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StyleItemCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubcameraDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub depth: i64,
    pub format: String,
    pub layer: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TaleDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "baseInterest")]
    pub base_interest: Option<::serde_json::Value>,
    #[serde(rename = "colonistOnly")]
    pub colonist_only: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "defSymbol")]
    pub def_symbol: Option<String>,
    #[serde(rename = "defType")]
    pub def_type: Option<String>,
    #[serde(rename = "expireDays")]
    pub expire_days: Option<i64>,
    #[serde(rename = "firstPawnSymbol")]
    pub first_pawn_symbol: Option<String>,
    #[serde(rename = "historyGraphColor")]
    pub history_graph_color: Option<String>,
    #[serde(rename = "ignoreChance")]
    pub ignore_chance: Option<f64>,
    pub label: Option<String>,
    #[serde(rename = "maxPerPawn")]
    pub max_per_pawn: Option<i64>,
    #[serde(rename = "rulePack")]
    pub rule_pack: Option<RulePack2>,
    #[serde(rename = "secondPawnSymbol")]
    pub second_pawn_symbol: Option<String>,
    #[serde(rename = "taleClass")]
    pub tale_class: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "usableForArt")]
    pub usable_for_art: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RulePack2 {
    #[serde(rename = "rulesStrings")]
    pub rules_strings: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TattooDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub category: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    pub label: String,
    #[serde(rename = "styleTags")]
    pub style_tags: Vec<String>,
    #[serde(rename = "tattooType")]
    pub tattoo_type: String,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TerrainAffordanceDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "affordanceOverlayColor")]
    pub affordance_overlay_color: Option<String>,
    #[serde(rename = "blockAffordanceOverlay")]
    pub block_affordance_overlay: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
    pub order: i64,
    #[serde(rename = "visualizeOnAffordanceOverlay")]
    pub visualize_on_affordance_overlay: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TerrainDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(default)]
    pub affordances: Vec<String>,
    #[serde(rename = "autoRebuildable")]
    pub auto_rebuildable: Option<bool>,
    #[serde(rename = "avoidWander")]
    pub avoid_wander: Option<bool>,
    pub bridge: Option<bool>,
    #[serde(rename = "burnedDef")]
    pub burned_def: Option<String>,
    pub changeable: Option<bool>,
    pub color: Option<String>,
    #[serde(rename = "constructEffect")]
    pub construct_effect: Option<String>,
    #[serde(rename = "constructionSkillPrerequisite")]
    pub construction_skill_prerequisite: Option<i64>,
    #[serde(rename = "costList")]
    pub cost_list: Option<CostList>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "designationCategory")]
    pub designation_category: Option<String>,
    #[serde(rename = "designationHotKey")]
    pub designation_hot_key: Option<String>,
    #[serde(rename = "designatorDropdown")]
    pub designator_dropdown: Option<String>,
    #[serde(rename = "destroyBuildingsOnDestroyed")]
    pub destroy_buildings_on_destroyed: Option<bool>,
    #[serde(rename = "destroyEffect")]
    pub destroy_effect: Option<String>,
    #[serde(rename = "destroyEffectWater")]
    pub destroy_effect_water: Option<String>,
    #[serde(rename = "destroyOnBombDamageThreshold")]
    pub destroy_on_bomb_damage_threshold: Option<i64>,
    #[serde(rename = "driesTo")]
    pub dries_to: Option<String>,
    #[serde(rename = "edgeType")]
    pub edge_type: Option<String>,
    #[serde(rename = "extinguishesFire")]
    pub extinguishes_fire: Option<bool>,
    #[serde(rename = "extraDeteriorationFactor")]
    pub extra_deterioration_factor: Option<i64>,
    #[serde(rename = "extraDraftedPerceivedPathCost")]
    pub extra_drafted_perceived_path_cost: Option<i64>,
    #[serde(rename = "extraNonDraftedPerceivedPathCost")]
    pub extra_non_drafted_perceived_path_cost: Option<i64>,
    pub fertility: Option<::serde_json::Value>,
    #[serde(rename = "filthAcceptanceMask")]
    #[serde(default)]
    pub filth_acceptance_mask: Vec<String>,
    #[serde(rename = "generatedFilth")]
    pub generated_filth: Option<String>,
    #[serde(rename = "holdSnow")]
    pub hold_snow: Option<bool>,
    pub label: Option<String>,
    pub layerable: Option<bool>,
    pub passability: Option<String>,
    #[serde(rename = "pathCost")]
    pub path_cost: Option<i64>,
    #[serde(rename = "renderPrecedence")]
    pub render_precedence: Option<i64>,
    #[serde(rename = "researchPrerequisites")]
    #[serde(default)]
    pub research_prerequisites: Vec<String>,
    #[serde(rename = "resourcesFractionWhenDeconstructed")]
    pub resources_fraction_when_deconstructed: Option<i64>,
    #[serde(rename = "scatterType")]
    pub scatter_type: Option<String>,
    #[serde(rename = "statBases")]
    pub stat_bases: Option<StatBases2>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "takeFootprints")]
    pub take_footprints: Option<bool>,
    #[serde(rename = "takeSplashes")]
    pub take_splashes: Option<bool>,
    #[serde(rename = "terrainAffordanceNeeded")]
    pub terrain_affordance_needed: Option<String>,
    #[serde(rename = "texturePath")]
    pub texture_path: Option<String>,
    #[serde(default)]
    pub tools: Vec<Tool2>,
    #[serde(rename = "traversedThought")]
    pub traversed_thought: Option<String>,
    #[serde(rename = "uiIconPath")]
    pub ui_icon_path: Option<String>,
    #[serde(rename = "waterDepthShader")]
    pub water_depth_shader: Option<String>,
    #[serde(rename = "waterDepthShaderParameters")]
    pub water_depth_shader_parameters: Option<WaterDepthShaderParameters>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CostList {
    #[serde(rename = "BlocksGranite")]
    pub blocks_granite: Option<i64>,
    #[serde(rename = "BlocksLimestone")]
    pub blocks_limestone: Option<i64>,
    #[serde(rename = "BlocksMarble")]
    pub blocks_marble: Option<i64>,
    #[serde(rename = "BlocksSandstone")]
    pub blocks_sandstone: Option<i64>,
    #[serde(rename = "BlocksSlate")]
    pub blocks_slate: Option<i64>,
    #[serde(rename = "Cloth")]
    pub cloth: Option<i64>,
    #[serde(rename = "Gold")]
    pub gold: Option<i64>,
    #[serde(rename = "Hay")]
    pub hay: Option<i64>,
    #[serde(rename = "Silver")]
    pub silver: Option<i64>,
    #[serde(rename = "Steel")]
    pub steel: Option<i64>,
    #[serde(rename = "WoodLog")]
    pub wood_log: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatBases2 {
    #[serde(rename = "Beauty")]
    pub beauty: Option<i64>,
    #[serde(rename = "BeautyOutdoors")]
    pub beauty_outdoors: Option<i64>,
    #[serde(rename = "Cleanliness")]
    pub cleanliness: Option<::serde_json::Value>,
    #[serde(rename = "FilthMultiplier")]
    pub filth_multiplier: Option<f64>,
    #[serde(rename = "Flammability")]
    pub flammability: Option<f64>,
    #[serde(rename = "WorkToBuild")]
    pub work_to_build: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tool2 {
    pub capacities: Vec<String>,
    #[serde(rename = "cooldownTime")]
    pub cooldown_time: f64,
    pub hediff: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WaterDepthShaderParameters {
    #[serde(rename = "_UseWaterOffset")]
    pub use_water_offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingCategoryDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    pub label: String,
    pub parent: Option<String>,
    #[serde(rename = "resourceReadoutRoot")]
    pub resource_readout_root: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "allowedArchonexusCount")]
    pub allowed_archonexus_count: Option<i64>,
    #[serde(rename = "altitudeLayer")]
    pub altitude_layer: Option<String>,
    #[serde(rename = "alwaysFlee")]
    pub always_flee: Option<bool>,
    #[serde(rename = "alwaysHaulable")]
    pub always_haulable: Option<bool>,
    pub apparel: Option<Apparel>,
    #[serde(rename = "blockLight")]
    pub block_light: Option<bool>,
    #[serde(rename = "blockPlants")]
    pub block_plants: Option<bool>,
    #[serde(rename = "blockWind")]
    pub block_wind: Option<bool>,
    pub building: Option<Building>,
    #[serde(rename = "burnableByRecipe")]
    pub burnable_by_recipe: Option<bool>,
    #[serde(rename = "butcherProducts")]
    pub butcher_products: Option<ButcherProducts>,
    #[serde(rename = "canBeUsedUnderRoof")]
    pub can_be_used_under_roof: Option<bool>,
    #[serde(rename = "canGenerateDefaultDesignator")]
    pub can_generate_default_designator: Option<bool>,
    #[serde(rename = "canOverlapZones")]
    pub can_overlap_zones: Option<bool>,
    #[serde(rename = "castEdgeShadows")]
    pub cast_edge_shadows: Option<bool>,
    pub category: Option<String>,
    #[serde(rename = "clearBuildingArea")]
    pub clear_building_area: Option<bool>,
    #[serde(rename = "colorGenerator")]
    pub color_generator: Option<ColorGenerator>,
    #[serde(rename = "colorGeneratorInTraderStock")]
    pub color_generator_in_trader_stock: Option<ColorGeneratorInTraderStock>,
    #[serde(rename = "colorPerStuff")]
    #[serde(default)]
    pub color_per_stuff: Vec<ColorPerStuff>,
    #[serde(default)]
    pub comps: Vec<Comp7>,
    #[serde(rename = "constructEffect")]
    pub construct_effect: Option<String>,
    #[serde(rename = "constructionSkillPrerequisite")]
    pub construction_skill_prerequisite: Option<i64>,
    #[serde(rename = "costList")]
    pub cost_list: Option<CostList2>,
    #[serde(rename = "costListForDifficulty")]
    pub cost_list_for_difficulty: Option<CostListForDifficulty>,
    #[serde(rename = "costStuffCount")]
    pub cost_stuff_count: Option<i64>,
    #[serde(rename = "coversFloor")]
    pub covers_floor: Option<bool>,
    #[serde(rename = "damageMultipliers")]
    #[serde(default)]
    pub damage_multipliers: Vec<DamageMultiplier>,
    #[serde(rename = "deepCommonality")]
    pub deep_commonality: Option<::serde_json::Value>,
    #[serde(rename = "deepCountPerPortion")]
    pub deep_count_per_portion: Option<i64>,
    #[serde(rename = "deepLumpSizeRange")]
    pub deep_lump_size_range: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "defaultPlacingRot")]
    pub default_placing_rot: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "designateHaulable")]
    pub designate_haulable: Option<bool>,
    #[serde(rename = "designationCategory")]
    pub designation_category: Option<String>,
    #[serde(rename = "designationHotKey")]
    pub designation_hot_key: Option<String>,
    #[serde(rename = "designatorDropdown")]
    pub designator_dropdown: Option<String>,
    #[serde(rename = "destroyOnDrop")]
    pub destroy_on_drop: Option<bool>,
    pub destroyable: Option<bool>,
    #[serde(rename = "devNote")]
    pub dev_note: Option<String>,
    #[serde(rename = "drawDamagedOverlay")]
    pub draw_damaged_overlay: Option<bool>,
    #[serde(rename = "drawGUIOverlay")]
    pub draw_guioverlay: Option<bool>,
    #[serde(rename = "drawOffscreen")]
    pub draw_offscreen: Option<bool>,
    #[serde(rename = "drawPlaceWorkersWhileSelected")]
    pub draw_place_workers_while_selected: Option<bool>,
    #[serde(rename = "drawerType")]
    pub drawer_type: Option<String>,
    #[serde(rename = "equipmentType")]
    pub equipment_type: Option<String>,
    #[serde(rename = "equippedAngleOffset")]
    pub equipped_angle_offset: Option<i64>,
    #[serde(rename = "equippedStatOffsets")]
    pub equipped_stat_offsets: Option<EquippedStatOffsets>,
    pub fertility: Option<::serde_json::Value>,
    #[serde(rename = "fillPercent")]
    pub fill_percent: Option<::serde_json::Value>,
    pub filth: Option<Filth>,
    #[serde(rename = "filthLeaving")]
    pub filth_leaving: Option<String>,
    #[serde(rename = "forceDebugSpawnable")]
    pub force_debug_spawnable: Option<bool>,
    pub gas: Option<Gas>,
    #[serde(rename = "generateAllowChance")]
    pub generate_allow_chance: Option<f64>,
    #[serde(rename = "generateCommonality")]
    pub generate_commonality: Option<::serde_json::Value>,
    #[serde(rename = "graphicData")]
    pub graphic_data: Option<GraphicData3>,
    #[serde(rename = "hasInteractionCell")]
    pub has_interaction_cell: Option<bool>,
    #[serde(rename = "hasTooltip")]
    pub has_tooltip: Option<bool>,
    #[serde(rename = "healthAffectsPrice")]
    pub health_affects_price: Option<bool>,
    #[serde(rename = "hideAtSnowDepth")]
    pub hide_at_snow_depth: Option<f64>,
    #[serde(rename = "holdsRoof")]
    pub holds_roof: Option<bool>,
    #[serde(rename = "ideoBuildingNamerBase")]
    pub ideo_building_namer_base: Option<IdeoBuildingNamerBase>,
    pub ingestible: Option<::serde_json::Value>,
    pub ingredient: Option<Ingredient2>,
    #[serde(rename = "inspectorTabs")]
    #[serde(default)]
    pub inspector_tabs: Vec<::serde_json::Value>,
    #[serde(rename = "interactionCellIcon")]
    pub interaction_cell_icon: Option<String>,
    #[serde(rename = "interactionCellIconReverse")]
    pub interaction_cell_icon_reverse: Option<bool>,
    #[serde(rename = "interactionCellOffset")]
    pub interaction_cell_offset: Option<String>,
    pub intricate: Option<bool>,
    #[serde(rename = "isSaveable")]
    pub is_saveable: Option<bool>,
    #[serde(rename = "isTechHediff")]
    pub is_tech_hediff: Option<bool>,
    #[serde(rename = "isUnfinishedThing")]
    pub is_unfinished_thing: Option<bool>,
    #[serde(rename = "killedLeavings")]
    pub killed_leavings: Option<KilledLeavings>,
    pub label: Option<String>,
    #[serde(rename = "leaveResourcesWhenKilled")]
    pub leave_resources_when_killed: Option<bool>,
    #[serde(rename = "maxTechLevelToBuild")]
    pub max_tech_level_to_build: Option<String>,
    #[serde(rename = "messageOnDeteriorateInStorage")]
    pub message_on_deteriorate_in_storage: Option<bool>,
    #[serde(rename = "minRewardCount")]
    pub min_reward_count: Option<i64>,
    #[serde(rename = "minTechLevelToBuild")]
    pub min_tech_level_to_build: Option<String>,
    pub mineable: Option<bool>,
    #[serde(rename = "minifiedDef")]
    pub minified_def: Option<::serde_json::Value>,
    pub mote: Option<Mote>,
    #[serde(rename = "neverMultiSelect")]
    pub never_multi_select: Option<bool>,
    #[serde(rename = "neverOverlapFloors")]
    pub never_overlap_floors: Option<bool>,
    #[serde(rename = "orderedTakeGroup")]
    pub ordered_take_group: Option<String>,
    pub passability: Option<String>,
    #[serde(rename = "pathCost")]
    pub path_cost: Option<i64>,
    #[serde(rename = "pathCostIgnoreRepeat")]
    pub path_cost_ignore_repeat: Option<bool>,
    #[serde(rename = "placeWorkers")]
    #[serde(default)]
    pub place_workers: Vec<String>,
    #[serde(rename = "placingDraggableDimensions")]
    pub placing_draggable_dimensions: Option<i64>,
    pub plant: Option<Plant>,
    pub projectile: Option<Projectile>,
    #[serde(rename = "projectileWhenLoaded")]
    pub projectile_when_loaded: Option<String>,
    pub race: Option<Race>,
    #[serde(rename = "randomizeRotationOnSpawn")]
    pub randomize_rotation_on_spawn: Option<bool>,
    #[serde(rename = "receivesSignals")]
    pub receives_signals: Option<bool>,
    #[serde(rename = "recipeMaker")]
    pub recipe_maker: Option<RecipeMaker>,
    #[serde(default)]
    pub recipes: Vec<::serde_json::Value>,
    #[serde(rename = "recoilPower")]
    pub recoil_power: Option<i64>,
    #[serde(rename = "relicChance")]
    pub relic_chance: Option<i64>,
    #[serde(rename = "repairEffect")]
    pub repair_effect: Option<String>,
    #[serde(rename = "researchPrerequisites")]
    #[serde(default)]
    pub research_prerequisites: Vec<String>,
    #[serde(rename = "resourceReadoutAlwaysShow")]
    pub resource_readout_always_show: Option<bool>,
    #[serde(rename = "resourceReadoutPriority")]
    pub resource_readout_priority: Option<String>,
    #[serde(rename = "resourcesFractionWhenDeconstructed")]
    pub resources_fraction_when_deconstructed: Option<i64>,
    #[serde(rename = "ritualFocus")]
    pub ritual_focus: Option<RitualFocus>,
    pub rotatable: Option<bool>,
    #[serde(rename = "saveCompressible")]
    pub save_compressible: Option<bool>,
    #[serde(rename = "scatterableOnMapGen")]
    pub scatterable_on_map_gen: Option<bool>,
    #[serde(rename = "seeThroughFog")]
    pub see_through_fog: Option<bool>,
    pub selectable: Option<bool>,
    pub size: Option<String>,
    pub skyfaller: Option<::serde_json::Value>,
    #[serde(rename = "slagDef")]
    pub slag_def: Option<String>,
    #[serde(rename = "smallVolume")]
    pub small_volume: Option<bool>,
    #[serde(rename = "smeltProducts")]
    pub smelt_products: Option<SmeltProducts>,
    pub smeltable: Option<bool>,
    #[serde(rename = "socialPropernessMatters")]
    pub social_properness_matters: Option<bool>,
    #[serde(rename = "soundDrop")]
    pub sound_drop: Option<String>,
    #[serde(rename = "soundImpactDefault")]
    pub sound_impact_default: Option<String>,
    #[serde(rename = "soundInteract")]
    pub sound_interact: Option<String>,
    #[serde(rename = "specialDisplayRadius")]
    pub special_display_radius: Option<f64>,
    #[serde(rename = "stackLimit")]
    pub stack_limit: Option<i64>,
    #[serde(rename = "startingHpRange")]
    pub starting_hp_range: Option<::serde_json::Value>,
    #[serde(rename = "statBases")]
    pub stat_bases: Option<StatBases3>,
    #[serde(rename = "staticSunShadowHeight")]
    pub static_sun_shadow_height: Option<::serde_json::Value>,
    pub stealable: Option<bool>,
    #[serde(rename = "storedConceptLearnOpportunity")]
    pub stored_concept_learn_opportunity: Option<String>,
    #[serde(rename = "stuffCategories")]
    #[serde(default)]
    pub stuff_categories: Vec<Option<String>>,
    #[serde(rename = "stuffProps")]
    pub stuff_props: Option<StuffProps>,
    #[serde(rename = "surfaceType")]
    pub surface_type: Option<String>,
    #[serde(rename = "techHediffsTags")]
    #[serde(default)]
    pub tech_hediffs_tags: Vec<String>,
    #[serde(rename = "techLevel")]
    pub tech_level: Option<String>,
    #[serde(rename = "terrainAffordanceNeeded")]
    pub terrain_affordance_needed: Option<::serde_json::Value>,
    #[serde(rename = "thingCategories")]
    #[serde(default)]
    pub thing_categories: Vec<Option<String>>,
    #[serde(rename = "thingClass")]
    pub thing_class: Option<String>,
    #[serde(rename = "thingSetMakerTags")]
    #[serde(default)]
    pub thing_set_maker_tags: Vec<String>,
    #[serde(rename = "tickerType")]
    pub ticker_type: Option<String>,
    #[serde(default)]
    pub tools: Vec<Tool3>,
    #[serde(rename = "tradeNeverStack")]
    pub trade_never_stack: Option<bool>,
    #[serde(rename = "tradeTags")]
    #[serde(default)]
    pub trade_tags: Vec<String>,
    pub tradeability: Option<String>,
    #[serde(rename = "uiIconForStackCount")]
    pub ui_icon_for_stack_count: Option<i64>,
    #[serde(rename = "uiIconOffset")]
    pub ui_icon_offset: Option<String>,
    #[serde(rename = "uiIconPath")]
    pub ui_icon_path: Option<String>,
    #[serde(rename = "uiIconPathsStuff")]
    #[serde(default)]
    pub ui_icon_paths_stuff: Vec<UiIconPathsStuff>,
    #[serde(rename = "uiIconScale")]
    pub ui_icon_scale: Option<::serde_json::Value>,
    #[serde(rename = "useHitPoints")]
    pub use_hit_points: Option<bool>,
    #[serde(rename = "useStuffTerrainAffordance")]
    pub use_stuff_terrain_affordance: Option<bool>,
    #[serde(default)]
    pub verbs: Vec<Verb2>,
    #[serde(rename = "weaponClasses")]
    #[serde(default)]
    pub weapon_classes: Vec<String>,
    #[serde(rename = "weaponTags")]
    #[serde(default)]
    pub weapon_tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Apparel {
    #[serde(rename = "ai_pickUpOpportunistically")]
    pub ai_pick_up_opportunistically: Option<bool>,
    #[serde(rename = "bodyPartGroups")]
    #[serde(default)]
    pub body_part_groups: Vec<String>,
    #[serde(rename = "canBeDesiredForIdeo")]
    pub can_be_desired_for_ideo: Option<bool>,
    #[serde(rename = "careIfDamaged")]
    pub care_if_damaged: Option<bool>,
    #[serde(rename = "careIfWornByCorpse")]
    pub care_if_worn_by_corpse: Option<bool>,
    #[serde(rename = "countsAsClothingForNudity")]
    pub counts_as_clothing_for_nudity: Option<bool>,
    #[serde(rename = "defaultOutfitTags")]
    #[serde(default)]
    pub default_outfit_tags: Vec<String>,
    #[serde(rename = "hatRenderedFrontOfFace")]
    pub hat_rendered_front_of_face: Option<bool>,
    #[serde(rename = "ideoDesireAllowedFactionCategoryTags")]
    #[serde(default)]
    pub ideo_desire_allowed_faction_category_tags: Vec<String>,
    #[serde(rename = "ignoredByNonViolent")]
    pub ignored_by_non_violent: Option<bool>,
    #[serde(default)]
    pub layers: Vec<String>,
    #[serde(rename = "scoreOffset")]
    pub score_offset: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "useDeflectMetalEffect")]
    pub use_deflect_metal_effect: Option<bool>,
    #[serde(rename = "wearPerDay")]
    pub wear_per_day: Option<i64>,
    #[serde(rename = "wornGraphicData")]
    pub worn_graphic_data: Option<WornGraphicData>,
    #[serde(rename = "wornGraphicPath")]
    pub worn_graphic_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WornGraphicData {
    pub east: East,
    pub fat: Fat,
    pub female: Female,
    pub hulk: Hulk,
    pub male: Male,
    pub north: North,
    #[serde(rename = "renderUtilityAsPack")]
    pub render_utility_as_pack: bool,
    pub south: South,
    pub thin: Thin3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct East {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Fat {
    pub scale: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Female {
    pub scale: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Hulk {
    pub scale: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Male {
    pub scale: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct North {
    pub fat: Fat2,
    pub female: Female2,
    pub hulk: Hulk2,
    pub male: Male2,
    pub thin: Thin,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Fat2 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Female2 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Hulk2 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Male2 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Thin {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct South {
    pub fat: Fat3,
    pub female: Female3,
    pub hulk: Hulk3,
    pub male: Male3,
    pub thin: Thin2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Fat3 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Female3 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Hulk3 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Male3 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Thin2 {
    pub offset: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Thin3 {
    pub scale: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Building {
    #[serde(rename = "ai_chillDestination")]
    pub ai_chill_destination: Option<bool>,
    #[serde(rename = "ai_combatDangerous")]
    pub ai_combat_dangerous: Option<bool>,
    #[serde(rename = "ai_neverTrashThis")]
    pub ai_never_trash_this: Option<bool>,
    #[serde(rename = "allowAutoroof")]
    pub allow_autoroof: Option<bool>,
    #[serde(rename = "allowWireConnection")]
    pub allow_wire_connection: Option<bool>,
    #[serde(rename = "alwaysDeconstructible")]
    pub always_deconstructible: Option<bool>,
    #[serde(rename = "artificialForMeditationPurposes")]
    pub artificial_for_meditation_purposes: Option<bool>,
    #[serde(rename = "bed_caravansCanUse")]
    pub bed_caravans_can_use: Option<bool>,
    #[serde(rename = "bed_defaultMedical")]
    pub bed_default_medical: Option<bool>,
    #[serde(rename = "bed_healPerDay")]
    pub bed_heal_per_day: Option<i64>,
    pub bed_humanlike: Option<bool>,
    #[serde(rename = "bed_maxBodySize")]
    pub bed_max_body_size: Option<f64>,
    #[serde(rename = "bed_showSleeperBody")]
    pub bed_show_sleeper_body: Option<bool>,
    #[serde(rename = "blueprintClass")]
    pub blueprint_class: Option<String>,
    #[serde(rename = "blueprintGraphicData")]
    pub blueprint_graphic_data: Option<BlueprintGraphicData>,
    #[serde(rename = "boughtConceptLearnOpportunity")]
    pub bought_concept_learn_opportunity: Option<String>,
    #[serde(rename = "buildingSizeCategory")]
    pub building_size_category: Option<String>,
    #[serde(rename = "buildingTags")]
    #[serde(default)]
    pub building_tags: Vec<String>,
    #[serde(rename = "canBuildNonEdificesUnder")]
    pub can_build_non_edifices_under: Option<bool>,
    #[serde(rename = "canPlaceOverImpassablePlant")]
    pub can_place_over_impassable_plant: Option<bool>,
    #[serde(rename = "canPlaceOverWall")]
    pub can_place_over_wall: Option<bool>,
    pub claimable: Option<bool>,
    #[serde(rename = "combatPower")]
    pub combat_power: Option<i64>,
    pub deconstructible: Option<bool>,
    #[serde(rename = "defaultPlantToGrow")]
    pub default_plant_to_grow: Option<String>,
    #[serde(rename = "defaultStorageSettings")]
    pub default_storage_settings: Option<DefaultStorageSettings>,
    #[serde(rename = "destroyShakeAmount")]
    pub destroy_shake_amount: Option<i64>,
    #[serde(rename = "destroySound")]
    pub destroy_sound: Option<String>,
    #[serde(rename = "effectWatching")]
    pub effect_watching: Option<String>,
    #[serde(rename = "expandHomeArea")]
    pub expand_home_area: Option<bool>,
    #[serde(rename = "fixedStorageSettings")]
    pub fixed_storage_settings: Option<FixedStorageSettings>,
    #[serde(rename = "forceShowRoomStats")]
    pub force_show_room_stats: Option<bool>,
    #[serde(rename = "fullGraveGraphicData")]
    pub full_grave_graphic_data: Option<FullGraveGraphicData>,
    #[serde(rename = "hasFuelingPort")]
    pub has_fueling_port: Option<bool>,
    #[serde(rename = "haulToContainerDuration")]
    pub haul_to_container_duration: Option<i64>,
    #[serde(rename = "heatPerTickWhileWorking")]
    pub heat_per_tick_while_working: Option<f64>,
    #[serde(rename = "ignoreStoredThingsBeauty")]
    pub ignore_stored_things_beauty: Option<bool>,
    #[serde(rename = "isEdifice")]
    pub is_edifice: Option<bool>,
    #[serde(rename = "isFence")]
    pub is_fence: Option<bool>,
    #[serde(rename = "isInert")]
    pub is_inert: Option<bool>,
    #[serde(rename = "isMealSource")]
    pub is_meal_source: Option<bool>,
    #[serde(rename = "isNaturalRock")]
    pub is_natural_rock: Option<bool>,
    #[serde(rename = "isPlaceOverableWall")]
    pub is_place_overable_wall: Option<bool>,
    #[serde(rename = "isPlayerEjectable")]
    pub is_player_ejectable: Option<bool>,
    #[serde(rename = "isResourceRock")]
    pub is_resource_rock: Option<bool>,
    #[serde(rename = "isSittable")]
    pub is_sittable: Option<bool>,
    #[serde(rename = "isTrap")]
    pub is_trap: Option<bool>,
    #[serde(rename = "joyKind")]
    pub joy_kind: Option<String>,
    #[serde(rename = "mineableDropChance")]
    pub mineable_drop_chance: Option<f64>,
    #[serde(rename = "mineableScatterCommonality")]
    pub mineable_scatter_commonality: Option<::serde_json::Value>,
    #[serde(rename = "mineableScatterLumpSizeRange")]
    pub mineable_scatter_lump_size_range: Option<String>,
    #[serde(rename = "mineableThing")]
    pub mineable_thing: Option<String>,
    #[serde(rename = "mineableYield")]
    pub mineable_yield: Option<i64>,
    #[serde(rename = "mineableYieldWasteable")]
    pub mineable_yield_wasteable: Option<bool>,
    #[serde(rename = "nutritionCostPerDispense")]
    pub nutrition_cost_per_dispense: Option<f64>,
    #[serde(rename = "preventDeteriorationInside")]
    pub prevent_deterioration_inside: Option<bool>,
    #[serde(rename = "preventDeteriorationOnTop")]
    pub prevent_deterioration_on_top: Option<bool>,
    #[serde(rename = "relatedBuildCommands")]
    #[serde(default)]
    pub related_build_commands: Vec<String>,
    pub repairable: Option<bool>,
    #[serde(rename = "roamerCanOpen")]
    pub roamer_can_open: Option<bool>,
    #[serde(rename = "roofCollapseDamageMultiplier")]
    pub roof_collapse_damage_multiplier: Option<f64>,
    #[serde(rename = "shipPart")]
    pub ship_part: Option<bool>,
    #[serde(rename = "smoothedThing")]
    pub smoothed_thing: Option<::serde_json::Value>,
    #[serde(rename = "soundAmbient")]
    pub sound_ambient: Option<String>,
    #[serde(rename = "soundDispense")]
    pub sound_dispense: Option<String>,
    #[serde(rename = "soundDoorCloseManual")]
    pub sound_door_close_manual: Option<String>,
    #[serde(rename = "soundDoorClosePowered")]
    pub sound_door_close_powered: Option<String>,
    #[serde(rename = "soundDoorOpenManual")]
    pub sound_door_open_manual: Option<String>,
    #[serde(rename = "soundDoorOpenPowered")]
    pub sound_door_open_powered: Option<String>,
    #[serde(rename = "sowTag")]
    pub sow_tag: Option<String>,
    #[serde(rename = "spawnedConceptLearnOpportunity")]
    pub spawned_concept_learn_opportunity: Option<String>,
    #[serde(rename = "trapDamageCategory")]
    pub trap_damage_category: Option<String>,
    #[serde(rename = "trapDestroyOnSpring")]
    pub trap_destroy_on_spring: Option<bool>,
    #[serde(rename = "trapPeacefulWildAnimalsSpringChanceFactor")]
    pub trap_peaceful_wild_animals_spring_chance_factor: Option<f64>,
    #[serde(rename = "turretBurstCooldownTime")]
    pub turret_burst_cooldown_time: Option<f64>,
    #[serde(rename = "turretBurstWarmupTime")]
    pub turret_burst_warmup_time: Option<f64>,
    #[serde(rename = "turretGunDef")]
    pub turret_gun_def: Option<String>,
    #[serde(rename = "turretTopDrawSize")]
    pub turret_top_draw_size: Option<f64>,
    #[serde(rename = "turretTopOffset")]
    pub turret_top_offset: Option<String>,
    #[serde(rename = "uninstallWork")]
    pub uninstall_work: Option<i64>,
    #[serde(rename = "unpoweredWorkTableWorkSpeedFactor")]
    pub unpowered_work_table_work_speed_factor: Option<f64>,
    #[serde(rename = "wantsHopperAdjacent")]
    pub wants_hopper_adjacent: Option<bool>,
    #[serde(rename = "watchBuildingInSameRoom")]
    pub watch_building_in_same_room: Option<bool>,
    #[serde(rename = "watchBuildingStandDistanceRange")]
    pub watch_building_stand_distance_range: Option<String>,
    #[serde(rename = "watchBuildingStandRectWidth")]
    pub watch_building_stand_rect_width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BlueprintGraphicData {
    #[serde(rename = "graphicClass")]
    pub graphic_class: Option<String>,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DefaultStorageSettings {
    pub filter: Filter3,
    pub priority: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter3 {
    pub categories: Vec<String>,
    #[serde(rename = "disallowedThingDefs")]
    #[serde(default)]
    pub disallowed_thing_defs: Vec<String>,
    #[serde(rename = "specialFiltersToDisallow")]
    #[serde(default)]
    pub special_filters_to_disallow: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedStorageSettings {
    pub filter: Filter4,
    pub priority: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter4 {
    pub categories: Vec<String>,
    #[serde(rename = "disallowNotEverStorable")]
    pub disallow_not_ever_storable: Option<bool>,
    #[serde(rename = "specialFiltersToDisallow")]
    #[serde(default)]
    pub special_filters_to_disallow: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FullGraveGraphicData {
    #[serde(rename = "drawSize")]
    pub draw_size: String,
    #[serde(rename = "graphicClass")]
    pub graphic_class: String,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ButcherProducts {
    #[serde(rename = "BlocksGranite")]
    pub blocks_granite: Option<i64>,
    #[serde(rename = "BlocksLimestone")]
    pub blocks_limestone: Option<i64>,
    #[serde(rename = "BlocksMarble")]
    pub blocks_marble: Option<i64>,
    #[serde(rename = "BlocksSandstone")]
    pub blocks_sandstone: Option<i64>,
    #[serde(rename = "BlocksSlate")]
    pub blocks_slate: Option<i64>,
    #[serde(rename = "Plasteel")]
    pub plasteel: Option<i64>,
    #[serde(rename = "Steel")]
    pub steel: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ColorGenerator {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(default)]
    pub options: Vec<Option>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Option {
    pub max: Option<String>,
    pub min: Option<String>,
    pub only: Option<String>,
    pub weight: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ColorGeneratorInTraderStock {
    #[serde(rename = "Class")]
    pub class: String,
    pub options: Vec<Option2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Option2 {
    pub max: Option<String>,
    pub min: Option<String>,
    pub only: Option<String>,
    pub weight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ColorPerStuff {
    pub color: String,
    pub stuff: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp7 {
    #[serde(rename = "Class")]
    pub class: Option<String>,
    pub addictiveness: Option<f64>,
    #[serde(rename = "allowNonPlayer")]
    pub allow_non_player: Option<bool>,
    #[serde(rename = "ammoCountToRefill")]
    pub ammo_count_to_refill: Option<i64>,
    #[serde(rename = "ammoDef")]
    pub ammo_def: Option<String>,
    #[serde(rename = "anyColonistCloseCheckRadius")]
    pub any_colonist_close_check_radius: Option<i64>,
    #[serde(rename = "applyDamageToExplosionCellsNeighbors")]
    pub apply_damage_to_explosion_cells_neighbors: Option<bool>,
    #[serde(rename = "autoRefuelPercent")]
    pub auto_refuel_percent: Option<::serde_json::Value>,
    #[serde(rename = "basePowerConsumption")]
    pub base_power_consumption: Option<i64>,
    #[serde(rename = "baseReloadTicks")]
    pub base_reload_ticks: Option<i64>,
    #[serde(rename = "brainDamageChance")]
    pub brain_damage_chance: Option<f64>,
    #[serde(rename = "canBeEnjoyedAsArt")]
    pub can_be_enjoyed_as_art: Option<bool>,
    #[serde(rename = "canWakeUpFogged")]
    pub can_wake_up_fogged: Option<bool>,
    #[serde(rename = "chanceNeverExplodeFromDamage")]
    pub chance_never_explode_from_damage: Option<f64>,
    #[serde(rename = "chanceToStartFire")]
    pub chance_to_start_fire: Option<f64>,
    #[serde(rename = "chargeNoun")]
    pub charge_noun: Option<String>,
    pub chemical: Option<String>,
    pub color: Option<String>,
    #[serde(rename = "commandDescKey")]
    pub command_desc_key: Option<String>,
    #[serde(rename = "commandLabelKey")]
    pub command_label_key: Option<String>,
    #[serde(rename = "commandTexture")]
    pub command_texture: Option<String>,
    #[serde(rename = "compClass")]
    pub comp_class: Option<String>,
    #[serde(rename = "conditionDef")]
    pub condition_def: Option<String>,
    #[serde(rename = "consumeFuelOnlyWhenUsed")]
    pub consume_fuel_only_when_used: Option<bool>,
    #[serde(rename = "damageFalloff")]
    pub damage_falloff: Option<bool>,
    #[serde(rename = "damagePerTickRare")]
    pub damage_per_tick_rare: Option<i64>,
    #[serde(rename = "daysToRadius")]
    pub days_to_radius: Option<i64>,
    #[serde(rename = "daysToRotStart")]
    pub days_to_rot_start: Option<::serde_json::Value>,
    #[serde(rename = "descriptionMaker")]
    pub description_maker: Option<String>,
    #[serde(rename = "destroyOnEmpty")]
    pub destroy_on_empty: Option<bool>,
    #[serde(rename = "destroyThingOnExplosionSize")]
    pub destroy_thing_on_explosion_size: Option<i64>,
    #[serde(rename = "disableIfHatcher")]
    pub disable_if_hatcher: Option<bool>,
    #[serde(rename = "displayGizmoWhileUndrafted")]
    pub display_gizmo_while_undrafted: Option<bool>,
    #[serde(rename = "doCameraShake")]
    pub do_camera_shake: Option<bool>,
    #[serde(rename = "drawAssignmentOverlay")]
    pub draw_assignment_overlay: Option<bool>,
    #[serde(rename = "drawFuelGaugeInMap")]
    pub draw_fuel_gauge_in_map: Option<bool>,
    #[serde(rename = "drawOutOfFuelOverlay")]
    pub draw_out_of_fuel_overlay: Option<bool>,
    #[serde(rename = "drawStackLabel")]
    pub draw_stack_label: Option<bool>,
    #[serde(rename = "droneLevel")]
    pub drone_level: Option<i64>,
    #[serde(rename = "droneLevelIncreaseInterval")]
    pub drone_level_increase_interval: Option<i64>,
    pub efficiency: Option<f64>,
    #[serde(rename = "eggCountRange")]
    pub egg_count_range: Option<::serde_json::Value>,
    #[serde(rename = "eggFertilizationCountMax")]
    pub egg_fertilization_count_max: Option<i64>,
    #[serde(rename = "eggFertilizedDef")]
    pub egg_fertilized_def: Option<String>,
    #[serde(rename = "eggLayIntervalDays")]
    pub egg_lay_interval_days: Option<::serde_json::Value>,
    #[serde(rename = "eggProgressUnfertilizedMax")]
    pub egg_progress_unfertilized_max: Option<f64>,
    #[serde(rename = "eggUnfertilizedDef")]
    pub egg_unfertilized_def: Option<String>,
    #[serde(rename = "emissionInterval")]
    pub emission_interval: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<f64>,
    #[serde(rename = "energyPerSecond")]
    pub energy_per_second: Option<i64>,
    #[serde(rename = "existingAddictionSeverityOffset")]
    pub existing_addiction_severity_offset: Option<f64>,
    #[serde(rename = "expireEffect")]
    pub expire_effect: Option<String>,
    #[serde(rename = "explosionEffect")]
    pub explosion_effect: Option<String>,
    #[serde(rename = "explosionSound")]
    pub explosion_sound: Option<String>,
    #[serde(rename = "explosiveDamageType")]
    pub explosive_damage_type: Option<String>,
    #[serde(rename = "explosiveExpandPerFuel")]
    pub explosive_expand_per_fuel: Option<f64>,
    #[serde(rename = "explosiveExpandPerStackcount")]
    pub explosive_expand_per_stackcount: Option<f64>,
    #[serde(rename = "explosiveRadius")]
    pub explosive_radius: Option<::serde_json::Value>,
    #[serde(rename = "factorByDifficulty")]
    pub factor_by_difficulty: Option<bool>,
    #[serde(rename = "filthDef")]
    pub filth_def: Option<String>,
    #[serde(rename = "fireSize")]
    pub fire_size: Option<::serde_json::Value>,
    pub fleck: Option<String>,
    #[serde(rename = "fleckDef")]
    pub fleck_def: Option<String>,
    #[serde(rename = "fleckOnTarget")]
    pub fleck_on_target: Option<String>,
    #[serde(rename = "fleckOnUsed")]
    pub fleck_on_used: Option<String>,
    #[serde(rename = "fleckOnUsedScale")]
    pub fleck_on_used_scale: Option<i64>,
    #[serde(rename = "fleshCorpsesOnly")]
    pub flesh_corpses_only: Option<bool>,
    #[serde(rename = "focusTypes")]
    #[serde(default)]
    pub focus_types: Vec<String>,
    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: Option<::serde_json::Value>,
    #[serde(rename = "fuelConsumptionPerTickInRain")]
    pub fuel_consumption_per_tick_in_rain: Option<f64>,
    #[serde(rename = "fuelConsumptionRate")]
    pub fuel_consumption_rate: Option<f64>,
    #[serde(rename = "fuelFilter")]
    pub fuel_filter: Option<FuelFilter>,
    #[serde(rename = "fuelGizmoLabel")]
    pub fuel_gizmo_label: Option<String>,
    #[serde(rename = "fuelIconPath")]
    pub fuel_icon_path: Option<String>,
    #[serde(rename = "fuelIsMortarBarrel")]
    pub fuel_is_mortar_barrel: Option<bool>,
    #[serde(rename = "fuelLabel")]
    pub fuel_label: Option<String>,
    #[serde(rename = "fuelMultiplier")]
    pub fuel_multiplier: Option<::serde_json::Value>,
    #[serde(rename = "glowColor")]
    pub glow_color: Option<String>,
    #[serde(rename = "glowRadius")]
    pub glow_radius: Option<::serde_json::Value>,
    #[serde(rename = "goodwillImpact")]
    pub goodwill_impact: Option<i64>,
    #[serde(rename = "graphicData")]
    pub graphic_data: Option<GraphicData2>,
    #[serde(rename = "hatcherDaystoHatch")]
    pub hatcher_daysto_hatch: Option<::serde_json::Value>,
    #[serde(rename = "hatcherPawn")]
    pub hatcher_pawn: Option<String>,
    #[serde(rename = "heatPerSecond")]
    pub heat_per_second: Option<::serde_json::Value>,
    #[serde(rename = "heatPushMaxTemperature")]
    pub heat_push_max_temperature: Option<i64>,
    #[serde(rename = "heatPushMinTemperature")]
    pub heat_push_min_temperature: Option<i64>,
    #[serde(rename = "hotKey")]
    pub hot_key: Option<String>,
    #[serde(rename = "ignorePlayerFactionPawns")]
    pub ignore_player_faction_pawns: Option<bool>,
    #[serde(rename = "incidentTargetWhileStarting")]
    pub incident_target_while_starting: Option<String>,
    #[serde(rename = "inheritFaction")]
    pub inherit_faction: Option<bool>,
    #[serde(rename = "initialAllowAutoRefuel")]
    pub initial_allow_auto_refuel: Option<bool>,
    #[serde(rename = "initialConfigurableTargetFuelLevel")]
    pub initial_configurable_target_fuel_level: Option<i64>,
    #[serde(rename = "initialFuelPercent")]
    pub initial_fuel_percent: Option<i64>,
    #[serde(rename = "initialPawnsPoints")]
    pub initial_pawns_points: Option<i64>,
    #[serde(rename = "isCombatEnhancingDrug")]
    pub is_combat_enhancing_drug: Option<bool>,
    #[serde(rename = "largeOverdoseChance")]
    pub large_overdose_chance: Option<f64>,
    #[serde(rename = "lifespanTicks")]
    pub lifespan_ticks: Option<i64>,
    #[serde(rename = "linkableFacilities")]
    #[serde(default)]
    pub linkable_facilities: Vec<::serde_json::Value>,
    #[serde(rename = "listOrder")]
    pub list_order: Option<i64>,
    #[serde(rename = "lordJob")]
    pub lord_job: Option<String>,
    pub mag: Option<f64>,
    #[serde(rename = "manWorkType")]
    pub man_work_type: Option<String>,
    #[serde(rename = "maxCharges")]
    pub max_charges: Option<i64>,
    #[serde(rename = "maxDistance")]
    pub max_distance: Option<i64>,
    #[serde(rename = "maxSafeTemperature")]
    pub max_safe_temperature: Option<i64>,
    #[serde(rename = "maxSimultaneous")]
    pub max_simultaneous: Option<i64>,
    #[serde(rename = "maxSpawnedPawnsPoints")]
    pub max_spawned_pawns_points: Option<i64>,
    #[serde(rename = "milkAmount")]
    pub milk_amount: Option<i64>,
    #[serde(rename = "milkDef")]
    pub milk_def: Option<String>,
    #[serde(rename = "milkFemaleOnly")]
    pub milk_female_only: Option<bool>,
    #[serde(rename = "milkIntervalDays")]
    pub milk_interval_days: Option<i64>,
    #[serde(rename = "minCountToEmpty")]
    pub min_count_to_empty: Option<i64>,
    #[serde(rename = "minQualityForArtistic")]
    pub min_quality_for_artistic: Option<String>,
    #[serde(rename = "minSafeTemperature")]
    pub min_safe_temperature: Option<i64>,
    #[serde(rename = "minToleranceToAddict")]
    pub min_tolerance_to_addict: Option<f64>,
    #[serde(rename = "minimumFueledThreshold")]
    pub minimum_fueled_threshold: Option<i64>,
    pub mote: Option<String>,
    #[serde(rename = "mustBeFullGrave")]
    pub must_be_full_grave: Option<bool>,
    #[serde(rename = "mustBePlacedAdjacent")]
    pub must_be_placed_adjacent: Option<bool>,
    #[serde(rename = "mustBePlacedAdjacentCardinalToBedHead")]
    pub must_be_placed_adjacent_cardinal_to_bed_head: Option<bool>,
    #[serde(rename = "nameMaker")]
    pub name_maker: Option<String>,
    #[serde(rename = "needLevelOffset")]
    pub need_level_offset: Option<f64>,
    #[serde(rename = "nonDessicatedCorpsesOnly")]
    pub non_dessicated_corpses_only: Option<bool>,
    #[serde(rename = "offMessage")]
    pub off_message: Option<String>,
    pub offset: Option<String>,
    #[serde(rename = "offsetMax")]
    pub offset_max: Option<String>,
    #[serde(rename = "offsetMin")]
    pub offset_min: Option<String>,
    #[serde(default)]
    pub offsets: Vec<Offset>,
    #[serde(rename = "outOfFuelMessage")]
    pub out_of_fuel_message: Option<String>,
    #[serde(rename = "overdoseSeverityOffset")]
    pub overdose_severity_offset: Option<String>,
    #[serde(rename = "overlightRadius")]
    pub overlight_radius: Option<f64>,
    #[serde(rename = "performMergeCompatibilityChecks")]
    pub perform_merge_compatibility_checks: Option<bool>,
    #[serde(rename = "postExplosionSpawnChance")]
    pub post_explosion_spawn_chance: Option<i64>,
    #[serde(rename = "postExplosionSpawnThingCount")]
    pub post_explosion_spawn_thing_count: Option<i64>,
    #[serde(rename = "postExplosionSpawnThingDef")]
    pub post_explosion_spawn_thing_def: Option<String>,
    #[serde(rename = "preExplosionSpawnChance")]
    pub pre_explosion_spawn_chance: Option<i64>,
    #[serde(rename = "preExplosionSpawnThingDef")]
    pub pre_explosion_spawn_thing_def: Option<String>,
    #[serde(rename = "progressPerDegreePerTick")]
    pub progress_per_degree_per_tick: Option<f64>,
    #[serde(rename = "psychicSensitiveTargetsOnly")]
    pub psychic_sensitive_targets_only: Option<bool>,
    pub radius: Option<::serde_json::Value>,
    #[serde(rename = "radiusPerDayCurve")]
    pub radius_per_day_curve: Option<RadiusPerDayCurve>,
    #[serde(rename = "requiredDamageTypeToExplode")]
    pub required_damage_type_to_explode: Option<String>,
    #[serde(rename = "requiresPower")]
    pub requires_power: Option<bool>,
    #[serde(rename = "restEffectiveness")]
    pub rest_effectiveness: Option<f64>,
    #[serde(rename = "rotDestroys")]
    pub rot_destroys: Option<bool>,
    #[serde(rename = "rotationRate")]
    pub rotation_rate: Option<String>,
    #[serde(rename = "saveKeysPrefix")]
    pub save_keys_prefix: Option<String>,
    pub scale: Option<String>,
    #[serde(rename = "scanFindGuaranteedDays")]
    pub scan_find_guaranteed_days: Option<i64>,
    #[serde(rename = "scanFindMtbDays")]
    pub scan_find_mtb_days: Option<i64>,
    #[serde(rename = "scanSpeedStat")]
    pub scan_speed_stat: Option<String>,
    #[serde(rename = "shearIntervalDays")]
    pub shear_interval_days: Option<i64>,
    #[serde(rename = "shortCircuitInRain")]
    pub short_circuit_in_rain: Option<bool>,
    #[serde(rename = "showAllowAutoRefuelToggle")]
    pub show_allow_auto_refuel_toggle: Option<bool>,
    #[serde(rename = "showFuelGizmo")]
    pub show_fuel_gizmo: Option<bool>,
    #[serde(rename = "showMessageIfOwned")]
    pub show_message_if_owned: Option<bool>,
    #[serde(rename = "skyColors")]
    pub sky_colors: Option<SkyColors>,
    #[serde(rename = "skyfallerLeaving")]
    pub skyfaller_leaving: Option<String>,
    pub sound: Option<String>,
    #[serde(rename = "soundAmbientProducingPower")]
    pub sound_ambient_producing_power: Option<String>,
    #[serde(rename = "soundOnUsed")]
    pub sound_on_used: Option<String>,
    #[serde(rename = "soundReload")]
    pub sound_reload: Option<String>,
    #[serde(rename = "soundWorking")]
    pub sound_working: Option<String>,
    #[serde(rename = "spawnCount")]
    pub spawn_count: Option<i64>,
    #[serde(rename = "spawnCountOnSpawn")]
    pub spawn_count_on_spawn: Option<i64>,
    #[serde(rename = "spawnForbidden")]
    pub spawn_forbidden: Option<bool>,
    #[serde(rename = "spawnIntervalRange")]
    pub spawn_interval_range: Option<::serde_json::Value>,
    #[serde(rename = "spawnMaxAdjacent")]
    pub spawn_max_adjacent: Option<i64>,
    #[serde(rename = "spawnMtbHours")]
    pub spawn_mtb_hours: Option<i64>,
    #[serde(rename = "spawnRadius")]
    pub spawn_radius: Option<i64>,
    #[serde(rename = "spawnSound")]
    pub spawn_sound: Option<String>,
    #[serde(rename = "spawnablePawnKinds")]
    #[serde(default)]
    pub spawnable_pawn_kinds: Vec<String>,
    #[serde(rename = "stackLimit")]
    pub stack_limit: Option<i64>,
    #[serde(rename = "startTime")]
    pub start_time: Option<f64>,
    #[serde(rename = "startWickHitPointsPercent")]
    pub start_wick_hit_points_percent: Option<::serde_json::Value>,
    #[serde(rename = "startWickOnDamageTaken")]
    #[serde(default)]
    pub start_wick_on_damage_taken: Vec<String>,
    #[serde(rename = "startsDormant")]
    pub starts_dormant: Option<bool>,
    #[serde(rename = "statDef")]
    pub stat_def: Option<String>,
    #[serde(rename = "statOffsets")]
    pub stat_offsets: Option<StatOffsets2>,
    #[serde(rename = "storedEnergyMax")]
    pub stored_energy_max: Option<i64>,
    #[serde(rename = "sustainerActive")]
    pub sustainer_active: Option<String>,
    pub target: Option<String>,
    #[serde(rename = "targetFuelLevelConfigurable")]
    pub target_fuel_level_configurable: Option<bool>,
    #[serde(rename = "thingToSpawn")]
    pub thing_to_spawn: Option<String>,
    #[serde(rename = "ticksHealthy")]
    pub ticks_healthy: Option<i64>,
    #[serde(rename = "ticksNeedsMaintenance")]
    pub ticks_needs_maintenance: Option<i64>,
    #[serde(rename = "ticksPerHeal")]
    pub ticks_per_heal: Option<i64>,
    #[serde(rename = "transmitsPower")]
    pub transmits_power: Option<bool>,
    #[serde(rename = "useDuration")]
    pub use_duration: Option<i64>,
    #[serde(rename = "useJob")]
    pub use_job: Option<String>,
    #[serde(rename = "useLabel")]
    pub use_label: Option<String>,
    #[serde(rename = "velocityX")]
    pub velocity_x: Option<String>,
    #[serde(rename = "velocityY")]
    pub velocity_y: Option<String>,
    #[serde(rename = "wakeUpOnDamage")]
    pub wake_up_on_damage: Option<bool>,
    #[serde(rename = "wakeUpSound")]
    pub wake_up_sound: Option<String>,
    #[serde(rename = "wickTicks")]
    pub wick_ticks: Option<::serde_json::Value>,
    pub width: Option<::serde_json::Value>,
    #[serde(rename = "woolAmount")]
    pub wool_amount: Option<i64>,
    #[serde(rename = "woolDef")]
    pub wool_def: Option<String>,
    #[serde(rename = "worldRange")]
    pub world_range: Option<i64>,
    #[serde(rename = "writeTimeLeftToSpawn")]
    pub write_time_left_to_spawn: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FuelFilter {
    #[serde(rename = "thingDefs")]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GraphicData2 {
    #[serde(rename = "drawSize")]
    pub draw_size: String,
    #[serde(rename = "graphicClass")]
    pub graphic_class: String,
    #[serde(rename = "texPath")]
    pub tex_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Offset {
    #[serde(rename = "Class")]
    pub class: String,
    pub curve: Option<Curve7>,
    #[serde(default)]
    pub defs: Vec<::serde_json::Value>,
    #[serde(rename = "explanationKey")]
    pub explanation_key: Option<String>,
    #[serde(rename = "explanationKeyAbstract")]
    pub explanation_key_abstract: Option<String>,
    #[serde(rename = "focusPerFullGrave")]
    pub focus_per_full_grave: Option<f64>,
    #[serde(rename = "focusPerQuality")]
    pub focus_per_quality: Option<FocusPerQuality>,
    #[serde(rename = "maxBuildings")]
    pub max_buildings: Option<i64>,
    pub offset: Option<f64>,
    #[serde(rename = "offsetPerBuilding")]
    pub offset_per_building: Option<f64>,
    pub radius: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Curve7 {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FocusPerQuality {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RadiusPerDayCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkyColors {
    pub overlay: String,
    pub saturation: f64,
    pub shadow: String,
    pub sky: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatOffsets2 {
    #[serde(rename = "Comfort")]
    pub comfort: Option<f64>,
    #[serde(rename = "ImmunityGainSpeedFactor")]
    pub immunity_gain_speed_factor: Option<f64>,
    #[serde(rename = "MedicalTendQualityOffset")]
    pub medical_tend_quality_offset: Option<f64>,
    #[serde(rename = "ResearchSpeedFactor")]
    pub research_speed_factor: Option<f64>,
    #[serde(rename = "SurgerySuccessChanceFactor")]
    pub surgery_success_chance_factor: Option<f64>,
    #[serde(rename = "WorkTableWorkSpeedFactor")]
    pub work_table_work_speed_factor: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CostList2 {
    #[serde(rename = "AIPersonaCore")]
    pub aipersona_core: Option<i64>,
    #[serde(rename = "Chemfuel")]
    pub chemfuel: Option<i64>,
    #[serde(rename = "Cloth")]
    pub cloth: Option<i64>,
    #[serde(rename = "ComponentIndustrial")]
    pub component_industrial: Option<i64>,
    #[serde(rename = "ComponentSpacer")]
    pub component_spacer: Option<i64>,
    #[serde(rename = "Gold")]
    pub gold: Option<i64>,
    #[serde(rename = "MedicineHerbal")]
    pub medicine_herbal: Option<i64>,
    #[serde(rename = "Neutroamine")]
    pub neutroamine: Option<i64>,
    #[serde(rename = "Plasteel")]
    pub plasteel: Option<i64>,
    #[serde(rename = "PsychoidLeaves")]
    pub psychoid_leaves: Option<i64>,
    #[serde(rename = "Shell_AntigrainWarhead")]
    pub shell_antigrain_warhead: Option<i64>,
    #[serde(rename = "Shell_EMP")]
    pub shell_emp: Option<i64>,
    #[serde(rename = "Shell_Firefoam")]
    pub shell_firefoam: Option<i64>,
    #[serde(rename = "Shell_HighExplosive")]
    pub shell_high_explosive: Option<i64>,
    #[serde(rename = "Shell_Incendiary")]
    pub shell_incendiary: Option<i64>,
    #[serde(rename = "Shell_Smoke")]
    pub shell_smoke: Option<i64>,
    #[serde(rename = "SmokeleafLeaves")]
    pub smokeleaf_leaves: Option<i64>,
    #[serde(rename = "Steel")]
    pub steel: Option<i64>,
    #[serde(rename = "Uranium")]
    pub uranium: Option<i64>,
    #[serde(rename = "WoodLog")]
    pub wood_log: Option<i64>,
    #[serde(rename = "Yayo")]
    pub yayo: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CostListForDifficulty {
    #[serde(rename = "costList")]
    pub cost_list: CostList3,
    #[serde(rename = "costStuffCount")]
    pub cost_stuff_count: Option<i64>,
    #[serde(rename = "difficultyVar")]
    pub difficulty_var: String,
    pub invert: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CostList3 {
    #[serde(rename = "Chemfuel")]
    pub chemfuel: Option<i64>,
    #[serde(rename = "ComponentIndustrial")]
    pub component_industrial: Option<i64>,
    #[serde(rename = "ReinforcedBarrel")]
    pub reinforced_barrel: Option<i64>,
    #[serde(rename = "Steel")]
    pub steel: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DamageMultiplier {
    #[serde(rename = "damageDef")]
    pub damage_def: String,
    pub multiplier: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EquippedStatOffsets {
    #[serde(rename = "MoveSpeed")]
    pub move_speed: Option<f64>,
    #[serde(rename = "PainShockThreshold")]
    pub pain_shock_threshold: Option<f64>,
    #[serde(rename = "PsychicSensitivity")]
    pub psychic_sensitivity: Option<f64>,
    #[serde(rename = "SlaveSuppressionOffset")]
    pub slave_suppression_offset: Option<SlaveSuppressionOffset>,
    #[serde(rename = "SocialImpact")]
    pub social_impact: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SlaveSuppressionOffset {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filth {
    #[serde(rename = "allowsFire")]
    pub allows_fire: Option<bool>,
    #[serde(rename = "canFilthAttach")]
    pub can_filth_attach: Option<bool>,
    #[serde(rename = "cleaningSound")]
    pub cleaning_sound: Option<String>,
    #[serde(rename = "cleaningWorkToReduceThickness")]
    pub cleaning_work_to_reduce_thickness: Option<i64>,
    #[serde(rename = "disappearsInDays")]
    pub disappears_in_days: Option<String>,
    #[serde(rename = "ignoreFilthMultiplierStat")]
    pub ignore_filth_multiplier_stat: Option<bool>,
    #[serde(rename = "maxThickness")]
    pub max_thickness: Option<i64>,
    #[serde(rename = "placementMask")]
    #[serde(default)]
    pub placement_mask: Vec<String>,
    #[serde(rename = "rainWashes")]
    pub rain_washes: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Gas {
    #[serde(rename = "accuracyPenalty")]
    pub accuracy_penalty: f64,
    #[serde(rename = "blockTurretTracking")]
    pub block_turret_tracking: bool,
    #[serde(rename = "expireSeconds")]
    pub expire_seconds: String,
    #[serde(rename = "rotationSpeed")]
    pub rotation_speed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GraphicData3 {
    #[serde(rename = "allowFlip")]
    pub allow_flip: Option<bool>,
    #[serde(rename = "asymmetricLink")]
    pub asymmetric_link: Option<AsymmetricLink>,
    pub color: Option<String>,
    #[serde(rename = "colorTwo")]
    pub color_two: Option<String>,
    #[serde(rename = "damageData")]
    pub damage_data: Option<DamageData>,
    #[serde(rename = "drawOffset")]
    pub draw_offset: Option<String>,
    #[serde(rename = "drawRotated")]
    pub draw_rotated: Option<bool>,
    #[serde(rename = "drawSize")]
    pub draw_size: Option<::serde_json::Value>,
    #[serde(rename = "flipExtraRotation")]
    pub flip_extra_rotation: Option<i64>,
    #[serde(rename = "graphicClass")]
    pub graphic_class: Option<String>,
    #[serde(rename = "linkFlags")]
    #[serde(default)]
    pub link_flags: Vec<String>,
    #[serde(rename = "linkType")]
    pub link_type: Option<String>,
    #[serde(rename = "onGroundRandomRotateAngle")]
    pub on_ground_random_rotate_angle: Option<i64>,
    #[serde(rename = "shaderParameters")]
    pub shader_parameters: Option<ShaderParameters2>,
    #[serde(rename = "shaderType")]
    pub shader_type: Option<String>,
    #[serde(rename = "shadowData")]
    pub shadow_data: Option<ShadowData3>,
    #[serde(rename = "texPath")]
    pub tex_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AsymmetricLink {
    #[serde(rename = "drawDoorBorderEast")]
    pub draw_door_border_east: DrawDoorBorderEast,
    #[serde(rename = "drawDoorBorderWest")]
    pub draw_door_border_west: DrawDoorBorderWest,
    #[serde(rename = "linkFlags")]
    pub link_flags: Vec<String>,
    #[serde(rename = "linkToDoors")]
    pub link_to_doors: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DrawDoorBorderEast {
    pub color: String,
    pub offset: String,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DrawDoorBorderWest {
    pub color: String,
    pub offset: String,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DamageData {
    #[serde(rename = "cornerBL")]
    pub corner_bl: Option<String>,
    #[serde(rename = "cornerBR")]
    pub corner_br: Option<String>,
    #[serde(rename = "cornerTL")]
    pub corner_tl: Option<String>,
    #[serde(rename = "cornerTR")]
    pub corner_tr: Option<String>,
    #[serde(rename = "edgeBot")]
    pub edge_bot: Option<String>,
    #[serde(rename = "edgeLeft")]
    pub edge_left: Option<String>,
    #[serde(rename = "edgeRight")]
    pub edge_right: Option<String>,
    #[serde(rename = "edgeTop")]
    pub edge_top: Option<String>,
    pub enabled: Option<bool>,
    pub rect: Option<String>,
    #[serde(rename = "rectE")]
    pub rect_e: Option<String>,
    #[serde(rename = "rectN")]
    pub rect_n: Option<String>,
    #[serde(rename = "rectS")]
    pub rect_s: Option<String>,
    #[serde(rename = "rectW")]
    pub rect_w: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShaderParameters2 {
    #[serde(rename = "_FallBehaviorEnabled")]
    pub fall_behavior_enabled: Option<i64>,
    #[serde(rename = "_FallColorDestination")]
    pub fall_color_destination: Option<String>,
    #[serde(rename = "_FallColorSource")]
    pub fall_color_source: Option<String>,
    #[serde(rename = "_FallTransitionRange")]
    pub fall_transition_range: Option<String>,
    #[serde(rename = "_brightnessMultiplier")]
    pub brightness_multiplier: Option<f64>,
    #[serde(rename = "_distortionIntensity")]
    pub distortion_intensity: Option<f64>,
    #[serde(rename = "_distortionScale")]
    pub distortion_scale: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShadowData3 {
    pub offset: Option<String>,
    pub volume: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IdeoBuildingNamerBase {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Ingredient2 {
    #[serde(rename = "mergeCompatibilityTags")]
    pub merge_compatibility_tags: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct KilledLeavings {
    #[serde(rename = "ChunkSlagSteel")]
    pub chunk_slag_steel: Option<i64>,
    #[serde(rename = "ComponentIndustrial")]
    pub component_industrial: Option<i64>,
    #[serde(rename = "ComponentSpacer")]
    pub component_spacer: Option<i64>,
    #[serde(rename = "InsectJelly")]
    pub insect_jelly: Option<i64>,
    #[serde(rename = "Plasteel")]
    pub plasteel: Option<i64>,
    #[serde(rename = "Steel")]
    pub steel: Option<i64>,
    #[serde(rename = "Uranium")]
    pub uranium: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Mote {
    pub acceleration: Option<String>,
    #[serde(rename = "attachedDrawOffset")]
    pub attached_draw_offset: Option<String>,
    pub collide: Option<bool>,
    #[serde(rename = "fadeInTime")]
    pub fade_in_time: Option<::serde_json::Value>,
    #[serde(rename = "fadeOutTime")]
    pub fade_out_time: Option<::serde_json::Value>,
    #[serde(rename = "growthRate")]
    pub growth_rate: Option<::serde_json::Value>,
    #[serde(rename = "needsMaintenance")]
    pub needs_maintenance: Option<bool>,
    #[serde(rename = "realTime")]
    pub real_time: Option<bool>,
    #[serde(rename = "rotateTowardsMoveDirection")]
    pub rotate_towards_move_direction: Option<bool>,
    #[serde(rename = "rotateTowardsTarget")]
    pub rotate_towards_target: Option<bool>,
    #[serde(rename = "solidTime")]
    pub solid_time: ::serde_json::Value,
    #[serde(rename = "speedPerTime")]
    pub speed_per_time: Option<::serde_json::Value>,
    #[serde(rename = "unattachedDrawOffset")]
    pub unattached_draw_offset: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Plant {
    #[serde(rename = "allowAutoCut")]
    pub allow_auto_cut: Option<bool>,
    #[serde(rename = "blockAdjacentSow")]
    pub block_adjacent_sow: Option<bool>,
    #[serde(rename = "burnedThingDef")]
    pub burned_thing_def: Option<String>,
    #[serde(rename = "cavePlant")]
    pub cave_plant: Option<bool>,
    #[serde(rename = "cavePlantWeight")]
    pub cave_plant_weight: Option<f64>,
    #[serde(rename = "dieIfLeafless")]
    pub die_if_leafless: Option<bool>,
    #[serde(rename = "fertilityMin")]
    pub fertility_min: Option<f64>,
    #[serde(rename = "fertilitySensitivity")]
    pub fertility_sensitivity: Option<::serde_json::Value>,
    #[serde(rename = "forceIsTree")]
    pub force_is_tree: Option<bool>,
    #[serde(rename = "growDays")]
    pub grow_days: Option<::serde_json::Value>,
    #[serde(rename = "growMinGlow")]
    pub grow_min_glow: Option<::serde_json::Value>,
    #[serde(rename = "growOptimalGlow")]
    pub grow_optimal_glow: Option<i64>,
    #[serde(rename = "harvestAfterGrowth")]
    pub harvest_after_growth: Option<f64>,
    #[serde(rename = "harvestFailable")]
    pub harvest_failable: Option<bool>,
    #[serde(rename = "harvestMinGrowth")]
    pub harvest_min_growth: Option<f64>,
    #[serde(rename = "harvestTag")]
    pub harvest_tag: Option<String>,
    #[serde(rename = "harvestWork")]
    pub harvest_work: Option<i64>,
    #[serde(rename = "harvestYield")]
    pub harvest_yield: Option<i64>,
    #[serde(rename = "harvestedThingDef")]
    pub harvested_thing_def: Option<String>,
    #[serde(rename = "humanFoodPlant")]
    pub human_food_plant: Option<bool>,
    #[serde(rename = "immatureGraphicPath")]
    pub immature_graphic_path: Option<String>,
    #[serde(rename = "interferesWithRoof")]
    pub interferes_with_roof: Option<bool>,
    #[serde(rename = "leaflessGraphicPath")]
    pub leafless_graphic_path: Option<String>,
    #[serde(rename = "lifespanDaysPerGrowDays")]
    pub lifespan_days_per_grow_days: Option<i64>,
    #[serde(rename = "maxMeshCount")]
    pub max_mesh_count: Option<i64>,
    #[serde(rename = "mustBeWildToSow")]
    pub must_be_wild_to_sow: Option<bool>,
    #[serde(rename = "neverBlightable")]
    pub never_blightable: Option<bool>,
    pub purpose: Option<String>,
    #[serde(rename = "soundHarvestFinish")]
    pub sound_harvest_finish: Option<String>,
    #[serde(rename = "soundHarvesting")]
    pub sound_harvesting: Option<String>,
    #[serde(rename = "sowMinSkill")]
    pub sow_min_skill: Option<i64>,
    #[serde(rename = "sowResearchPrerequisites")]
    #[serde(default)]
    pub sow_research_prerequisites: Vec<String>,
    #[serde(rename = "sowTags")]
    #[serde(default)]
    pub sow_tags: Vec<String>,
    #[serde(rename = "sowWork")]
    pub sow_work: Option<i64>,
    #[serde(rename = "topWindExposure")]
    pub top_wind_exposure: Option<f64>,
    #[serde(rename = "treeCategory")]
    pub tree_category: Option<String>,
    #[serde(rename = "treeLoversCareIfChopped")]
    pub tree_lovers_care_if_chopped: Option<bool>,
    #[serde(rename = "visualSizeRange")]
    pub visual_size_range: Option<String>,
    #[serde(rename = "wildClusterRadius")]
    pub wild_cluster_radius: Option<i64>,
    #[serde(rename = "wildClusterWeight")]
    pub wild_cluster_weight: Option<::serde_json::Value>,
    #[serde(rename = "wildEqualLocalDistribution")]
    pub wild_equal_local_distribution: Option<bool>,
    #[serde(rename = "wildOrder")]
    pub wild_order: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Projectile {
    #[serde(rename = "ai_IsIncendiary")]
    pub ai_is_incendiary: Option<bool>,
    #[serde(rename = "alwaysFreeIntercept")]
    pub always_free_intercept: Option<bool>,
    #[serde(rename = "applyDamageToExplosionCellsNeighbors")]
    pub apply_damage_to_explosion_cells_neighbors: Option<bool>,
    #[serde(rename = "arcHeightFactor")]
    pub arc_height_factor: Option<::serde_json::Value>,
    #[serde(rename = "armorPenetrationBase")]
    pub armor_penetration_base: Option<f64>,
    #[serde(rename = "damageAmountBase")]
    pub damage_amount_base: Option<i64>,
    #[serde(rename = "damageDef")]
    pub damage_def: Option<String>,
    #[serde(rename = "explosionChanceToStartFire")]
    pub explosion_chance_to_start_fire: Option<f64>,
    #[serde(rename = "explosionDamageFalloff")]
    pub explosion_damage_falloff: Option<bool>,
    #[serde(rename = "explosionDelay")]
    pub explosion_delay: Option<i64>,
    #[serde(rename = "explosionEffect")]
    pub explosion_effect: Option<String>,
    #[serde(rename = "explosionRadius")]
    pub explosion_radius: Option<::serde_json::Value>,
    #[serde(rename = "flyOverhead")]
    pub fly_overhead: Option<bool>,
    #[serde(rename = "postExplosionSpawnChance")]
    pub post_explosion_spawn_chance: Option<i64>,
    #[serde(rename = "postExplosionSpawnThingCount")]
    pub post_explosion_spawn_thing_count: Option<i64>,
    #[serde(rename = "postExplosionSpawnThingDef")]
    pub post_explosion_spawn_thing_def: Option<String>,
    #[serde(rename = "preExplosionSpawnChance")]
    pub pre_explosion_spawn_chance: Option<f64>,
    #[serde(rename = "preExplosionSpawnThingDef")]
    pub pre_explosion_spawn_thing_def: Option<String>,
    #[serde(rename = "shadowSize")]
    pub shadow_size: Option<f64>,
    #[serde(rename = "soundAmbient")]
    pub sound_ambient: Option<String>,
    #[serde(rename = "soundExplode")]
    pub sound_explode: Option<String>,
    #[serde(rename = "soundHitThickRoof")]
    pub sound_hit_thick_roof: Option<String>,
    #[serde(rename = "soundImpactAnticipate")]
    pub sound_impact_anticipate: Option<String>,
    pub speed: Option<::serde_json::Value>,
    #[serde(rename = "stoppingPower")]
    pub stopping_power: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Race {
    #[serde(rename = "ageGenerationCurve")]
    pub age_generation_curve: Option<AgeGenerationCurve>,
    #[serde(rename = "animalType")]
    pub animal_type: Option<String>,
    #[serde(rename = "baseBodySize")]
    pub base_body_size: Option<::serde_json::Value>,
    #[serde(rename = "baseHealthScale")]
    pub base_health_scale: Option<::serde_json::Value>,
    #[serde(rename = "baseHungerRate")]
    pub base_hunger_rate: Option<::serde_json::Value>,
    #[serde(rename = "bloodDef")]
    pub blood_def: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "canBePredatorPrey")]
    pub can_be_predator_prey: Option<bool>,
    #[serde(rename = "deathActionWorkerClass")]
    pub death_action_worker_class: Option<String>,
    #[serde(rename = "executionRange")]
    pub execution_range: Option<i64>,
    #[serde(rename = "fleshType")]
    pub flesh_type: Option<String>,
    #[serde(rename = "foodType")]
    pub food_type: Option<String>,
    #[serde(rename = "gestationPeriodDays")]
    pub gestation_period_days: Option<::serde_json::Value>,
    #[serde(rename = "hasGenders")]
    pub has_genders: Option<bool>,
    #[serde(rename = "hediffGiverSets")]
    #[serde(default)]
    pub hediff_giver_sets: Vec<String>,
    #[serde(rename = "herdAnimal")]
    pub herd_animal: Option<bool>,
    #[serde(rename = "herdMigrationAllowed")]
    pub herd_migration_allowed: Option<bool>,
    pub intelligence: Option<String>,
    #[serde(rename = "leatherDef")]
    pub leather_def: Option<String>,
    #[serde(rename = "lifeExpectancy")]
    pub life_expectancy: Option<i64>,
    #[serde(rename = "lifeStageAges")]
    #[serde(default)]
    pub life_stage_ages: Vec<LifeStageAge>,
    #[serde(rename = "litterSizeCurve")]
    pub litter_size_curve: Option<LitterSizeCurve>,
    #[serde(rename = "makesFootprints")]
    pub makes_footprints: Option<bool>,
    #[serde(rename = "manhunterOnDamageChance")]
    pub manhunter_on_damage_chance: Option<f64>,
    #[serde(rename = "manhunterOnTameFailChance")]
    pub manhunter_on_tame_fail_chance: Option<f64>,
    #[serde(rename = "mateMtbHours")]
    pub mate_mtb_hours: Option<i64>,
    #[serde(rename = "maxPreyBodySize")]
    pub max_prey_body_size: Option<::serde_json::Value>,
    #[serde(rename = "meatColor")]
    pub meat_color: Option<String>,
    #[serde(rename = "meatLabel")]
    pub meat_label: Option<String>,
    #[serde(rename = "meatMarketValue")]
    pub meat_market_value: Option<f64>,
    #[serde(rename = "nameCategory")]
    pub name_category: Option<String>,
    #[serde(rename = "nameGenerator")]
    pub name_generator: Option<String>,
    #[serde(rename = "nameGeneratorFemale")]
    pub name_generator_female: Option<String>,
    #[serde(rename = "nameOnTameChance")]
    pub name_on_tame_chance: Option<i64>,
    #[serde(rename = "needsRest")]
    pub needs_rest: Option<bool>,
    #[serde(rename = "nuzzleMtbHours")]
    pub nuzzle_mtb_hours: Option<i64>,
    #[serde(rename = "packAnimal")]
    pub pack_animal: Option<bool>,
    pub petness: Option<::serde_json::Value>,
    pub predator: Option<bool>,
    #[serde(rename = "roamMtbDays")]
    pub roam_mtb_days: Option<i64>,
    #[serde(rename = "soundCallIntervalRange")]
    pub sound_call_interval_range: Option<String>,
    #[serde(rename = "soundMeleeDodge")]
    pub sound_melee_dodge: Option<String>,
    #[serde(rename = "soundMeleeHitBuilding")]
    pub sound_melee_hit_building: Option<String>,
    #[serde(rename = "soundMeleeHitPawn")]
    pub sound_melee_hit_pawn: Option<String>,
    #[serde(rename = "soundMeleeMiss")]
    pub sound_melee_miss: Option<String>,
    #[serde(rename = "specialShadowData")]
    pub special_shadow_data: Option<SpecialShadowData>,
    #[serde(rename = "thinkTreeConstant")]
    pub think_tree_constant: Option<String>,
    #[serde(rename = "thinkTreeMain")]
    pub think_tree_main: Option<String>,
    pub trainability: Option<String>,
    #[serde(rename = "useMeatFrom")]
    pub use_meat_from: Option<String>,
    pub wildness: Option<::serde_json::Value>,
    #[serde(rename = "willNeverEat")]
    #[serde(default)]
    pub will_never_eat: Vec<WillNeverEat>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AgeGenerationCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LifeStageAge {
    pub def: String,
    #[serde(rename = "minAge")]
    pub min_age: ::serde_json::Value,
    #[serde(rename = "soundAngry")]
    pub sound_angry: Option<String>,
    #[serde(rename = "soundCall")]
    pub sound_call: Option<String>,
    #[serde(rename = "soundDeath")]
    pub sound_death: Option<String>,
    #[serde(rename = "soundWounded")]
    pub sound_wounded: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LitterSizeCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SpecialShadowData {
    pub offset: String,
    pub volume: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WillNeverEat {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RecipeMaker {
    #[serde(rename = "bulkRecipeCount")]
    pub bulk_recipe_count: Option<i64>,
    #[serde(rename = "defaultIngredientFilter")]
    pub default_ingredient_filter: Option<DefaultIngredientFilter2>,
    #[serde(rename = "effectWorking")]
    pub effect_working: Option<String>,
    #[serde(rename = "factionPrerequisiteTags")]
    #[serde(default)]
    pub faction_prerequisite_tags: Vec<String>,
    #[serde(rename = "recipeUsers")]
    #[serde(default)]
    pub recipe_users: Vec<String>,
    #[serde(rename = "requiredGiverWorkType")]
    pub required_giver_work_type: Option<String>,
    #[serde(rename = "researchPrerequisite")]
    pub research_prerequisite: Option<::serde_json::Value>,
    #[serde(rename = "skillRequirements")]
    pub skill_requirements: Option<SkillRequirements2>,
    #[serde(rename = "soundWorking")]
    pub sound_working: Option<String>,
    #[serde(rename = "targetCountAdjustment")]
    pub target_count_adjustment: Option<i64>,
    #[serde(rename = "unfinishedThingDef")]
    pub unfinished_thing_def: Option<String>,
    #[serde(rename = "useIngredientsForColor")]
    pub use_ingredients_for_color: Option<bool>,
    #[serde(rename = "workAmount")]
    pub work_amount: Option<i64>,
    #[serde(rename = "workSkill")]
    pub work_skill: Option<String>,
    #[serde(rename = "workSpeedStat")]
    pub work_speed_stat: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DefaultIngredientFilter2 {
    pub categories: Vec<String>,
    #[serde(rename = "disallowedThingDefs")]
    pub disallowed_thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkillRequirements2 {
    #[serde(rename = "Cooking")]
    pub cooking: Option<i64>,
    #[serde(rename = "Crafting")]
    pub crafting: Option<i64>,
    #[serde(rename = "Intellectual")]
    pub intellectual: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RitualFocus {
    #[serde(rename = "allowedSpectateSides")]
    pub allowed_spectate_sides: String,
    #[serde(rename = "spectateDistance")]
    pub spectate_distance: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SmeltProducts {
    #[serde(rename = "Steel")]
    pub steel: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatBases3 {
    #[serde(rename = "AccuracyLong")]
    pub accuracy_long: Option<f64>,
    #[serde(rename = "AccuracyMedium")]
    pub accuracy_medium: Option<f64>,
    #[serde(rename = "AccuracyShort")]
    pub accuracy_short: Option<f64>,
    #[serde(rename = "AccuracyTouch")]
    pub accuracy_touch: Option<f64>,
    #[serde(rename = "ArmorRating_Blunt")]
    pub armor_rating_blunt: Option<f64>,
    #[serde(rename = "ArmorRating_Heat")]
    pub armor_rating_heat: Option<f64>,
    #[serde(rename = "ArmorRating_Sharp")]
    pub armor_rating_sharp: Option<f64>,
    #[serde(rename = "Beauty")]
    pub beauty: Option<::serde_json::Value>,
    #[serde(rename = "BeautyOutdoors")]
    pub beauty_outdoors: Option<i64>,
    #[serde(rename = "BedRestEffectiveness")]
    pub bed_rest_effectiveness: Option<::serde_json::Value>,
    #[serde(rename = "BluntDamageMultiplier")]
    pub blunt_damage_multiplier: Option<::serde_json::Value>,
    #[serde(rename = "CaravanRidingSpeedFactor")]
    pub caravan_riding_speed_factor: Option<f64>,
    #[serde(rename = "Cleanliness")]
    pub cleanliness: Option<i64>,
    #[serde(rename = "Comfort")]
    pub comfort: Option<f64>,
    #[serde(rename = "ComfyTemperatureMax")]
    pub comfy_temperature_max: Option<i64>,
    #[serde(rename = "ComfyTemperatureMin")]
    pub comfy_temperature_min: Option<i64>,
    #[serde(rename = "ConstructionSpeedFactor")]
    pub construction_speed_factor: Option<f64>,
    #[serde(rename = "DeteriorationRate")]
    pub deterioration_rate: Option<::serde_json::Value>,
    #[serde(rename = "DoorOpenSpeed")]
    pub door_open_speed: Option<f64>,
    #[serde(rename = "EnergyShieldEnergyMax")]
    pub energy_shield_energy_max: Option<f64>,
    #[serde(rename = "EnergyShieldRechargeRate")]
    pub energy_shield_recharge_rate: Option<f64>,
    #[serde(rename = "EquipDelay")]
    pub equip_delay: Option<::serde_json::Value>,
    #[serde(rename = "FilthRate")]
    pub filth_rate: Option<i64>,
    #[serde(rename = "Flammability")]
    pub flammability: Option<::serde_json::Value>,
    #[serde(rename = "FoodPoisonChanceFixedHuman")]
    pub food_poison_chance_fixed_human: Option<f64>,
    #[serde(rename = "ImmunityGainSpeedFactor")]
    pub immunity_gain_speed_factor: Option<f64>,
    #[serde(rename = "Insulation_Cold")]
    pub insulation_cold: Option<::serde_json::Value>,
    #[serde(rename = "Insulation_Heat")]
    pub insulation_heat: Option<i64>,
    #[serde(rename = "JoyGainFactor")]
    pub joy_gain_factor: Option<::serde_json::Value>,
    #[serde(rename = "LeatherAmount")]
    pub leather_amount: Option<i64>,
    #[serde(rename = "MarketValue")]
    pub market_value: Option<::serde_json::Value>,
    #[serde(rename = "Mass")]
    pub mass: Option<::serde_json::Value>,
    #[serde(rename = "MaxHitPoints")]
    pub max_hit_points: Option<i64>,
    #[serde(rename = "MeatAmount")]
    pub meat_amount: Option<i64>,
    #[serde(rename = "MedicalPotency")]
    pub medical_potency: Option<f64>,
    #[serde(rename = "MedicalQualityMax")]
    pub medical_quality_max: Option<f64>,
    #[serde(rename = "MedicalTendQualityOffset")]
    pub medical_tend_quality_offset: Option<f64>,
    #[serde(rename = "MeditationFocusStrength")]
    pub meditation_focus_strength: Option<f64>,
    #[serde(rename = "MoveSpeed")]
    pub move_speed: Option<::serde_json::Value>,
    #[serde(rename = "Nutrition")]
    pub nutrition: Option<::serde_json::Value>,
    #[serde(rename = "PsychicSensitivity")]
    pub psychic_sensitivity: Option<f64>,
    #[serde(rename = "RangedWeapon_Cooldown")]
    pub ranged_weapon_cooldown: Option<::serde_json::Value>,
    #[serde(rename = "ResearchSpeedFactor")]
    pub research_speed_factor: Option<f64>,
    #[serde(rename = "RoyalFavorValue")]
    pub royal_favor_value: Option<::serde_json::Value>,
    #[serde(rename = "SellPriceFactor")]
    pub sell_price_factor: Option<f64>,
    #[serde(rename = "SharpDamageMultiplier")]
    pub sharp_damage_multiplier: Option<::serde_json::Value>,
    #[serde(rename = "ShootingAccuracyTurret")]
    pub shooting_accuracy_turret: Option<f64>,
    #[serde(rename = "SmokepopBeltRadius")]
    pub smokepop_belt_radius: Option<f64>,
    #[serde(rename = "StuffEffectMultiplierArmor")]
    pub stuff_effect_multiplier_armor: Option<f64>,
    #[serde(rename = "StuffEffectMultiplierInsulation_Cold")]
    pub stuff_effect_multiplier_insulation_cold: Option<f64>,
    #[serde(rename = "StuffEffectMultiplierInsulation_Heat")]
    pub stuff_effect_multiplier_insulation_heat: Option<::serde_json::Value>,
    #[serde(rename = "StuffPower_Armor_Blunt")]
    pub stuff_power_armor_blunt: Option<::serde_json::Value>,
    #[serde(rename = "StuffPower_Armor_Heat")]
    pub stuff_power_armor_heat: Option<f64>,
    #[serde(rename = "StuffPower_Armor_Sharp")]
    pub stuff_power_armor_sharp: Option<f64>,
    #[serde(rename = "StuffPower_Insulation_Cold")]
    pub stuff_power_insulation_cold: Option<i64>,
    #[serde(rename = "StuffPower_Insulation_Heat")]
    pub stuff_power_insulation_heat: Option<i64>,
    #[serde(rename = "StyleDominance")]
    pub style_dominance: Option<StyleDominance>,
    #[serde(rename = "SurgerySuccessChanceFactor")]
    pub surgery_success_chance_factor: Option<::serde_json::Value>,
    #[serde(rename = "ToxicSensitivity")]
    pub toxic_sensitivity: Option<::serde_json::Value>,
    #[serde(rename = "TrapMeleeDamage")]
    pub trap_melee_damage: Option<i64>,
    #[serde(rename = "TrapSpringChance")]
    pub trap_spring_chance: Option<f64>,
    #[serde(rename = "WorkTableEfficiencyFactor")]
    pub work_table_efficiency_factor: Option<f64>,
    #[serde(rename = "WorkTableWorkSpeedFactor")]
    pub work_table_work_speed_factor: Option<f64>,
    #[serde(rename = "WorkToBuild")]
    pub work_to_build: Option<i64>,
    #[serde(rename = "WorkToMake")]
    pub work_to_make: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StyleDominance {
    #[serde(rename = "$value")]
    pub value: i64,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StuffProps {
    #[serde(rename = "allowColorGenerators")]
    pub allow_color_generators: Option<bool>,
    pub appearance: Option<String>,
    #[serde(rename = "canSuggestUseDefaultStuff")]
    pub can_suggest_use_default_stuff: Option<bool>,
    #[serde(default)]
    pub categories: Vec<String>,
    pub color: Option<String>,
    pub commonality: Option<f64>,
    #[serde(rename = "constructEffect")]
    pub construct_effect: Option<String>,
    #[serde(rename = "soundImpactStuff")]
    pub sound_impact_stuff: Option<String>,
    #[serde(rename = "soundMeleeHitBlunt")]
    pub sound_melee_hit_blunt: Option<String>,
    #[serde(rename = "soundMeleeHitSharp")]
    pub sound_melee_hit_sharp: Option<String>,
    #[serde(rename = "statFactors")]
    pub stat_factors: Option<StatFactors3>,
    #[serde(rename = "statOffsets")]
    pub stat_offsets: Option<StatOffsets3>,
    #[serde(rename = "stuffAdjective")]
    pub stuff_adjective: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatFactors3 {
    #[serde(rename = "Beauty")]
    pub beauty: Option<::serde_json::Value>,
    #[serde(rename = "BedRestEffectiveness")]
    pub bed_rest_effectiveness: Option<f64>,
    #[serde(rename = "DoorOpenSpeed")]
    pub door_open_speed: Option<f64>,
    #[serde(rename = "Flammability")]
    pub flammability: Option<::serde_json::Value>,
    #[serde(rename = "MarketValue")]
    pub market_value: Option<f64>,
    #[serde(rename = "MaxHitPoints")]
    pub max_hit_points: Option<::serde_json::Value>,
    #[serde(rename = "MeleeWeapon_CooldownMultiplier")]
    pub melee_weapon_cooldown_multiplier: Option<f64>,
    #[serde(rename = "WorkToBuild")]
    pub work_to_build: Option<f64>,
    #[serde(rename = "WorkToMake")]
    pub work_to_make: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatOffsets3 {
    #[serde(rename = "Beauty")]
    pub beauty: Option<i64>,
    #[serde(rename = "WorkToBuild")]
    pub work_to_build: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tool3 {
    #[serde(rename = "alwaysTreatAsWeapon")]
    pub always_treat_as_weapon: Option<bool>,
    #[serde(rename = "armorPenetration")]
    pub armor_penetration: Option<f64>,
    pub capacities: Vec<String>,
    #[serde(rename = "chanceFactor")]
    pub chance_factor: Option<f64>,
    #[serde(rename = "cooldownTime")]
    pub cooldown_time: ::serde_json::Value,
    #[serde(rename = "ensureLinkedBodyPartsGroupAlwaysUsable")]
    pub ensure_linked_body_parts_group_always_usable: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "labelUsedInLogging")]
    pub label_used_in_logging: Option<bool>,
    #[serde(rename = "linkedBodyPartsGroup")]
    pub linked_body_parts_group: Option<String>,
    pub power: ::serde_json::Value,
    #[serde(rename = "soundMeleeHit")]
    pub sound_melee_hit: Option<String>,
    #[serde(rename = "soundMeleeMiss")]
    pub sound_melee_miss: Option<String>,
    #[serde(rename = "surpriseAttack")]
    pub surprise_attack: Option<SurpriseAttack>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SurpriseAttack {
    #[serde(rename = "extraMeleeDamages")]
    pub extra_melee_damages: Vec<ExtraMeleeDamage>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ExtraMeleeDamage {
    pub amount: i64,
    pub def: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct UiIconPathsStuff {
    pub appearance: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Verb2 {
    #[serde(rename = "ai_AvoidFriendlyFireRadius")]
    pub ai_avoid_friendly_fire_radius: Option<i64>,
    #[serde(rename = "ai_IsBuildingDestroyer")]
    pub ai_is_building_destroyer: Option<bool>,
    #[serde(rename = "burstShotCount")]
    pub burst_shot_count: Option<i64>,
    #[serde(rename = "colonyWideTaleDef")]
    pub colony_wide_tale_def: Option<String>,
    #[serde(rename = "consumeFuelPerShot")]
    pub consume_fuel_per_shot: Option<i64>,
    #[serde(rename = "defaultProjectile")]
    pub default_projectile: Option<String>,
    #[serde(rename = "forceNormalTimeSpeed")]
    pub force_normal_time_speed: Option<bool>,
    #[serde(rename = "forcedMissRadius")]
    pub forced_miss_radius: Option<::serde_json::Value>,
    #[serde(rename = "forcedMissRadiusClassicMortars")]
    pub forced_miss_radius_classic_mortars: Option<i64>,
    #[serde(rename = "hasStandardCommand")]
    pub has_standard_command: Option<bool>,
    #[serde(rename = "isMortar")]
    pub is_mortar: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "minRange")]
    pub min_range: Option<f64>,
    #[serde(rename = "muzzleFlashScale")]
    pub muzzle_flash_scale: Option<i64>,
    #[serde(rename = "noiseRadius")]
    pub noise_radius: Option<i64>,
    #[serde(rename = "nonInterruptingSelfCast")]
    pub non_interrupting_self_cast: Option<bool>,
    #[serde(rename = "onlyManualCast")]
    pub only_manual_cast: Option<bool>,
    pub range: Option<::serde_json::Value>,
    #[serde(rename = "rangedFireRulepack")]
    pub ranged_fire_rulepack: Option<String>,
    #[serde(rename = "requireLineOfSight")]
    pub require_line_of_sight: Option<bool>,
    #[serde(rename = "soundAiming")]
    pub sound_aiming: Option<String>,
    #[serde(rename = "soundCast")]
    pub sound_cast: Option<String>,
    #[serde(rename = "soundCastTail")]
    pub sound_cast_tail: Option<String>,
    #[serde(rename = "spawnDef")]
    pub spawn_def: Option<String>,
    #[serde(rename = "stopBurstWithoutLos")]
    pub stop_burst_without_los: Option<bool>,
    #[serde(rename = "targetParams")]
    pub target_params: Option<TargetParams2>,
    pub targetable: Option<bool>,
    #[serde(rename = "ticksBetweenBurstShots")]
    pub ticks_between_burst_shots: Option<i64>,
    #[serde(rename = "verbClass")]
    pub verb_class: String,
    pub violent: Option<bool>,
    #[serde(rename = "warmupTime")]
    pub warmup_time: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TargetParams2 {
    #[serde(rename = "canTargetBuildings")]
    pub can_target_buildings: Option<bool>,
    #[serde(rename = "canTargetLocations")]
    pub can_target_locations: Option<bool>,
    #[serde(rename = "neverTargetIncapacitated")]
    pub never_target_incapacitated: Option<bool>,
    #[serde(rename = "onlyTargetPsychicSensitive")]
    pub only_target_psychic_sensitive: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingSetMakerDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "debugParams")]
    pub debug_params: Option<DebugParams>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub root: Root3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DebugParams {
    #[serde(rename = "countRange")]
    pub count_range: Option<String>,
    #[serde(rename = "totalMarketValueRange")]
    pub total_market_value_range: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root3 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "fixedParams")]
    pub fixed_params: Option<FixedParams>,
    #[serde(default)]
    pub options: Vec<Option3>,
    #[serde(rename = "resolveInOrder")]
    pub resolve_in_order: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedParams {
    pub filter: Option<Filter5>,
    #[serde(rename = "qualityGenerator")]
    pub quality_generator: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter5 {
    #[serde(rename = "thingDefs")]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Option3 {
    pub chance: Option<::serde_json::Value>,
    #[serde(rename = "maxMarketValue")]
    pub max_market_value: Option<i64>,
    #[serde(rename = "minMarketValue")]
    pub min_market_value: Option<i64>,
    #[serde(rename = "minTotalMarketValue")]
    pub min_total_market_value: Option<i64>,
    #[serde(rename = "thingSetMaker")]
    pub thing_set_maker: ThingSetMaker,
    pub weight: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingSetMaker {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "fixedParams")]
    pub fixed_params: Option<FixedParams2>,
    #[serde(rename = "makingFaction")]
    pub making_faction: Option<MakingFaction>,
    #[serde(default)]
    pub options: Vec<Option4>,
    #[serde(rename = "requireNonNull")]
    pub require_non_null: Option<bool>,
    #[serde(rename = "thingSetMaker")]
    pub thing_set_maker: Option<ThingSetMaker5>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedParams2 {
    #[serde(rename = "countRange")]
    pub count_range: Option<String>,
    pub filter: Option<Filter6>,
    #[serde(rename = "minSingleItemMarketValuePct")]
    pub min_single_item_market_value_pct: Option<f64>,
    #[serde(rename = "qualityGenerator")]
    pub quality_generator: Option<String>,
    #[serde(rename = "totalMarketValueRange")]
    pub total_market_value_range: Option<String>,
    #[serde(rename = "totalNutritionRange")]
    pub total_nutrition_range: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter6 {
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(rename = "disallowCheaperThan")]
    pub disallow_cheaper_than: Option<i64>,
    #[serde(rename = "disallowInedibleByHuman")]
    pub disallow_inedible_by_human: Option<bool>,
    #[serde(rename = "disallowWithComp")]
    pub disallow_with_comp: Option<String>,
    #[serde(rename = "disallowWorsePreferability")]
    pub disallow_worse_preferability: Option<String>,
    #[serde(rename = "disallowedThingDefs")]
    #[serde(default)]
    pub disallowed_thing_defs: Vec<String>,
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<String>,
    #[serde(rename = "thingSetMakerTagsToAllow")]
    #[serde(default)]
    pub thing_set_maker_tags_to_allow: Vec<String>,
    #[serde(rename = "thingSetMakerTagsToDisallow")]
    #[serde(default)]
    pub thing_set_maker_tags_to_disallow: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MakingFaction {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Option4 {
    #[serde(rename = "thingSetMaker")]
    pub thing_set_maker: ThingSetMaker2,
    pub weight: ::serde_json::Value,
    #[serde(rename = "weightIfPlayerHasNoItem")]
    pub weight_if_player_has_no_item: Option<i64>,
    #[serde(rename = "weightIfPlayerHasNoItemItem")]
    pub weight_if_player_has_no_item_item: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingSetMaker2 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "fixedParams")]
    pub fixed_params: Option<FixedParams3>,
    #[serde(rename = "makingFaction")]
    pub making_faction: Option<MakingFaction2>,
    #[serde(rename = "makingFactionCategories")]
    #[serde(default)]
    pub making_faction_categories: Vec<String>,
    #[serde(rename = "marketValueFactor")]
    pub market_value_factor: Option<f64>,
    #[serde(rename = "minMaxTotalMarketValue")]
    pub min_max_total_market_value: Option<i64>,
    #[serde(rename = "requireNonNull")]
    pub require_non_null: Option<bool>,
    #[serde(rename = "thingSetMaker")]
    pub thing_set_maker: Option<ThingSetMaker3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedParams3 {
    #[serde(rename = "allowNonStackableDuplicates")]
    pub allow_non_stackable_duplicates: Option<bool>,
    pub filter: Filter7,
    #[serde(rename = "maxThingMarketValue")]
    pub max_thing_market_value: Option<i64>,
    #[serde(rename = "qualityGenerator")]
    pub quality_generator: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter7 {
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(rename = "disallowCheaperThan")]
    pub disallow_cheaper_than: Option<i64>,
    #[serde(rename = "disallowedThingDefs")]
    #[serde(default)]
    pub disallowed_thing_defs: Vec<String>,
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<String>,
    #[serde(rename = "thingSetMakerTagsToAllow")]
    #[serde(default)]
    pub thing_set_maker_tags_to_allow: Vec<String>,
    #[serde(rename = "thingSetMakerTagsToDisallow")]
    #[serde(default)]
    pub thing_set_maker_tags_to_disallow: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MakingFaction2 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingSetMaker3 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "fixedParams")]
    pub fixed_params: Option<FixedParams4>,
    #[serde(rename = "researchProject")]
    pub research_project: Option<String>,
    #[serde(rename = "thingSetMaker")]
    pub thing_set_maker: Option<ThingSetMaker4>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedParams4 {
    #[serde(rename = "allowNonStackableDuplicates")]
    pub allow_non_stackable_duplicates: Option<bool>,
    pub filter: Filter8,
    #[serde(rename = "qualityGenerator")]
    pub quality_generator: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter8 {
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<ThingDef2>,
    #[serde(rename = "thingSetMakerTagsToAllow")]
    #[serde(default)]
    pub thing_set_maker_tags_to_allow: Vec<String>,
    #[serde(rename = "thingSetMakerTagsToDisallow")]
    #[serde(default)]
    pub thing_set_maker_tags_to_disallow: Vec<String>,
    #[serde(rename = "tradeTagsToAllow")]
    #[serde(default)]
    pub trade_tags_to_allow: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingDef2 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingSetMaker4 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "fixedParams")]
    pub fixed_params: FixedParams5,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedParams5 {
    pub filter: Filter9,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter9 {
    #[serde(rename = "thingDefs")]
    pub thing_defs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThingSetMaker5 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "fixedParams")]
    pub fixed_params: FixedParams6,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixedParams6 {
    #[serde(rename = "countRange")]
    pub count_range: String,
    pub filter: Filter10,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filter10 {
    #[serde(rename = "thingSetMakerTagsToAllow")]
    pub thing_set_maker_tags_to_allow: Vec<String>,
    #[serde(rename = "thingSetMakerTagsToDisallow")]
    pub thing_set_maker_tags_to_disallow: Vec<String>,
    #[serde(rename = "tradeTagsToAllow")]
    pub trade_tags_to_allow: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThinkTreeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "thinkRoot")]
    pub think_root: ThinkRoot,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThinkRoot {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "subNodes")]
    pub sub_nodes: Vec<SubNode6>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode6 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    #[serde(rename = "dutyHook")]
    pub duty_hook: Option<String>,
    #[serde(rename = "inBedOnly")]
    pub in_bed_only: Option<bool>,
    #[serde(rename = "insertTag")]
    pub insert_tag: Option<String>,
    pub invert: Option<bool>,
    #[serde(rename = "leaveJoinableLordIfIssuesJob")]
    pub leave_joinable_lord_if_issues_job: Option<bool>,
    #[serde(rename = "mtbDays")]
    pub mtb_days: Option<i64>,
    #[serde(rename = "mtbHours")]
    pub mtb_hours: Option<i64>,
    #[serde(rename = "pawnKind")]
    pub pawn_kind: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "stateClass")]
    pub state_class: Option<String>,
    #[serde(default)]
    pub states: Vec<String>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode7>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
    #[serde(rename = "treeDef")]
    pub tree_def: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode7 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    #[serde(rename = "canBash")]
    pub can_bash: Option<bool>,
    #[serde(rename = "defaultLocomotion")]
    pub default_locomotion: Option<String>,
    #[serde(rename = "dutyHook")]
    pub duty_hook: Option<String>,
    pub emergency: Option<bool>,
    #[serde(rename = "forceCanDig")]
    pub force_can_dig: Option<bool>,
    #[serde(rename = "inBedOnly")]
    pub in_bed_only: Option<bool>,
    pub invert: Option<bool>,
    #[serde(rename = "leaveJoinableLordIfIssuesJob")]
    pub leave_joinable_lord_if_issues_job: Option<bool>,
    #[serde(rename = "locomotionUrgency")]
    pub locomotion_urgency: Option<String>,
    #[serde(rename = "maxDanger")]
    pub max_danger: Option<String>,
    #[serde(rename = "minCategory")]
    pub min_category: Option<String>,
    #[serde(rename = "minIntelligence")]
    pub min_intelligence: Option<String>,
    #[serde(rename = "minPriority")]
    pub min_priority: Option<f64>,
    #[serde(rename = "mtbDays")]
    pub mtb_days: Option<i64>,
    #[serde(rename = "mtbHours")]
    pub mtb_hours: Option<f64>,
    pub need: Option<String>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode8>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
    pub threshold: Option<f64>,
    #[serde(rename = "ticksBetweenWandersRange")]
    pub ticks_between_wanders_range: Option<String>,
    #[serde(rename = "treeDef")]
    pub tree_def: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode8 {
    #[serde(rename = "Class")]
    pub class: String,
    pub chance: Option<f64>,
    #[serde(rename = "defaultLocomotion")]
    pub default_locomotion: Option<String>,
    #[serde(rename = "expiryInterval")]
    pub expiry_interval: Option<i64>,
    #[serde(rename = "forceScanWholeMap")]
    pub force_scan_whole_map: Option<bool>,
    pub invert: Option<bool>,
    #[serde(rename = "jobMaxDuration")]
    pub job_max_duration: Option<i64>,
    #[serde(rename = "leaveJoinableLordIfIssuesJob")]
    pub leave_joinable_lord_if_issues_job: Option<bool>,
    #[serde(rename = "locomotionUrgency")]
    pub locomotion_urgency: Option<String>,
    #[serde(rename = "maxDanger")]
    pub max_danger: Option<String>,
    #[serde(rename = "maxLevelPercentage")]
    pub max_level_percentage: Option<f64>,
    pub min: Option<f64>,
    #[serde(rename = "minCategory")]
    pub min_category: Option<String>,
    pub need: Option<String>,
    #[serde(rename = "requiredCapacities")]
    #[serde(default)]
    pub required_capacities: Vec<String>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode9>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
    pub threshold: Option<f64>,
    #[serde(rename = "ticksBetweenWandersRange")]
    pub ticks_between_wanders_range: Option<String>,
    pub trainable: Option<String>,
    #[serde(rename = "treeDef")]
    pub tree_def: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode9 {
    #[serde(rename = "Class")]
    pub class: String,
    pub chance: Option<f64>,
    #[serde(rename = "defaultLocomotion")]
    pub default_locomotion: Option<String>,
    #[serde(rename = "expiryInterval")]
    pub expiry_interval: Option<i64>,
    #[serde(rename = "leaveJoinableLordIfIssuesJob")]
    pub leave_joinable_lord_if_issues_job: Option<bool>,
    #[serde(rename = "locomotionUrgency")]
    pub locomotion_urgency: Option<String>,
    #[serde(rename = "maxDanger")]
    pub max_danger: Option<String>,
    #[serde(rename = "minCategory")]
    pub min_category: Option<String>,
    pub radius: Option<i64>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode10>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
    #[serde(rename = "ticksBetweenWandersRange")]
    pub ticks_between_wanders_range: Option<String>,
    pub trainable: Option<String>,
    #[serde(rename = "treeDef")]
    pub tree_def: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode10 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "attackMeleeThreatEvenIfNotHostile")]
    pub attack_melee_threat_even_if_not_hostile: Option<bool>,
    #[serde(rename = "defaultLocomotion")]
    pub default_locomotion: Option<String>,
    #[serde(rename = "expiryInterval")]
    pub expiry_interval: Option<i64>,
    #[serde(rename = "failIfCantJoinOrCreateCaravan")]
    pub fail_if_cant_join_or_create_caravan: Option<bool>,
    pub invert: Option<bool>,
    #[serde(rename = "maxDanger")]
    pub max_danger: Option<String>,
    #[serde(rename = "subNodes")]
    #[serde(default)]
    pub sub_nodes: Vec<SubNode11>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
    #[serde(rename = "ticksBetweenWandersRange")]
    pub ticks_between_wanders_range: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SubNode11 {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "locomotionUrgency")]
    pub locomotion_urgency: Option<String>,
    #[serde(rename = "maxDanger")]
    pub max_danger: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ThoughtDef {
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<bool>,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "defName")]
    pub def_name: Option<String>,
    #[serde(rename = "doNotApplyToQuestLodgers")]
    pub do_not_apply_to_quest_lodgers: Option<bool>,
    #[serde(rename = "durationDays")]
    pub duration_days: Option<::serde_json::Value>,
    #[serde(rename = "effectMultiplyingStat")]
    pub effect_multiplying_stat: Option<String>,
    #[serde(rename = "gameCondition")]
    pub game_condition: Option<String>,
    pub hediff: Option<String>,
    pub icon: Option<String>,
    pub invert: Option<bool>,
    #[serde(rename = "maxCumulatedOpinionOffset")]
    pub max_cumulated_opinion_offset: Option<i64>,
    #[serde(rename = "neverNullifyIfAnyTrait")]
    #[serde(default)]
    pub never_nullify_if_any_trait: Vec<String>,
    #[serde(rename = "nextThought")]
    pub next_thought: Option<String>,
    #[serde(rename = "nullifiedIfNotColonist")]
    pub nullified_if_not_colonist: Option<bool>,
    #[serde(rename = "nullifyingOwnTales")]
    #[serde(default)]
    pub nullifying_own_tales: Vec<String>,
    #[serde(rename = "nullifyingPrecepts")]
    #[serde(default)]
    pub nullifying_precepts: Vec<NullifyingPrecept2>,
    #[serde(rename = "nullifyingTraits")]
    #[serde(default)]
    pub nullifying_traits: Vec<String>,
    #[serde(rename = "requiredTraits")]
    #[serde(default)]
    pub required_traits: Vec<String>,
    #[serde(rename = "requiredTraitsDegree")]
    pub required_traits_degree: Option<i64>,
    #[serde(rename = "showBubble")]
    pub show_bubble: Option<bool>,
    #[serde(rename = "stackLimit")]
    pub stack_limit: Option<i64>,
    #[serde(rename = "stackLimitForSameOtherPawn")]
    pub stack_limit_for_same_other_pawn: Option<i64>,
    #[serde(rename = "stackedEffectMultiplier")]
    pub stacked_effect_multiplier: Option<::serde_json::Value>,
    #[serde(default)]
    pub stages: Vec<Stage>,
    #[serde(rename = "taleDef")]
    pub tale_def: Option<String>,
    #[serde(rename = "thoughtClass")]
    pub thought_class: Option<String>,
    #[serde(rename = "thoughtToMake")]
    pub thought_to_make: Option<String>,
    #[serde(rename = "validWhileDespawned")]
    pub valid_while_despawned: Option<bool>,
    #[serde(rename = "workerClass")]
    pub worker_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NullifyingPrecept2 {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Stage {
    #[serde(rename = "IsNull")]
    pub is_null: Option<bool>,
    #[serde(rename = "baseMoodEffect")]
    pub base_mood_effect: Option<i64>,
    #[serde(rename = "baseOpinionOffset")]
    pub base_opinion_offset: Option<::serde_json::Value>,
    pub description: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "labelSocial")]
    pub label_social: Option<String>,
    pub visible: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TimeAssignmentDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowJoy")]
    pub allow_joy: Option<bool>,
    #[serde(rename = "allowRest")]
    pub allow_rest: Option<bool>,
    pub color: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TipSetDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub tips: Vec<Tip>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tip {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "TKey")]
    pub tkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ToolCapacityDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TraderKindDef {
    #[serde(rename = "Class")]
    pub class: String,
    pub category: Option<String>,
    #[serde(rename = "commonalityMultFromPopulationIntent")]
    pub commonality_mult_from_population_intent: Option<CommonalityMultFromPopulationIntent>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: Option<String>,
    pub orbital: Option<bool>,
    pub requestable: Option<bool>,
    #[serde(rename = "stockGenerators")]
    pub stock_generators: Vec<StockGenerator>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonalityMultFromPopulationIntent {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StockGenerator {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "MayRequire")]
    pub may_require: Option<String>,
    #[serde(rename = "categoryDef")]
    pub category_def: Option<String>,
    #[serde(rename = "checkTemperature")]
    pub check_temperature: Option<bool>,
    #[serde(rename = "countChances")]
    #[serde(default)]
    pub count_chances: Vec<CountChance>,
    #[serde(rename = "countRange")]
    pub count_range: Option<::serde_json::Value>,
    #[serde(rename = "createMatingPair")]
    #[serde(default)]
    pub create_mating_pair: Vec<String>,
    #[serde(rename = "customCountRanges")]
    pub custom_count_ranges: Option<CustomCountRanges>,
    #[serde(rename = "excludedCategories")]
    #[serde(default)]
    pub excluded_categories: Vec<String>,
    #[serde(rename = "excludedThingDefs")]
    #[serde(default)]
    pub excluded_thing_defs: Vec<String>,
    #[serde(rename = "kindCountRange")]
    pub kind_count_range: Option<::serde_json::Value>,
    #[serde(rename = "maxTechLevelBuy")]
    pub max_tech_level_buy: Option<String>,
    #[serde(rename = "maxTechLevelGenerate")]
    pub max_tech_level_generate: Option<String>,
    #[serde(rename = "maxWildness")]
    pub max_wildness: Option<f64>,
    #[serde(rename = "respectPopulationIntent")]
    pub respect_population_intent: Option<bool>,
    pub tag: Option<String>,
    #[serde(rename = "thingDef")]
    pub thing_def: Option<String>,
    #[serde(rename = "thingDefCountRange")]
    pub thing_def_count_range: Option<::serde_json::Value>,
    #[serde(rename = "thingDefs")]
    #[serde(default)]
    pub thing_defs: Vec<String>,
    #[serde(rename = "totalPriceRange")]
    pub total_price_range: Option<String>,
    #[serde(rename = "tradeTag")]
    pub trade_tag: Option<String>,
    #[serde(rename = "tradeTagsBuy")]
    #[serde(default)]
    pub trade_tags_buy: Vec<String>,
    #[serde(rename = "tradeTagsSell")]
    #[serde(default)]
    pub trade_tags_sell: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CountChance {
    pub chance: ::serde_json::Value,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CustomCountRanges {
    #[serde(rename = "Hyperweave")]
    pub hyperweave: String,
    #[serde(rename = "Luciferium")]
    pub luciferium: Option<String>,
    #[serde(rename = "MedicineUltratech")]
    pub medicine_ultratech: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TrainabilityDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "intelligenceOrder")]
    pub intelligence_order: i64,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TrainableDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "defaultTrainable")]
    pub default_trainable: bool,
    pub description: String,
    pub difficulty: i64,
    pub icon: String,
    pub label: String,
    #[serde(rename = "listPriority")]
    pub list_priority: i64,
    #[serde(rename = "minBodySize")]
    pub min_body_size: Option<f64>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(rename = "requiredTrainability")]
    pub required_trainability: String,
    pub steps: i64,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TraitDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "allowOnHostileSpawn")]
    pub allow_on_hostile_spawn: Option<bool>,
    pub commonality: Option<::serde_json::Value>,
    #[serde(rename = "commonalityFemale")]
    pub commonality_female: Option<f64>,
    #[serde(rename = "conflictingPassions")]
    #[serde(default)]
    pub conflicting_passions: Vec<String>,
    #[serde(rename = "conflictingTraits")]
    #[serde(default)]
    pub conflicting_traits: Vec<String>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "degreeDatas")]
    pub degree_datas: Vec<DegreeData>,
    #[serde(rename = "disabledWorkTags")]
    pub disabled_work_tags: Option<String>,
    #[serde(rename = "exclusionTags")]
    #[serde(default)]
    pub exclusion_tags: Vec<String>,
    #[serde(rename = "forcedPassions")]
    #[serde(default)]
    pub forced_passions: Vec<String>,
    #[serde(rename = "requiredWorkTags")]
    #[serde(default)]
    pub required_work_tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DegreeData {
    #[serde(rename = "allowedMeditationFocusTypes")]
    #[serde(default)]
    pub allowed_meditation_focus_types: Vec<String>,
    pub degree: Option<i64>,
    pub description: String,
    #[serde(rename = "disallowedInspirations")]
    #[serde(default)]
    pub disallowed_inspirations: Vec<String>,
    #[serde(rename = "disallowedMeditationFocusTypes")]
    #[serde(default)]
    pub disallowed_meditation_focus_types: Vec<String>,
    #[serde(rename = "disallowedMentalStates")]
    #[serde(default)]
    pub disallowed_mental_states: Vec<String>,
    #[serde(rename = "disallowedThoughtsFromIngestion")]
    #[serde(default)]
    pub disallowed_thoughts_from_ingestion: Vec<DisallowedThoughtsFromIngestion>,
    #[serde(rename = "extraThoughtsFromIngestion")]
    #[serde(default)]
    pub extra_thoughts_from_ingestion: Vec<ExtraThoughtsFromIngestion>,
    #[serde(rename = "hungerRateFactor")]
    pub hunger_rate_factor: Option<f64>,
    pub label: String,
    #[serde(rename = "marketValueFactorOffset")]
    pub market_value_factor_offset: Option<f64>,
    #[serde(rename = "mentalBreakInspirationGainChance")]
    pub mental_break_inspiration_gain_chance: Option<f64>,
    #[serde(rename = "mentalBreakInspirationGainReasonText")]
    pub mental_break_inspiration_gain_reason_text: Option<String>,
    #[serde(rename = "mentalBreakInspirationGainSet")]
    #[serde(default)]
    pub mental_break_inspiration_gain_set: Vec<String>,
    #[serde(rename = "randomDiseaseMtbDays")]
    pub random_disease_mtb_days: Option<i64>,
    #[serde(rename = "randomMentalState")]
    pub random_mental_state: Option<String>,
    #[serde(rename = "randomMentalStateMtbDaysMoodCurve")]
    pub random_mental_state_mtb_days_mood_curve: Option<RandomMentalStateMtbDaysMoodCurve>,
    #[serde(rename = "skillGains")]
    #[serde(default)]
    pub skill_gains: Vec<SkillGain>,
    #[serde(rename = "socialFightChanceFactor")]
    pub social_fight_chance_factor: Option<i64>,
    #[serde(rename = "statFactors")]
    pub stat_factors: Option<StatFactors4>,
    #[serde(rename = "statOffsets")]
    pub stat_offsets: Option<StatOffsets4>,
    #[serde(rename = "theOnlyAllowedMentalBreaks")]
    #[serde(default)]
    pub the_only_allowed_mental_breaks: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DisallowedThoughtsFromIngestion {
    #[serde(rename = "meatSource")]
    pub meat_source: String,
    pub thoughts: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ExtraThoughtsFromIngestion {
    #[serde(rename = "meatSource")]
    pub meat_source: String,
    #[serde(rename = "thoughtsAsIngredient")]
    pub thoughts_as_ingredient: Vec<String>,
    #[serde(rename = "thoughtsDirect")]
    pub thoughts_direct: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RandomMentalStateMtbDaysMoodCurve {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkillGain {
    pub key: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatFactors4 {
    #[serde(rename = "CertaintyLossFactor")]
    pub certainty_loss_factor: Option<CertaintyLossFactor>,
    #[serde(rename = "IncomingDamageFactor")]
    pub incoming_damage_factor: Option<f64>,
    #[serde(rename = "PawnTrapSpringChance")]
    pub pawn_trap_spring_chance: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CertaintyLossFactor {
    #[serde(rename = "$value")]
    pub value: ::serde_json::Value,
    #[serde(rename = "MayRequire")]
    pub may_require: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StatOffsets4 {
    #[serde(rename = "AimingDelayFactor")]
    pub aiming_delay_factor: Option<f64>,
    #[serde(rename = "GlobalLearningFactor")]
    pub global_learning_factor: Option<f64>,
    #[serde(rename = "ImmunityGainSpeed")]
    pub immunity_gain_speed: Option<f64>,
    #[serde(rename = "MeleeDodgeChance")]
    pub melee_dodge_chance: Option<i64>,
    #[serde(rename = "MeleeHitChance")]
    pub melee_hit_chance: Option<i64>,
    #[serde(rename = "MentalBreakThreshold")]
    pub mental_break_threshold: Option<f64>,
    #[serde(rename = "MoveSpeed")]
    pub move_speed: Option<f64>,
    #[serde(rename = "PainShockThreshold")]
    pub pain_shock_threshold: Option<f64>,
    #[serde(rename = "PawnBeauty")]
    pub pawn_beauty: Option<i64>,
    #[serde(rename = "PsychicSensitivity")]
    pub psychic_sensitivity: Option<::serde_json::Value>,
    #[serde(rename = "RestRateMultiplier")]
    pub rest_rate_multiplier: Option<f64>,
    #[serde(rename = "ShootingAccuracyPawn")]
    pub shooting_accuracy_pawn: Option<i64>,
    #[serde(rename = "WorkSpeedGlobal")]
    pub work_speed_global: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TransferableSorterDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "comparerClass")]
    pub comparer_class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WeaponClassDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WeatherDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "accuracyMultiplier")]
    pub accuracy_multiplier: Option<f64>,
    #[serde(rename = "ambientSounds")]
    pub ambient_sounds: Vec<String>,
    #[serde(rename = "commonalityRainfallFactor")]
    pub commonality_rainfall_factor: Option<CommonalityRainfallFactor>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "durationRange")]
    pub duration_range: Option<String>,
    #[serde(rename = "eventMakers")]
    #[serde(default)]
    pub event_makers: Vec<EventMaker>,
    #[serde(rename = "exposedThought")]
    pub exposed_thought: Option<String>,
    pub favorability: String,
    #[serde(rename = "isBad")]
    pub is_bad: Option<bool>,
    pub label: String,
    #[serde(rename = "moveSpeedMultiplier")]
    pub move_speed_multiplier: Option<f64>,
    #[serde(rename = "overlayClasses")]
    #[serde(default)]
    pub overlay_classes: Vec<String>,
    #[serde(rename = "perceivePriority")]
    pub perceive_priority: i64,
    #[serde(rename = "rainRate")]
    pub rain_rate: Option<i64>,
    pub repeatable: Option<bool>,
    #[serde(rename = "skyColorsDay")]
    pub sky_colors_day: SkyColorsDay,
    #[serde(rename = "skyColorsDusk")]
    pub sky_colors_dusk: SkyColorsDusk,
    #[serde(rename = "skyColorsNightEdge")]
    pub sky_colors_night_edge: SkyColorsNightEdge,
    #[serde(rename = "skyColorsNightMid")]
    pub sky_colors_night_mid: SkyColorsNightMid,
    #[serde(rename = "snowRate")]
    pub snow_rate: Option<f64>,
    #[serde(rename = "temperatureRange")]
    pub temperature_range: Option<String>,
    #[serde(rename = "windSpeedFactor")]
    pub wind_speed_factor: Option<f64>,
    #[serde(rename = "windSpeedOffset")]
    pub wind_speed_offset: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonalityRainfallFactor {
    pub points: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EventMaker {
    #[serde(rename = "averageInterval")]
    pub average_interval: i64,
    #[serde(rename = "eventClass")]
    pub event_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkyColorsDay {
    pub overlay: String,
    pub saturation: f64,
    pub shadow: String,
    pub sky: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkyColorsDusk {
    pub overlay: String,
    pub saturation: f64,
    pub shadow: String,
    pub sky: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkyColorsNightEdge {
    pub overlay: String,
    pub saturation: f64,
    pub shadow: String,
    pub sky: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SkyColorsNightMid {
    pub overlay: String,
    pub saturation: f64,
    pub shadow: String,
    pub sky: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WorkGiverDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "autoTakeablePriorityDrafted")]
    pub auto_takeable_priority_drafted: Option<i64>,
    #[serde(rename = "billGiversAllAnimals")]
    pub bill_givers_all_animals: Option<bool>,
    #[serde(rename = "billGiversAllAnimalsCorpses")]
    pub bill_givers_all_animals_corpses: Option<bool>,
    #[serde(rename = "billGiversAllHumanlikes")]
    pub bill_givers_all_humanlikes: Option<bool>,
    #[serde(rename = "billGiversAllHumanlikesCorpses")]
    pub bill_givers_all_humanlikes_corpses: Option<bool>,
    #[serde(rename = "canBeDoneWhileDrafted")]
    pub can_be_done_while_drafted: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    #[serde(rename = "directOrderable")]
    pub direct_orderable: Option<bool>,
    #[serde(rename = "doesSmoothing")]
    pub does_smoothing: Option<bool>,
    pub emergency: Option<bool>,
    #[serde(rename = "equivalenceGroup")]
    pub equivalence_group: Option<String>,
    #[serde(rename = "feedAnimalsOnly")]
    pub feed_animals_only: Option<bool>,
    #[serde(rename = "feedHumanlikesOnly")]
    pub feed_humanlikes_only: Option<bool>,
    #[serde(rename = "fixedBillGiverDefs")]
    #[serde(default)]
    pub fixed_bill_giver_defs: Vec<String>,
    #[serde(rename = "forceFleck")]
    pub force_fleck: Option<String>,
    pub gerund: Option<String>,
    #[serde(rename = "giverClass")]
    pub giver_class: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "nonColonistsCanDo")]
    pub non_colonists_can_do: Option<bool>,
    #[serde(rename = "prioritizeSustains")]
    pub prioritize_sustains: Option<bool>,
    #[serde(rename = "priorityInType")]
    pub priority_in_type: Option<i64>,
    #[serde(rename = "requiredCapacities")]
    #[serde(default)]
    pub required_capacities: Vec<String>,
    #[serde(rename = "scanCells")]
    pub scan_cells: Option<bool>,
    #[serde(rename = "scanThings")]
    pub scan_things: Option<bool>,
    #[serde(rename = "scannerDef")]
    pub scanner_def: Option<String>,
    #[serde(rename = "tagToGive")]
    pub tag_to_give: Option<String>,
    #[serde(rename = "tendToAnimalsOnly")]
    pub tend_to_animals_only: Option<bool>,
    #[serde(rename = "tendToHumanlikesOnly")]
    pub tend_to_humanlikes_only: Option<bool>,
    pub verb: Option<String>,
    #[serde(rename = "workTags")]
    #[serde(default)]
    pub work_tags: Vec<String>,
    #[serde(rename = "workType")]
    pub work_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WorkGiverEquivalenceGroupDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WorkTypeDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "alwaysStartActive")]
    pub always_start_active: Option<bool>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: String,
    #[serde(rename = "disabledForSlaves")]
    pub disabled_for_slaves: Option<bool>,
    #[serde(rename = "gerundLabel")]
    pub gerund_label: String,
    #[serde(rename = "labelShort")]
    pub label_short: String,
    #[serde(rename = "naturalPriority")]
    pub natural_priority: i64,
    #[serde(rename = "pawnLabel")]
    pub pawn_label: String,
    #[serde(rename = "relevantSkills")]
    #[serde(default)]
    pub relevant_skills: Vec<String>,
    #[serde(rename = "requireCapableColonist")]
    pub require_capable_colonist: Option<bool>,
    pub verb: String,
    #[serde(rename = "workTags")]
    #[serde(default)]
    pub work_tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WorldGenStepDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub order: i64,
    #[serde(rename = "worldGenStep")]
    pub world_gen_step: WorldGenStep,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WorldGenStep {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "ancientSitesPer100kTiles")]
    pub ancient_sites_per100_k_tiles: Option<String>,
    #[serde(rename = "maximumSegmentCurviness")]
    pub maximum_segment_curviness: Option<f64>,
    #[serde(rename = "maximumSiteCurve")]
    pub maximum_site_curve: Option<i64>,
    #[serde(rename = "minimumChain")]
    pub minimum_chain: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WorldObjectDef {
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "IncidentTargetTags")]
    #[serde(default)]
    pub incident_target_tags: Vec<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "allowCaravanIncidentsWhichGenerateMap")]
    pub allow_caravan_incidents_which_generate_map: Option<bool>,
    #[serde(rename = "blockExitGridUntilBattleIsWon")]
    pub block_exit_grid_until_battle_is_won: Option<bool>,
    #[serde(rename = "canBePlayerHome")]
    pub can_be_player_home: Option<bool>,
    #[serde(default)]
    pub comps: Vec<Comp8>,
    #[serde(rename = "defName")]
    pub def_name: String,
    pub description: Option<String>,
    #[serde(rename = "expandMore")]
    pub expand_more: Option<bool>,
    #[serde(rename = "expandingIcon")]
    pub expanding_icon: Option<bool>,
    #[serde(rename = "expandingIconPriority")]
    pub expanding_icon_priority: Option<i64>,
    #[serde(rename = "expandingIconTexture")]
    pub expanding_icon_texture: Option<String>,
    #[serde(rename = "inspectorTabs")]
    #[serde(default)]
    pub inspector_tabs: Vec<String>,
    #[serde(rename = "isTempIncidentMapOwner")]
    pub is_temp_incident_map_owner: Option<bool>,
    pub label: Option<String>,
    #[serde(rename = "mapGenerator")]
    pub map_generator: Option<String>,
    pub saved: Option<bool>,
    pub texture: Option<String>,
    #[serde(rename = "useDynamicDrawer")]
    pub use_dynamic_drawer: Option<bool>,
    #[serde(rename = "worldObjectClass")]
    pub world_object_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comp8 {
    #[serde(rename = "Class")]
    pub class: String,
}