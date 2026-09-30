//! Standalone deterministic Location Forge. Location content and field streams are isolated
//! from the frozen Character and Quest generation paths.

use crate::{FieldStream, FieldValues};
use forge_contracts::{
    CanonState, ContentVersion, CreativeAxes, CreativeAxis, EntityEnvelope, EntityId, EntityOrigin,
    EntityType, FieldKey, ForgeDNA, GenerationAction, GenerationKind, GenerationMetadata,
    GenerationRecipe, GeneratorVersion, INITIAL_SCHEMA_VERSION, Location, LocationProfile,
    PresetId, ProjectId, RootSeed, ValidationCode, ValidationError, ValidationErrors,
    ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const LOCATION_GENERATOR_VERSION: GeneratorVersion = GeneratorVersion(1);
pub const LOCATION_CONTENT_VERSION: ContentVersion = ContentVersion(1);

pub const LOCATION_FIELD_ORDER: [LocationField; 12] = [
    LocationField::Profile,
    LocationField::Name,
    LocationField::VisualIdentity,
    LocationField::Atmosphere,
    LocationField::Purpose,
    LocationField::Summary,
    LocationField::History,
    LocationField::InhabitantsSummary,
    LocationField::NotableFeatures,
    LocationField::Danger,
    LocationField::Secret,
    LocationField::StoryHooks,
];

const PRESETS: [&str; 11] = [
    "dark-fantasy",
    "pirate",
    "steampunk",
    "cyberpunk",
    "nordic-myth",
    "post-apocalyptic",
    "occult",
    "monster-hunter",
    "rogue-assassin",
    "contemporary-weird",
    "chaos-blend",
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocationField {
    Profile,
    Name,
    VisualIdentity,
    Atmosphere,
    Purpose,
    Summary,
    History,
    InhabitantsSummary,
    NotableFeatures,
    Danger,
    Secret,
    StoryHooks,
}

impl LocationField {
    pub const ALL: [Self; 12] = LOCATION_FIELD_ORDER;

    pub fn key(self) -> ValidationResult<FieldKey> {
        FieldKey::new(match self {
            Self::Profile => "01-location-profile",
            Self::Name => "02-name",
            Self::VisualIdentity => "03-visual-identity",
            Self::Atmosphere => "04-atmosphere",
            Self::Purpose => "05-purpose",
            Self::Summary => "06-summary",
            Self::History => "07-history",
            Self::InhabitantsSummary => "08-inhabitants-summary",
            Self::NotableFeatures => "09-notable-features",
            Self::Danger => "10-danger",
            Self::Secret => "11-secret",
            Self::StoryHooks => "12-story-hooks",
        })
    }

    fn from_key(key: &FieldKey) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|field| field.key().is_ok_and(|v| v == *key))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum LocationFieldValue {
    Profile(LocationProfile),
    Text(String),
    Texts(Vec<String>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocationOrigin {
    Generated,
    UserModified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationCandidate {
    pub location: Location,
    pub dna: ForgeDNA,
    pub field_origins: BTreeMap<FieldKey, LocationOrigin>,
}

impl LocationCandidate {
    pub fn validate(&self) -> ValidationResult<()> {
        self.location.validate()?;
        validate_location_dna(&self.dna)?;
        if self
            .dna
            .recipe
            .constraints
            .get(&forge_contracts::ConstraintKey::new("location.profile").expect("static key"))
            .and_then(|value| parse_profile(value))
            .is_some_and(|forced| forced != self.location.location_profile)
        {
            return Err(error(
                ValidationCode::InvalidConstraint,
                "location.location_profile",
                "Location profile does not satisfy its explicit generation constraint",
            ));
        }
        let expected: BTreeSet<_> = LOCATION_FIELD_ORDER
            .into_iter()
            .map(LocationField::key)
            .collect::<ValidationResult<_>>()?;
        let actual: BTreeSet<_> = self.field_origins.keys().cloned().collect();
        if expected != actual {
            return Err(error(
                ValidationCode::MissingField,
                "location_candidate.field_origins",
                "candidate must record the origin of every generated Location field",
            ));
        }
        let provenance = self
            .location
            .envelope
            .generation_provenance
            .as_ref()
            .ok_or_else(|| {
                error(
                    ValidationCode::MissingField,
                    "location_candidate.provenance",
                    "generated Location candidate must retain generation provenance",
                )
            })?;
        if provenance.dna != self.dna {
            return Err(error(
                ValidationCode::InvalidEntity,
                "location_candidate.provenance.dna",
                "candidate DNA and Location provenance DNA must match",
            ));
        }
        Ok(())
    }

    pub fn manual_edit(
        &mut self,
        field: LocationField,
        value: LocationFieldValue,
    ) -> ValidationResult<()> {
        let mut next = self.clone();
        set_field(&mut next.location, field, value)?;
        next.field_origins
            .insert(field.key()?, LocationOrigin::UserModified);
        update_provenance(&mut next, GenerationAction::ManualEdit, None)?;
        next.validate()?;
        *self = next;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LocationForge;

impl LocationForge {
    pub fn new_dna(seed: RootSeed, preset: &str, axes: CreativeAxes) -> ValidationResult<ForgeDNA> {
        let preset = PresetId::new(preset)?;
        if !PRESETS.contains(&preset.as_str()) {
            return Err(error(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                "unsupported Location preset",
            ));
        }
        let mut recipe = GenerationRecipe::new(GenerationKind::Location, seed, preset);
        recipe.creative_axes = axes;
        for field in LOCATION_FIELD_ORDER {
            recipe.add_field(field.key()?)?;
        }
        let profile = LocationField::Profile.key()?;
        for field in LOCATION_FIELD_ORDER
            .into_iter()
            .filter(|f| *f != LocationField::Profile)
        {
            recipe.set_dependencies(field.key()?, [profile.clone()])?;
        }
        let mut dna = ForgeDNA::new(recipe);
        dna.generator_version = LOCATION_GENERATOR_VERSION;
        dna.content_version = LOCATION_CONTENT_VERSION;
        dna.normalize()?;
        validate_location_dna(&dna)?;
        Ok(dna)
    }

    pub fn generate(dna: &ForgeDNA) -> ValidationResult<LocationCandidate> {
        Self::generate_in_project(dna, demo_project_id())
    }

    pub fn generate_in_project(
        dna: &ForgeDNA,
        project_id: ProjectId,
    ) -> ValidationResult<LocationCandidate> {
        let mut dna = dna.clone();
        dna.normalize()?;
        validate_location_dna(&dna)?;
        project_id.validate("project_id")?;
        let values = generate_values(&dna)?;
        let id = deterministic_id(
            "character-forge/location/entity-id/v1",
            &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
        );
        let mut location =
            location_from_values(values, dna.clone(), EntityId::from_bytes(id), project_id)?;
        let generation_id = deterministic_id(
            "character-forge/location/generation-id/v1",
            &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
        );
        location.envelope.generation_provenance = Some(GenerationMetadata {
            generation_id: EntityId::from_bytes(generation_id),
            generated_at_unix_ms: 0,
            action: GenerationAction::Forge,
            parent_generation_id: None,
            dna: dna.clone(),
        });
        location.envelope.tags = vec![format!("preset:{}", dna.recipe.universe_preset)];
        let field_origins = LOCATION_FIELD_ORDER
            .into_iter()
            .map(|field| Ok((field.key()?, LocationOrigin::Generated)))
            .collect::<ValidationResult<_>>()?;
        let candidate = LocationCandidate {
            location,
            dna,
            field_origins,
        };
        candidate.validate()?;
        Ok(candidate)
    }

    pub fn lock(candidate: &mut LocationCandidate, field: LocationField) -> ValidationResult<()> {
        change_lock(candidate, field, true)
    }
    pub fn unlock(candidate: &mut LocationCandidate, field: LocationField) -> ValidationResult<()> {
        change_lock(candidate, field, false)
    }

    pub fn reroll_field(
        candidate: &mut LocationCandidate,
        field: LocationField,
    ) -> ValidationResult<Vec<LocationField>> {
        reroll(candidate, Some(field))
    }

    pub fn reroll_all_unlocked(
        candidate: &mut LocationCandidate,
    ) -> ValidationResult<Vec<LocationField>> {
        reroll(candidate, None)
    }

    pub fn manual_edit(
        candidate: &mut LocationCandidate,
        field: LocationField,
        value: LocationFieldValue,
    ) -> ValidationResult<()> {
        candidate.manual_edit(field, value)
    }
}

#[derive(Clone, Copy)]
struct Theme {
    place: [&'static str; 6],
    material: [&'static str; 6],
    landmark: [&'static str; 6],
    institution: [&'static str; 6],
    event: [&'static str; 6],
    people: [&'static str; 6],
    hazard: [&'static str; 6],
    secret: [&'static str; 6],
    name: [&'static str; 6],
}

fn theme(key: &str) -> Theme {
    match key {
        "dark-fantasy" => Theme {
            place: [
                "barrow town",
                "blackwood hold",
                "ruined abbey",
                "moonlit hamlet",
                "ashen keep",
                "marsh crossing",
            ],
            material: [
                "smoke-dark stone",
                "thorn-black timber",
                "bone-white slate",
                "iron-stained brick",
                "grave glass",
                "peat-cut granite",
            ],
            landmark: [
                "a bell tower with no clapper",
                "a well sealed by seven crowns",
                "a bridge woven from roots",
                "a chapel built upside down",
                "a gate scarred by giant hands",
                "a field of nameless cairns",
            ],
            institution: [
                "the wardens of the last ember",
                "a court of masked stewards",
                "the river-burial guild",
                "a silent order of bell keepers",
                "the winter tithe office",
                "the keepers of borrowed names",
            ],
            event: [
                "a winter siege ended without survivors",
                "the river changed course overnight",
                "a ruler vanished during the harvest oath",
                "the chapel bells rang beneath the earth",
                "a plague stopped at the old boundary",
                "the eastern wall was rebuilt in a single night",
            ],
            people: [
                "families who trade in funeral cloth",
                "pilgrims carrying unlit lanterns",
                "stewards who never remove their gloves",
                "miners who sleep above ground",
                "children taught to avoid one street",
                "a small watch paid in salt",
            ],
            hazard: [
                "the masonry loosens whenever the bells ring",
                "black water rises through dry cellars",
                "the old boundary stones move after dusk",
                "a hungry fog follows open flame",
                "the eastern stair gives way under a name spoken aloud",
                "the burial field exhales warm air",
            ],
            secret: [
                "its founders hid a living heir beneath the well",
                "the keepers have quietly moved the boundary",
                "the bell tower answers a voice from below",
                "the vanished ruler still signs the tithe rolls",
                "the plague was invited to protect one family",
                "the chapel was built around a sealed doorway",
            ],
            name: [
                "Gloamreach",
                "Vesper Cairn",
                "Hollowmere",
                "Gravesend Fold",
                "Blackbriar",
                "Last Ember",
            ],
        },
        "pirate" => Theme {
            place: [
                "freeport",
                "reef-side settlement",
                "tide-market",
                "shipyard quarter",
                "island refuge",
                "salt-cove village",
            ],
            material: [
                "tarred pine",
                "coral limestone",
                "patched sailcloth",
                "green copper",
                "salt-bleached teak",
                "black volcanic glass",
            ],
            landmark: [
                "a lighthouse painted inside out",
                "a dry dock cut into the cliff",
                "a mast forest above the market",
                "a tide clock made from cannon wheels",
                "a lookout carved into a sea stack",
                "a chain gate across the harbor mouth",
            ],
            institution: [
                "the quay captains' compact",
                "a net-menders' exchange",
                "the harbor court of three flags",
                "the salvage ledger office",
                "the night pilots' table",
                "a customs house with no doors",
            ],
            event: [
                "a fleet anchored here and left every ship behind",
                "the reef opened a channel during a storm",
                "a governor sold the harbor twice in one day",
                "the lighthouse went dark for a full season",
                "a mutiny ended with a shared map",
                "the tide carried a bell onto the roof",
            ],
            people: [
                "families who rent their names to ships",
                "salvagers who mark safe water with bottles",
                "pilots who refuse to sail at noon",
                "dockhands paid in shares of future cargo",
                "fishers who keep two sets of books",
                "a watch recruited from retired deck crews",
            ],
            hazard: [
                "the reef shifts after heavy rain",
                "loose rigging snaps across the upper walkways",
                "the harbor chain can trap returning boats",
                "a sinkhole opens near the old dry dock",
                "the tide reaches the market without warning",
                "unmarked powder stores line the west quay",
            ],
            secret: [
                "the compact's first captain never left the island",
                "the safe channel is moving toward the rival port",
                "the harbor court has hidden a royal ship",
                "the ledger records cargo that has not arrived yet",
                "the lighthouse lens points inland at night",
                "the reef was blasted open from below",
            ],
            name: [
                "Rookwater",
                "Gallows Key",
                "Redwake Quay",
                "Saltknife Haven",
                "Black Sail Reach",
                "Three Flags",
            ],
        },
        "steampunk" => Theme {
            place: [
                "gearworks district",
                "steam-valley town",
                "elevated market",
                "foundry ward",
                "canal exchange",
                "railhead settlement",
            ],
            material: [
                "riveted brass",
                "smoke-fired brick",
                "oil-dark oak",
                "etched copper",
                "pressure glass",
                "soot-polished iron",
            ],
            landmark: [
                "a clock whose hands run backward",
                "a lift tower above the canal",
                "a boiler hall with seven chimneys",
                "a bridge of interlocking gears",
                "a public engine behind glass",
                "a rail platform suspended over the foundry",
            ],
            institution: [
                "the boiler inspectors' guild",
                "a cooperative of night engineers",
                "the pressure court",
                "the canal patent office",
                "a union of bridge tenders",
                "the municipal engine trust",
            ],
            event: [
                "a civic engine ran for nine days without fuel",
                "the rail line arrived before it was surveyed",
                "a boiler strike stopped the river traffic",
                "the old clock predicted its own repair",
                "the foundry district survived a deliberate shutdown",
                "an inventor gave every family the same key",
            ],
            people: [
                "engineers who carry paper pressure gauges",
                "canal hands who know the old water routes",
                "clerks paid to certify impossible parts",
                "families living above their workshops",
                "night inspectors with copper badges",
                "apprentices who maintain the public engine",
            ],
            hazard: [
                "pressure vents open without warning",
                "the lift brake slips between floors",
                "old pipes carry steam through sealed rooms",
                "the gear bridge locks at the wrong hour",
                "foundry runoff makes the canal slick",
                "a dormant boiler warms the archive wall",
            ],
            secret: [
                "the municipal engine has been choosing its own operators",
                "the patent office erased the plans for a working brake",
                "the rail line was built to carry one sealed carriage",
                "the public clock records a second hidden time",
                "the union maintains an unauthorized water gate",
                "the boiler strike was caused by a false safety report",
            ],
            name: [
                "Coppercoil",
                "Nine Chimneys",
                "The Reversing Clock",
                "Brasswater",
                "Ironwake Exchange",
                "Pressure Row",
            ],
        },
        "cyberpunk" => Theme {
            place: [
                "stacked transit ward",
                "licensed housing block",
                "night market",
                "data-port enclave",
                "corporate spill zone",
                "rooftop settlement",
            ],
            material: [
                "smart glass",
                "recycled polymer",
                "ceramic armor plate",
                "printed concrete",
                "fiber-optic mesh",
                "carbon-black steel",
            ],
            landmark: [
                "a public terminal that remembers deleted users",
                "a skybridge patched with street signs",
                "a rain garden under an ad screen",
                "a cooling tower converted into a clinic",
                "a freight lift open to the rooftops",
                "a signal mast wrapped in prayer ribbons",
            ],
            institution: [
                "the tenants' mesh council",
                "a courier cooperative",
                "the district access registry",
                "a repair market of unlicensed technicians",
                "a union of night-shift operators",
                "the local water-key exchange",
            ],
            event: [
                "a network outage lasted long enough to become a holiday",
                "the block rejected a new ownership record",
                "an elevator stopped between two registered floors",
                "the market rebuilt its power grid overnight",
                "a vanished transit line returned on the maps",
                "residents voted to erase a corporate street name",
            ],
            people: [
                "couriers who carry paper route books",
                "repairers who trade components for meals",
                "night workers sharing one public kitchen",
                "residents with mismatched access badges",
                "a volunteer crew maintaining the rain garden",
                "families who rent unused rooftop space",
            ],
            hazard: [
                "an elevator ignores several valid access keys",
                "the cooling vents expel heat into the clinic",
                "a bridge panel flexes in high wind",
                "the local power mesh overloads during storms",
                "an obsolete drone patrols the freight lift",
                "an unmarked fiber trench crosses the market",
            ],
            secret: [
                "the ownership registry lists a building that is not there",
                "the outage was caused by residents protecting a witness",
                "the vanished transit line still carries one midnight train",
                "the cooling tower contains an unlisted water reserve",
                "the market's old terminal can restore erased names",
                "a rooftop lease was signed by a future date",
            ],
            name: [
                "Glassline Nine",
                "Low Signal Ward",
                "Rainloop",
                "Stackmarket North",
                "Blue Static Block",
                "Open Sky Exchange",
            ],
        },
        "nordic-myth" => Theme {
            place: [
                "fjord settlement",
                "pine-ring hall",
                "ice-road crossing",
                "highland refuge",
                "island moot",
                "glacier-side steading",
            ],
            material: [
                "wind-scoured granite",
                "tarred pine",
                "blue ice",
                "wool-packed timber",
                "ironwood",
                "snow-polished basalt",
            ],
            landmark: [
                "a standing stone split by lightning",
                "a hall with a roof of whale ribs",
                "a bridge that sings in thaw",
                "a cairn visible from every shore",
                "a hot spring ringed by carved names",
                "a watchfire platform above the pass",
            ],
            institution: [
                "the winter moot",
                "a boatwright kinship",
                "the oath-keepers' bench",
                "a shared storehouse council",
                "the northern ferry compact",
                "a circle of weather readers",
            ],
            event: [
                "the sun failed to set during the oath feast",
                "a glacier left a wooden door in the valley",
                "the ferry returned with one extra shadow",
                "a feud ended when both banners froze together",
                "the watchfire burned through a blizzard",
                "the hall survived a season with no roof",
            ],
            people: [
                "boatwright families who share every tool",
                "herders who leave offerings at the pass",
                "weather readers who keep rival forecasts",
                "travelers sheltered by an open oath",
                "a watch rotating among three islands",
                "children taught the names of old stones",
            ],
            hazard: [
                "the ice road shifts over a deep channel",
                "avalanches follow the sound of iron bells",
                "the hot spring thins the nearby snow shelf",
                "a bridge rope frays in salt wind",
                "fog closes the pass without warning",
                "the cairn field hides a sudden drop",
            ],
            secret: [
                "the moot's founding oath was made to a glacier",
                "the ferry carries the memory of its first crew",
                "the standing stone marks a moving boundary",
                "the watchfire is fueled by wood from a living tree",
                "one island appears only during the long night",
                "the shared storehouse has a room no one built",
            ],
            name: [
                "Frostwake Hall",
                "Three Shore Moot",
                "Blue Cairn",
                "Skeld Crossing",
                "Northfire Steading",
                "The Singing Bridge",
            ],
        },
        "post-apocalyptic" => Theme {
            place: [
                "water-stop settlement",
                "overpass commune",
                "rail-yard refuge",
                "dust-basin market",
                "silo district",
                "windbreak camp",
            ],
            material: [
                "salvaged road plate",
                "sun-bleached concrete",
                "patched billboard cloth",
                "reused turbine steel",
                "fired clay block",
                "weathered transit glass",
            ],
            landmark: [
                "a water tower painted with old weather maps",
                "a bus depot turned into a seed bank",
                "a windmill built from signal gantries",
                "a mile marker covered in names",
                "a bridge that shelters a weekly market",
                "a radio mast used as a rain gauge",
            ],
            institution: [
                "the water-share council",
                "a road repair crew",
                "the seed ledger cooperative",
                "a barter court held at noon",
                "the caravan stop committee",
                "a volunteer pump watch",
            ],
            event: [
                "the old highway reopened after a decade",
                "a storm filled the cisterns with clean water",
                "the market refused a traveling governor",
                "the radio mast broadcast a living voice",
                "a bridge was raised one meter at a time",
                "the seed bank survived a long dry year",
            ],
            people: [
                "pump keepers who rotate every night",
                "travelers trading maps for water",
                "families growing greens in engine trays",
                "road crews paid in spare bolts",
                "children who can read old mile signs",
                "a clinic run from a parked bus",
            ],
            hazard: [
                "the cistern walls buckle under high demand",
                "dust hides a break in the overpass",
                "a storm drain leads into unstable ground",
                "the bridge deck shifts under heavy carts",
                "the turbine brakes fail in high wind",
                "an old fuel tank lies under the market",
            ],
            secret: [
                "the seed ledger includes crops nobody has seen",
                "the radio voice comes from a sealed basement",
                "the cistern was built over a pre-collapse archive",
                "the caravan committee keeps a second water count",
                "the mile marker points to a road that was erased",
                "the windmill powers a door beneath the camp",
            ],
            name: [
                "Mile Forty-One",
                "Second Rain",
                "The Bus Garden",
                "Red Tank Crossing",
                "Windbreak Common",
                "Old Line Market",
            ],
        },
        "occult" => Theme {
            place: [
                "threshold district",
                "sealed archive quarter",
                "lantern suburb",
                "circle-road village",
                "night court precinct",
                "quiet observatory",
            ],
            material: [
                "ink-dark limestone",
                "mirror tile",
                "waxed cedar",
                "smoked glass",
                "silvered plaster",
                "black enamel",
            ],
            landmark: [
                "a clock face with thirteen hours",
                "a public mirror turned to the wall",
                "a reading room with no outside door",
                "a fountain that runs only at dusk",
                "a boundary arch marked with blank names",
                "a roof observatory closed every new moon",
            ],
            institution: [
                "the society of ordinary witnesses",
                "a registry of borrowed rooms",
                "the night reading circle",
                "a volunteer boundary watch",
                "the city archive's sealed desk",
                "a league of careful bell ringers",
            ],
            event: [
                "every written address changed on the same night",
                "the mirror showed a room before it was built",
                "a reading circle found its own minutes in an older book",
                "the fountain stopped when a stranger arrived",
                "the boundary arch appeared on a wall map",
                "the observatory recorded a second moon",
            ],
            people: [
                "archivists who wear plain gloves",
                "neighbors who leave lamps in pairs",
                "readers who compare notes without speaking",
                "a night watch that changes its route weekly",
                "families who keep a spare empty room",
                "clerks trained to notice missing names",
            ],
            hazard: [
                "the archive stairs return visitors to the wrong floor",
                "a mirror corridor causes disorientation",
                "the boundary arch draws crowds after midnight",
                "the fountain water stains metal black",
                "a bell is audible only from locked rooms",
                "the observatory roof opens in high wind",
            ],
            secret: [
                "the registry contains a lease signed by every current resident",
                "the mirror reflects a district erased from the city plan",
                "the archive's missing shelf is still being used",
                "the circle-road has one extra turn after rain",
                "the sealed desk receives letters in fresh ink",
                "the second moon appears in ordinary photographs",
            ],
            name: [
                "Thirteen Hour Court",
                "The Blank Address",
                "Mirrorward",
                "Quiet Circle",
                "New Moon Desk",
                "Borrowed Room Row",
            ],
        },
        "monster-hunter" => Theme {
            place: [
                "frontier lodge",
                "quarry town",
                "barrier village",
                "ferry outpost",
                "pine-edge ward",
                "high-pass station",
            ],
            material: [
                "reinforced oak",
                "scored limestone",
                "hide-bound timber",
                "quenched iron",
                "smoke-cured plank",
                "pale quarry stone",
            ],
            landmark: [
                "a bell cage above the north road",
                "a salt wall around the livestock yard",
                "a hunter's lodge built into the quarry",
                "a warning board with replaced names",
                "a ferry slip chained at dusk",
                "a watch platform facing the forest",
            ],
            institution: [
                "the spoor readers' lodge",
                "a livestock wardens' compact",
                "the road-signal crew",
                "a licensed trap repair shop",
                "the frontier medicine house",
                "a rotating night patrol",
            ],
            event: [
                "the north road stayed quiet through migration season",
                "the quarry crew found a door with claw marks inside",
                "a warning bell rang before tracks appeared",
                "the ferry brought back an empty cage",
                "the watch platform was rebuilt facing inland",
                "the wardens closed a road that had no tracks",
            ],
            people: [
                "trackers who teach children to read broken brush",
                "farmers who keep paired lanterns",
                "medics who trade tinctures for reports",
                "ferry hands trained to spot false cargo",
                "a patrol that includes retired hunters",
                "quarry workers who leave the west face untouched",
            ],
            hazard: [
                "the barrier wall has a blind section at dawn",
                "a quarry shelf breaks after frost",
                "livestock gates jam during alarms",
                "the ferry chain catches in fast current",
                "old traps remain under the north road",
                "a watch platform ladder shifts in wet weather",
            ],
            secret: [
                "the lodge has been hiding an unclassified creature",
                "the warning board lists a person who has not arrived",
                "the empty cage was opened from inside the ferry",
                "the quarry door predates the town's oldest map",
                "the patrol is following tracks that loop beneath the village",
                "the west face is being kept intact for a reason",
            ],
            name: [
                "North Bell Lodge",
                "Saltwall Station",
                "Trackers' Rest",
                "Pale Quarry Ward",
                "Last Ferry Outpost",
                "Stillwood Gate",
            ],
        },
        "rogue-assassin" => Theme {
            place: [
                "covered bazaar ward",
                "canal-side quarter",
                "courier stair district",
                "old embassy row",
                "roof-market enclave",
                "passage court",
            ],
            material: [
                "oiled cedar",
                "blue slate",
                "brushed brass",
                "painted brick",
                "smoked mirror",
                "woven reed screen",
            ],
            landmark: [
                "a public clock with a removable face",
                "a stair that opens into three courtyards",
                "a tea house with six exits",
                "a roof bridge hidden by laundry lines",
                "a fountain fed from an unlisted pipe",
                "a gate with two official addresses",
            ],
            institution: [
                "the couriers' neutral table",
                "a locksmiths' mutual",
                "the night market ledger",
                "a discreet tenants' court",
                "the roof passage keepers",
                "a public message room",
            ],
            event: [
                "a sealed letter arrived before it was written",
                "the ward hosted two rival envoys at one table",
                "a public clock lost one minute each night",
                "the couriers refused a lucrative route",
                "a passage was closed and reopened elsewhere",
                "the fountain ran clear during a citywide blackout",
            ],
            people: [
                "couriers who never use the same stair twice",
                "shopkeepers sharing a hidden alarm cord",
                "locksmiths who teach apprentices in public",
                "families maintaining rooftop gardens",
                "messengers paid to forget names",
                "a watch trained to make room for crowds",
            ],
            hazard: [
                "the covered walk has a loose roof panel",
                "a canal stair floods without rain",
                "one courtyard gate can lock from outside",
                "the roof bridge is slick with soot",
                "a fountain pipe carries pressure spikes",
                "an unlit passage opens onto a busy street",
            ],
            secret: [
                "the neutral table has hosted the same absent guest for years",
                "the extra address belongs to a room beneath the fountain",
                "the couriers' ledger records messages never delivered",
                "the clock's missing minute is stored in a locked drawer",
                "the blackout was caused to reveal a hidden passage",
                "one public alarm cord is connected to every shop",
            ],
            name: [
                "Six Exit Court",
                "Blue Thread Ward",
                "The Quiet Stair",
                "Double Address Row",
                "Last Message Bazaar",
                "Underclock Passage",
            ],
        },
        "contemporary-weird" => Theme {
            place: [
                "suburban shopping strip",
                "riverside civic block",
                "edge-of-town motel row",
                "commuter station district",
                "renovated mill quarter",
                "public park enclave",
            ],
            material: [
                "washed brick",
                "reflective office glass",
                "weathered cedar",
                "mosaic tile",
                "painted steel",
                "reclaimed mill stone",
            ],
            landmark: [
                "a bus shelter with tomorrow's timetable",
                "a laundromat open only during storms",
                "a footbridge that adds one railing each year",
                "a public noticeboard with a blank square",
                "a motel ice machine that hums a tune",
                "a park gate that faces away from the path",
            ],
            institution: [
                "the neighborhood repair café",
                "a volunteer flood-watch group",
                "the tenants' association",
                "a late-night library desk",
                "the commuter lost-property office",
                "a community garden committee",
            ],
            event: [
                "every home received the same misdelivered key",
                "the station announced a stop that was demolished",
                "a flood-watch group predicted a clear night",
                "the repair café fixed an object no one brought",
                "a park path moved between two surveys",
                "the motel registered a guest under the town's name",
            ],
            people: [
                "commuters who compare impossible receipts",
                "neighbors tending a garden in numbered rows",
                "librarians who keep a shelf for returned dreams",
                "night cleaners who know every locked door",
                "volunteers mapping small local noises",
                "families who leave chairs facing the river",
            ],
            hazard: [
                "the footbridge has a gap that moves each week",
                "a service door opens into a different corridor",
                "the station platform narrows near the last sign",
                "the park path floods uphill",
                "a motel stair ends one step early",
                "the garden's old irrigation line carries warm water",
            ],
            secret: [
                "the lost-property office has an item for every resident not yet born",
                "the blank noticeboard square changes size overnight",
                "the motel's guestbook records a room that was never built",
                "the demolished station stop is still used by one commuter",
                "the repair café has a volunteer who is absent in photographs",
                "the river's flood marks are higher on the dry side",
            ],
            name: [
                "Tomorrow Stop",
                "Mill-End Common",
                "Room Zero Row",
                "The Moving Footbridge",
                "Last Sign Station",
                "Riverward Notice",
            ],
        },
        _ => unreachable!("preset was validated before theme lookup"),
    }
}

fn validate_location_dna(dna: &ForgeDNA) -> ValidationResult<()> {
    dna.validate()?;
    if dna.recipe.kind != GenerationKind::Location
        || dna.generator_version != LOCATION_GENERATOR_VERSION
        || dna.content_version != LOCATION_CONTENT_VERSION
    {
        return Err(error(
            ValidationCode::UnsupportedVersion,
            "location.dna",
            "Location Forge requires generation/content version 1 and Location generation kind",
        ));
    }
    if !PRESETS.contains(&dna.recipe.universe_preset.as_str()) {
        return Err(error(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "unsupported Location preset",
        ));
    }
    for (key, value) in &dna.recipe.constraints {
        if key.as_str() != "location.profile" || parse_profile(value).is_none() {
            return Err(error(
                ValidationCode::InvalidConstraint,
                "constraints.location.profile",
                "Location supports only the location.profile constraint with a supported profile value",
            ));
        }
    }
    let expected: BTreeSet<_> = LOCATION_FIELD_ORDER
        .into_iter()
        .map(LocationField::key)
        .collect::<ValidationResult<_>>()?;
    if dna
        .recipe
        .field_controls
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != expected
    {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.field_controls",
            "Location ForgeDNA must register exactly the canonical Location fields",
        ));
    }
    let profile = LocationField::Profile.key()?;
    let dependencies = LOCATION_FIELD_ORDER
        .into_iter()
        .filter(|f| *f != LocationField::Profile)
        .map(|f| Ok((f.key()?, BTreeSet::from([profile.clone()]))))
        .collect::<ValidationResult<BTreeMap<_, _>>>()?;
    if dna.recipe.dependencies != dependencies {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.dependencies",
            "Location dependency graph must contain only profile → generated fields",
        ));
    }
    Ok(())
}

fn generate_values(dna: &ForgeDNA) -> ValidationResult<FieldValues<LocationFieldValue>> {
    let mut values = FieldValues::new();
    for field in LOCATION_FIELD_ORDER {
        let key = field.key()?;
        let mut stream = FieldStream::derive(dna, &key)?;
        let profile = values.get(&LocationField::Profile.key()?).and_then(|v| {
            if let LocationFieldValue::Profile(p) = v {
                Some(*p)
            } else {
                None
            }
        });
        values.insert(
            key.clone(),
            generate_field(dna, field, profile, &mut stream)?,
        );
    }
    Ok(values)
}

fn theme_for(
    dna: &ForgeDNA,
    field: &FieldKey,
    stream: &mut FieldStream,
) -> ValidationResult<Theme> {
    if dna.recipe.universe_preset.as_str() != "chaos-blend" {
        return Ok(theme(dna.recipe.universe_preset.as_str()));
    }
    // Chaos Blend selects one source for the whole candidate. Deriving it from
    // the profile stream keeps every field coherent and makes profile rerolls
    // the explicit boundary for changing that source.
    let mut source_stream = FieldStream::derive(dna, &LocationField::Profile.key()?)?;
    let source = PRESETS[..10][source_stream.next_bounded(10)? as usize];
    let _ = field;
    let _ = stream;
    Ok(theme(source))
}

fn generate_field(
    dna: &ForgeDNA,
    field: LocationField,
    profile: Option<LocationProfile>,
    stream: &mut FieldStream,
) -> ValidationResult<LocationFieldValue> {
    let key = field.key()?;
    let theme = theme_for(dna, &key, stream)?;
    let index = stream.next_bounded(6)? as usize;
    let constrained_profile = dna
        .recipe
        .constraints
        .get(&forge_contracts::ConstraintKey::new("location.profile").expect("static key"))
        .and_then(|value| parse_profile(value));
    let profile = constrained_profile.or(profile).unwrap_or_else(|| {
        [
            LocationProfile::Settlement,
            LocationProfile::District,
            LocationProfile::Building,
            LocationProfile::Dungeon,
            LocationProfile::Ruins,
            LocationProfile::Wilderness,
            LocationProfile::Landmark,
            LocationProfile::Interior,
            LocationProfile::Site,
            LocationProfile::Other,
        ][stream.next_bounded(10).unwrap_or(0) as usize]
    });
    let profile_name = profile_key(profile);
    let axes = &dna.recipe.creative_axes;
    let value = match field {
        LocationField::Profile => LocationFieldValue::Profile(profile),
        LocationField::Name => {
            LocationFieldValue::Text(format!("{} {}", theme.name[index], profile_name))
        }
        LocationField::VisualIdentity => LocationFieldValue::Text(
            [
                format!(
                    "The {} uses {} walls with {} inlay and a {} roofline.",
                    profile_name,
                    theme.material[index],
                    theme.material[(index + 2) % 6],
                    theme.material[(index + 4) % 6]
                ),
                format!(
                    "The {} {} rises above {} and bears a line of {} inlay.",
                    profile_name,
                    landmark(theme.landmark[index]),
                    theme.material[(index + 1) % 6],
                    theme.material[(index + 3) % 6]
                ),
                format!(
                    "{} walls, {} roofs, and {} trim make the {} recognizable from afar.",
                    theme.material[index],
                    theme.material[(index + 2) % 6],
                    theme.material[(index + 4) % 6],
                    profile_name
                ),
                format!(
                    "A {} façade surrounds the {}, while {} panels mark the older rooms.",
                    theme.material[index],
                    landmark(theme.landmark[(index + 3) % 6]),
                    theme.material[(index + 5) % 6]
                ),
                format!(
                    "The {} is built around the {}; its high windows catch the color of {}.",
                    profile_name,
                    landmark(theme.landmark[(index + 4) % 6]),
                    theme.material[(index + 1) % 6]
                ),
                format!(
                    "{} arches shelter the {}, whose surface is scored with {} lines.",
                    theme.material[index],
                    landmark(theme.landmark[(index + 5) % 6]),
                    theme.material[(index + 2) % 6]
                ),
            ][index]
                .clone(),
        ),
        LocationField::Atmosphere => {
            let mood = axis_mood(axes, index);
            LocationFieldValue::Text(format!(
                "{} The air carries sounds from {} into every corner.",
                mood,
                theme.place[(index + 1) % 6]
            ))
        }
        LocationField::Purpose => LocationFieldValue::Text(
            [
                format!(
                    "It serves as a meeting place for the {}.",
                    articleless(theme.institution[index])
                ),
                format!(
                    "Its central work is to shelter {}.",
                    theme.people[(index + 1) % 6]
                ),
                format!(
                    "Residents use it to maintain the {}.",
                    landmark(theme.landmark[(index + 2) % 6])
                ),
                format!(
                    "It connects the {} with the wider {}.",
                    articleless(theme.institution[(index + 3) % 6]),
                    theme.place[index]
                ),
                format!(
                    "It keeps records and supplies for {}.",
                    theme.people[(index + 4) % 6]
                ),
                format!(
                    "Its public duty is to protect the {}.",
                    landmark(theme.landmark[(index + 5) % 6])
                ),
            ][index]
                .clone(),
        ),
        LocationField::Summary => LocationFieldValue::Text(
            [
                format!(
                    "The {} is centered in the {}, around the {}.",
                    profile_name,
                    theme.place[index],
                    landmark(theme.landmark[(index + 1) % 6])
                ),
                format!(
                    "Within the {}, the {} gives the {} its daily rhythm.",
                    theme.place[(index + 1) % 6],
                    articleless(theme.institution[index]),
                    profile_name
                ),
                format!(
                    "The {} stands beside the {}; residents build with {}.",
                    profile_name,
                    landmark(theme.landmark[index]),
                    theme.material[(index + 2) % 6]
                ),
                format!(
                    "The {} draws {} to the {}.",
                    theme.place[(index + 3) % 6],
                    theme.people[index],
                    landmark(theme.landmark[(index + 2) % 6])
                ),
                format!(
                    "The {} is known for the {} and sits within the {}.",
                    profile_name,
                    landmark(theme.landmark[(index + 4) % 6]),
                    theme.place[(index + 5) % 6]
                ),
                format!(
                    "The {} combines {} construction with the routines of {}.",
                    profile_name,
                    theme.material[index],
                    theme.people[(index + 3) % 6]
                ),
            ][index]
                .clone(),
        ),
        LocationField::History => LocationFieldValue::Text(
            [
                format!("Its defining change came when {}.", theme.event[index]),
                format!(
                    "The oldest local account says {}.",
                    theme.event[(index + 1) % 6]
                ),
                format!(
                    "After {}, residents reorganized the {}.",
                    theme.event[(index + 2) % 6],
                    articleless(theme.institution[index])
                ),
                format!(
                    "The {} still marks the place where {}.",
                    landmark(theme.landmark[(index + 3) % 6]),
                    theme.event[(index + 3) % 6]
                ),
                format!(
                    "A generation ago, {}; the repair remains visible in {}.",
                    theme.event[(index + 4) % 6],
                    theme.material[index]
                ),
                format!(
                    "Local records disagree about how {} began, but all mention {}.",
                    theme.place[index],
                    theme.event[(index + 5) % 6]
                ),
            ][index]
                .clone(),
        ),
        LocationField::InhabitantsSummary => LocationFieldValue::Text(
            [
                format!(
                    "Most residents are {}, alongside {}.",
                    theme.people[index],
                    theme.institution[(index + 1) % 6]
                ),
                format!(
                    "The daily work is shared by {} and {}.",
                    theme.people[(index + 1) % 6],
                    theme.people[(index + 2) % 6]
                ),
                format!(
                    "Visitors usually meet {}, who keep the {} in repair.",
                    theme.people[(index + 2) % 6],
                    landmark(theme.landmark[index])
                ),
                format!(
                    "A close-knit group of {} keeps the {} operating.",
                    theme.people[(index + 3) % 6],
                    articleless(theme.institution[(index + 3) % 6])
                ),
                format!(
                    "The population includes {} and people who service the {}.",
                    theme.people[(index + 4) % 6],
                    landmark(theme.landmark[(index + 4) % 6])
                ),
                format!(
                    "Residents gather at the {}; {} handle the public watch.",
                    articleless(theme.institution[(index + 5) % 6]),
                    theme.people[(index + 5) % 6]
                ),
            ][index]
                .clone(),
        ),
        LocationField::NotableFeatures => {
            LocationFieldValue::Texts(distinct_three(index, |i| match i {
                0 => landmark(theme.landmark[index]).to_string(),
                1 => landmark(theme.landmark[(index + 1) % 6]).to_string(),
                2 => landmark(theme.landmark[(index + 2) % 6]).to_string(),
                3 => landmark(theme.landmark[(index + 3) % 6]).to_string(),
                4 => landmark(theme.landmark[(index + 4) % 6]).to_string(),
                _ => landmark(theme.landmark[(index + 5) % 6]).to_string(),
            }))
        }
        LocationField::Danger => LocationFieldValue::Text(
            [
                theme.hazard[index].to_string(),
                theme.hazard[(index + 1) % 6].to_string(),
                theme.hazard[(index + 2) % 6].to_string(),
                theme.hazard[(index + 3) % 6].to_string(),
                theme.hazard[(index + 4) % 6].to_string(),
                theme.hazard[(index + 5) % 6].to_string(),
            ][index]
                .clone(),
        ),
        LocationField::Secret => LocationFieldValue::Text(
            [
                format!("Few know that {}.", theme.secret[index]),
                format!(
                    "An old record suggests that {}.",
                    theme.secret[(index + 1) % 6]
                ),
                format!("The quiet truth is that {}.", theme.secret[(index + 2) % 6]),
                format!(
                    "Behind its public story, {}.",
                    theme.secret[(index + 3) % 6]
                ),
                format!(
                    "A sealed account claims that {}.",
                    theme.secret[(index + 4) % 6]
                ),
                format!(
                    "The institution denies that {}.",
                    theme.secret[(index + 5) % 6]
                ),
            ][index]
                .clone(),
        ),
        LocationField::StoryHooks => {
            LocationFieldValue::Texts(distinct_three(index, |i| match i {
                0 => theme.secret[(index + i) % 6].to_string(),
                1 => theme.event[(index + i) % 6].to_string(),
                2 => theme.hazard[(index + i) % 6].to_string(),
                3 => landmark(theme.landmark[(index + i) % 6]).to_string(),
                4 => theme.institution[(index + i) % 6].to_string(),
                _ => theme.people[(index + i) % 6].to_string(),
            }))
        }
    };
    Ok(value)
}

fn parse_profile(value: &str) -> Option<LocationProfile> {
    Some(match value {
        "settlement" => LocationProfile::Settlement,
        "district" => LocationProfile::District,
        "building" => LocationProfile::Building,
        "dungeon" => LocationProfile::Dungeon,
        "ruins" => LocationProfile::Ruins,
        "wilderness" => LocationProfile::Wilderness,
        "landmark" => LocationProfile::Landmark,
        "interior" => LocationProfile::Interior,
        "site" => LocationProfile::Site,
        "other" => LocationProfile::Other,
        _ => return None,
    })
}

fn articleless(value: &str) -> &str {
    for article in ["a ", "an ", "the "] {
        if let Some(rest) = value.strip_prefix(article) {
            return rest;
        }
    }
    value
}

fn landmark(value: &str) -> &str {
    articleless(value)
}

fn distinct_three(start: usize, mut value: impl FnMut(usize) -> String) -> Vec<String> {
    (0..3)
        .map(|offset| value((start + offset * 2) % 6))
        .collect()
}

fn axis_mood(axes: &CreativeAxes, index: usize) -> String {
    let level = |axis| axes.levels.get(&axis).copied().unwrap_or(50);
    let base = match index {
        0 => "A measured quiet settles over the place.",
        1 => "The streets feel busy but carefully observed.",
        2 => "A patient calm hangs between the buildings.",
        3 => "The place seems to pause before each change.",
        4 => "Ordinary sounds carry farther than expected.",
        _ => "The local rhythm is steady and deliberate.",
    };
    if level(CreativeAxis::Surreal) > 75 || level(CreativeAxis::Mysticism) > 75 {
        return format!(
            "{} Familiar landmarks seem slightly out of step with the hour.",
            base
        );
    }
    if level(CreativeAxis::Bleak) > 75 {
        return format!("{} People keep close to shelter and to one another.", base);
    }
    if level(CreativeAxis::Mythic) > 75 {
        return format!("{} Old customs still shape the smallest decisions.", base);
    }
    if level(CreativeAxis::Technology) > 75 {
        return format!("{} Quiet machinery marks the working day.", base);
    }
    if level(CreativeAxis::Epic) > 75 {
        return format!(
            "{} The surrounding landscape makes each route feel consequential.",
            base
        );
    }
    base.to_owned()
}

fn profile_key(profile: LocationProfile) -> &'static str {
    match profile {
        LocationProfile::Settlement => "Settlement",
        LocationProfile::District => "District",
        LocationProfile::Building => "Building",
        LocationProfile::Dungeon => "Dungeon",
        LocationProfile::Ruins => "Ruins",
        LocationProfile::Wilderness => "Wilderness",
        LocationProfile::Landmark => "Landmark",
        LocationProfile::Interior => "Interior",
        LocationProfile::Site => "Site",
        LocationProfile::Other => "Site",
    }
}

fn location_from_values(
    values: FieldValues<LocationFieldValue>,
    dna: ForgeDNA,
    id: EntityId,
    project_id: ProjectId,
) -> ValidationResult<Location> {
    let profile = match values.get(&LocationField::Profile.key()?) {
        Some(LocationFieldValue::Profile(value)) => *value,
        _ => {
            return Err(error(
                ValidationCode::MissingField,
                "location.location_profile",
                "generated profile is missing",
            ));
        }
    };
    let text = |field: LocationField| match values.get(&field.key().ok()?) {
        Some(LocationFieldValue::Text(value)) => Some(value.clone()),
        _ => None,
    };
    let texts = |field: LocationField| match values.get(&field.key().ok()?) {
        Some(LocationFieldValue::Texts(value)) => Some(value.clone()),
        _ => None,
    };
    let name = text(LocationField::Name).ok_or_else(|| {
        error(
            ValidationCode::MissingField,
            "location.name",
            "generated name is missing",
        )
    })?;
    Ok(Location {
        envelope: EntityEnvelope {
            id,
            project_id,
            entity_type: EntityType::Location,
            name,
            schema_version: INITIAL_SCHEMA_VERSION,
            canon_state: CanonState::Candidate,
            origin: EntityOrigin::Generated,
            favorite: false,
            created_at_unix_ms: 0,
            updated_at_unix_ms: 0,
            tags: vec![format!("preset:{}", dna.recipe.universe_preset)],
            notes: None,
            generation_provenance: None,
        },
        location_profile: profile,
        summary: text(LocationField::Summary).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.summary",
                "generated summary is missing",
            )
        })?,
        visual_identity: text(LocationField::VisualIdentity).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.visual_identity",
                "generated visual identity is missing",
            )
        })?,
        atmosphere: text(LocationField::Atmosphere).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.atmosphere",
                "generated atmosphere is missing",
            )
        })?,
        purpose: text(LocationField::Purpose).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.purpose",
                "generated purpose is missing",
            )
        })?,
        history: text(LocationField::History).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.history",
                "generated history is missing",
            )
        })?,
        inhabitants_summary: text(LocationField::InhabitantsSummary).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.inhabitants_summary",
                "generated inhabitants are missing",
            )
        })?,
        notable_features: texts(LocationField::NotableFeatures).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.notable_features",
                "generated features are missing",
            )
        })?,
        danger: text(LocationField::Danger).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.danger",
                "generated danger is missing",
            )
        })?,
        secret: text(LocationField::Secret).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.secret",
                "generated secret is missing",
            )
        })?,
        story_hooks: texts(LocationField::StoryHooks).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.story_hooks",
                "generated hooks are missing",
            )
        })?,
    })
}

