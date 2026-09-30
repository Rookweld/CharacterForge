# THE FORGE B5 — Creature sample review

**Generated/content-author review by Codex; not independent human playtesting or operator acceptance.**

## Reproduction and boundary

Run `cargo run -p forge-app-core --example creature_samples` from the repository, capturing stdout as UTF-8. Creature generator/content versions are 1/1; axes are neutral. Four seeds (0–3) for each of ten non-Chaos presets plus twelve Chaos seeds (10000–10011) produce these 52 complete records. Every record contains all fourteen fields. Named places/people are prose hints, not entities or Relationship IDs.

## Assessment

Classifications: **17 usable as-is, 35 minor edit, 0 weak**. Exact complete-record duplicates: 0. The profile-specific diet/energy, response, tracking clue, resource and countermeasure are reviewed as a unit; preset habitat/origin and secret/hooks carry the local fiction. No unresolved hard biological contradiction or runtime-stat leakage was found. Shape validation and exact replay are automated evidence, distinct from this editorial judgment.

The corpus deliberately favors observable local encounters, refuges and safe counterplay. Repeated boundary/guide/survey plots and short descriptive name pools remain a creative limitation. This is a bounded original corpus, not a claim of unrestricted narrative diversity. Minor-edit notes identify details needed before using an encounter, particularly remembered duties/events, witness agreements, Boss counterplay and Chaos geography. No exact clone or structurally weak record was found; repeated motifs are reported below rather than hidden behind the clone result.

Review repairs before freezing content v1: terminated anatomy sentences; corrected indefinite articles; removed repeated survey-attribution filler; replaced single-body anatomy modifiers for Spirit, Swarm and Elemental with profile-appropriate visual material; removed doubled shelter wording. The final sample capture uses direct UTF-8 subprocess bytes, avoiding local PowerShell output-decoding damage; source corpus Unicode remained intact.

The 52 records represent ten profiles (Undead is absent from these fixed seeds). The separate 1,024 Chaos sweep and all-profile/axis tests cover all eleven profiles, including Undead; no claim that these 52 alone cover every profile is made.

## Measured repetition in this packet

| Field | Distinct values / 52 | Largest identical-value count |
|---|---:|---:|
| name | 37 | 3 |
| profile | 10 | 8 |
| appearance | 49 | 2 |
| habitat | 51 | 2 |
| behavior | 45 | 2 |
| ecology | 39 | 3 |
| threat | 36 | 3 |
| abilities_summary | 39 | 3 |
| weakness | 41 | 3 |
| origin | 50 | 2 |
| signs_of_presence | 36 | 4 |
| loot_or_resource | 37 | 3 |
| secret | 33 | 8 |
| story_hooks | 37 | 3 |

## Automated quality sweeps

- 256 × 11 = 2,816 Candidates: valid, complete, unchanged input DNA, exact replay, zero full-content clones within each preset.
- 1,024 Chaos seeds: valid, exact replay, zero full-content clones; 640 source patterns and 1,013 mixed-source Candidates. Each of three source groups uses all ten corpora. Maximum group/source share is 121/1024 = 11.82%; no source exceeds the tested 160/1024 bound.
- Profile counts: Beast 106; Monster 83; Undead 75; Construct 94; Aberration 104; Spirit 117; Elemental 85; Humanoid Creature 95; Swarm 85; Boss 100; Other 80.

| Preset source | Physical/ecological group | Threat/counterplay group | Origin/narrative group |
|---|---:|---:|---:|
| contemporary-weird | 94 | 103 | 97 |
| cyberpunk | 100 | 113 | 96 |
| dark-fantasy | 91 | 104 | 95 |
| monster-hunter | 121 | 100 | 109 |
| nordic-myth | 98 | 97 | 115 |
| occult | 109 | 89 | 94 |
| pirate | 117 | 116 | 91 |
| post-apocalyptic | 98 | 103 | 112 |
| rogue-assassin | 112 | 96 | 113 |
| steampunk | 84 | 103 | 102 |

## Complete samples

## dark-fantasy / seed 0

**usable as-is** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay.

- name: The Cinder Witness
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. Old abrasions form a recognizable crescent. A distinctive ash abbey mineral leaves a faint sheen.
- habitat: It occupies a covered recess where patterns repeat. It favors an approach with more than one retreat. A plague cordon provides cover. A covered maintenance path offers a second approach.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It checks the same hiding places on each circuit. Surveyors observe from the ash abbey shelter rather than pursuing it.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for ash abbey scavengers.
- threat: Its experiments disrupt routes and confuse travelers. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It notices repeated patterns that observers overlook. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Breaking a repeated arrangement distracts it from pursuit. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: A disturbance brought it from an unrecorded neighboring environment. The refuge was already occupied when the surrounding route opened. An abandoned quarantine allowed it to spread.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The caretaker at ash abbey has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Convince two rival witnesses at ash abbey to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along tithe roads before stranded travelers attempt to force the crossing. | Recover a lost survey book from ash abbey before an eviction crew destroys the refuge.

## dark-fantasy / seed 1

**minor edit** — Tokens, exchange behavior and witnessed agreement fit; specify the agreement and the detained traveler.

- name: The Ash Reader
- profile: HumanoidCreature
- appearance: An upright being with a masked angular face. A darker patch makes the center easy to distinguish. A coating of ash abbey residue disguises the lower outline.
- habitat: It keeps a private sheltered resting place beside a traveled route. It favors an approach with more than one retreat. A plague cordon provides cover. A covered maintenance path offers a second approach.
- behavior: It watches exchanges and offers passage in return for recognizable tokens. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It gathers discarded provisions and maintains a small refuge. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: It detains travelers who break an agreement made at its threshold. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It recalls exchanged tokens and recognizes their bearers. The effect is most obvious along its habitual route. The oldest warning marker still identifies the limit of a safe approach.
- weakness: Presenting a witnessed agreement prevents mistaken detention. The countermeasure works best before the approach is blocked. The required approach can be prepared from the ash abbey shelter.
- origin: Its community withdrew while it remained to maintain a crossing. The earliest surviving account is disputed. The abbey once sheltered it during a famine.
- signs_of_presence: Tied markers and carefully stacked provisions reveal an inhabited route. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Abandoned woven tokens document agreements with earlier travelers. Collect it after the route is quiet. Collectors at the ash abbey prefer shed samples to hunted trophies.
- secret: A witness at ash abbey can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Open a blocked retreat along tithe roads before stranded travelers attempt to force the crossing. | Recover a lost survey book from ash abbey before an eviction crew destroys the refuge. | Escort a caretaker along tithe roads to restore warning markers without provoking a pursuit.

## dark-fantasy / seed 2

**minor edit** — The territorial guardian is coherent; specify the genuinely new approach that defeats its practiced interception.

- name: The Ash Reader
- profile: Boss
- appearance: An imposing territorial guardian with an armored crest. Old abrasions form a recognizable crescent. A distinctive ash abbey mineral leaves a faint sheen.
- habitat: It keeps a sheltered central refuge overlooking an important approach. It favors an approach with more than one retreat. A plague cordon provides cover. A covered maintenance path offers a second approach.
- behavior: It inspects challengers and drives sustained intrusions back to the boundary. It studies unattended bundles before approaching people. Travelers copy ash abbey warning marks before attempting the crossing.
- ecology: It consumes scavenged provisions; its territory shelters smaller creatures from pursuit. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for ash abbey scavengers.
- threat: It blocks the only convenient crossing and disrupts supply journeys. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It anticipates repeated approaches from marks left by earlier intruders. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: A genuinely new approach evades its practiced interception. The countermeasure works best before the approach is blocked. The required approach can be prepared from the ash abbey shelter.
- origin: A long occupation made it the dominant guardian of the refuge. Two witnesses disagree about the date of the first visit. Grave workers concealed its first appearance.
- signs_of_presence: Deep boundary scars and deliberately displaced markers announce its range. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Naturally shed crest fragments preserve a record of old territorial injuries. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The oldest map of ash abbey shows a safe approach now omitted from public copies.
- story_hooks: Escort a caretaker along tithe roads to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at ash abbey remains safe. | Find the missing supplies along tithe roads; preserve evidence before a proposed cull erases the trail.

## dark-fantasy / seed 3

**minor edit** — The territorial guardian is coherent; specify the genuinely new approach that defeats its practiced interception.

- name: The Mourning Guest
- profile: Boss
- appearance: An imposing territorial guardian with an armored crest. Its outline is broken by overlapping ridges. Old route-marker pigments stain its outer surface.
- habitat: It keeps a sheltered central refuge overlooking an important approach. It occupies a refuge screened by discarded material. A flooded crypt road provides cover. Travelers shelter beyond a clearly marked boundary.
- behavior: It inspects challengers and drives sustained intrusions back to the boundary. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It consumes scavenged provisions; its territory shelters smaller creatures from pursuit. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for ash abbey scavengers.
- threat: It blocks the only convenient crossing and disrupts supply journeys. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It anticipates repeated approaches from marks left by earlier intruders. Observers can learn its reach by studying undisturbed marks. At the ash abbey, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: A genuinely new approach evades its practiced interception. The countermeasure works best before the approach is blocked. The required approach can be prepared from the ash abbey shelter.
- origin: A long occupation made it the dominant guardian of the refuge. The refuge was already occupied when the surrounding route opened. An abandoned quarantine allowed it to spread.
- signs_of_presence: Deep boundary scars and deliberately displaced markers announce its range. New debris interrupts an otherwise repeated pattern. The public route has been swept clean, making fresh signs conspicuous.
- loot_or_resource: Naturally shed crest fragments preserve a record of old territorial injuries. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: An abandoned survey of ash abbey records a retreat route that a recent barrier has blocked.
- story_hooks: Convince two rival witnesses at ash abbey to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along tithe roads before stranded travelers attempt to force the crossing. | Recover a lost survey book from ash abbey before an eviction crew destroys the refuge.

## pirate / seed 0

**minor edit** — Contact-based investigation and clean trails fit the organism; sharpen its visual identity beyond the broad folded shape.

- name: The Keel Whisper
- profile: Other
- appearance: A folded ambulatory organism with a patterned outer surface. The outer surface has pale bands at the joints. Shelter material hangs from its sides.
- habitat: It rests in a covered recess alongside a quiet passage. It keeps to the shaded side of the approach. A ship graveyard provides cover. A shallow drainage route passes behind the refuge.
- behavior: It investigates unfamiliar objects and returns them in altered arrangements. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It absorbs decaying surface films and leaves clean trails through debris. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: Its investigations hide useful objects beneath its resting folds. A supply convoy risks abandoning its loads. A relief party has begun using the approach without a guide.
- abilities_summary: It distinguishes materials by prolonged contact. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Offering a harmless unfamiliar sample redirects its investigation. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: Its first observers could not agree on a familiar classification. The earliest surviving account is disputed. Salvagers disturbed its refuge beneath a wreck.
- signs_of_presence: Clean trails and reordered objects indicate a recent visit. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Discarded surface films help identify materials it has handled. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The oldest map of tidal haven shows a safe approach now omitted from public copies.
- story_hooks: Convince two rival witnesses at tidal haven to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along smuggling channels before stranded travelers attempt to force the crossing. | Recover a lost survey book from tidal haven before an eviction crew destroys the refuge.

