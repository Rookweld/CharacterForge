//! Authored, preset-specific material used by the standalone Faction Forge.

pub(crate) struct FactionTheme {
    pub names: [&'static str; 6],
    pub identities: [&'static str; 6],
    pub ideologies: [&'static str; 6],
    pub goals: [&'static str; 6],
    pub methods: [&'static str; 6],
    pub cultures: [&'static str; 6],
    pub resources: [&'static str; 6],
    pub public_faces: [&'static str; 6],
    pub hidden_truths: [&'static str; 6],
    pub symbols: [&'static str; 6],
    pub reputations: [&'static str; 6],
    pub hooks: [&'static str; 6],
}

pub(crate) fn theme(preset: &str) -> Option<&'static FactionTheme> {
    Some(match preset {
        "dark-fantasy" => &DARK,
        "pirate" => &PIRATE,
        "steampunk" => &STEAMPUNK,
        "cyberpunk" => &CYBERPUNK,
        "nordic-myth" => &NORDIC,
        "post-apocalyptic" => &POST_APOCALYPTIC,
        "occult" => &OCCULT,
        "monster-hunter" => &MONSTER_HUNTER,
        "rogue-assassin" => &ROGUE_ASSASSIN,
        "contemporary-weird" => &CONTEMPORARY_WEIRD,
        _ => return None,
    })
}

macro_rules! theme {
    ($names:expr, $identities:expr, $ideologies:expr, $goals:expr, $methods:expr,
     $cultures:expr, $resources:expr, $faces:expr, $truths:expr, $symbols:expr,
     $reputations:expr, $hooks:expr) => {
        FactionTheme {
            names: $names,
            identities: $identities,
            ideologies: $ideologies,
            goals: $goals,
            methods: $methods,
            cultures: $cultures,
            resources: $resources,
            public_faces: $faces,
            hidden_truths: $truths,
            symbols: $symbols,
            reputations: $reputations,
            hooks: $hooks,
        }
    };
}

static DARK: FactionTheme = theme!(
    [
        "Ashen Concord",
        "Gloamwardens",
        "Cinder Synod",
        "Mourners' Compact",
        "Black Lantern Court",
        "Thornwake Circle"
    ],
    [
        "wardens who keep roads open after the dead bell",
        "undertakers who bargain for safe passage",
        "judges who settle debts owed to old spirits",
        "healers preserving villages through the long dusk",
        "pilgrims mapping places erased from memory",
        "craftsfolk binding restless relics"
    ],
    [
        "the living owe care to the dead they inherit",
        "mercy without witness becomes another kind of cruelty",
        "a promise matters most when its keeper is gone",
        "fear should warn a community, never govern it",
        "forgotten names deserve a place in every record",
        "power is safest when shared with those it protects"
    ],
    [
        "reopen the buried road before winter closes the valley",
        "restore the names removed from the parish rolls",
        "free a village from its inherited blood tithe",
        "keep the last hospice supplied through the siege",
        "find who is ringing the bell beneath the lake",
        "return a stolen relic before its oath wakes"
    ],
    [
        "escort small convoys by lantern code",
        "trade recorded testimony for safe conduct",
        "hold public hearings at the cemetery gate",
        "leave food and medicine at marked thresholds",
        "copy forbidden maps into ordinary ledgers",
        "bind relics with witnessed vows"
    ],
    [
        "sharing a meal with the newest bereaved family",
        "keeping a spoken roll of every local name",
        "leaving one chair empty at winter council",
        "teaching children the warning bells by rhythm",
        "repairing neighbors' doors before their own",
        "carrying a light for anyone travelling alone"
    ],
    [
        "burial ledgers, hospice stores, and trusted road guides",
        "graveyard keys, ferrymen, and copied parish records",
        "witnesses, bell towers, and a network of healers",
        "winter grain, apothecaries, and shelter houses",
        "maps, safe lamps, and the trust of isolated hamlets",
        "sealed reliquaries and trained oath-keepers"
    ],
    [
        "a dusk relief society",
        "a cemetery mutual-aid guild",
        "a village court for unsettled inheritances",
        "a hospice and winter pantry",
        "a historical survey of abandoned roads",
        "a conservators' workshop for old objects"
    ],
    [
        "its first founder still answers petitions from an unmarked grave",
        "the safest route they publish leads past a sealed mass tomb",
        "the founder's judgment was built on a testimony they altered",
        "their stores include medicine reserved for a rival village",
        "one erased name belongs to the current head",
        "the relics are containing a debt the group cannot repay"
    ],
    [
        "a black lantern crossed by a white thread",
        "a small key tied to a funeral bell",
        "a scale balanced with a candle stub",
        "a stitched red heart beneath a grey cloak",
        "a road winding through an open eye",
        "a thorn wrapped around a bone flute"
    ],
    [
        "keeps a difficult promise when no reward remains",
        "takes the dead seriously and the powerful carefully",
        "hears both sides before naming a culprit",
        "arrives with practical help rather than sermons",
        "can find a path where official maps end",
        "handles dangerous heirlooms without boasting"
    ],
    [
        "A bell rings at a house whose dead were never entered in the ledger.",
        "A sealed road permit bears the signature of someone buried last spring.",
        "Two villages claim the same hospice stores were promised to them.",
        "The winter pantry is full, but every jar carries a stranger's name.",
        "A copied map restores a street that the town insists never existed.",
        "A reliquary has begun answering questions no keeper asked."
    ]
);

static PIRATE: FactionTheme = theme!(
    [
        "Tidemark Compact",
        "Nine Wake Union",
        "Saltglass Company",
        "Free Keel Assembly",
        "Black Current League",
        "Lantern Reef Syndicate"
    ],
    [
        "pilots who guide ships through reefs absent from charts",
        "dock stewards who arbitrate claims between crews",
        "salvagers who recover cargo before the navy arrives",
        "island councils coordinating warning lights",
        "smugglers moving medicine past a blockade",
        "divers mapping wrecks for their next owners"
    ],
    [
        "a crew earns only what it can answer for",
        "the sea makes borders temporary and debts memorable",
        "a fair share keeps a rough bargain honest",
        "no harbor should be ruled by a distant flag",
        "rescue at sea creates an obligation, not ownership",
        "a chart is a promise to everyone who follows it"
    ],
    [
        "open a storm channel to the winter islands",
        "buy captives' passage out of a private prison",
        "recover the treaty chest lost at Knife Shoal",
        "keep three harbors supplied through the blockade",
        "expose who is selling convoy routes to raiders",
        "secure a neutral anchorage for rival crews"
    ],
    [
        "exchange tide tables for a share of salvage",
        "settle disputes before ships leave port",
        "use decoy lanterns to divert patrols",
        "rotate watch crews between independent islands",
        "hide medicine inside ordinary cargo manifests",
        "send divers to mark hazards before a race"
    ],
    [
        "sharing the first catch with a storm-bereaved crew",
        "letting every deckhand speak at the tally",
        "naming a rescued ship in the harbor roll",
        "teaching signals before anyone handles a weapon",
        "returning a borrowed tool before taking a share",
        "keeping one dry berth open for strangers"
    ],
    [
        "pilot charts, dry docks, and reliable harbor beacons",
        "salvage divers, rope makers, and neutral notaries",
        "spare sails, coded manifests, and hidden coves",
        "island signal towers and crews who know each reef",
        "medicine, false cargo seals, and quiet boatyards",
        "wreck maps, diving bells, and tide readers"
    ],
    [
        "a pilots' association",
        "a harbor claims office",
        "a licensed salvage company",
        "an island warning network",
        "a coastal medicine courier",
        "a wreck survey society"
    ],
    [
        "its best chart omits a safe inlet used by people it claims to protect",
        "the current share ledger counts one missing crew as still aboard",
        "the company has already sold the recovered treaty chest twice",
        "a signal tower is dark because its keeper shelters a fugitive",
        "the blockade medicine is partly funded by a navy quartermaster",
        "the map of the newest wreck was drawn before the storm"
    ],
    [
        "a red pennant stitched with a silver tide line",
        "nine notches cut into a brass compass",
        "a salt-white glass bead on a black cord",
        "a free anchor with its chain broken",
        "a dark wave under a starless sky",
        "a lantern set inside a ring of coral"
    ],
    [
        "shares salvage by an agreed rule",
        "can calm a dockside argument before fists land",
        "finds useful cargo where others see wreckage",
        "keeps island warnings timely",
        "gets urgent supplies through closed waters",
        "knows which wrecks are safe to enter"
    ],
    [
        "A harbor beacon repeats a signal from a ship declared lost years ago.",
        "Two crews produce original copies of the same salvage agreement.",
        "The treaty chest is offered for auction before the wreck is found.",
        "An island light answers a code no living keeper admits knowing.",
        "A medicine run reaches port with one extra passenger and no log entry.",
        "A fresh wreck map marks a cabin that does not appear on the ship's plans."
    ]
);

static STEAMPUNK: FactionTheme = theme!(
    [
        "Brass Meridian",
        "The Rivet Parliament",
        "Aetherwright Union",
        "Copper Ledger Society",
        "Clockwork Ward",
        "The Sootline Trust"
    ],
    [
        "engineers maintaining public boilers across the lower wards",
        "auditors tracing ownership through clockwork patents",
        "operators coordinating airship moorings and repair crews",
        "craft guilds sharing power from a river turbine",
        "inspectors testing machines before city demonstrations",
        "couriers carrying sealed parts between rival workshops"
    ],
    [
        "invention is a public obligation when it changes public life",
        "a measured result outranks the inventor's title",
        "maintenance deserves the same honor as discovery",
        "shared infrastructure must answer to its users",
        "a patent cannot erase the hands that built a device",
        "efficiency is hollow if its costs are hidden"
    ],
    [
        "repair the flood pumps before the lower wards sink",
        "publish the true operating cost of the sky tram",
        "recover the plans for a pressure-safe furnace",
        "win workers a vote on boiler allocation",
        "find who is diverting current from the hospital lifts",
        "open a training shop for apprentices without family marks"
    ],
    [
        "run witnessed stress tests in public yards",
        "trade repair hours for shared patent rights",
        "stamp every part with its workshop origin",
        "schedule power by posted neighborhood rotation",
        "test prototypes with volunteer crews",
        "send paired couriers so no component vanishes alone"
    ],
    [
        "ending every workshop shift with a tool count",
        "letting apprentices present one failed test each week",
        "sharing tea while the night boiler cools",
        "marking repairs with the initials of every hand",
        "holding open benches for neighborhood fixes",
        "reading machine warnings aloud before a start"
    ],
    [
        "pressure gauges, repair crews, and contracts with foundries",
        "patent ledgers, brass stock, and expert examiners",
        "airship riggers, lift access, and aether coils",
        "turbine rights, apprentices, and public workshops",
        "test chambers, inspectors, and calibrated instruments",
        "fast couriers, coded part stamps, and spare mechanisms"
    ],
    [
        "a public boiler-maintenance cooperative",
        "an independent patent review board",
        "an airship repair and mooring union",
        "a neighborhood power-sharing council",
        "a machine safety inspectorate",
        "a specialist parts courier guild"
    ],
    [
        "one celebrated test result was achieved with a concealed pressure leak",
        "the most disputed patent began as a worker's discarded sketch",
        "their repair crews have been quietly replacing unsafe company engines",
        "the posted rotation reserves extra power for a hidden clinic",
        "the inspectorate knows a famous machine cannot pass its own test",
        "one courier route carries letters between two supposedly hostile directors"
    ],
    [
        "a brass dial with one hand deliberately missing",
        "a rivet set through an open ledger",
        "a silver propeller over a black gear",
        "three copper pipes joining one turbine",
        "a gauge needle held just below the red",
        "a soot-dark envelope sealed with a steel pin"
    ],
    [
        "repairs what others abandon",
        "can explain a patent without hiding its cost",
        "keeps crews working through a long outage",
        "shares a scarce resource by a visible rule",
        "finds a flaw before a machine reaches the crowd",
        "delivers a critical component on time"
    ],
    [
        "The flood pumps fail only when their original makers inspect them.",
        "A patent ledger lists an apprentice as owner under a name erased from payroll.",
        "An airship's repair log includes a landing from tomorrow's date.",
        "The hospital lift has been drawing power from a private laboratory.",
        "A failed safety test was filed as a successful public demonstration.",
        "A courier returns with a component that has already been installed."
    ]
);

static CYBERPUNK: FactionTheme = theme!(
    [
        "Ghostline Cooperative",
        "Glass Circuit",
        "Night Index",
        "The Patchwork Exchange",
        "Proxy Assembly",
        "Lowlight Syndicate"
    ],
    [
        "repairers keeping neighborhood networks alive during corporate outages",
        "privacy advocates auditing biometric access systems",
        "data brokers selling copies back to people they were taken from",
        "clinic collectives sharing clean power and cold storage",
        "couriers moving identities beyond automated checkpoints",
        "street technicians removing unsafe implants"
    ],
    [
        "a person is more than the record that can be sold about them",
        "access without consent is another form of seizure",
        "infrastructure belongs to the people who keep it running",
        "a copy is not permission to erase its source",
        "survival should not depend on a perfect credit trail",
        "security that cannot be questioned is only control"
    ],
    [
        "restore the clinic mesh before the next blackout",
        "return a neighborhood's stolen biometric archive",
        "prove a transit gate is rejecting valid residents",
        "get debt-bound workers unlisted before a sweep",
        "publish who owns the district's utility relays",
        "build a clean route around a predictive policing net"
    ],
    [
        "route messages through volunteer-owned relays",
        "verify claims with two independent witnesses",
        "trade repair access for consent-based data deletion",
        "move people through decoy maintenance windows",
        "patch systems in place without exporting user records",
        "publish an audit trail that hides its subjects"
    ],
    [
        "sharing battery charge before a night shift",
        "asking before anyone scans a borrowed device",
        "teaching one repair skill at every meal",
        "keeping a paper contact tree for network failures",
        "letting affected residents veto a data release",
        "using chosen names in the public work log"
    ],
    [
        "relay access, refurbished hardware, and trusted clinic rooms",
        "forensic auditors, legal copies, and privacy-safe storage",
        "mirrored ledgers, decoy identities, and courier routes",
        "medical cold-chain cells and neighborhood generators",
        "forged transit windows and low-profile safe rooms",
        "repair kits, firmware keys, and volunteer technicians"
    ],
    [
        "a community network repair shop",
        "a civil-rights data auditor",
        "a cooperative neighborhood market",
        "a volunteer clinic and charging hub",
        "a travel-document assistance desk",
        "a device safety and repair collective"
    ],
    [
        "its cleanest data cache contains records the group promised to erase",
        "one audit was delayed to protect the relative of its founder",
        "a copied identity has been making choices without its source's knowledge",
        "the clinic's backup generator belongs to the corporation it opposes",
        "a safe route is also used by the city's predictive system",
        "some implants were removed without the owners' informed consent"
    ],
    [
        "a broken ring around a bright green node",
        "a glass eye with its serial number scratched away",
        "a paper index card wired to a circuit",
        "two mismatched patches joined by copper thread",
        "an unmarked proxy mask with one lit seam",
        "a dark screen showing a single streetlamp"
    ],
    [
        "fixes the network without keeping a copy of its users",
        "can show who accessed a record and when",
        "finds a route through hostile systems",
        "keeps medicine cold during a district outage",
        "moves people without exposing their old identities",
        "spots a dangerous modification before it causes harm"
    ],
    [
        "A deleted record appears in a public display under a living person's name.",
        "The transit net begins opening gates for people the cooperative never enrolled.",
        "A clinic generator has been sending a silent usage report to its owner.",
        "A paper contact list contains one future date beside every volunteer.",
        "An erased biometric archive is being used to deny housing.",
        "A repaired implant speaks a warning in the voice of its first technician."
    ]
);

static NORDIC: FactionTheme = theme!(
    [
        "Raven Moot",
        "Frostwake Hearths",
        "The Oathwood Ring",
        "Skeld River Compact",
        "Northstar Thing",
        "The Longhouse Ward"
    ],
    [
        "winter stewards coordinating grain and shelter between fjord villages",
        "oath-keepers mediating feuds before the spring assembly",
        "ferrymen maintaining crossings when rivers break their banks",
        "story-keepers preserving claims through generations of migration",
        "rangers guiding travelers across marked snowfields",
        "craft households rebuilding halls after raids"
    ],
    [
        "an oath is measured by what it costs its keeper",
        "no household should winter alone while neighbors have stores",
        "the dead deserve a truthful account from the living",
        "a leader owes the assembly an answer before a command",
        "hospitality is strongest when it has clear terms",
        "land and river use must be renewed by those who depend on them"
    ],
    [
        "settle the old boundary before thaw sends both herds north",
        "build a common storehouse above the flood line",
        "find who altered the winter oath record",
        "keep a ferry running between feuding shores",
        "return a stolen bell before the moot gathers",
        "bring a missing household home before the first hard frost"
    ],
    [
        "ask both hearths to name a witness",
        "keep rival crews on a shared ferry roster",
        "mark storehouse withdrawals in a public tally",
        "send two guides on each winter crossing",
        "carry disputes to the seasonal assembly",
        "repair a neighboring hall before claiming payment"
    ],
    [
        "reciting the winter stores before a shared meal",
        "offering the guest seat to the newest arrival",
        "settling tools beside the hearth before sleeping",
        "teaching children both sides of an old feud",
        "keeping promises in a song anyone may correct",
        "leaving a warm room ready for late travelers"
    ],
    [
        "grain stores, ferry rights, and guides who know the pass",
        "witness lists, moot halls, and negotiators trusted by both sides",
        "boats, rope crews, and marked river crossings",
        "oral histories, carved tallies, and elder witnesses",
        "shelters, snow markers, and trained rangers",
        "timber, smiths, and households skilled in rebuilding"
    ],
    [
        "a winter aid network",
        "an oath-mediation assembly",
        "a river crossing cooperative",
        "a public archive of migration and claims",
        "a northern trail service",
        "a shared longhouse rebuilding guild"
    ],
    [
        "the common storehouse was built from timber taken without the river village's leave",
        "one witness has been protecting both families by withholding a name",
        "the ferry roster includes a crossing everyone denies making",
        "the oldest song was rewritten to hide who first broke the oath",
        "the trail markers lead around a shelter that is occupied",
        "the hall's rebuilding fund is being kept for a village that no longer exists"
    ],
    [
        "a raven feather tied above a grain measure",
        "a blue thread knotted around a ring of iron",
        "a river stone split by a carved line",
        "a star over a tally stick",
        "two antlers enclosing a warm hearth",
        "a handprint cut into a longhouse beam"
    ],
    [
        "keeps winter stores visible to every household",
        "can bring old rivals to one fire",
        "gets travelers across a dangerous river",
        "remembers the people missing from official claims",
        "knows a safe trail after the markers vanish",
        "rebuilds before asking who deserves blame"
    ],
    [
        "A winter tally shows one extra household drawing grain each night.",
        "The ferry rope is cut, but both shores claim they were waiting for it.",
        "A moot witness remembers an oath no surviving family recalls making.",
        "Fresh tracks circle a shelter sealed since the last migration.",
        "A stolen bell rings from beneath the thawing river ice.",
        "The rebuilding crew finds a name carved into a beam dated next year."
    ]
);

static POST_APOCALYPTIC: FactionTheme = theme!(
    [
        "Wellmark Union",
        "The Green Mile Commons",
        "Tin Roof Assembly",
        "Ashline Caravans",
        "Reservoir Keepers",
        "The Seed Vault Circle"
    ],
    [
        "well crews rationing safe water between roadside settlements",
        "growers restoring soil around a contaminated rail spur",
        "roof builders moving families above the seasonal flood",
        "caravan guards keeping trade routes open between shelters",
        "reservoir engineers testing old filtration works",
        "seed librarians sharing crops adapted to poor ground"
    ],
    [
        "survival knowledge should outlast whoever first discovered it",
        "a scarce resource is safest under a rule everyone can inspect",
        "repair is a form of stewardship",
        "no settlement grows secure by making its neighbor desperate",
        "useful land should be returned to common hands",
        "a promise made during crisis still binds after it"
    ],
    [
        "find a clean route to the northern water table",
        "restore the soil beds before the next planting window",
        "move the flood families without surrendering their tools",
        "open the bridge market to smaller settlements",
        "test the old filters before the reservoir empties",
        "recover the seed ledger from a collapsed station"
    ],
    [
        "publish each water measure on a marked board",
        "rotate soil crews between new plots",
        "build modular roofs from salvaged panels",
        "escort caravans in exchange for shared route notes",
        "test one filter chamber before opening the whole system",
        "copy seed instructions onto durable cloth"
    ],
    [
        "testing a shared pump before filling private cans",
        "saving the first harvest seed before eating",
        "repairing a stranger's shelter during the dry shift",
        "teaching children how to read a contamination mark",
        "keeping a route note beside every trade",
        "leaving one seat open for a passing caravan"
    ],
    [
        "well maps, hand pumps, and trained water testers",
        "seed stores, soil beds, and careful growers",
        "salvage panels, scaffold crews, and flood boats",
        "route notes, spare tires, and caravan scouts",
        "filter parts, test strips, and reservoir records",
        "sealed seeds, weather logs, and copied growing guides"
    ],
    [
        "a cooperative water board",
        "a community soil-restoration crew",
        "a flood shelter and repair association",
        "a safe-route caravan exchange",
        "an independent water testing service",
        "a public seed library"
    ],
    [
        "one well marked clean is being reserved for a settlement blamed for the drought",
        "the seed vault's best crops are sterile without a missing pollinator",
        "a roof plan was changed to keep one family out of the dry tier",
        "a caravan route is safe because someone keeps diverting raiders",
        "the reservoir tests show a problem the keepers caused during repairs",
        "the copied ledger leaves out the seeds that grow fastest"
    ],
    [
        "three blue drops above a shared measuring cup",
        "a green shoot pushing through rail ash",
        "a tin roof crossed by a flood mark",
        "a pale road winding between two cans",
        "a black filter with one silver thread",
        "a seed held inside a small glass key"
    ],
    [
        "shares water by a rule people can verify",
        "can coax growth from damaged ground",
        "builds shelter with little waste",
        "finds a passable route between settlements",
        "catches contamination before people drink",
        "keeps useful seed knowledge from disappearing"
    ],
    [
        "A clean-water mark appears on a well no one has tested.",
        "The seed ledger describes a crop that has not yet been planted.",
        "A roof collapses upward, leaving a dry room beneath it.",
        "The caravan route ends at a settlement missing from every map.",
        "A reservoir test strip changes color after it has been sealed away.",
        "A new shoot grows through a sealed vault door."
    ]
);

static OCCULT: FactionTheme = theme!(
    [
        "The Quiet Meridian",
        "Ninefold Vigil",
        "The Wax Registry",
        "Candle and Key",
        "The Veiled Chapter",
        "Mothglass Society"
    ],
    [
        "archivists cataloguing rites without performing them",
        "night watchers identifying patterns before they become incidents",
        "notaries witnessing agreements with unusual terms",
        "keepers escorting cursed objects to neutral storage",
        "researchers comparing folklore across unrelated towns",
        "counselors helping witnesses distinguish memory from suggestion"
    ],
    [
        "curiosity needs limits that everyone can state aloud",
        "a mystery may be studied without being invited in",
        "testimony deserves care even when it sounds impossible",
        "secrecy can shelter a person but also shelter abuse",
        "names and objects should not be used without consent",
        "a warning is only useful when someone can act on it"
    ],
    [
        "identify who is leaving the same mark in distant archives",
        "move a sealed object before its storage room is demolished",
        "find a witness who vanished from three independent records",
        "stop a private club from repeating an untested rite",
        "publish a safe procedure for entering the old observatory",
        "learn why the same voice appears on disconnected tapes"
    ],
    [
        "compare accounts before naming a cause",
        "store dangerous artifacts under two-person custody",
        "record every research question before opening a file",
        "use a plain exit phrase during interviews",
        "test one mundane explanation before a ritual one",
        "offer witnesses the right to withdraw their account"
    ],
    [
        "opening a meeting by stating how anyone may leave",
        "labeling every artifact with its custody history",
        "sharing a quiet meal after difficult interviews",
        "asking permission before recording a witness",
        "keeping the research room well lit",
        "writing down uncertainty beside every conclusion"
    ],
    [
        "locked archives, trained interviewers, and secure transport cases",
        "night staff, incident maps, and trusted local contacts",
        "witness seals, duplicate ledgers, and independent notaries",
        "neutral storage rooms and object handlers",
        "folklore collections and comparative researchers",
        "counselors, audio logs, and safe interview spaces"
    ],
    [
        "a folklore research library",
        "a night incident observatory",
        "a specialist witnessing office",
        "a neutral artifact repository",
        "a comparative history institute",
        "a witness support and counseling service"
    ],
    [
        "the catalog includes an entry written in the handwriting of its reader",
        "a pattern map was altered to protect the researcher who started it",
        "one witness seal was placed before the agreement was spoken",
        "the sealed object is a decoy for a missing person",
        "the observatory has been receiving replies to tests it never ran",
        "a counselor recognizes a voice from a childhood recording"
    ],
    [
        "a brass compass with its north point covered",
        "nine small candles surrounding an empty dish",
        "a wax seal impressed with an open eye",
        "a key set beside a candle that casts no shadow",
        "a veil pinned with a plain iron clasp",
        "a moth wing pressed beneath glass"
    ],
    [
        "keeps a clear record of what is known and unknown",
        "notices a pattern before it becomes a panic",
        "can witness an agreement without taking sides",
        "moves a dangerous object without spectacle",
        "finds common details in conflicting accounts",
        "listens without turning a witness into a story"
    ],
    [
        "An archive entry appears under the name of its next visitor.",
        "A night map shows a moving light above an empty road.",
        "Two copies of one agreement have different witness marks.",
        "The repository's sealed room is open from the inside.",
        "An old tape contains a question the interviewer has not yet asked.",
        "A witness remembers leaving through a door that is absent from the building."
    ]
);

static MONSTER_HUNTER: FactionTheme = theme!(
    [
        "Silver Track Lodge",
        "The Lantern Hunt",
        "Marrowfield Wardens",
        "The Hollow Bell Company",
        "Ashhide Compact",
        "The Last Trail Office"
    ],
    [
        "trackers mapping migration routes before a settlement expands",
        "wardens training villages to recognize signs of a nearby beast",
        "healers treating hunters and animals injured by the same traps",
        "guides keeping roads open around nesting grounds",
        "craftsfolk testing equipment against unusual hides",
        "rangers documenting creatures before anyone declares them hostile"
    ],
    [
        "knowledge should make people safer before it makes them powerful",
        "a creature's behavior matters more than a rumor about its kind",
        "protecting a settlement includes protecting the ground around it",
        "a hunt is justified by a real threat and a clear end",
        "evidence deserves a place beside tradition",
        "the first duty is to prevent avoidable harm"
    ],
    [
        "learn why the herd has changed its route toward town",
        "stop a trapper selling poisoned bait to frightened villages",
        "clear the mountain pass without disturbing the nesting cliff",
        "find who is imitating the warning call",
        "protect a healer carrying the only antidote notes",
        "record what lives inside the abandoned bell tower"
    ],
    [
        "compare tracks with witness accounts before a hunt",
        "post safe-distance marks along the forest edge",
        "treat injured animals when doing so does not endanger people",
        "use tagged decoys instead of live bait",
        "pair new trackers with local naturalists",
        "record each tool's failure as carefully as its success"
    ],
    [
        "cleaning borrowed gear before returning it",
        "telling the whole account after every outing",
        "leaving water at a known animal crossing",
        "teaching children which tracks mean give way",
        "sharing a quiet meal with local healers",
        "retiring a tool after it fails a safety check"
    ],
    [
        "track maps, field medics, and tested protective gear",
        "warning lamps, trained village observers, and safe shelters",
        "antidote notes, clean bandages, and patient animal handlers",
        "guides, marked trails, and agreements with local stewards",
        "hide samples, tool benches, and careful craft records",
        "field journals, old specimens, and naturalist contacts"
    ],
    [
        "a field tracking and ecology lodge",
        "a village warning and training service",
        "a joint animal and human field clinic",
        "a safe-pass guide association",
        "a specialist protective-gear workshop",
        "a creature observation archive"
    ],
    [
        "the tracks they published were made by a decoy to draw hunters away",
        "one warning lamp is kept dark to protect a creature's den",
        "the clinic has been treating an animal blamed for a human illness",
        "the safe trail crosses land the guides promised to leave alone",
        "a sample labeled harmless has damaged every tool that touched it",
        "the bell tower record includes a creature sighting from tomorrow"
    ],
    [
        "a silver track enclosed by a lamp ring",
        "a green lantern beneath a curved claw",
        "a pale bone marker crossed by two trail lines",
        "a bell hung inside a hollow antler",
        "an ash-grey hide patch held by copper wire",
        "a boot print beside an open field journal"
    ],
    [
        "checks the tracks before choosing a target",
        "keeps villages prepared without promising easy safety",
        "treats field injuries with calm skill",
        "knows which pass remains safe after dark",
        "builds gear that works outside the workshop",
        "keeps records useful after a hunt ends"
    ],
    [
        "A warning lamp turns toward the forest with no wind to move it.",
        "A set of tracks ends at the village gate and continues on the roof.",
        "The bait seller's newest shipment bears a warden's mark.",
        "A familiar call comes from a trail closed for a decade.",
        "An animal arrives carrying a bandage tied in the lodge's knot.",
        "The bell tower journal records a hunt that every member remembers tomorrow."
    ]
);

static ROGUE_ASSASSIN: FactionTheme = theme!(
    [
        "The Sable Ledger",
        "Quiet Hand Exchange",
        "The Veiled Route",
        "Nine Ink Compact",
        "The Unmarked Door",
        "Black Thread Office"
    ],
    [
        "brokers matching dangerous clients with safer alternatives",
        "couriers moving witnesses before a contract can find them",
        "record-keepers tracing who ordered a political killing",
        "safehouse stewards coordinating discreet exits",
        "intermediaries settling disputes without naming a target",
        "investigators dismantling paid violence networks"
    ],
    [
        "a secret is not ownership of the person who carries it",
        "refusing an order can be the most valuable service",
        "violence creates obligations that payment cannot close",
        "silence protects the vulnerable only when it is chosen",
        "a witness is not a commodity",
        "an anonymous client still answers for the harm they commission"
    ],
    [
        "identify who is financing the attacks on three witnesses",
        "get a courier out before their false papers expire",
        "close the contract registry without exposing its informants",
        "find the safehouse leak before the next arrival",
        "prove who paid for the failed assassination",
        "offer a rival network a way to stand down"
    ],
    [
        "verify every contract through two separate channels",
        "move witnesses using routes chosen by the witnesses",
        "leave false leads that waste no innocent person's time",
        "keep sealed records under divided custody",
        "make refusal a recorded and final answer",
        "trace payment without publishing private addresses"
    ],
    [
        "asking a guest which door they prefer",
        "destroying expired route papers in witnessed batches",
        "leaving no name in the meal ledger",
        "teaching every courier how to refuse a parcel",
        "keeping a spare room ready without questions",
        "letting each witness choose their own code word"
    ],
    [
        "safe addresses, neutral couriers, and verified client records",
        "false travel papers, route guides, and trusted escorts",
        "sealed ledgers, financial traces, and patient investigators",
        "quiet rooms, transport funds, and discreet stewards",
        "witness statements, payment trails, and public archives",
        "mediation rooms, withdrawal channels, and independent witnesses"
    ],
    [
        "a confidential dispute mediation office",
        "a witness transport and document service",
        "a financial investigation bureau",
        "a discreet lodging cooperative",
        "a public inquiry into political violence",
        "a neutral contract review exchange"
    ],
    [
        "its most protected informant is the person who first exposed the office",
        "one safe address is being used to shelter a former client",
        "the ledger's blank page was removed by its original owner",
        "a room has been kept ready for someone who refuses to arrive",
        "the failed assassination was designed to create a public record",
        "the withdrawal channel has been used to warn an intended target"
    ],
    [
        "a blank ledger crossed by one sable thread",
        "an open hand behind a grey veil",
        "a route line ending before the border",
        "nine ink marks around an empty name",
        "a door outline with no handle",
        "a black thread tied in a knot that can be undone"
    ],
    [
        "can refuse a dangerous job without selling its witness",
        "moves a person quietly and with their consent",
        "finds who benefits from an attack",
        "keeps a guest safe without asking for their story",
        "makes hidden payments legible",
        "offers rivals an exit before more harm is done"
    ],
    [
        "A sealed contract lists a target who has not yet been born.",
        "The witness route ends at a safehouse no courier remembers.",
        "A payment ledger records a refusal as the largest sum.",
        "A guest leaves through a door that was bricked up years ago.",
        "The failed attack's only surviving witness is the person who ordered it.",
        "A blank page arrives with fresh ink on the other side."
    ]
);

static CONTEMPORARY_WEIRD: FactionTheme = theme!(
    [
        "Municipal Night Desk",
        "The Spare Key Society",
        "Third Shift Mutuals",
        "Quiet Hours Committee",
        "The Local Index",
        "Yellow Umbrella Circle"
    ],
    [
        "night clerks comparing reports about streets missing from city maps",
        "neighbors keeping spare keys for homes that move between blocks",
        "transit workers documenting passengers who board at closed stops",
        "librarians cataloguing books returned before they are borrowed",
        "volunteers answering calls from a disconnected switchboard",
        "building supers tracking elevators that stop on unlisted floors"
    ],
    [
        "an ordinary explanation deserves a fair test, not automatic trust",
        "a person should be believed about what they experienced",
        "public records should help neighbors find each other",
        "care is practical even when the problem is strange",
        "uncertainty is not a reason to abandon someone",
        "a map should leave room for a place that has not stayed put"
    ],
    [
        "find who is filing utility notices from the missing street",
        "return the spare key before its house changes address again",
        "learn why the closed platform appears on night shift tickets",
        "match the library's impossible return slips to a reader",
        "answer the disconnected switchboard before the line goes quiet",
        "find the elevator passenger who never reaches a listed floor"
    ],
    [
        "check paper records against what neighbors remember",
        "keep keys with a named and revocable steward",
        "ride the route with two workers and a clear return plan",
        "stamp unusual returns without inventing an author",
        "answer calls without promising who is on the other end",
        "record each floor stop before resetting the lift"
    ],
    [
        "keeping coffee ready for the last bus crew",
        "writing an address twice before sending anyone there",
        "leaving a note when borrowing a neighbor's key",
        "reading each unusual report aloud at the desk",
        "keeping the switchboard log open to callers",
        "taking the stairs together after an unlisted stop"
    ],
    [
        "night logs, transit passes, and clerks who know every route",
        "spare keys, address books, and patient neighbors",
        "shift rosters, closed-platform maps, and worker escorts",
        "return slips, archive shelves, and volunteer readers",
        "old telephone lines and someone who always answers",
        "lift schematics, maintenance keys, and building stewards"
    ],
    [
        "a neighborhood information and night assistance desk",
        "a key-holding mutual aid group",
        "a late-shift transit worker association",
        "a volunteer local history library",
        "a public call-answering service",
        "a building access maintenance committee"
    ],
    [
        "the missing street is still paying its water bill",
        "one spare key opens a house its holder has never seen",
        "a passenger has been on every closed-platform roster under a different name",
        "the returned book is still checked out to a living reader",
        "the switchboard caller is using the desk's own extension",
        "the elevator log records a stop below the building's foundations"
    ],
    [
        "a street sign with its final letter left blank",
        "two brass keys joined by a paper address",
        "a transit token stamped for a closed stop",
        "a library card with no borrower name",
        "a yellow telephone beside an open logbook",
        "an elevator arrow pointing between floors"
    ],
    [
        "knows how to find a person after their address changes",
        "returns a key without making its holder explain",
        "can get a night worker home safely",
        "keeps an impossible record without embellishing it",
        "answers a call when the listed number is dead",
        "keeps building access working despite strange stops"
    ],
    [
        "A utility bill arrives from a street scheduled for demolition yesterday.",
        "A spare key turns warm whenever its house is nearby.",
        "A closed platform appears on a train that has no driver.",
        "A returned library book contains tomorrow's checkout slip.",
        "The disconnected switchboard rings during the desk's own meeting.",
        "The elevator opens onto a floor visible only in the mirror."
    ]
);