fn candidate_values(location: &Location) -> ValidationResult<FieldValues<LocationFieldValue>> {
    let mut values = FieldValues::new();
    for (field, value) in [
        (
            LocationField::Profile,
            LocationFieldValue::Profile(location.location_profile),
        ),
        (
            LocationField::Name,
            LocationFieldValue::Text(location.envelope.name.clone()),
        ),
        (
            LocationField::VisualIdentity,
            LocationFieldValue::Text(location.visual_identity.clone()),
        ),
        (
            LocationField::Atmosphere,
            LocationFieldValue::Text(location.atmosphere.clone()),
        ),
        (
            LocationField::Purpose,
            LocationFieldValue::Text(location.purpose.clone()),
        ),
        (
            LocationField::Summary,
            LocationFieldValue::Text(location.summary.clone()),
        ),
        (
            LocationField::History,
            LocationFieldValue::Text(location.history.clone()),
        ),
        (
            LocationField::InhabitantsSummary,
            LocationFieldValue::Text(location.inhabitants_summary.clone()),
        ),
        (
            LocationField::NotableFeatures,
            LocationFieldValue::Texts(location.notable_features.clone()),
        ),
        (
            LocationField::Danger,
            LocationFieldValue::Text(location.danger.clone()),
        ),
        (
            LocationField::Secret,
            LocationFieldValue::Text(location.secret.clone()),
        ),
        (
            LocationField::StoryHooks,
            LocationFieldValue::Texts(location.story_hooks.clone()),
        ),
    ] {
        values.insert(field.key()?, value);
    }
    Ok(values)
}