## pirate / seed 1

**minor edit** — Tokens, exchange behavior and witnessed agreement fit; specify the agreement and the detained traveler.

- name: The Keel Whisper
- profile: HumanoidCreature
- appearance: An upright being with a masked angular face. Old abrasions form a recognizable crescent. A distinctive tidal haven mineral leaves a faint sheen.
- habitat: It keeps a private sheltered resting place beside a traveled route. Its resting site is above the usual flood line. A mangrove channel provides cover. The neighboring storage area has been abandoned.
- behavior: It watches exchanges and offers passage in return for recognizable tokens. It studies unattended bundles before approaching people. Travelers copy tidal haven warning marks before attempting the crossing.
- ecology: It gathers discarded provisions and maintains a small refuge. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It detains travelers who break an agreement made at its threshold. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the tidal haven warning procedure.
- abilities_summary: It recalls exchanged tokens and recognizes their bearers. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a tidal haven omen.
- weakness: Presenting a witnessed agreement prevents mistaken detention. A guide can demonstrate the safe procedure from outside the refuge. At the tidal haven, local warnings describe the method but omit its practical reason.
- origin: Its community withdrew while it remained to maintain a crossing. The local explanation omits an earlier displacement. A storm opened a route from the outer shoals.
- signs_of_presence: Tied markers and carefully stacked provisions reveal an inhabited route. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Abandoned woven tokens document agreements with earlier travelers. A local craft worker can identify an uncontaminated sample. A tidal haven archivist requests a labeled sample for comparison.
- secret: The caretaker at tidal haven has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Recover a lost survey book from tidal haven before an eviction crew destroys the refuge. | Escort a caretaker along smuggling channels to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at tidal haven remains safe.

## pirate / seed 2

**minor edit** — Tokens, exchange behavior and witnessed agreement fit; specify the agreement and the detained traveler.

- name: The Rope Shadow
- profile: HumanoidCreature
- appearance: An upright being with a masked angular face. Its outline is broken by overlapping ridges. Old route-marker pigments stain its outer surface.
- habitat: It keeps a private sheltered resting place beside a traveled route. It returns to the quietest alcove after disturbances. A salt-marsh causeway provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It watches exchanges and offers passage in return for recognizable tokens. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It gathers discarded provisions and maintains a small refuge. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It detains travelers who break an agreement made at its threshold. A supply convoy risks abandoning its loads. A relief party has begun using the approach without a guide.
- abilities_summary: It recalls exchanged tokens and recognizes their bearers. The effect is most obvious along its habitual route. The oldest warning marker still identifies the limit of a safe approach.
- weakness: Presenting a witnessed agreement prevents mistaken detention. The countermeasure works best before the approach is blocked. The required approach can be prepared from the tidal haven shelter.
- origin: Its community withdrew while it remained to maintain a crossing. An abandoned survey contains a more precise description. The harbor grew around its former shelter.
- signs_of_presence: Tied markers and carefully stacked provisions reveal an inhabited route. New debris interrupts an otherwise repeated pattern. The public route has been swept clean, making fresh signs conspicuous.
- loot_or_resource: Abandoned woven tokens document agreements with earlier travelers. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: A witness at tidal haven can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Find the missing supplies along smuggling channels; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at tidal haven to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along smuggling channels before stranded travelers attempt to force the crossing.

## pirate / seed 3

**minor edit** — The territorial guardian is coherent; specify the genuinely new approach that defeats its practiced interception.

- name: The Foam Guest
- profile: Boss
- appearance: An imposing territorial guardian with an armored crest. A narrow stripe marks its leading edge. Its surface carries traces from the nearby abandoned works.
- habitat: It keeps a sheltered central refuge overlooking an important approach. It returns to the quietest alcove after disturbances. A salt-marsh causeway provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It inspects challengers and drives sustained intrusions back to the boundary. It pauses before crossing a marked boundary. Visitors are told to leave unattended bundles outside the boundary.
- ecology: It consumes scavenged provisions; its territory shelters smaller creatures from pursuit. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It blocks the only convenient crossing and disrupts supply journeys. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It anticipates repeated approaches from marks left by earlier intruders. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: A genuinely new approach evades its practiced interception. Using the measure carelessly may erase useful tracking signs. A patient guide can demonstrate the countermeasure without harming the creature.
- origin: A long occupation made it the dominant guardian of the refuge. The refuge was already occupied when the surrounding route opened. Pearl divers first mapped its seasonal visits.
- signs_of_presence: Deep boundary scars and deliberately displaced markers announce its range. The signs become denser near its resting place. A disputed survey sketch records the same pattern.
- loot_or_resource: Naturally shed crest fragments preserve a record of old territorial injuries. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: The oldest map of tidal haven shows a safe approach now omitted from public copies.
- story_hooks: Escort a caretaker along smuggling channels to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at tidal haven remains safe. | Find the missing supplies along smuggling channels; preserve evidence before a proposed cull erases the trail.

## steampunk / seed 0

**usable as-is** — Sediment ecology, granular signs and removal of loose material make the response actionable.

- name: The Soot Guest
- profile: Elemental
- appearance: A shifting mass with a dense particulate core. A darker knot of grains marks the dense core. A coating of canal foundry residue disguises the lower outline.
- habitat: It collects in covered pockets where local material settles. Its resting site is above the usual flood line. An abandoned pump house provides cover. The neighboring storage area has been abandoned.
- behavior: It circles disturbances and settles again when the surroundings are quiet. It studies unattended bundles before approaching people. Travelers copy canal foundry warning marks before attempting the crossing.
- ecology: It gathers loose mineral matter and redistributes sediment. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: Its movement buries signs and obstructs narrow paths. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the canal foundry warning procedure.
- abilities_summary: It can shape loose grains into a temporary shelter. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Removing loose material from an approach limits its reach. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: Repeated environmental disturbances condensed its core. The earliest surviving account is disputed. A failed industrial survey exposed its refuge.
- signs_of_presence: Fresh granular ridges form around otherwise undisturbed surfaces. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Discarded mineral grains are useful for surveying recent disturbances. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: An abandoned survey of canal foundry records a retreat route that a recent barrier has blocked.
- story_hooks: Recover a lost survey book from canal foundry before an eviction crew destroys the refuge. | Escort a caretaker along pressure mains to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at canal foundry remains safe.

## steampunk / seed 1

**usable as-is** — Stalking, concealed refuge, scraped boundaries and clearing an ambush approach fit the predator.

- name: The Soot Guest
- profile: Monster
- appearance: A many-jointed predator with a ridged hide. Old abrasions form a recognizable crescent. A distinctive canal foundry mineral leaves a faint sheen.
- habitat: It keeps a concealed resting chamber near a passage. It uses covered recesses rather than exposed ground. A disused canal foundry provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It watches crossings and stalks isolated trespassers. It avoids a route immediately after loud disturbances. Caretakers clear the route only after a quiet interval.
- ecology: It consumes scavengers; abandoned dens shelter smaller animals. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It drags unsecured supplies into its refuge. The crossing remains accessible by a longer detour. The nearest settlement relies on this route for deliveries.
- abilities_summary: Its hooked limbs find purchase on narrow ledges. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a canal foundry omen.
- weakness: A broad clear approach removes its ambush advantage. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: Its range expanded after its former refuge was disturbed. The refuge was already occupied when the surrounding route opened. An engineer moved its shelter to avoid demolition.
- signs_of_presence: Scraped masonry and scattered empty containers mark a den. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Naturally shed hide plates can reinforce carrying cases. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: The caretaker at canal foundry has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Escort a caretaker along pressure mains to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at canal foundry remains safe. | Find the missing supplies along pressure mains; preserve evidence before a proposed cull erases the trail.

## steampunk / seed 2

**minor edit** — The territorial guardian is coherent; specify the genuinely new approach that defeats its practiced interception.

- name: The Flywheel Shadow
- profile: Boss
- appearance: An imposing territorial guardian with an armored crest. The outer surface has pale bands at the joints. Shelter material hangs from its sides.
- habitat: It keeps a sheltered central refuge overlooking an important approach. It keeps to the shaded side of the approach. A rail maintenance yard provides cover. A shallow drainage route passes behind the refuge.
- behavior: It inspects challengers and drives sustained intrusions back to the boundary. It pauses before crossing a marked boundary. Visitors are told to leave unattended bundles outside the boundary.
- ecology: It consumes scavenged provisions; its territory shelters smaller creatures from pursuit. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: It blocks the only convenient crossing and disrupts supply journeys. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It anticipates repeated approaches from marks left by earlier intruders. Observers can learn its reach by studying undisturbed marks. At the canal foundry, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: A genuinely new approach evades its practiced interception. The response lasts long enough for a cautious withdrawal. A caretaker can supply the missing preparation.
- origin: A long occupation made it the dominant guardian of the refuge. Two witnesses disagree about the date of the first visit. An obsolete safety experiment recorded its arrival.
- signs_of_presence: Deep boundary scars and deliberately displaced markers announce its range. New debris interrupts an otherwise repeated pattern. The public route has been swept clean, making fresh signs conspicuous.
- loot_or_resource: Naturally shed crest fragments preserve a record of old territorial injuries. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: An abandoned survey of canal foundry records a retreat route that a recent barrier has blocked.
- story_hooks: Convince two rival witnesses at canal foundry to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along pressure mains before stranded travelers attempt to force the crossing. | Recover a lost survey book from canal foundry before an eviction crew destroys the refuge.

## steampunk / seed 3

**usable as-is** — Sediment ecology, granular signs and removal of loose material make the response actionable.

