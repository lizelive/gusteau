use AnyDef::*;

use crate::magic::Def;

impl AnyDef {
    pub fn is_known(&self) -> bool {
        true
    }
    pub fn class(&self) -> &'static str {
        match self {
            PawnKind(_) => "PawnKindDef",
            Damage(_) => "DamageDef",
            KeyBindingCategory(_) => "KeyBindingCategoryDef",
            Expansion(_) => "ExpansionDef",
            PrisonerInteractionMode(_) => "PrisonerInteractionModeDef",
            HistoryAutoRecorder(_) => "HistoryAutoRecorderDef",
            PawnsArrivalMode(_) => "PawnsArrivalModeDef",
            Faction(_) => "FactionDef",
            BillStoreMode(_) => "BillStoreModeDef",
            Designation(_) => "DesignationDef",
            Clamor(_) => "ClamorDef",
            MessageType(_) => "MessageTypeDef",
            Terrain(_) => "TerrainDef",
            WorkGiver(_) => "WorkGiverDef",
            Interaction(_) => "InteractionDef",
            Record(_) => "RecordDef",
            Chemical(_) => "ChemicalDef",
            Culture(_) => "CultureDef",
            BodyPartGroup(_) => "BodyPartGroupDef",
            JoyKind(_) => "JoyKindDef",
            HibernatableState(_) => "HibernatableStateDef",
            ShaderType(_) => "ShaderTypeDef",
            RaidStrategy(_) => "RaidStrategyDef",
            Thought(_) => "ThoughtDef",
            MapGenerator(_) => "MapGeneratorDef",
            IncidentTargetTag(_) => "IncidentTargetTagDef",
            PawnRelation(_) => "PawnRelationDef",
            Song(_) => "SongDef",
            ScenPart(_) => "ScenPartDef",
            BodyPart(_) => "BodyPartDef",
            SpecialThingFilter(_) => "SpecialThingFilterDef",
            MentalState(_) => "MentalStateDef",
            LogEntry(_) => "LogEntryDef",
            PawnColumn(_) => "PawnColumnDef",
            FleshType(_) => "FleshTypeDef",
            Subcamera(_) => "SubcameraDef",
            Skill(_) => "SkillDef",
            TraderKind(_) => "TraderKindDef",
            TimeAssignment(_) => "TimeAssignmentDef",
            GameCondition(_) => "GameConditionDef",
            BillRepeatMode(_) => "BillRepeatModeDef",
            Job(_) => "JobDef",
            Hair(_) => "HairDef",
            PawnCapacity(_) => "PawnCapacityDef",
            SiteCore(_) => "SiteCoreDef",
            RitualOutcomeEffect(_) => "RitualOutcomeEffectDef",
            Feature(_) => "FeatureDef",
            Tattoo(_) => "TattooDef",
            TerrainAffordance(_) => "TerrainAffordanceDef",
            Thing(_) => "ThingDef",
            GoodwillSituation(_) => "GoodwillSituationDef",
            Beard(_) => "BeardDef",
            ApparelLayer(_) => "ApparelLayerDef",
            ImpactSoundType(_) => "ImpactSoundTypeDef",
            TransferableSorter(_) => "TransferableSorterDef",
            RulePack(_) => "RulePackDef",
            Hediff(_) => "HediffDef",
            ReservationLayer(_) => "ReservationLayerDef",
            WorldGenStep(_) => "WorldGenStepDef",
            StuffAppearance(_) => "StuffAppearanceDef",
            JoyGiver(_) => "JoyGiverDef",
            IncidentCategory(_) => "IncidentCategoryDef",
            MeditationFocus(_) => "MeditationFocusDef",
            StuffCategory(_) => "StuffCategoryDef",
            RitualPattern(_) => "RitualPatternDef",
            Tale(_) => "TaleDef",
            KeyBinding(_) => "KeyBindingDef",
            Stat(_) => "StatDef",
            MainButton(_) => "MainButtonDef",
            RoomStat(_) => "RoomStatDef",
            WorldObject(_) => "WorldObjectDef",
            Trait(_) => "TraitDef",
            Roof(_) => "RoofDef",
            Trainability(_) => "TrainabilityDef",
            MentalBreak(_) => "MentalBreakDef",
            DesignatorDropdownGroup(_) => "DesignatorDropdownGroupDef",
            Expectation(_) => "ExpectationDef",
            Maneuver(_) => "ManeuverDef",
            Rule(_) => "RuleDef",
            WorkGiverEquivalenceGroup(_) => "WorkGiverEquivalenceGroupDef",
            Difficulty(_) => "DifficultyDef",
            SitePart(_) => "SitePartDef",
            ToolCapacity(_) => "ToolCapacityDef",
            Weather(_) => "WeatherDef",
            Duty(_) => "DutyDef",
            Inspiration(_) => "InspirationDef",
            River(_) => "RiverDef",
            StatCategory(_) => "StatCategoryDef",
            BodyPartTag(_) => "BodyPartTagDef",
            Precept(_) => "PreceptDef",
            Recipe(_) => "RecipeDef",
            SketchResolver(_) => "SketchResolverDef",
            ResearchTab(_) => "ResearchTabDef",
            RitualVisualEffect(_) => "RitualVisualEffectDef",
            HistoryAutoRecorderGroup(_) => "HistoryAutoRecorderGroupDef",
            RoomRole(_) => "RoomRoleDef",
            GenStep(_) => "GenStepDef",
            Ability(_) => "AbilityDef",
            RoadPathing(_) => "RoadPathingDef",
            Fleck(_) => "FleckDef",
            Road(_) => "RoadDef",
            Scatterable(_) => "ScatterableDef",
            Biome(_) => "BiomeDef",
            Body(_) => "BodyDef",
            HistoryEvent(_) => "HistoryEventDef",
            LifeStage(_) => "LifeStageDef",
            HediffGiverSet(_) => "HediffGiverSetDef",
            ImplementOwnerType(_) => "ImplementOwnerTypeDef",
            BodyType(_) => "BodyTypeDef",
            Sound(_) => "SoundDef",
            TipSet(_) => "TipSetDef",
            Letter(_) => "LetterDef",
            DrugPolicy(_) => "DrugPolicyDef",
            WeaponClass(_) => "WeaponClassDef",
            QuestScript(_) => "QuestScriptDef",
            StyleItemCategory(_) => "StyleItemCategoryDef",
            ThinkTree(_) => "ThinkTreeDef",
            ThingSetMaker(_) => "ThingSetMakerDef",
            ResearchProject(_) => "ResearchProjectDef",
            Effecter(_) => "EffecterDef",
            OrderedTakeGroup(_) => "OrderedTakeGroupDef",
            Storyteller(_) => "StorytellerDef",
            Instruction(_) => "InstructionDef",
            PawnGroupKind(_) => "PawnGroupKindDef",
            DamageArmorCategory(_) => "DamageArmorCategoryDef",
            RoadWorldLayer(_) => "RoadWorldLayerDef",
            Incident(_) => "IncidentDef",
            ResearchProjectTag(_) => "ResearchProjectTagDef",
            ThingCategory(_) => "ThingCategoryDef",
            InventoryStockGroup(_) => "InventoryStockGroupDef",
            DesignationCategory(_) => "DesignationCategoryDef",
            WorkType(_) => "WorkTypeDef",
            Need(_) => "NeedDef",
            Trainable(_) => "TrainableDef",
            Concept(_) => "ConceptDef",
            PawnTable(_) => "PawnTableDef",
            Issue(_) => "IssueDef",
            Scenario(_) => "ScenarioDef",
            Gathering(_) => "GatheringDef",
        }
    }
    pub fn def_mut(&mut self) -> &mut Def {
        match self {
            PawnKind(def) => def,
            Damage(def) => def,
            KeyBindingCategory(def) => def,
            Expansion(def) => def,
            PrisonerInteractionMode(def) => def,
            HistoryAutoRecorder(def) => def,
            PawnsArrivalMode(def) => def,
            Faction(def) => def,
            BillStoreMode(def) => def,
            Designation(def) => def,
            Clamor(def) => def,
            MessageType(def) => def,
            Terrain(def) => def,
            WorkGiver(def) => def,
            Interaction(def) => def,
            Record(def) => def,
            Chemical(def) => def,
            Culture(def) => def,
            BodyPartGroup(def) => def,
            JoyKind(def) => def,
            HibernatableState(def) => def,
            ShaderType(def) => def,
            RaidStrategy(def) => def,
            Thought(def) => def,
            MapGenerator(def) => def,
            IncidentTargetTag(def) => def,
            PawnRelation(def) => def,
            Song(def) => def,
            ScenPart(def) => def,
            BodyPart(def) => def,
            SpecialThingFilter(def) => def,
            MentalState(def) => def,
            LogEntry(def) => def,
            PawnColumn(def) => def,
            FleshType(def) => def,
            Subcamera(def) => def,
            Skill(def) => def,
            TraderKind(def) => def,
            TimeAssignment(def) => def,
            GameCondition(def) => def,
            BillRepeatMode(def) => def,
            Job(def) => def,
            Hair(def) => def,
            PawnCapacity(def) => def,
            SiteCore(def) => def,
            RitualOutcomeEffect(def) => def,
            Feature(def) => def,
            Tattoo(def) => def,
            TerrainAffordance(def) => def,
            Thing(def) => def,
            GoodwillSituation(def) => def,
            Beard(def) => def,
            ApparelLayer(def) => def,
            ImpactSoundType(def) => def,
            TransferableSorter(def) => def,
            RulePack(def) => def,
            Hediff(def) => def,
            ReservationLayer(def) => def,
            WorldGenStep(def) => def,
            StuffAppearance(def) => def,
            JoyGiver(def) => def,
            IncidentCategory(def) => def,
            MeditationFocus(def) => def,
            StuffCategory(def) => def,
            RitualPattern(def) => def,
            Tale(def) => def,
            KeyBinding(def) => def,
            Stat(def) => def,
            MainButton(def) => def,
            RoomStat(def) => def,
            WorldObject(def) => def,
            Trait(def) => def,
            Roof(def) => def,
            Trainability(def) => def,
            MentalBreak(def) => def,
            DesignatorDropdownGroup(def) => def,
            Expectation(def) => def,
            Maneuver(def) => def,
            Rule(def) => def,
            WorkGiverEquivalenceGroup(def) => def,
            Difficulty(def) => def,
            SitePart(def) => def,
            ToolCapacity(def) => def,
            Weather(def) => def,
            Duty(def) => def,
            Inspiration(def) => def,
            River(def) => def,
            StatCategory(def) => def,
            BodyPartTag(def) => def,
            Precept(def) => def,
            Recipe(def) => def,
            SketchResolver(def) => def,
            ResearchTab(def) => def,
            RitualVisualEffect(def) => def,
            HistoryAutoRecorderGroup(def) => def,
            RoomRole(def) => def,
            GenStep(def) => def,
            Ability(def) => def,
            RoadPathing(def) => def,
            Fleck(def) => def,
            Road(def) => def,
            Scatterable(def) => def,
            Biome(def) => def,
            Body(def) => def,
            HistoryEvent(def) => def,
            LifeStage(def) => def,
            HediffGiverSet(def) => def,
            ImplementOwnerType(def) => def,
            BodyType(def) => def,
            Sound(def) => def,
            TipSet(def) => def,
            Letter(def) => def,
            DrugPolicy(def) => def,
            WeaponClass(def) => def,
            QuestScript(def) => def,
            StyleItemCategory(def) => def,
            ThinkTree(def) => def,
            ThingSetMaker(def) => def,
            ResearchProject(def) => def,
            Effecter(def) => def,
            OrderedTakeGroup(def) => def,
            Storyteller(def) => def,
            Instruction(def) => def,
            PawnGroupKind(def) => def,
            DamageArmorCategory(def) => def,
            RoadWorldLayer(def) => def,
            Incident(def) => def,
            ResearchProjectTag(def) => def,
            ThingCategory(def) => def,
            InventoryStockGroup(def) => def,
            DesignationCategory(def) => def,
            WorkType(def) => def,
            Need(def) => def,
            Trainable(def) => def,
            Concept(def) => def,
            PawnTable(def) => def,
            Issue(def) => def,
            Scenario(def) => def,
            Gathering(def) => def,
        }
    }

}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AnyDef {
    #[serde(rename = "PawnKindDef")]
    PawnKind(Def),
    #[serde(rename = "DamageDef")]
    Damage(Def),
    #[serde(rename = "KeyBindingCategoryDef")]
    KeyBindingCategory(Def),
    #[serde(rename = "ExpansionDef")]
    Expansion(Def),
    #[serde(rename = "PrisonerInteractionModeDef")]
    PrisonerInteractionMode(Def),
    #[serde(rename = "HistoryAutoRecorderDef")]
    HistoryAutoRecorder(Def),
    #[serde(rename = "PawnsArrivalModeDef")]
    PawnsArrivalMode(Def),
    #[serde(rename = "FactionDef")]
    Faction(Def),
    #[serde(rename = "BillStoreModeDef")]
    BillStoreMode(Def),
    #[serde(rename = "DesignationDef")]
    Designation(Def),
    #[serde(rename = "ClamorDef")]
    Clamor(Def),
    #[serde(rename = "MessageTypeDef")]
    MessageType(Def),
    #[serde(rename = "TerrainDef")]
    Terrain(Def),
    #[serde(rename = "WorkGiverDef")]
    WorkGiver(Def),
    #[serde(rename = "InteractionDef")]
    Interaction(Def),
    #[serde(rename = "RecordDef")]
    Record(Def),
    #[serde(rename = "ChemicalDef")]
    Chemical(Def),
    #[serde(rename = "CultureDef")]
    Culture(Def),
    #[serde(rename = "BodyPartGroupDef")]
    BodyPartGroup(Def),
    #[serde(rename = "JoyKindDef")]
    JoyKind(Def),
    #[serde(rename = "HibernatableStateDef")]
    HibernatableState(Def),
    #[serde(rename = "ShaderTypeDef")]
    ShaderType(Def),
    #[serde(rename = "RaidStrategyDef")]
    RaidStrategy(Def),
    #[serde(rename = "ThoughtDef")]
    Thought(Def),
    #[serde(rename = "MapGeneratorDef")]
    MapGenerator(Def),
    #[serde(rename = "IncidentTargetTagDef")]
    IncidentTargetTag(Def),
    #[serde(rename = "PawnRelationDef")]
    PawnRelation(Def),
    #[serde(rename = "SongDef")]
    Song(Def),
    #[serde(rename = "ScenPartDef")]
    ScenPart(Def),
    #[serde(rename = "BodyPartDef")]
    BodyPart(Def),
    #[serde(rename = "SpecialThingFilterDef")]
    SpecialThingFilter(Def),
    #[serde(rename = "MentalStateDef")]
    MentalState(Def),
    #[serde(rename = "LogEntryDef")]
    LogEntry(Def),
    #[serde(rename = "PawnColumnDef")]
    PawnColumn(Def),
    #[serde(rename = "FleshTypeDef")]
    FleshType(Def),
    #[serde(rename = "SubcameraDef")]
    Subcamera(Def),
    #[serde(rename = "SkillDef")]
    Skill(Def),
    #[serde(rename = "TraderKindDef")]
    TraderKind(Def),
    #[serde(rename = "TimeAssignmentDef")]
    TimeAssignment(Def),
    #[serde(rename = "GameConditionDef")]
    GameCondition(Def),
    #[serde(rename = "BillRepeatModeDef")]
    BillRepeatMode(Def),
    #[serde(rename = "JobDef")]
    Job(Def),
    #[serde(rename = "HairDef")]
    Hair(Def),
    #[serde(rename = "PawnCapacityDef")]
    PawnCapacity(Def),
    #[serde(rename = "SiteCoreDef")]
    SiteCore(Def),
    #[serde(rename = "RitualOutcomeEffectDef")]
    RitualOutcomeEffect(Def),
    #[serde(rename = "FeatureDef")]
    Feature(Def),
    #[serde(rename = "TattooDef")]
    Tattoo(Def),
    #[serde(rename = "TerrainAffordanceDef")]
    TerrainAffordance(Def),
    #[serde(rename = "ThingDef")]
    Thing(Def),
    #[serde(rename = "GoodwillSituationDef")]
    GoodwillSituation(Def),
    #[serde(rename = "BeardDef")]
    Beard(Def),
    #[serde(rename = "ApparelLayerDef")]
    ApparelLayer(Def),
    #[serde(rename = "ImpactSoundTypeDef")]
    ImpactSoundType(Def),
    #[serde(rename = "TransferableSorterDef")]
    TransferableSorter(Def),
    #[serde(rename = "RulePackDef")]
    RulePack(Def),
    #[serde(rename = "HediffDef")]
    Hediff(Def),
    #[serde(rename = "ReservationLayerDef")]
    ReservationLayer(Def),
    #[serde(rename = "WorldGenStepDef")]
    WorldGenStep(Def),
    #[serde(rename = "StuffAppearanceDef")]
    StuffAppearance(Def),
    #[serde(rename = "JoyGiverDef")]
    JoyGiver(Def),
    #[serde(rename = "IncidentCategoryDef")]
    IncidentCategory(Def),
    #[serde(rename = "MeditationFocusDef")]
    MeditationFocus(Def),
    #[serde(rename = "StuffCategoryDef")]
    StuffCategory(Def),
    #[serde(rename = "RitualPatternDef")]
    RitualPattern(Def),
    #[serde(rename = "TaleDef")]
    Tale(Def),
    #[serde(rename = "KeyBindingDef")]
    KeyBinding(Def),
    #[serde(rename = "StatDef")]
    Stat(Def),
    #[serde(rename = "MainButtonDef")]
    MainButton(Def),
    #[serde(rename = "RoomStatDef")]
    RoomStat(Def),
    #[serde(rename = "WorldObjectDef")]
    WorldObject(Def),
    #[serde(rename = "TraitDef")]
    Trait(Def),
    #[serde(rename = "RoofDef")]
    Roof(Def),
    #[serde(rename = "TrainabilityDef")]
    Trainability(Def),
    #[serde(rename = "MentalBreakDef")]
    MentalBreak(Def),
    #[serde(rename = "DesignatorDropdownGroupDef")]
    DesignatorDropdownGroup(Def),
    #[serde(rename = "ExpectationDef")]
    Expectation(Def),
    #[serde(rename = "ManeuverDef")]
    Maneuver(Def),
    #[serde(rename = "RuleDef")]
    Rule(Def),
    #[serde(rename = "WorkGiverEquivalenceGroupDef")]
    WorkGiverEquivalenceGroup(Def),
    #[serde(rename = "DifficultyDef")]
    Difficulty(Def),
    #[serde(rename = "SitePartDef")]
    SitePart(Def),
    #[serde(rename = "ToolCapacityDef")]
    ToolCapacity(Def),
    #[serde(rename = "WeatherDef")]
    Weather(Def),
    #[serde(rename = "DutyDef")]
    Duty(Def),
    #[serde(rename = "InspirationDef")]
    Inspiration(Def),
    #[serde(rename = "RiverDef")]
    River(Def),
    #[serde(rename = "StatCategoryDef")]
    StatCategory(Def),
    #[serde(rename = "BodyPartTagDef")]
    BodyPartTag(Def),
    #[serde(rename = "PreceptDef")]
    Precept(Def),
    #[serde(rename = "RecipeDef")]
    Recipe(Def),
    #[serde(rename = "SketchResolverDef")]
    SketchResolver(Def),
    #[serde(rename = "ResearchTabDef")]
    ResearchTab(Def),
    #[serde(rename = "RitualVisualEffectDef")]
    RitualVisualEffect(Def),
    #[serde(rename = "HistoryAutoRecorderGroupDef")]
    HistoryAutoRecorderGroup(Def),
    #[serde(rename = "RoomRoleDef")]
    RoomRole(Def),
    #[serde(rename = "GenStepDef")]
    GenStep(Def),
    #[serde(rename = "AbilityDef")]
    Ability(Def),
    #[serde(rename = "RoadPathingDef")]
    RoadPathing(Def),
    #[serde(rename = "FleckDef")]
    Fleck(Def),
    #[serde(rename = "RoadDef")]
    Road(Def),
    #[serde(rename = "ScatterableDef")]
    Scatterable(Def),
    #[serde(rename = "BiomeDef")]
    Biome(Def),
    #[serde(rename = "BodyDef")]
    Body(Def),
    #[serde(rename = "HistoryEventDef")]
    HistoryEvent(Def),
    #[serde(rename = "LifeStageDef")]
    LifeStage(Def),
    #[serde(rename = "HediffGiverSetDef")]
    HediffGiverSet(Def),
    #[serde(rename = "ImplementOwnerTypeDef")]
    ImplementOwnerType(Def),
    #[serde(rename = "BodyTypeDef")]
    BodyType(Def),
    #[serde(rename = "SoundDef")]
    Sound(Def),
    #[serde(rename = "TipSetDef")]
    TipSet(Def),
    #[serde(rename = "LetterDef")]
    Letter(Def),
    #[serde(rename = "DrugPolicyDef")]
    DrugPolicy(Def),
    #[serde(rename = "WeaponClassDef")]
    WeaponClass(Def),
    #[serde(rename = "QuestScriptDef")]
    QuestScript(Def),
    #[serde(rename = "StyleItemCategoryDef")]
    StyleItemCategory(Def),
    #[serde(rename = "ThinkTreeDef")]
    ThinkTree(Def),
    #[serde(rename = "ThingSetMakerDef")]
    ThingSetMaker(Def),
    #[serde(rename = "ResearchProjectDef")]
    ResearchProject(Def),
    #[serde(rename = "EffecterDef")]
    Effecter(Def),
    #[serde(rename = "OrderedTakeGroupDef")]
    OrderedTakeGroup(Def),
    #[serde(rename = "StorytellerDef")]
    Storyteller(Def),
    #[serde(rename = "InstructionDef")]
    Instruction(Def),
    #[serde(rename = "PawnGroupKindDef")]
    PawnGroupKind(Def),
    #[serde(rename = "DamageArmorCategoryDef")]
    DamageArmorCategory(Def),
    #[serde(rename = "RoadWorldLayerDef")]
    RoadWorldLayer(Def),
    #[serde(rename = "IncidentDef")]
    Incident(Def),
    #[serde(rename = "ResearchProjectTagDef")]
    ResearchProjectTag(Def),
    #[serde(rename = "ThingCategoryDef")]
    ThingCategory(Def),
    #[serde(rename = "InventoryStockGroupDef")]
    InventoryStockGroup(Def),
    #[serde(rename = "DesignationCategoryDef")]
    DesignationCategory(Def),
    #[serde(rename = "WorkTypeDef")]
    WorkType(Def),
    #[serde(rename = "NeedDef")]
    Need(Def),
    #[serde(rename = "TrainableDef")]
    Trainable(Def),
    #[serde(rename = "ConceptDef")]
    Concept(Def),
    #[serde(rename = "PawnTableDef")]
    PawnTable(Def),
    #[serde(rename = "IssueDef")]
    Issue(Def),
    #[serde(rename = "ScenarioDef")]
    Scenario(Def),
    #[serde(rename = "GatheringDef")]
    Gathering(Def),
}