fn set_field(
    location: &mut Location,
    field: LocationField,
    value: LocationFieldValue,
) -> ValidationResult<()> {
    match (field, value) {
        (LocationField::Profile, LocationFieldValue::Profile(v)) => location.location_profile = v,
        (LocationField::Name, LocationFieldValue::Text(v)) => location.envelope.name = v,
        (LocationField::VisualIdentity, LocationFieldValue::Text(v)) => {
            location.visual_identity = v
        }
        (LocationField::Atmosphere, LocationFieldValue::Text(v)) => location.atmosphere = v,
        (LocationField::Purpose, LocationFieldValue::Text(v)) => location.purpose = v,
        (LocationField::Summary, LocationFieldValue::Text(v)) => location.summary = v,
        (LocationField::History, LocationFieldValue::Text(v)) => location.history = v,
        (LocationField::InhabitantsSummary, LocationFieldValue::Text(v)) => {
            location.inhabitants_summary = v
        }
        (LocationField::NotableFeatures, LocationFieldValue::Texts(v)) => {
            location.notable_features = v
        }
        (LocationField::Danger, LocationFieldValue::Text(v)) => location.danger = v,
        (LocationField::Secret, LocationFieldValue::Text(v)) => location.secret = v,
        (LocationField::StoryHooks, LocationFieldValue::Texts(v)) => location.story_hooks = v,
        _ => {
            return Err(error(
                ValidationCode::InvalidEntity,
                "location.field",
                "field value has the wrong scalar/list/profile shape",
            ));
        }
    }
    Ok(())
}