- name: The Soot Guest
- profile: Elemental
- appearance: A shifting mass with a dense particulate core. Overlapping granular ridges break its outline. Old route-marker pigments stain its outer surface.
- habitat: It collects in covered pockets where local material settles. Its resting site is above the usual flood line. An abandoned pump house provides cover. The neighboring storage area has been abandoned.
- behavior: It circles disturbances and settles again when the surroundings are quiet. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It gathers loose mineral matter and redistributes sediment. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: Its movement buries signs and obstructs narrow paths. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the canal foundry warning procedure.
- abilities_summary: It can shape loose grains into a temporary shelter. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Removing loose material from an approach limits its reach. Repeat the measure consistently rather than crowding the boundary. A copied survey diagram shows where to stand safely.
- origin: Repeated environmental disturbances condensed its core. Two witnesses disagree about the date of the first visit. An obsolete safety experiment recorded its arrival.
- signs_of_presence: Fresh granular ridges form around otherwise undisturbed surfaces. The signs become denser near its resting place. A disputed survey sketch records the same pattern.
- loot_or_resource: Discarded mineral grains are useful for surveying recent disturbances. A local craft worker can identify an uncontaminated sample. A canal foundry archivist requests a labeled sample for comparison.
- secret: The caretaker at canal foundry has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Escort a caretaker along pressure mains to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at canal foundry remains safe. | Find the missing supplies along pressure mains; preserve evidence before a proposed cull erases the trail.

## cyberpunk / seed 0

**minor edit** — The territorial guardian is coherent; specify the genuinely new approach that defeats its practiced interception.

- name: The Static Listener
- profile: Boss
- appearance: An imposing territorial guardian with an armored crest. The outer surface has pale bands at the joints. Shelter material hangs from its sides.
- habitat: It keeps a sheltered central refuge overlooking an important approach. It keeps to the shaded side of the approach. A flood barrier access tunnel provides cover. A shallow drainage route passes behind the refuge.
- behavior: It inspects challengers and drives sustained intrusions back to the boundary. It checks the same hiding places on each circuit. Surveyors observe from the service underpass shelter rather than pursuing it.
- ecology: It consumes scavenged provisions; its territory shelters smaller creatures from pursuit. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for service underpass scavengers.
- threat: It blocks the only convenient crossing and disrupts supply journeys. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It anticipates repeated approaches from marks left by earlier intruders. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: A genuinely new approach evades its practiced interception. A guide can demonstrate the safe procedure from outside the refuge. At the service underpass, local warnings describe the method but omit its practical reason.
- origin: A long occupation made it the dominant guardian of the refuge. The local explanation omits an earlier displacement. The district expanded over its old territory.
- signs_of_presence: Deep boundary scars and deliberately displaced markers announce its range. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Naturally shed crest fragments preserve a record of old territorial injuries. The material is most useful with a record of where it was found. A craft worker knows how to separate useful material from ordinary debris.
- secret: The caretaker at service underpass has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Recover a lost survey book from service underpass before an eviction crew destroys the refuge. | Escort a caretaker along blackout districts to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at service underpass remains safe.

## cyberpunk / seed 1

**minor edit** — Contact-based investigation and clean trails fit the organism; sharpen its visual identity beyond the broad folded shape.

- name: The Signal Guest
- profile: Other
- appearance: A folded ambulatory organism with a patterned outer surface. Old abrasions form a recognizable crescent. A distinctive service underpass mineral leaves a faint sheen.
- habitat: It rests in a covered recess alongside a quiet passage. It uses covered recesses rather than exposed ground. A sealed service underpass provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It investigates unfamiliar objects and returns them in altered arrangements. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It absorbs decaying surface films and leaves clean trails through debris. Repeated visits keep one passage clear of refuse. Sheltered refuse supports scavengers beside the crossing.
- threat: Its investigations hide useful objects beneath its resting folds. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It distinguishes materials by prolonged contact. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Offering a harmless unfamiliar sample redirects its investigation. Using the measure carelessly may erase useful tracking signs. A patient guide can demonstrate the countermeasure without harming the creature.
- origin: Its first observers could not agree on a familiar classification. A caretaker’s private notes predate the public record. Unlicensed surveyors recorded it before the blackout.
- signs_of_presence: Clean trails and reordered objects indicate a recent visit. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Discarded surface films help identify materials it has handled. The material is most useful with a record of where it was found. A craft worker knows how to separate useful material from ordinary debris.
- secret: The oldest map of service underpass shows a safe approach now omitted from public copies.
- story_hooks: Escort a caretaker along blackout districts to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at service underpass remains safe. | Find the missing supplies along blackout districts; preserve evidence before a proposed cull erases the trail.

## cyberpunk / seed 2

**usable as-is** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay.

- name: The Relay Shadow
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. Old abrasions form a recognizable crescent. A distinctive service underpass mineral leaves a faint sheen.
- habitat: It occupies a covered recess where patterns repeat. It keeps to the shaded side of the approach. A flood barrier access tunnel provides cover. A shallow drainage route passes behind the refuge.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It pauses before crossing a marked boundary. Visitors are told to leave unattended bundles outside the boundary.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: Its experiments disrupt routes and confuse travelers. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It notices repeated patterns that observers overlook. Old boundary marks reveal where the effect stops. A damaged observation shelter makes the behavior difficult to study.
- weakness: Breaking a repeated arrangement distracts it from pursuit. A guide can demonstrate the safe procedure from outside the refuge. At the service underpass, local warnings describe the method but omit its practical reason.
- origin: A disturbance brought it from an unrecorded neighboring environment. Two witnesses disagree about the date of the first visit. An erased environmental assessment described its range.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. A local craft worker can identify an uncontaminated sample. A service underpass archivist requests a labeled sample for comparison.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Compare fresh signs with an old account to find which approach at service underpass remains safe. | Find the missing supplies along blackout districts; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at service underpass to demonstrate their incompatible explanations from a safe distance.

## cyberpunk / seed 3

**usable as-is** — Sediment ecology, granular signs and removal of loose material make the response actionable.

- name: The Relay Shadow
- profile: Elemental
- appearance: A shifting mass with a dense particulate core. A darker knot of grains marks the dense core. A coating of service underpass residue disguises the lower outline.
- habitat: It collects in covered pockets where local material settles. Its resting site is above the usual flood line. An abandoned cooling plant provides cover. The neighboring storage area has been abandoned.
- behavior: It circles disturbances and settles again when the surroundings are quiet. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It gathers loose mineral matter and redistributes sediment. The refuge provides cover for insects during bad weather. A disused drain carries waste away from the public path.
- threat: Its movement buries signs and obstructs narrow paths. The crossing remains accessible by a longer detour. The nearest settlement relies on this route for deliveries.
- abilities_summary: It can shape loose grains into a temporary shelter. Old boundary marks reveal where the effect stops. A damaged observation shelter makes the behavior difficult to study.
- weakness: Removing loose material from an approach limits its reach. The countermeasure works best before the approach is blocked. The required approach can be prepared from the service underpass shelter.
- origin: Repeated environmental disturbances condensed its core. Two witnesses disagree about the date of the first visit. An erased environmental assessment described its range.
- signs_of_presence: Fresh granular ridges form around otherwise undisturbed surfaces. A cleaned approach shows new marks before the next visit. An abandoned observation post offers a safe view of the trail.
- loot_or_resource: Discarded mineral grains are useful for surveying recent disturbances. A local craft worker can identify an uncontaminated sample. A service underpass archivist requests a labeled sample for comparison.
- secret: An abandoned survey of service underpass records a retreat route that a recent barrier has blocked.
- story_hooks: Escort a caretaker along blackout districts to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at service underpass remains safe. | Find the missing supplies along blackout districts; preserve evidence before a proposed cull erases the trail.

## nordic-myth / seed 0

**usable as-is** — Scavenger diet, scent-following behavior, territorial pursuit and an open retreat are mutually compatible.

- name: The Birch Vigil
- profile: Beast
- appearance: A broad-backed animal with a low muzzle. A darker patch makes the center easy to distinguish. A coating of fjord sanctuary residue disguises the lower outline.
- habitat: It rests under cover beside a regularly used passage. It uses covered recesses rather than exposed ground. A sheltered fjord inlet provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It follows scent trails and withdraws when strangers keep their distance. It checks the same hiding places on each circuit. Surveyors observe from the fjord sanctuary shelter rather than pursuing it.
- ecology: It feeds on small scavengers and shelters its young away from traffic. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: Cornering it provokes territorial pursuit. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the fjord sanctuary warning procedure.
- abilities_summary: It distinguishes familiar footsteps through the ground. Observers can learn its reach by studying undisturbed marks. At the fjord sanctuary, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: Leaving an open retreat lets it disengage. A guide can demonstrate the safe procedure from outside the refuge. At the fjord sanctuary, local warnings describe the method but omit its practical reason.
- origin: Its lineage predates the surrounding settlement. The refuge was already occupied when the surrounding route opened. A displaced household kept its refuge secret.
- signs_of_presence: Paired prints and rubbed bark mark its route. New debris interrupts an otherwise repeated pattern. The public route has been swept clean, making fresh signs conspicuous.
- loot_or_resource: Shed underfur is useful for insulating travel cases. A local craft worker can identify an uncontaminated sample. A fjord sanctuary archivist requests a labeled sample for comparison.
- secret: A witness at fjord sanctuary can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Convince two rival witnesses at fjord sanctuary to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along winter crossings before stranded travelers attempt to force the crossing. | Recover a lost survey book from fjord sanctuary before an eviction crew destroys the refuge.

## nordic-myth / seed 1

**minor edit** — Tokens, exchange behavior and witnessed agreement fit; specify the agreement and the detained traveler.

- name: The Birch Vigil
- profile: HumanoidCreature
- appearance: An upright being with a masked angular face. A narrow stripe marks its leading edge. Its surface carries traces from the nearby abandoned works.
- habitat: It keeps a private sheltered resting place beside a traveled route. It favors an approach with more than one retreat. An abandoned winter hall provides cover. A covered maintenance path offers a second approach.
- behavior: It watches exchanges and offers passage in return for recognizable tokens. It studies unattended bundles before approaching people. Travelers copy fjord sanctuary warning marks before attempting the crossing.
- ecology: It gathers discarded provisions and maintains a small refuge. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: It detains travelers who break an agreement made at its threshold. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It recalls exchanged tokens and recognizes their bearers. Old boundary marks reveal where the effect stops. A damaged observation shelter makes the behavior difficult to study.
- weakness: Presenting a witnessed agreement prevents mistaken detention. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: Its community withdrew while it remained to maintain a crossing. A caretaker’s private notes predate the public record. A winter crossing exposed its shelter.
- signs_of_presence: Tied markers and carefully stacked provisions reveal an inhabited route. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Abandoned woven tokens document agreements with earlier travelers. A local craft worker can identify an uncontaminated sample. A fjord sanctuary archivist requests a labeled sample for comparison.
- secret: An abandoned survey of fjord sanctuary records a retreat route that a recent barrier has blocked.
- story_hooks: Find the missing supplies along winter crossings; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at fjord sanctuary to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along winter crossings before stranded travelers attempt to force the crossing.

## nordic-myth / seed 2

**usable as-is** — Refuse consumption, shell dust and sealing exposed refuse give a readable colony problem.

