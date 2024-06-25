use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AbilityDetail {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Ability {
    pub ability: AbilityDetail,
    pub is_hidden: bool,
    pub slot: u32,
}

#[derive(Deserialize, Debug)]
pub struct Cry {
    pub latest: String,
    pub legacy: String,
}

#[derive(Deserialize, Debug)]
pub struct Form {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct GameIndex {
    pub game_index: u32,
    pub version: Version,
}

#[derive(Deserialize, Debug)]
pub struct ItemDetail {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionDetail {
    pub rarity: u32,
    pub version: Version,
}

#[derive(Deserialize, Debug)]
pub struct HeldItem {
    pub item: ItemDetail,
    pub version_details: Vec<VersionDetail>,
}

#[derive(Deserialize, Debug)]
pub struct MoveDetail {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct MoveLearnMethod {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionGroup {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionGroupDetail {
    pub level_learned_at: u32,
    pub move_learn_method: MoveLearnMethod,
    pub version_group: VersionGroup,
}

#[derive(Deserialize, Debug)]
pub struct Move {
    pub r#move: MoveDetail,
    pub version_group_details: Vec<VersionGroupDetail>,
}

#[derive(Deserialize, Debug)]
pub struct Species {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Sprites {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
    pub other: Option<OtherSprites>,
    pub versions: Option<Versions>,
}

#[derive(Deserialize, Debug)]
pub struct OtherSprites {
    pub dream_world: Option<DreamWorld>,
    pub home: Option<Home>,
    pub official_artwork: Option<OfficialArtwork>,
    pub showdown: Option<Showdown>,
}

#[derive(Deserialize, Debug)]
pub struct DreamWorld {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Home {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OfficialArtwork {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Showdown {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Versions {
    pub generation_i: Option<GenerationI>,
    pub generation_ii: Option<GenerationII>,
    pub generation_iii: Option<GenerationIII>,
    pub generation_iv: Option<GenerationIV>,
    pub generation_v: Option<GenerationV>,
    pub generation_vi: Option<GenerationVI>,
    pub generation_vii: Option<GenerationVII>,
    pub generation_viii: Option<GenerationVIII>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationI {
    pub red_blue: Option<RedBlue>,
    pub yellow: Option<Yellow>,
}

#[derive(Deserialize, Debug)]
pub struct RedBlue {
    pub back_default: Option<String>,
    pub back_gray: Option<String>,
    pub back_transparent: Option<String>,
    pub front_default: Option<String>,
    pub front_gray: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Yellow {
    pub back_default: Option<String>,
    pub back_gray: Option<String>,
    pub back_transparent: Option<String>,
    pub front_default: Option<String>,
    pub front_gray: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationII {
    pub crystal: Option<Crystal>,
    pub gold: Option<Gold>,
    pub silver: Option<Silver>,
}

#[derive(Deserialize, Debug)]
pub struct Crystal {
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_transparent: Option<String>,
    pub back_transparent: Option<String>,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_transparent: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Gold {
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Silver {
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationIII {
    pub emerald: Option<Emerald>,
    pub firered_leafgreen: Option<FireRedLeafGreen>,
    pub ruby_sapphire: Option<RubySapphire>,
}

#[derive(Deserialize, Debug)]
pub struct Emerald {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct FireRedLeafGreen {
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RubySapphire {
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationIV {
    pub diamond_pearl: Option<DiamondPearl>,
    pub heartgold_soulsilver: Option<HeartGoldSoulSilver>,
    pub platinum: Option<Platinum>,
}

#[derive(Deserialize, Debug)]
pub struct DiamondPearl {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct HeartGoldSoulSilver {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Platinum {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationV {
    pub black_white: Option<BlackWhite>,
}

#[derive(Deserialize, Debug)]
pub struct BlackWhite {
    pub animated: Option<Animated>,
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Animated {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationVI {
    pub omegaruby_alphasapphire: Option<OmegaRubyAlphaSapphire>,
    pub x_y: Option<XY>,
}

#[derive(Deserialize, Debug)]
pub struct OmegaRubyAlphaSapphire {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct XY {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationVII {
    pub icons: Option<Icons>,
    pub ultra_sun_ultra_moon: Option<UltraSunUltraMoon>,
}

#[derive(Deserialize, Debug)]
pub struct Icons {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UltraSunUltraMoon {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerationVIII {
    pub icons: Option<Icons>,
}

#[derive(Deserialize, Debug)]
pub struct StatDetail {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub base_stat: u32,
    pub effort: u32,
    pub stat: StatDetail,
}

#[derive(Deserialize, Debug)]
pub struct TypeDetail {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Type {
    pub slot: u32,
    pub r#type: TypeDetail,
}

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub abilities: Vec<Ability>,
    pub base_experience: u32,
    pub cries: Cry,
    pub forms: Vec<Form>,
    pub game_indices: Vec<GameIndex>,
    pub height: u32,
    pub held_items: Vec<HeldItem>,
    pub id: u32,
    pub is_default: bool,
    pub location_area_encounters: String,
    pub moves: Vec<Move>,
    pub name: String,
    pub order: u32,
    pub past_abilities: Vec<Ability>,
    pub past_types: Vec<Type>,
    pub species: Species,
    pub sprites: Sprites,
    pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: u32,
}