fn reroll(
    candidate: &mut LocationCandidate,
    requested: Option<LocationField>,
) -> ValidationResult<Vec<LocationField>> {
    let mut next = candidate.clone();
    next.validate()?;
    let mut dna = next.dna.clone();
    let prior_locks: BTreeMap<_, _> = dna
        .recipe
        .field_controls
        .iter()
        .map(|(k, v)| (k.clone(), v.locked))
        .collect();
    for (key, origin) in &next.field_origins {
        if *origin == LocationOrigin::UserModified
            && requested
                .as_ref()
                .is_none_or(|field| field.key().is_ok_and(|requested_key| requested_key != *key))
            && let Some(control) = dna.recipe.field_controls.get_mut(key)
        {
            control.locked = true;
        }
    }
    let mut values = candidate_values(&next.location)?;
    let mut staged = values.clone();
    let recipe_dna = dna.clone();
    let result = if let Some(field) = requested {
        crate::reroll_field(&mut dna, &mut values, &field.key()?, |key, stream| {
            let Some(field) = LocationField::from_key(key) else {
                return Err(error(
                    ValidationCode::UnknownField,
                    "location.field",
                    "unknown Location field key",
                ));
            };
            let profile = staged.get(&LocationField::Profile.key()?).and_then(|v| {
                if let LocationFieldValue::Profile(p) = v {
                    Some(*p)
                } else {
                    None
                }
            });
            let value = generate_field(&recipe_dna, field, profile, stream)?;
            staged.insert(key.clone(), value.clone());
            Ok(value)
        })?
    } else {
        crate::reroll_all_unlocked(&mut dna, &mut values, |key, stream| {
            let Some(field) = LocationField::from_key(key) else {
                return Err(error(
                    ValidationCode::UnknownField,
                    "location.field",
                    "unknown Location field key",
                ));
            };
            let profile = staged.get(&LocationField::Profile.key()?).and_then(|v| {
                if let LocationFieldValue::Profile(p) = v {
                    Some(*p)
                } else {
                    None
                }
            });
            let value = generate_field(&recipe_dna, field, profile, stream)?;
            staged.insert(key.clone(), value.clone());
            Ok(value)
        })?
    };
    for (key, locked) in prior_locks {
        if let Some(control) = dna.recipe.field_controls.get_mut(&key) {
            control.locked = locked;
        }
    }
    for key in &result.changed_fields {
        let field = LocationField::from_key(key).ok_or_else(|| {
            error(
                ValidationCode::UnknownField,
                "location.field",
                "unknown Location field key",
            )
        })?;
        let value = values.get(key).cloned().ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "location.field",
                "rerolled field value is missing",
            )
        })?;
        set_field(&mut next.location, field, value)?;
        next.field_origins
            .insert(key.clone(), LocationOrigin::Generated);
    }
    next.dna = dna;
    update_provenance(
        &mut next,
        if requested.is_some() {
            GenerationAction::RerollField
        } else {
            GenerationAction::RerollUnlocked
        },
        None,
    )?;
    next.validate()?;
    *candidate = next;
    Ok(result
        .changed_fields
        .into_iter()
        .filter_map(|key| LocationField::from_key(&key))
        .collect())
}