- name: The Rune Watch
- profile: Swarm
- appearance: A coordinated cloud of small shell-backed creatures. Its members form overlapping waves. Old route-marker pigments stain its outer surface.
- habitat: It clusters in sheltered cracks beside a used passage. It keeps to the shaded side of the approach. A cairn-marked pass provides cover. A shallow drainage route passes behind the refuge.
- behavior: It converges on disturbed debris and disperses when activity stops. It pauses before crossing a marked boundary. Visitors are told to leave unattended bundles outside the boundary.
- ecology: It consumes organic refuse and competes with local scavengers. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It covers unsecured containers and makes supplies difficult to retrieve. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: Its members signal through synchronized tapping. The effect is most obvious along its habitual route. The oldest warning marker still identifies the limit of a safe approach.
- weakness: Sealing exposed refuse draws the cluster away from travelers. The response lasts long enough for a cautious withdrawal. A caretaker can supply the missing preparation.
- origin: A transported nesting fragment established the first colony. Two witnesses disagree about the date of the first visit. The settlement once granted it sanctuary.
- signs_of_presence: Fine shell dust and repeated tapping identify an active cluster. A cleaned approach shows new marks before the next visit. An abandoned observation post offers a safe view of the trail.
- loot_or_resource: Empty shell casings provide a light abrasive for repair work. Collect it after the route is quiet. Collectors at the fjord sanctuary prefer shed samples to hunted trophies.
- secret: An abandoned survey of fjord sanctuary records a retreat route that a recent barrier has blocked.
- story_hooks: Recover a lost survey book from fjord sanctuary before an eviction crew destroys the refuge. | Escort a caretaker along winter crossings to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at fjord sanctuary remains safe.

## nordic-myth / seed 3

**minor edit** — Contact-based investigation and clean trails fit the organism; sharpen its visual identity beyond the broad folded shape.

- name: The Thaw Shadow
- profile: Other
- appearance: A folded ambulatory organism with a patterned outer surface. A narrow stripe marks its leading edge. Its surface carries traces from the nearby abandoned works.
- habitat: It rests in a covered recess alongside a quiet passage. It favors an approach with more than one retreat. An abandoned winter hall provides cover. A covered maintenance path offers a second approach.
- behavior: It investigates unfamiliar objects and returns them in altered arrangements. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It absorbs decaying surface films and leaves clean trails through debris. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: Its investigations hide useful objects beneath its resting folds. The crossing remains accessible by a longer detour. The nearest settlement relies on this route for deliveries.
- abilities_summary: It distinguishes materials by prolonged contact. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Offering a harmless unfamiliar sample redirects its investigation. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: Its first observers could not agree on a familiar classification. Two witnesses disagree about the date of the first visit. The settlement once granted it sanctuary.
- signs_of_presence: Clean trails and reordered objects indicate a recent visit. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Discarded surface films help identify materials it has handled. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Convince two rival witnesses at fjord sanctuary to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along winter crossings before stranded travelers attempt to force the crossing. | Recover a lost survey book from fjord sanctuary before an eviction crew destroys the refuge.

## post-apocalyptic / seed 0

**minor edit** — Non-feeding memory presence, condensation signs and acknowledgment fit; name the remembered event before use.

- name: The Reservoir Watch
- profile: Spirit
- appearance: A translucent presence with a trailing silhouette. Its silhouette appears doubled from behind. At the water station, local dust clings to the edges.
- habitat: It gathers in a sheltered place associated with a recurring memory. It uses covered recesses rather than exposed ground. A disused water station provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It replays a remembered encounter and follows recognizable voices. It pauses before crossing a marked boundary. Visitors are told to leave unattended bundles outside the boundary.
- ecology: It takes no physical food; prolonged manifestations disturb nesting creatures. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: Its repeated summons lure people away from marked paths. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It carries faint voices across its shelter. Observers can learn its reach by studying undisturbed marks. At the water station, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: Acknowledging the remembered event makes its summons subside. The countermeasure works best before the approach is blocked. The required approach can be prepared from the water station shelter.
- origin: A neglected memory became anchored to a refuge. An abandoned survey contains a more precise description. A cleanup convoy left its refuge unsealed.
- signs_of_presence: Condensation outlines appear where no solid visitor passed. The signs become denser near its resting place. A disputed survey sketch records the same pattern.
- loot_or_resource: Residue on sheltered surfaces can preserve a fleeting image. Collect it after the route is quiet. Collectors at the water station prefer shed samples to hunted trophies.
- secret: The oldest map of water station shows a safe approach now omitted from public copies.
- story_hooks: Escort a caretaker along salvage trails to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at water station remains safe. | Find the missing supplies along salvage trails; preserve evidence before a proposed cull erases the trail.

## post-apocalyptic / seed 1

**minor edit** — Stored power, inspection routine and maintenance marker fit; specify the original commission and marker.

- name: The Dust Witness
- profile: Construct
- appearance: An articulated sentinel with a recessed faceplate. Old abrasions form a recognizable crescent. A distinctive water station mineral leaves a faint sheen.
- habitat: It shelters near an old work station and its access passage. It favors an approach with more than one retreat. A buried drainage channel provides cover. A covered maintenance path offers a second approach.
- behavior: It inspects the same boundaries and sorts unattended materials. It studies unattended bundles before approaching people. Travelers copy water station warning marks before attempting the crossing.
- ecology: It draws on residual stored power and keeps debris out of its circuit. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: It blocks passage when unfamiliar objects violate its instructions. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the water station warning procedure.
- abilities_summary: It senses movement through its structural frame. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: A recognized maintenance marker makes it pause for inspection. The countermeasure works best before the approach is blocked. The required approach can be prepared from the water station shelter.
- origin: An abandoned commission left its instructions unresolved. Two witnesses disagree about the date of the first visit. A shelter keeper concealed it from scavengers.
- signs_of_presence: Identical pressure marks recur beside neatly sorted debris. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Loose casing fragments reveal an obsolete joining technique. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: A witness at water station can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Recover a lost survey book from water station before an eviction crew destroys the refuge. | Escort a caretaker along salvage trails to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at water station remains safe.

## post-apocalyptic / seed 2

**minor edit** — Stored power, inspection routine and maintenance marker fit; specify the original commission and marker.

- name: The Reservoir Watch
- profile: Construct
- appearance: An articulated sentinel with a recessed faceplate. Its silhouette looks uneven when viewed from behind. At the water station, local dust clings to the edges.
- habitat: It shelters near an old work station and its access passage. It returns to the quietest alcove after disturbances. A shaded communications bunker provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It inspects the same boundaries and sorts unattended materials. It checks the same hiding places on each circuit. Surveyors observe from the water station shelter rather than pursuing it.
- ecology: It draws on residual stored power and keeps debris out of its circuit. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It blocks passage when unfamiliar objects violate its instructions. The crossing remains accessible by a longer detour. The nearest settlement relies on this route for deliveries.
- abilities_summary: It senses movement through its structural frame. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: A recognized maintenance marker makes it pause for inspection. A guide can demonstrate the safe procedure from outside the refuge. At the water station, local warnings describe the method but omit its practical reason.
- origin: An abandoned commission left its instructions unresolved. The refuge was already occupied when the surrounding route opened. A failed survey misidentified its tracks as machinery.
- signs_of_presence: Identical pressure marks recur beside neatly sorted debris. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Loose casing fragments reveal an obsolete joining technique. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: The warning signs at water station were copied from another site; the original observer left a correction in hiding.
- story_hooks: Find the missing supplies along salvage trails; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at water station to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along salvage trails before stranded travelers attempt to force the crossing.

## post-apocalyptic / seed 3

**minor edit** — Stored power, inspection routine and maintenance marker fit; specify the original commission and marker.

- name: The Rust Listener
- profile: Construct
- appearance: An articulated sentinel with a recessed faceplate. Old abrasions form a recognizable crescent. A distinctive water station mineral leaves a faint sheen.
- habitat: It shelters near an old work station and its access passage. It occupies a refuge screened by discarded material. A salvage depot courtyard provides cover. Travelers shelter beyond a clearly marked boundary.
- behavior: It inspects the same boundaries and sorts unattended materials. It avoids a route immediately after loud disturbances. Caretakers clear the route only after a quiet interval.
- ecology: It draws on residual stored power and keeps debris out of its circuit. The refuge provides cover for insects during bad weather. A disused drain carries waste away from the public path.
- threat: It blocks passage when unfamiliar objects violate its instructions. A supply convoy risks abandoning its loads. A relief party has begun using the approach without a guide.
- abilities_summary: It senses movement through its structural frame. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a water station omen.
- weakness: A recognized maintenance marker makes it pause for inspection. The response lasts long enough for a cautious withdrawal. A caretaker can supply the missing preparation.
- origin: An abandoned commission left its instructions unresolved. An abandoned survey contains a more precise description. A cleanup convoy left its refuge unsealed.
- signs_of_presence: Identical pressure marks recur beside neatly sorted debris. The signs become denser near its resting place. A disputed survey sketch records the same pattern.
- loot_or_resource: Loose casing fragments reveal an obsolete joining technique. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: An abandoned survey of water station records a retreat route that a recent barrier has blocked.
- story_hooks: Open a blocked retreat along salvage trails before stranded travelers attempt to force the crossing. | Recover a lost survey book from water station before an eviction crew destroys the refuge. | Escort a caretaker along salvage trails to restore warning markers without provoking a pursuit.

## occult / seed 0

**minor edit** — Tokens, exchange behavior and witnessed agreement fit; specify the agreement and the detained traveler.

- name: The Mirror Shadow
- profile: HumanoidCreature
- appearance: An upright being with a masked angular face. The outer surface has pale bands at the joints. Shelter material hangs from its sides.
- habitat: It keeps a private sheltered resting place beside a traveled route. Its resting site is above the usual flood line. A candle maker’s storeroom provides cover. The neighboring storage area has been abandoned.
- behavior: It watches exchanges and offers passage in return for recognizable tokens. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It gathers discarded provisions and maintains a small refuge. The refuge provides cover for insects during bad weather. A disused drain carries waste away from the public path.
- threat: It detains travelers who break an agreement made at its threshold. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the sealed observatory warning procedure.
- abilities_summary: It recalls exchanged tokens and recognizes their bearers. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a sealed observatory omen.
- weakness: Presenting a witnessed agreement prevents mistaken detention. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: Its community withdrew while it remained to maintain a crossing. An abandoned survey contains a more precise description. An abandoned observatory left its sanctuary unguarded.
- signs_of_presence: Tied markers and carefully stacked provisions reveal an inhabited route. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Abandoned woven tokens document agreements with earlier travelers. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: The oldest map of sealed observatory shows a safe approach now omitted from public copies.
- story_hooks: Find the missing supplies along ritual approaches; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at sealed observatory to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along ritual approaches before stranded travelers attempt to force the crossing.

## occult / seed 1

**usable as-is** — Refuse consumption, shell dust and sealing exposed refuse give a readable colony problem.

- name: The Veiled Guest
- profile: Swarm
- appearance: A coordinated cloud of small shell-backed creatures. Darker members gather at the center of the cluster. A coating of sealed observatory residue disguises the lower outline.
- habitat: It clusters in sheltered cracks beside a used passage. It favors an approach with more than one retreat. An abandoned ritual court provides cover. A covered maintenance path offers a second approach.
- behavior: It converges on disturbed debris and disperses when activity stops. It studies unattended bundles before approaching people. Travelers copy sealed observatory warning marks before attempting the crossing.
- ecology: It consumes organic refuse and competes with local scavengers. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It covers unsecured containers and makes supplies difficult to retrieve. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the sealed observatory warning procedure.
- abilities_summary: Its members signal through synchronized tapping. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Sealing exposed refuse draws the cluster away from travelers. Using the measure carelessly may erase useful tracking signs. A patient guide can demonstrate the countermeasure without harming the creature.
- origin: A transported nesting fragment established the first colony. Two witnesses disagree about the date of the first visit. The archivist hid a description in a household ledger.
- signs_of_presence: Fine shell dust and repeated tapping identify an active cluster. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Empty shell casings provide a light abrasive for repair work. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Convince two rival witnesses at sealed observatory to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along ritual approaches before stranded travelers attempt to force the crossing. | Recover a lost survey book from sealed observatory before an eviction crew destroys the refuge.

## occult / seed 2

**usable as-is** — Sediment ecology, granular signs and removal of loose material make the response actionable.

- name: The Candle Witness
- profile: Elemental
- appearance: A shifting mass with a dense particulate core. Its outline disperses into an uneven wake from behind. At the sealed observatory, local dust clings to the edges.
- habitat: It collects in covered pockets where local material settles. It uses covered recesses rather than exposed ground. A sealed observatory cellar provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It circles disturbances and settles again when the surroundings are quiet. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It gathers loose mineral matter and redistributes sediment. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: Its movement buries signs and obstructs narrow paths. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It can shape loose grains into a temporary shelter. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a sealed observatory omen.
- weakness: Removing loose material from an approach limits its reach. The countermeasure works best before the approach is blocked. The required approach can be prepared from the sealed observatory shelter.
- origin: Repeated environmental disturbances condensed its core. The earliest surviving account is disputed. A private séance recorded its first appearance.
- signs_of_presence: Fresh granular ridges form around otherwise undisturbed surfaces. A cleaned approach shows new marks before the next visit. An abandoned observation post offers a safe view of the trail.
- loot_or_resource: Discarded mineral grains are useful for surveying recent disturbances. A local craft worker can identify an uncontaminated sample. A sealed observatory archivist requests a labeled sample for comparison.
- secret: An abandoned survey of sealed observatory records a retreat route that a recent barrier has blocked.
- story_hooks: Convince two rival witnesses at sealed observatory to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along ritual approaches before stranded travelers attempt to force the crossing. | Recover a lost survey book from sealed observatory before an eviction crew destroys the refuge.

## occult / seed 3

**usable as-is** — Scavenger diet, scent-following behavior, territorial pursuit and an open retreat are mutually compatible.

- name: The Threshold Watch
- profile: Beast
- appearance: A broad-backed animal with a low muzzle. Old abrasions form a recognizable crescent. A distinctive sealed observatory mineral leaves a faint sheen.
- habitat: It rests under cover beside a regularly used passage. It occupies a refuge screened by discarded material. A crypt survey passage provides cover. Travelers shelter beyond a clearly marked boundary.
- behavior: It follows scent trails and withdraws when strangers keep their distance. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It feeds on small scavengers and shelters its young away from traffic. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: Cornering it provokes territorial pursuit. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It distinguishes familiar footsteps through the ground. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Leaving an open retreat lets it disengage. The countermeasure works best before the approach is blocked. The required approach can be prepared from the sealed observatory shelter.
- origin: Its lineage predates the surrounding settlement. The earliest surviving account is disputed. A private séance recorded its first appearance.
- signs_of_presence: Paired prints and rubbed bark mark its route. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Shed underfur is useful for insulating travel cases. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: An abandoned survey of sealed observatory records a retreat route that a recent barrier has blocked.
- story_hooks: Escort a caretaker along ritual approaches to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at sealed observatory remains safe. | Find the missing supplies along ritual approaches; preserve evidence before a proposed cull erases the trail.

## monster-hunter / seed 0

**minor edit** — Tokens, exchange behavior and witnessed agreement fit; specify the agreement and the detained traveler.

- name: The Antler Witness
- profile: HumanoidCreature
- appearance: An upright being with a masked angular face. Its silhouette looks uneven when viewed from behind. At the survey camp, local dust clings to the edges.
- habitat: It keeps a private sheltered resting place beside a traveled route. Its resting site is above the usual flood line. A reed-fringed watering site provides cover. The neighboring storage area has been abandoned.
- behavior: It watches exchanges and offers passage in return for recognizable tokens. It studies unattended bundles before approaching people. Travelers copy survey camp warning marks before attempting the crossing.
- ecology: It gathers discarded provisions and maintains a small refuge. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: It detains travelers who break an agreement made at its threshold. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It recalls exchanged tokens and recognizes their bearers. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Presenting a witnessed agreement prevents mistaken detention. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: Its community withdrew while it remained to maintain a crossing. An abandoned survey contains a more precise description. The first hunter concealed it to prevent a cull.
- signs_of_presence: Tied markers and carefully stacked provisions reveal an inhabited route. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Abandoned woven tokens document agreements with earlier travelers. Collect it after the route is quiet. Collectors at the survey camp prefer shed samples to hunted trophies.
- secret: The caretaker at survey camp has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Compare fresh signs with an old account to find which approach at survey camp remains safe. | Find the missing supplies along warden trails; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at survey camp to demonstrate their incompatible explanations from a safe distance.

## monster-hunter / seed 1

**minor edit** — The territorial guardian is coherent; specify the genuinely new approach that defeats its practiced interception.

- name: The Ravine Watch
- profile: Boss
- appearance: An imposing territorial guardian with an armored crest. Old abrasions form a recognizable crescent. A distinctive survey camp mineral leaves a faint sheen.
- habitat: It keeps a sheltered central refuge overlooking an important approach. It returns to the quietest alcove after disturbances. An overgrown trapping court provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It inspects challengers and drives sustained intrusions back to the boundary. It checks the same hiding places on each circuit. Surveyors observe from the survey camp shelter rather than pursuing it.
- ecology: It consumes scavenged provisions; its territory shelters smaller creatures from pursuit. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: It blocks the only convenient crossing and disrupts supply journeys. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It anticipates repeated approaches from marks left by earlier intruders. The effect is most obvious along its habitual route. The oldest warning marker still identifies the limit of a safe approach.
- weakness: A genuinely new approach evades its practiced interception. Using the measure carelessly may erase useful tracking signs. A patient guide can demonstrate the countermeasure without harming the creature.
- origin: A long occupation made it the dominant guardian of the refuge. A caretaker’s private notes predate the public record. A failed trapping expedition exposed its shelter.
- signs_of_presence: Deep boundary scars and deliberately displaced markers announce its range. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Naturally shed crest fragments preserve a record of old territorial injuries. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: An abandoned survey of survey camp records a retreat route that a recent barrier has blocked.
- story_hooks: Convince two rival witnesses at survey camp to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along warden trails before stranded travelers attempt to force the crossing. | Recover a lost survey book from survey camp before an eviction crew destroys the refuge.

## monster-hunter / seed 2

**usable as-is** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay.

- name: The Ravine Watch
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. The outer surface has pale bands at the joints. Shelter material hangs from its sides.
- habitat: It occupies a covered recess where patterns repeat. It keeps to the shaded side of the approach. A limestone observation cave provides cover. A shallow drainage route passes behind the refuge.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for survey camp scavengers.
- threat: Its experiments disrupt routes and confuse travelers. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It notices repeated patterns that observers overlook. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a survey camp omen.
- weakness: Breaking a repeated arrangement distracts it from pursuit. A guide can demonstrate the safe procedure from outside the refuge. At the survey camp, local warnings describe the method but omit its practical reason.
- origin: A disturbance brought it from an unrecorded neighboring environment. The refuge was already occupied when the surrounding route opened. A seasonal migration brought it into the patrol district.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. New debris interrupts an otherwise repeated pattern. The public route has been swept clean, making fresh signs conspicuous.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. A local craft worker can identify an uncontaminated sample. A survey camp archivist requests a labeled sample for comparison.
- secret: The caretaker at survey camp has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Recover a lost survey book from survey camp before an eviction crew destroys the refuge. | Escort a caretaker along warden trails to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at survey camp remains safe.

## monster-hunter / seed 3

**minor edit** — Contact-based investigation and clean trails fit the organism; sharpen its visual identity beyond the broad folded shape.

- name: The Ridge Guest
- profile: Other
- appearance: A folded ambulatory organism with a patterned outer surface. Its silhouette looks uneven when viewed from behind. At the survey camp, local dust clings to the edges.
- habitat: It rests in a covered recess alongside a quiet passage. It returns to the quietest alcove after disturbances. An overgrown trapping court provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It investigates unfamiliar objects and returns them in altered arrangements. It checks the same hiding places on each circuit. Surveyors observe from the survey camp shelter rather than pursuing it.
- ecology: It absorbs decaying surface films and leaves clean trails through debris. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: Its investigations hide useful objects beneath its resting folds. The crossing remains accessible by a longer detour. The nearest settlement relies on this route for deliveries.
- abilities_summary: It distinguishes materials by prolonged contact. The effect is most obvious along its habitual route. The oldest warning marker still identifies the limit of a safe approach.
- weakness: Offering a harmless unfamiliar sample redirects its investigation. The countermeasure works best before the approach is blocked. The required approach can be prepared from the survey camp shelter.
- origin: Its first observers could not agree on a familiar classification. The refuge was already occupied when the surrounding route opened. A seasonal migration brought it into the patrol district.
- signs_of_presence: Clean trails and reordered objects indicate a recent visit. A cleaned approach shows new marks before the next visit. An abandoned observation post offers a safe view of the trail.
- loot_or_resource: Discarded surface films help identify materials it has handled. The material is most useful with a record of where it was found. A craft worker knows how to separate useful material from ordinary debris.
- secret: A witness at survey camp can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Escort a caretaker along warden trails to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at survey camp remains safe. | Find the missing supplies along warden trails; preserve evidence before a proposed cull erases the trail.

## rogue-assassin / seed 0

**usable as-is** — Sediment ecology, granular signs and removal of loose material make the response actionable.