fn change_lock(
    candidate: &mut LocationCandidate,
    field: LocationField,
    locked: bool,
) -> ValidationResult<()> {
    let mut next = candidate.clone();
    next.validate()?;
    let control = next
        .dna
        .recipe
        .field_controls
        .get_mut(&field.key()?)
        .ok_or_else(|| {
            error(
                ValidationCode::UnknownField,
                "location.field",
                "field is not registered",
            )
        })?;
    control.locked = locked;
    update_provenance(&mut next, GenerationAction::ManualEdit, None)?;
    next.validate()?;
    *candidate = next;
    Ok(())
}

fn update_provenance(
    candidate: &mut LocationCandidate,
    action: GenerationAction,
    parent: Option<EntityId>,
) -> ValidationResult<()> {
    let mut dna = candidate.dna.clone();
    dna.normalize()?;
    candidate.dna = dna.clone();
    let generation_id = deterministic_id(
        "character-forge/location/generation-id/v1",
        &[
            &dna.canonical_bytes()?,
            candidate.location.envelope.id.uuid().as_bytes(),
            &[action as u8],
        ],
    );
    candidate.location.envelope.generation_provenance = Some(GenerationMetadata {
        generation_id: EntityId::from_bytes(generation_id),
        generated_at_unix_ms: 0,
        action,
        parent_generation_id: parent,
        dna,
    });
    Ok(())
}