- name: The Cipher Vigil
- profile: Elemental
- appearance: A shifting mass with a dense particulate core. A darker knot of grains marks the dense core. A coating of courier refuge residue disguises the lower outline.
- habitat: It collects in covered pockets where local material settles. Its resting site is above the usual flood line. A walled kitchen garden provides cover. The neighboring storage area has been abandoned.
- behavior: It circles disturbances and settles again when the surroundings are quiet. It checks the same hiding places on each circuit. Surveyors observe from the courier refuge rather than pursuing it.
- ecology: It gathers loose mineral matter and redistributes sediment. The refuge provides cover for insects during bad weather. A disused drain carries waste away from the public path.
- threat: Its movement buries signs and obstructs narrow paths. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It can shape loose grains into a temporary shelter. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Removing loose material from an approach limits its reach. The response lasts long enough for a cautious withdrawal. A caretaker can supply the missing preparation.
- origin: Repeated environmental disturbances condensed its core. The local explanation omits an earlier displacement. A witness keeper protected its refuge under a false lease.
- signs_of_presence: Fresh granular ridges form around otherwise undisturbed surfaces. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Discarded mineral grains are useful for surveying recent disturbances. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Convince two rival witnesses at courier refuge to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along hidden approaches before stranded travelers attempt to force the crossing. | Recover a lost survey book from courier refuge before an eviction crew destroys the refuge.

## rogue-assassin / seed 1

**minor edit** — Contact-based investigation and clean trails fit the organism; sharpen its visual identity beyond the broad folded shape.

- name: The Ink Listener
- profile: Other
- appearance: A folded ambulatory organism with a patterned outer surface. Old abrasions form a recognizable crescent. A distinctive courier refuge mineral leaves a faint sheen.
- habitat: It rests in a covered recess alongside a quiet passage. It uses covered recesses rather than exposed ground. A sheltered courier passage provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It investigates unfamiliar objects and returns them in altered arrangements. It checks the same hiding places on each circuit. Surveyors observe from the courier refuge rather than pursuing it.
- ecology: It absorbs decaying surface films and leaves clean trails through debris. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: Its investigations hide useful objects beneath its resting folds. A supply convoy risks abandoning its loads. A relief party has begun using the approach without a guide.
- abilities_summary: It distinguishes materials by prolonged contact. Old boundary marks reveal where the effect stops. A damaged observation shelter makes the behavior difficult to study.
- weakness: Offering a harmless unfamiliar sample redirects its investigation. Repeat the measure consistently rather than crowding the boundary. A copied survey diagram shows where to stand safely.
- origin: Its first observers could not agree on a familiar classification. The refuge was already occupied when the surrounding route opened. An old border route explains its unusual movements.
- signs_of_presence: Clean trails and reordered objects indicate a recent visit. A cleaned approach shows new marks before the next visit. An abandoned observation post offers a safe view of the trail.
- loot_or_resource: Discarded surface films help identify materials it has handled. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The warning signs at courier refuge were copied from another site; the original observer left a correction in hiding.
- story_hooks: Recover a lost survey book from courier refuge before an eviction crew destroys the refuge. | Escort a caretaker along hidden approaches to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at courier refuge remains safe.

## rogue-assassin / seed 2

**usable as-is** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay.

- name: The Cipher Vigil
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. Old abrasions form a recognizable crescent. A distinctive courier refuge mineral leaves a faint sheen.
- habitat: It occupies a covered recess where patterns repeat. It keeps to the shaded side of the approach. A disused cellar stair provides cover. A shallow drainage route passes behind the refuge.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: Its experiments disrupt routes and confuse travelers. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It notices repeated patterns that observers overlook. Observers can learn its reach by studying undisturbed marks. At the courier refuge, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: Breaking a repeated arrangement distracts it from pursuit. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: A disturbance brought it from an unrecorded neighboring environment. The refuge was already occupied when the surrounding route opened. An old border route explains its unusual movements.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The warning signs at courier refuge were copied from another site; the original observer left a correction in hiding.
- story_hooks: Compare fresh signs with an old account to find which approach at courier refuge remains safe. | Find the missing supplies along hidden approaches; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at courier refuge to demonstrate their incompatible explanations from a safe distance.

## rogue-assassin / seed 3

**usable as-is** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay.

- name: The Passage Watch
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. A narrow stripe marks its leading edge. Its surface carries traces from the nearby abandoned works.
- habitat: It occupies a covered recess where patterns repeat. It uses covered recesses rather than exposed ground. A sheltered courier passage provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It checks the same hiding places on each circuit. Surveyors observe from the courier refuge rather than pursuing it.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Repeated visits keep one passage clear of refuse. Sheltered refuse supports scavengers beside the crossing.
- threat: Its experiments disrupt routes and confuse travelers. A supply convoy risks abandoning its loads. A relief party has begun using the approach without a guide.
- abilities_summary: It notices repeated patterns that observers overlook. Old boundary marks reveal where the effect stops. A damaged observation shelter makes the behavior difficult to study.
- weakness: Breaking a repeated arrangement distracts it from pursuit. Repeat the measure consistently rather than crowding the boundary. A copied survey diagram shows where to stand safely.
- origin: A disturbance brought it from an unrecorded neighboring environment. An abandoned survey contains a more precise description. A failed surveillance operation first recorded it.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. Collect it after the route is quiet. Collectors at the courier refuge prefer shed samples to hunted trophies.
- secret: An abandoned survey of courier refuge records a retreat route that a recent barrier has blocked.
- story_hooks: Open a blocked retreat along hidden approaches before stranded travelers attempt to force the crossing. | Recover a lost survey book from courier refuge before an eviction crew destroys the refuge. | Escort a caretaker along hidden approaches to restore warning markers without provoking a pursuit.

## contemporary-weird / seed 0

**usable as-is** — Scavenger diet, scent-following behavior, territorial pursuit and an open retreat are mutually compatible.

- name: The Ticket Guest
- profile: Beast
- appearance: A broad-backed animal with a low muzzle. The outer surface has pale bands at the joints. Shelter material hangs from its sides.
- habitat: It rests under cover beside a regularly used passage. It uses covered recesses rather than exposed ground. A sheltered municipal culvert provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It follows scent trails and withdraws when strangers keep their distance. It pauses before crossing a marked boundary. Visitors are told to leave unattended bundles outside the boundary.
- ecology: It feeds on small scavengers and shelters its young away from traffic. The refuge provides cover for insects during bad weather. A disused drain carries waste away from the public path.
- threat: Cornering it provokes territorial pursuit. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It distinguishes familiar footsteps through the ground. Observers can learn its reach by studying undisturbed marks. At the municipal shelter, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: Leaving an open retreat lets it disengage. Using the measure carelessly may erase useful tracking signs. A patient guide can demonstrate the countermeasure without harming the creature.
- origin: Its lineage predates the surrounding settlement. A caretaker’s private notes predate the public record. A night worker recorded its visits in a complaint book.
- signs_of_presence: Paired prints and rubbed bark mark its route. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Shed underfur is useful for insulating travel cases. A local craft worker can identify an uncontaminated sample. A municipal shelter archivist requests a labeled sample for comparison.
- secret: The warning signs at municipal shelter were copied from another site; the original observer left a correction in hiding.
- story_hooks: Convince two rival witnesses at municipal shelter to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along closed public paths before stranded travelers attempt to force the crossing. | Recover a lost survey book from municipal shelter before an eviction crew destroys the refuge.

## contemporary-weird / seed 1

**usable as-is** — Scavenger diet, scent-following behavior, territorial pursuit and an open retreat are mutually compatible.

- name: The Culvert Watch
- profile: Beast
- appearance: A broad-backed animal with a low muzzle. A darker patch makes the center easy to distinguish. A coating of municipal shelter residue disguises the lower outline.
- habitat: It rests under cover beside a regularly used passage. It returns to the quietest alcove after disturbances. A shuttered community hall provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It follows scent trails and withdraws when strangers keep their distance. It avoids a route immediately after loud disturbances. Caretakers clear the route only after a quiet interval.
- ecology: It feeds on small scavengers and shelters its young away from traffic. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: Cornering it provokes territorial pursuit. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It distinguishes familiar footsteps through the ground. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Leaving an open retreat lets it disengage. Repeat the measure consistently rather than crowding the boundary. A copied survey diagram shows where to stand safely.
- origin: Its lineage predates the surrounding settlement. The refuge was already occupied when the surrounding route opened. A caretaker quietly renewed the lease on its refuge.
- signs_of_presence: Paired prints and rubbed bark mark its route. A cleaned approach shows new marks before the next visit. An abandoned observation post offers a safe view of the trail.
- loot_or_resource: Shed underfur is useful for insulating travel cases. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: A witness at municipal shelter can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Escort a caretaker along closed public paths to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at municipal shelter remains safe. | Find the missing supplies along closed public paths; preserve evidence before a proposed cull erases the trail.

## contemporary-weird / seed 2

**minor edit** — Tokens, exchange behavior and witnessed agreement fit; specify the agreement and the detained traveler.

- name: The Ticket Guest
- profile: HumanoidCreature
- appearance: An upright being with a masked angular face. Its silhouette looks uneven when viewed from behind. At the municipal shelter, local dust clings to the edges.
- habitat: It keeps a private sheltered resting place beside a traveled route. It occupies a refuge screened by discarded material. A fenced drainage reserve provides cover. Travelers shelter beyond a clearly marked boundary.
- behavior: It watches exchanges and offers passage in return for recognizable tokens. It avoids a route immediately after loud disturbances. Caretakers clear the route only after a quiet interval.
- ecology: It gathers discarded provisions and maintains a small refuge. The refuge provides cover for insects during bad weather. A disused drain carries waste away from the public path.
- threat: It detains travelers who break an agreement made at its threshold. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It recalls exchanged tokens and recognizes their bearers. Observers can learn its reach by studying undisturbed marks. At the municipal shelter, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: Presenting a witnessed agreement prevents mistaken detention. Using the measure carelessly may erase useful tracking signs. A patient guide can demonstrate the countermeasure without harming the creature.
- origin: Its community withdrew while it remained to maintain a crossing. Two witnesses disagree about the date of the first visit. A demolition delay preserved its shelter.
- signs_of_presence: Tied markers and carefully stacked provisions reveal an inhabited route. The signs become denser near its resting place. A disputed survey sketch records the same pattern.
- loot_or_resource: Abandoned woven tokens document agreements with earlier travelers. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: A witness at municipal shelter can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Convince two rival witnesses at municipal shelter to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along closed public paths before stranded travelers attempt to force the crossing. | Recover a lost survey book from municipal shelter before an eviction crew destroys the refuge.

## contemporary-weird / seed 3

**minor edit** — Stored power, inspection routine and maintenance marker fit; specify the original commission and marker.

- name: The Porch Listener
- profile: Construct
- appearance: An articulated sentinel with a recessed faceplate. Its silhouette looks uneven when viewed from behind. At the municipal shelter, local dust clings to the edges.
- habitat: It shelters near an old work station and its access passage. It returns to the quietest alcove after disturbances. A shuttered community hall provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It inspects the same boundaries and sorts unattended materials. It studies unattended bundles before approaching people. Travelers copy municipal shelter warning marks before attempting the crossing.
- ecology: It draws on residual stored power and keeps debris out of its circuit. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: It blocks passage when unfamiliar objects violate its instructions. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It senses movement through its structural frame. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: A recognized maintenance marker makes it pause for inspection. The countermeasure works best before the approach is blocked. The required approach can be prepared from the municipal shelter.
- origin: An abandoned commission left its instructions unresolved. The local explanation omits an earlier displacement. A diverted public path exposed its usual route.
- signs_of_presence: Identical pressure marks recur beside neatly sorted debris. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Loose casing fragments reveal an obsolete joining technique. The material is most useful with a record of where it was found. A craft worker knows how to separate useful material from ordinary debris.
- secret: An abandoned survey of municipal shelter records a retreat route that a recent barrier has blocked.
- story_hooks: Compare fresh signs with an old account to find which approach at municipal shelter remains safe. | Find the missing supplies along closed public paths; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at municipal shelter to demonstrate their incompatible explanations from a safe distance.

## chaos-blend / seed 10000

**minor edit** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Static Listener
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. A narrow stripe marks its leading edge. Its surface carries traces from the nearby abandoned works.
- habitat: It occupies a covered recess where patterns repeat. It uses covered recesses rather than exposed ground. A sealed service underpass provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It studies unattended bundles before approaching people. Travelers copy service underpass warning marks before attempting the crossing.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for service underpass scavengers.
- threat: Its experiments disrupt routes and confuse travelers. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It notices repeated patterns that observers overlook. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Breaking a repeated arrangement distracts it from pursuit. Repeat the measure consistently rather than crowding the boundary. A copied survey diagram shows where to stand safely.
- origin: A disturbance brought it from an unrecorded neighboring environment. An abandoned survey contains a more precise description. A cleanup convoy left its refuge unsealed.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Compare fresh signs with an old account to find which approach at water station remains safe. | Find the missing supplies along salvage trails; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at water station to demonstrate their incompatible explanations from a safe distance.

## chaos-blend / seed 10001

**minor edit** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Bus-Stop Vigil
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. Its silhouette looks uneven when viewed from behind. At the municipal shelter, local dust clings to the edges.
- habitat: It occupies a covered recess where patterns repeat. It keeps to the shaded side of the approach. A disused library courtyard provides cover. A shallow drainage route passes behind the refuge.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It avoids a route immediately after loud disturbances. Caretakers clear the route only after a quiet interval.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: Its experiments disrupt routes and confuse travelers. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It notices repeated patterns that observers overlook. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Breaking a repeated arrangement distracts it from pursuit. A guide can demonstrate the safe procedure from outside the refuge. At the ash abbey, local warnings describe the method but omit its practical reason.
- origin: A disturbance brought it from an unrecorded neighboring environment. An abandoned survey contains a more precise description. A cleanup convoy left its refuge unsealed.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. The signs become denser near its resting place. A disputed survey sketch records the same pattern.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. The material is most useful with a record of where it was found. A craft worker knows how to separate useful material from ordinary debris.
- secret: A witness at water station can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Compare fresh signs with an old account to find which approach at water station remains safe. | Find the missing supplies along salvage trails; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at water station to demonstrate their incompatible explanations from a safe distance.

## chaos-blend / seed 10002

**minor edit** — Scavenger diet, scent-following behavior, territorial pursuit and an open retreat are mutually compatible. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Cable Witness
- profile: Beast
- appearance: A broad-backed animal with a low muzzle. Its silhouette looks uneven when viewed from behind. At the service underpass, local dust clings to the edges.
- habitat: It rests under cover beside a regularly used passage. It occupies a refuge screened by discarded material. A rooftop utility shelter provides cover. Travelers shelter beyond a clearly marked boundary.
- behavior: It follows scent trails and withdraws when strangers keep their distance. It studies unattended bundles before approaching people. Travelers copy service underpass warning marks before attempting the crossing.
- ecology: It feeds on small scavengers and shelters its young away from traffic. Repeated visits keep one passage clear of refuse. Sheltered refuse supports scavengers beside the crossing.
- threat: Cornering it provokes territorial pursuit. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It distinguishes familiar footsteps through the ground. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Leaving an open retreat lets it disengage. The countermeasure works best before the approach is blocked. The required approach can be prepared from the sealed observatory shelter.
- origin: Its lineage predates the surrounding settlement. The local explanation omits an earlier displacement. A witness keeper protected its refuge under a false lease.
- signs_of_presence: Paired prints and rubbed bark mark its route. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Shed underfur is useful for insulating travel cases. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: A witness at courier refuge can demonstrate a peaceful encounter but fears being accused of sheltering a threat.
- story_hooks: Recover a lost survey book from courier refuge before an eviction crew destroys the refuge. | Escort a caretaker along hidden approaches to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at courier refuge remains safe.

## chaos-blend / seed 10003

**minor edit** — Scavenger diet, scent-following behavior, territorial pursuit and an open retreat are mutually compatible. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Chalk Listener
- profile: Beast
- appearance: A broad-backed animal with a low muzzle. A darker patch makes the center easy to distinguish. A coating of sealed observatory residue disguises the lower outline.
- habitat: It rests under cover beside a regularly used passage. It returns to the quietest alcove after disturbances. A walled herb garden provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It follows scent trails and withdraws when strangers keep their distance. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It feeds on small scavengers and shelters its young away from traffic. Abandoned shelter material becomes nesting cover. An old shelter continues to protect smaller wildlife.
- threat: Cornering it provokes territorial pursuit. Unattended shelter equipment is especially vulnerable. A missing guide left travelers without the municipal shelter warning procedure.
- abilities_summary: It distinguishes familiar footsteps through the ground. Observers can learn its reach by studying undisturbed marks. At the municipal shelter, local observers preserve measurements as sketches rather than claims of invincibility.
- weakness: Leaving an open retreat lets it disengage. The countermeasure works best before the approach is blocked. The required approach can be prepared from the municipal shelter.
- origin: Its lineage predates the surrounding settlement. The local explanation omits an earlier displacement. A witness keeper protected its refuge under a false lease.
- signs_of_presence: Paired prints and rubbed bark mark its route. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Shed underfur is useful for insulating travel cases. Collect it after the route is quiet. Collectors at the municipal shelter prefer shed samples to hunted trophies.
- secret: The caretaker at courier refuge has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Recover a lost survey book from courier refuge before an eviction crew destroys the refuge. | Escort a caretaker along hidden approaches to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at courier refuge remains safe.

## chaos-blend / seed 10004

**minor edit** — Refuse consumption, shell dust and sealing exposed refuse give a readable colony problem. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Low-Tide Witness
- profile: Swarm
- appearance: A coordinated cloud of small shell-backed creatures. A narrow trail of separated members marks its leading edge. Its surface carries traces from the nearby abandoned works.
- habitat: It clusters in sheltered cracks beside a used passage. It uses covered recesses rather than exposed ground. A sheltered tidal cave provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It converges on disturbed debris and disperses when activity stops. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It consumes organic refuse and competes with local scavengers. Local cleaners find its trail easier to follow than to erase. Careful cleanup reduces disturbances without emptying the refuge.
- threat: It covers unsecured containers and makes supplies difficult to retrieve. The crossing remains accessible by a longer detour. The nearest settlement relies on this route for deliveries.
- abilities_summary: Its members signal through synchronized tapping. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a ash abbey omen.
- weakness: Sealing exposed refuse draws the cluster away from travelers. The countermeasure works best before the approach is blocked. The required approach can be prepared from the ash abbey shelter.
- origin: A transported nesting fragment established the first colony. The local explanation omits an earlier displacement. A diverted public path exposed its usual route.
- signs_of_presence: Fine shell dust and repeated tapping identify an active cluster. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Empty shell casings provide a light abrasive for repair work. Careless removal destroys the most informative traces. The shelter workshop can preserve the sample without altering it.
- secret: An abandoned survey of municipal shelter records a retreat route that a recent barrier has blocked.
- story_hooks: Find the missing supplies along closed public paths; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at municipal shelter to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along closed public paths before stranded travelers attempt to force the crossing.

## chaos-blend / seed 10005

**minor edit** — Stalking, concealed refuge, scraped boundaries and clearing an ambush approach fit the predator. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Rune Watch
- profile: Monster
- appearance: A many-jointed predator with a ridged hide. The outer surface has pale bands at the joints. Shelter material hangs from its sides.
- habitat: It keeps a concealed resting chamber near a passage. It favors an approach with more than one retreat. An abandoned winter hall provides cover. A covered maintenance path offers a second approach.
- behavior: It watches crossings and stalks isolated trespassers. It retreats when a familiar warning is repeated. Residents mark safe waiting places with tied cords.
- ecology: It consumes scavengers; abandoned dens shelter smaller animals. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: It drags unsecured supplies into its refuge. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: Its hooked limbs find purchase on narrow ledges. The effect is most obvious along its habitual route. The oldest warning marker still identifies the limit of a safe approach.
- weakness: A broad clear approach removes its ambush advantage. A guide can demonstrate the safe procedure from outside the refuge. At the courier refuge, local warnings describe the method but omit its practical reason.
- origin: Its range expanded after its former refuge was disturbed. Two witnesses disagree about the date of the first visit. A shelter keeper concealed it from scavengers.
- signs_of_presence: Scraped masonry and scattered empty containers mark a den. Fresh marks overlap the oldest boundary line. A shelter ledger records when the signs first returned.
- loot_or_resource: Naturally shed hide plates can reinforce carrying cases. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Compare fresh signs with an old account to find which approach at water station remains safe. | Find the missing supplies along salvage trails; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at water station to demonstrate their incompatible explanations from a safe distance.

## chaos-blend / seed 10006