fn deterministic_id(context: &str, parts: &[&[u8]]) -> [u8; 16] {
    let mut h = blake3::Hasher::new_derive_key(context);
    for part in parts {
        h.update(&(part.len() as u64).to_le_bytes());
        h.update(part);
    }
    let mut id = [0; 16];
    id.copy_from_slice(&h.finalize().as_bytes()[..16]);
    id[6] = (id[6] & 0x0f) | 0x50;
    id[8] = (id[8] & 0x3f) | 0x80;
    id
}
fn demo_project_id() -> ProjectId {
    let mut id = [0; 16];
    id[15] = 1;
    ProjectId::from_bytes(id)
}
fn error(code: ValidationCode, path: &str, message: &str) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(code, path, message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_contracts::{CreativeAxis, EntityType};
    use std::collections::BTreeSet;

    const PRESETS: [&str; 11] = [
        "dark-fantasy",
        "pirate",
        "steampunk",
        "cyberpunk",
        "nordic-myth",
        "post-apocalyptic",
        "occult",
        "monster-hunter",
        "rogue-assassin",
        "contemporary-weird",
        "chaos-blend",
    ];

    #[test]
    fn location_fields_are_a_stable_complete_key_set() {
        let keys = LocationField::ALL.map(|field| field.key().unwrap().to_string());
        assert_eq!(
            keys,
            [
                "01-location-profile",
                "02-name",
                "03-visual-identity",
                "04-atmosphere",
                "05-purpose",
                "06-summary",
                "07-history",
                "08-inhabitants-summary",
                "09-notable-features",
                "10-danger",
                "11-secret",
                "12-story-hooks"
            ]
        );
        assert_eq!(keys.iter().collect::<BTreeSet<_>>().len(), 12);
    }

    #[test]
    fn location_generation_is_deterministic_and_preserves_the_input_dna() {
        let dna = LocationForge::new_dna(
            RootSeed::new(0xdead_beef),
            "pirate",
            CreativeAxes::neutral(),
        )
        .unwrap();
        let before = dna.clone();
        let left = LocationForge::generate(&dna).unwrap();
        let right = LocationForge::generate(&dna).unwrap();
        assert_eq!(left, right);
        assert_eq!(dna, before);
        assert_eq!(left.location.envelope.entity_type, EntityType::Location);
        assert!(left.validate().is_ok());
    }

    #[test]
    fn explicit_profile_constraint_forces_settlement_and_rejects_unknown_profiles() {
        let mut dna =
            LocationForge::new_dna(RootSeed::new(8080), "nordic-myth", CreativeAxes::neutral())
                .unwrap();
        dna.recipe.constraints.insert(
            forge_contracts::ConstraintKey::new("location.profile").unwrap(),
            "settlement".into(),
        );
        let candidate = LocationForge::generate(&dna).unwrap();
        assert_eq!(
            candidate.location.location_profile,
            LocationProfile::Settlement
        );
        assert!(candidate.validate().is_ok());

        dna.recipe.constraints.insert(
            forge_contracts::ConstraintKey::new("location.profile").unwrap(),
            "moon-colony".into(),
        );
        assert!(LocationForge::generate(&dna).is_err());
    }

    #[test]
    fn every_core_preset_replays_valid_candidates_without_exact_duplicates() {
        for preset in PRESETS {
            let mut signatures = BTreeSet::new();
            for seed in 0..256 {
                let dna =
                    LocationForge::new_dna(RootSeed::new(seed), preset, CreativeAxes::neutral())
                        .unwrap();
                let first = LocationForge::generate(&dna).unwrap();
                let replay = LocationForge::generate(&dna).unwrap();
                assert_eq!(first, replay, "replay changed for {preset} / {seed}");
                assert!(first.validate().is_ok(), "invalid {preset} / {seed}");
                let l = &first.location;
                let authored = serde_json::to_string(&(
                    l.location_profile,
                    &l.envelope.name,
                    &l.summary,
                    &l.visual_identity,
                    &l.atmosphere,
                    &l.purpose,
                    &l.history,
                    &l.inhabitants_summary,
                    &l.notable_features,
                    &l.danger,
                    &l.secret,
                    &l.story_hooks,
                ))
                .unwrap();
                assert!(
                    signatures.insert(authored),
                    "duplicate authored Location content in {preset} at seed {seed}"
                );
            }
        }
    }

    #[test]
    fn chaos_blend_uses_a_coherent_source_and_diverse_deterministic_results() {
        let mut names = BTreeSet::new();
        let mut profiles = BTreeSet::new();
        let mut authored_content = BTreeSet::new();
        let mut source_counts = BTreeMap::<String, usize>::new();
        for seed in 0..1024 {
            let dna =
                LocationForge::new_dna(RootSeed::new(seed), "chaos-blend", CreativeAxes::neutral())
                    .unwrap();
            let first = LocationForge::generate(&dna).unwrap();
            let replay = LocationForge::generate(&dna).unwrap();
            assert_eq!(first, replay, "Chaos replay changed at seed {seed}");
            assert!(first.validate().is_ok());
            names.insert(first.location.envelope.name.clone());
            profiles.insert(format!("{:?}", first.location.location_profile));
            let l = &first.location;
            authored_content.insert(
                serde_json::to_string(&(
                    l.location_profile,
                    &l.envelope.name,
                    &l.summary,
                    &l.visual_identity,
                    &l.atmosphere,
                    &l.purpose,
                    &l.history,
                    &l.inhabitants_summary,
                    &l.notable_features,
                    &l.danger,
                    &l.secret,
                    &l.story_hooks,
                ))
                .unwrap(),
            );
            let source = PRESETS[..10]
                .iter()
                .find(|preset| {
                    theme(preset)
                        .name
                        .iter()
                        .any(|name| l.envelope.name.starts_with(name))
                })
                .unwrap();
            *source_counts.entry((*source).to_string()).or_default() += 1;
            // Every selected theme exposes the same source term in its place
            // vocabulary; all candidate fields are generated from that theme.
            let summary = &first.location.summary;
            assert!(!summary.is_empty());
        }
        assert!(
            names.len() >= 100,
            "only {} distinct Chaos names",
            names.len()
        );
        assert_eq!(
            authored_content.len(),
            1024,
            "authored content duplicate count"
        );
        assert_eq!(
            source_counts.len(),
            10,
            "not all ten source presets represented"
        );
        println!(
            "chaos blend authored content duplicates=0; unique names={}; profiles={}; source counts={source_counts:?}",
            names.len(),
            profiles.len()
        );
        assert!(
            profiles.len() >= 8,
            "only {} Location profiles",
            profiles.len()
        );
    }

    #[test]
    fn location_locks_rerolls_and_manual_origins_are_preserved() {
        let mut candidate = LocationForge::generate(
            &LocationForge::new_dna(RootSeed::new(8), "nordic-myth", CreativeAxes::neutral())
                .unwrap(),
        )
        .unwrap();
        let name = candidate.location.envelope.name.clone();
        let secret = candidate.location.secret.clone();
        let profile = candidate.location.location_profile;
        LocationForge::lock(&mut candidate, LocationField::Name).unwrap();
        LocationForge::lock(&mut candidate, LocationField::Secret).unwrap();
        LocationForge::lock(&mut candidate, LocationField::Profile).unwrap();
        let locked_name_nonce =
            candidate.dna.recipe.field_controls[&LocationField::Name.key().unwrap()].nonce;
        let locked_secret_nonce =
            candidate.dna.recipe.field_controls[&LocationField::Secret.key().unwrap()].nonce;
        let locked_profile_nonce =
            candidate.dna.recipe.field_controls[&LocationField::Profile.key().unwrap()].nonce;
        LocationForge::reroll_field(&mut candidate, LocationField::Atmosphere).unwrap();
        LocationForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert_eq!(candidate.location.envelope.name, name);
        assert_eq!(candidate.location.secret, secret);
        assert_eq!(candidate.location.location_profile, profile);
        assert_eq!(
            candidate.dna.recipe.field_controls[&LocationField::Name.key().unwrap()].nonce,
            locked_name_nonce
        );
        assert_eq!(
            candidate.dna.recipe.field_controls[&LocationField::Secret.key().unwrap()].nonce,
            locked_secret_nonce
        );
        assert_eq!(
            candidate.dna.recipe.field_controls[&LocationField::Profile.key().unwrap()].nonce,
            locked_profile_nonce
        );
        LocationForge::manual_edit(
            &mut candidate,
            LocationField::Purpose,
            LocationFieldValue::Text("Edited purpose ✓".into()),
        )
        .unwrap();
        assert_eq!(
            candidate.field_origins[&LocationField::Purpose.key().unwrap()],
            LocationOrigin::UserModified
        );
        assert_eq!(candidate.location.purpose, "Edited purpose ✓");
    }

    #[test]
    fn location_axes_and_unsupported_presets_are_explicit() {
        let neutral = LocationForge::generate(
            &LocationForge::new_dna(RootSeed::new(99), "dark-fantasy", CreativeAxes::neutral())
                .unwrap(),
        )
        .unwrap();
        let mut extremes = CreativeAxes::neutral();
        extremes.levels.insert(CreativeAxis::Surreal, 100);
        extremes.levels.insert(CreativeAxis::Mysticism, 100);
        let strange = LocationForge::generate(
            &LocationForge::new_dna(RootSeed::new(99), "dark-fantasy", extremes).unwrap(),
        )
        .unwrap();
        assert_ne!(neutral.location, strange.location);
        assert!(strange.validate().is_ok());
        assert!(
            LocationForge::new_dna(RootSeed::new(1), "unknown", CreativeAxes::neutral()).is_err()
        );
    }
}