**minor edit** — Pattern-seeking behavior, decay ecology, spiral clues and breaking arrangements provide consistent counterplay. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Dust Witness
- profile: Aberration
- appearance: An asymmetric being with layered translucent folds. Old abrasions form a recognizable crescent. A distinctive water station mineral leaves a faint sheen.
- habitat: It occupies a covered recess where patterns repeat. It keeps to the shaded side of the approach. A cracked reservoir gallery provides cover. A shallow drainage route passes behind the refuge.
- behavior: It imitates repeated gestures and rearranges objects into unfamiliar sequences. It checks the same hiding places on each circuit. Surveyors observe from the water station shelter rather than pursuing it.
- ecology: It draws sustenance from decaying matter and leaves unusual fungal growth. Small scavengers gather beyond its usual circuit. Seasonal traffic brings discarded provisions to the boundary.
- threat: Its experiments disrupt routes and confuse travelers. A supply convoy risks abandoning its loads. A relief party has begun using the approach without a guide.
- abilities_summary: It notices repeated patterns that observers overlook. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Breaking a repeated arrangement distracts it from pursuit. Evidence from an old visit can reveal the required preparation. The original warning markers indicate an undisturbed retreat.
- origin: A disturbance brought it from an unrecorded neighboring environment. Two witnesses disagree about the date of the first visit. The settlement once granted it sanctuary.
- signs_of_presence: Objects sorted into spirals and discolored fungal patches mark visits. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Discarded membrane filters preserve traces of local pollutants. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The caretaker at fjord sanctuary has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Convince two rival witnesses at fjord sanctuary to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along winter crossings before stranded travelers attempt to force the crossing. | Recover a lost survey book from fjord sanctuary before an eviction crew destroys the refuge.

## chaos-blend / seed 10007

**minor edit** — Stalking, concealed refuge, scraped boundaries and clearing an ambush approach fit the predator. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Ridge Guest
- profile: Monster
- appearance: A many-jointed predator with a ridged hide. A narrow stripe marks its leading edge. Its surface carries traces from the nearby abandoned works.
- habitat: It keeps a concealed resting chamber near a passage. It uses covered recesses rather than exposed ground. A sheltered survey ravine provides cover. The public approach is screened by a ruined boundary wall.
- behavior: It watches crossings and stalks isolated trespassers. It checks the same hiding places on each circuit. Surveyors observe from the survey camp shelter rather than pursuing it.
- ecology: It consumes scavengers; abandoned dens shelter smaller animals. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for survey camp scavengers.
- threat: It drags unsecured supplies into its refuge. A supply convoy risks abandoning its loads. A relief party has begun using the approach without a guide.
- abilities_summary: Its hooked limbs find purchase on narrow ledges. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a tidal haven omen.
- weakness: A broad clear approach removes its ambush advantage. Using the measure carelessly may erase useful tracking signs. A patient guide can demonstrate the countermeasure without harming the creature.
- origin: Its range expanded after its former refuge was disturbed. The local explanation omits an earlier displacement. The district expanded over its old territory.
- signs_of_presence: Scraped masonry and scattered empty containers mark a den. Sheltered marks remain legible after rain. A roadside marker preserves a matching older trace.
- loot_or_resource: Naturally shed hide plates can reinforce carrying cases. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Compare fresh signs with an old account to find which approach at service underpass remains safe. | Find the missing supplies along blackout districts; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at service underpass to demonstrate their incompatible explanations from a safe distance.

## chaos-blend / seed 10008

**minor edit** — Non-feeding memory presence, condensation signs and acknowledgment fit; name the remembered event before use. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Fjord Guest
- profile: Spirit
- appearance: A translucent presence with a trailing silhouette. Its silhouette appears doubled from behind. At the fjord sanctuary, local dust clings to the edges.
- habitat: It gathers in a sheltered place associated with a recurring memory. It occupies a refuge screened by discarded material. A thawed boat shed provides cover. Travelers shelter beyond a clearly marked boundary.
- behavior: It replays a remembered encounter and follows recognizable voices. It checks the same hiding places on each circuit. Surveyors observe from the fjord sanctuary shelter rather than pursuing it.
- ecology: It takes no physical food; prolonged manifestations disturb nesting creatures. Repeated visits keep one passage clear of refuse. Sheltered refuse supports scavengers beside the crossing.
- threat: Its repeated summons lure people away from marked paths. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It carries faint voices across its shelter. Its response is more precise after repeated intrusions. A caretaker records each disturbance before clearing the route.
- weakness: Acknowledging the remembered event makes its summons subside. The countermeasure works best before the approach is blocked. The required approach can be prepared from the courier refuge.
- origin: A neglected memory became anchored to a refuge. The refuge was already occupied when the surrounding route opened. Pearl divers first mapped its seasonal visits.
- signs_of_presence: Condensation outlines appear where no solid visitor passed. A cleaned approach shows new marks before the next visit. An abandoned observation post offers a safe view of the trail.
- loot_or_resource: Residue on sheltered surfaces can preserve a fleeting image. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: The authorities blamed missing supplies on this creature; a second trail leads to an occupied storehouse.
- story_hooks: Recover a lost survey book from tidal haven before an eviction crew destroys the refuge. | Escort a caretaker along smuggling channels to restore warning markers without provoking a pursuit. | Compare fresh signs with an old account to find which approach at tidal haven remains safe.

## chaos-blend / seed 10009

**minor edit** — Sediment ecology, granular signs and removal of loose material make the response actionable. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Thorn Vigil
- profile: Elemental
- appearance: A shifting mass with a dense particulate core. Its outline disperses into an uneven wake from behind. At the ash abbey, local dust clings to the edges.
- habitat: It collects in covered pockets where local material settles. It favors an approach with more than one retreat. A plague cordon provides cover. A covered maintenance path offers a second approach.
- behavior: It circles disturbances and settles again when the surroundings are quiet. It returns after visitors have stopped moving. Guides use a repeated signal before entering the approach.
- ecology: It gathers loose mineral matter and redistributes sediment. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for ash abbey scavengers.
- threat: Its movement buries signs and obstructs narrow paths. A survey party may become separated from its guide. A survey contract rewards anyone who reports the approach safe.
- abilities_summary: It can shape loose grains into a temporary shelter. The effect is most obvious along its habitual route. The oldest warning marker still identifies the limit of a safe approach.
- weakness: Removing loose material from an approach limits its reach. The response lasts long enough for a cautious withdrawal. A caretaker can supply the missing preparation.
- origin: Repeated environmental disturbances condensed its core. Two witnesses disagree about the date of the first visit. A forgery made its territory appear unoccupied.
- signs_of_presence: Fresh granular ridges form around otherwise undisturbed surfaces. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Discarded mineral grains are useful for surveying recent disturbances. The material is most useful with a record of where it was found. A craft worker knows how to separate useful material from ordinary debris.
- secret: The caretaker at courier refuge has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Open a blocked retreat along hidden approaches before stranded travelers attempt to force the crossing. | Recover a lost survey book from courier refuge before an eviction crew destroys the refuge. | Escort a caretaker along hidden approaches to restore warning markers without provoking a pursuit.

## chaos-blend / seed 10010

**minor edit** — The territorial guardian is coherent; specify the genuinely new approach that defeats its practiced interception. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Streetlight Shadow
- profile: Boss
- appearance: An imposing territorial guardian with an armored crest. Old abrasions form a recognizable crescent. A distinctive municipal shelter mineral leaves a faint sheen.
- habitat: It keeps a sheltered central refuge overlooking an important approach. It returns to the quietest alcove after disturbances. A shuttered community hall provides cover. A locked service gate divides the refuge from the usual route.
- behavior: It inspects challengers and drives sustained intrusions back to the boundary. It avoids a route immediately after loud disturbances. Caretakers clear the route only after a quiet interval.
- ecology: It consumes scavenged provisions; its territory shelters smaller creatures from pursuit. Its absence allows scavengers to reclaim the passage. The quiet approach has become a refuge for municipal shelter scavengers.
- threat: It blocks the only convenient crossing and disrupts supply journeys. Travelers mistake a warning circuit for permission to enter. An eviction order may destroy the only clear detour.
- abilities_summary: It anticipates repeated approaches from marks left by earlier intruders. A sheltered approach conceals the first sign of activity. Travelers mistake the repeated response for a water station omen.
- weakness: A genuinely new approach evades its practiced interception. Repeat the measure consistently rather than crowding the boundary. A copied survey diagram shows where to stand safely.
- origin: A long occupation made it the dominant guardian of the refuge. A caretaker’s private notes predate the public record. Unlicensed surveyors recorded it before the blackout.
- signs_of_presence: Deep boundary scars and deliberately displaced markers announce its range. The signs become denser near its resting place. A disputed survey sketch records the same pattern.
- loot_or_resource: Naturally shed crest fragments preserve a record of old territorial injuries. A small sample is enough to compare with an old record. A survey keeper wants a sample accompanied by a route sketch.
- secret: An abandoned survey of service underpass records a retreat route that a recent barrier has blocked.
- story_hooks: Open a blocked retreat along blackout districts before stranded travelers attempt to force the crossing. | Recover a lost survey book from service underpass before an eviction crew destroys the refuge. | Escort a caretaker along blackout districts to restore warning markers without provoking a pursuit.

## chaos-blend / seed 10011

**minor edit** — Scavenger diet, scent-following behavior, territorial pursuit and an open retreat are mutually compatible. Bridge the mixed cultural sites into one route or explain travel between them.

- name: The Low-Tide Witness
- profile: Beast
- appearance: A broad-backed animal with a low muzzle. A darker patch makes the center easy to distinguish. A coating of tidal haven residue disguises the lower outline.
- habitat: It rests under cover beside a regularly used passage. Its resting site is above the usual flood line. A mangrove channel provides cover. The neighboring storage area has been abandoned.
- behavior: It follows scent trails and withdraws when strangers keep their distance. It avoids a route immediately after loud disturbances. Caretakers clear the route only after a quiet interval.
- ecology: It feeds on small scavengers and shelters its young away from traffic. The refuge provides cover for insects during bad weather. A disused drain carries waste away from the public path.
- threat: Cornering it provokes territorial pursuit. Repeated attempts to displace it worsen the disruption. A storage keeper wants the boundary moved before the next supply journey.
- abilities_summary: It distinguishes familiar footsteps through the ground. A patient observer can distinguish preparation from pursuit. An old inspection book distinguishes ordinary movement from an active response.
- weakness: Leaving an open retreat lets it disengage. The countermeasure works best before the approach is blocked. The required approach can be prepared from the municipal shelter.
- origin: Its lineage predates the surrounding settlement. Two witnesses disagree about the date of the first visit. The archivist hid a description in a household ledger.
- signs_of_presence: Paired prints and rubbed bark mark its route. The pattern turns toward cover before the public route ends. A covered storage floor preserves traces from an earlier visit.
- loot_or_resource: Shed underfur is useful for insulating travel cases. Collecting shed material avoids disturbing the refuge. A caretaker exchanges safe collection advice for repairing the boundary markers.
- secret: The caretaker at sealed observatory has been removing warnings to keep travelers away from a concealed shelter.
- story_hooks: Find the missing supplies along ritual approaches; preserve evidence before a proposed cull erases the trail. | Convince two rival witnesses at sealed observatory to demonstrate their incompatible explanations from a safe distance. | Open a blocked retreat along ritual approaches before stranded travelers attempt to force the crossing.
