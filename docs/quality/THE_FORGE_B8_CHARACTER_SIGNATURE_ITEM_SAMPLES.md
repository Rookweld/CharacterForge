# THE FORGE B8 — Character → Signature Item quality samples

## Review method and verdict

Generated and content-author reviewed on 2026-10-01 from the B8 worktree at master `ecdd143061d2e66b4dbfb5667f1fe5b944328235`. This is an agent content review, not independent human playtesting or native operator acceptance.

44 examples: four source seeds (0–3) for each of ten non-Chaos presets and four Chaos Blend examples. Source Character content v2, generator v1, neutral axes, saved Candidate write version 1. Explicit action seed = 100 + source seed. Action `character.forge-signature-item` v1; context `character-signature-item-v1` v1. Reproduce with `cargo run -p forge-app-core --example character_signature_item_samples`.

**Classification: 0 usable as-is; 44 usable with minor edit; 0 weak/unusable.** All require editorial tightening of repeated quoted source facts. Individual notes below identify any additional clarification. No severe Character/Item contradiction found in the reviewed set.

All 44 visibly incorporate exact signature gear in Appearance/Purpose/Function, exact skills and abilities in Function (explicitly separated from Item powers), backstory in History and secret in Secret/hooks. Purpose/cost/hooks retain Character goals, fear and flaw. All complete creative bodies are distinct; names can repeat from the unchanged name corpus. Zero generated UUID/hash leakage or conventional game-stat fields found. Profile distribution: artifact: 1, book: 4, key-item: 2, other: 3, resource: 1, tool: 21, weapon: 12.

Initial review exposed decoration-based profile errors and arbitrary standalone Tool/Other functions. The new contextual path was corrected to prioritize the principal object and use bounded gear-grounded functions/properties/drawbacks; this packet was regenerated afterward. Standalone Item generation is unchanged. A contextual Profile can remain the same when rerolled because an explicit gear noun strongly constrains it; nonce/review provenance still advances.

Origins and histories are fictional proposals, not claims of existing project truth. Candidate Owns requires review/save and independent Canon promotion. A quoted secret is intentional local creative output, not a diagnostic log. Poetic names such as Lantern Levy need not literally identify a lantern. Chaos intentionally mixes secondary preset imagery around one source Character intention.

## Individual examples

### 01 — dark-fantasy / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Underwater-wick lantern supports a pilgrim surgeon; shorten the repeated burial-bell desire without giving the lantern the Character's hearing ability.

**Frozen Character facts:**

- **Abilities:** can hear a recent death in the last note of a struck bell
- **Appearance:** Funeral linen covers one eye and bears a fresh name each dawn.
- **Archetype:** Grave-Tithe Cartographer
- **Backstory Hook:** their map was commissioned by a town erased from every parish record
- **Desire:** to return a stolen burial bell before the next thaw
- **Fear:** being recognized by the voice that calls from sealed wells
- **Flaw:** keeps a promise long after it hurts the living
- **Name:** Iven Vell
- **Personality:** listens for the part people leave unsaid / counts every favor before accepting it
- **Relationship Hook:** a ferryman asks them to carry one sealed letter across the marsh
- **Role:** surgeon for pilgrims returning from the wrong graveyard
- **Secret:** they know which elder is buried under the wrong name
- **Signature Gear:** a brass lantern with a wick that burns underwater
- **Skills:** finding dry ground in a drowned parish / binding a wound with clean funeral linen
- **Title:** The Widow's Last Lantern

**Proposed Item Candidate:**

- **Name:** Widow’s Due
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a brass lantern with a wick that burns underwater”. The repaired edges retain thorn stitching and iron rivets. Its carrying arrangement is recognizable beside Iven Vell, described as “Funeral linen covers one eye and bears a fresh name each dawn.”.
- **Purpose:** Give Iven Vell a dependable companion for “to return a stolen burial bell before the next thaw”, shaped around the familiar “a brass lantern with a wick that burns underwater”.
- **Properties:** Handling marks distinguish this interpretation of “a brass lantern with a wick that burns underwater” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “finding dry ground in a drowned parish / binding a wound with clean funeral linen”; ownership does not grant those talents.
- **Origin:** Made by an almshouse that sheltered deserters. A surviving commission identifies its intended keeper as Iven Vell, “The Widow's Last Lantern”, and describes the requested object as “a brass lantern with a wick that burns underwater”.
- **History:** It was pawned to keep the almshouse ovens burning. A later custody note connects it with this account of Iven Vell: “their map was commissioned by a town erased from every parish record”. The note needs corroboration.
- **Power Or Function:** It provides a carried light whose supply and condition must be maintained. Iven Vell uses “a brass lantern with a wick that burns underwater” alongside “finding dry ground in a drowned parish / binding a wound with clean funeral linen”. Keepers distinguish its function from the Character's own abilities: “can hear a recent death in the last note of a struck bell”.
- **Cost Or Drawback:** Using “a brass lantern with a wick that burns underwater” well requires the keeper's judgment; it cannot decide whom to trust. For Iven Vell, the difficult choice is whether using it could bring about “being recognized by the voice that calls from sealed wells”; “keeps a promise long after it hurts the living” makes that judgment harder.
- **Rarity:** Few survive outside the workshop’s own circle.
- **Condition:** Well preserved beneath a worn protective wrapping.
- **Secret:** Someone recognizes “a brass lantern with a wick that burns underwater” because of “they know which elder is buried under the wrong name”, a fact Iven Vell has kept out of the public account.
- **Story Hooks:** A dispute over “a brass lantern with a wick that burns underwater” draws in this connection: “a ferryman asks them to carry one sealed letter across the marsh”. Who can speak for Iven Vell's claim? / Evidence concerning “their map was commissioned by a town erased from every parish record” calls the item's custody into question; Iven Vell must decide what to disclose. / Using “a brass lantern with a wick that burns underwater” might advance “to return a stolen burial bell before the next thaw”, but a witness asks about “they know which elder is buried under the wrong name” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 02 — dark-fantasy / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Chapel pistol is retained despite the witness/medic skills; those skills stay with the Character. Clarify the custody note rather than inventing weapon powers.

**Frozen Character facts:**

- **Abilities:** can keep a lantern lit in air that snuffs every flame
- **Appearance:** Their boots leave wax prints on stone untouched by flame.
- **Archetype:** Hedge Saint of the Flooded Road
- **Backstory Hook:** they grew up in a parish house that appears only during funerals
- **Desire:** to learn why the chapel ledger lists them as already mourned
- **Fear:** a blue candle burning in the house where they were born
- **Flaw:** mistakes endurance for consent
- **Name:** Iven Vell
- **Personality:** asks the dead person's name before hearing the accusation / protects strangers before trusted friends
- **Relationship Hook:** a ferryman asks them to carry one sealed letter across the marsh
- **Role:** witness paid to remember the dead
- **Secret:** they move one grave marker a handspan every night
- **Signature Gear:** a six-shot chapel pistol wrapped in funeral linen
- **Skills:** setting a bone by lantern light / binding a wound with clean funeral linen
- **Title:** Keeper of the Unlit Mile

**Proposed Item Candidate:**

- **Name:** Last Alms
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a six-shot chapel pistol wrapped in funeral linen”. The sheltered recesses preserve faded velvet and smoke-dark enamel. Its carrying arrangement is recognizable beside Iven Vell, described as “Their boots leave wax prints on stone untouched by flame.”.
- **Purpose:** Keep “a six-shot chapel pistol wrapped in funeral linen” ready for Iven Vell's work as “witness paid to remember the dead”; its keeper hopes it can help with “to learn why the chapel ledger lists them as already mourned”.
- **Properties:** Handling marks distinguish this interpretation of “a six-shot chapel pistol wrapped in funeral linen” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “setting a bone by lantern light / binding a wound with clean funeral linen”; ownership does not grant those talents.
- **Origin:** Made by a smithy kept alive during the ash winter. A surviving commission identifies its intended keeper as Iven Vell, “Keeper of the Unlit Mile”, and describes the requested object as “a six-shot chapel pistol wrapped in funeral linen”.
- **History:** It passed between families paying a burial debt. A later custody note connects it with this account of Iven Vell: “they grew up in a parish house that appears only during funerals”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Iven Vell uses “a six-shot chapel pistol wrapped in funeral linen” alongside “setting a bone by lantern light / binding a wound with clean funeral linen”. Keepers distinguish its function from the Character's own abilities: “can keep a lantern lit in air that snuffs every flame”.
- **Cost Or Drawback:** Carrying “a six-shot chapel pistol wrapped in funeral linen” makes the keeper recognizable to people who know the object. For Iven Vell, the difficult choice is whether using it could bring about “a blue candle burning in the house where they were born”; “mistakes endurance for consent” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** The most guarded account of “a six-shot chapel pistol wrapped in funeral linen” concerns Iven Vell: “they move one grave marker a handspan every night”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a six-shot chapel pistol wrapped in funeral linen” draws in this connection: “a ferryman asks them to carry one sealed letter across the marsh”. Who can speak for Iven Vell's claim? / Evidence concerning “they grew up in a parish house that appears only during funerals” calls the item's custody into question; Iven Vell must decide what to disclose. / Using “a six-shot chapel pistol wrapped in funeral linen” might advance “to learn why the chapel ledger lists them as already mourned”, but a witness asks about “they move one grave marker a handspan every night” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 03 — dark-fantasy / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Field saw stays a Tool despite its monastery seal; cutting and medic handling agree. Tighten the quoted backstory.

**Frozen Character facts:**

- **Abilities:** knows which old stone will hold a ward through one storm
- **Appearance:** Bone buttons fasten a coat whose hem is wet with river silt.
- **Archetype:** Lantern Warden
- **Backstory Hook:** their map was commissioned by a town erased from every parish record
- **Desire:** to prove the dead can still keep a promise
- **Fear:** a blue candle burning in the house where they were born
- **Flaw:** will trade tomorrow's safety for one answer tonight
- **Name:** Orren Crowl
- **Personality:** protects strangers before trusted friends / asks the dead person's name before hearing the accusation
- **Relationship Hook:** a novice priest keeps removing their name from the death roll
- **Role:** field medic to a disgraced company
- **Secret:** they have been charging the dead for passage they never granted
- **Signature Gear:** a bone-handled field saw stamped with a monastery seal
- **Skills:** binding a wound with clean funeral linen / matching grave soil to the parish map
- **Title:** Keeper of the Unlit Mile

**Proposed Item Candidate:**

- **Name:** Grave Promise
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a bone-handled field saw stamped with a monastery seal”. The repaired edges retain thorn stitching and iron rivets. Its carrying arrangement is recognizable beside Orren Crowl, described as “Bone buttons fasten a coat whose hem is wet with river silt.”.
- **Purpose:** Give Orren Crowl a dependable companion for “to prove the dead can still keep a promise”, shaped around the familiar “a bone-handled field saw stamped with a monastery seal”.
- **Properties:** Handling marks distinguish this interpretation of “a bone-handled field saw stamped with a monastery seal” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “binding a wound with clean funeral linen / matching grave soil to the parish map”; ownership does not grant those talents.
- **Origin:** Made by an almshouse that sheltered deserters. A surviving commission identifies its intended keeper as Orren Crowl, “Keeper of the Unlit Mile”, and describes the requested object as “a bone-handled field saw stamped with a monastery seal”.
- **History:** It was pawned to keep the almshouse ovens burning. A later custody note connects it with this account of Orren Crowl: “their map was commissioned by a town erased from every parish record”. The note needs corroboration.
- **Power Or Function:** It supports controlled cutting with the keeper's own trained hands. Orren Crowl uses “a bone-handled field saw stamped with a monastery seal” alongside “binding a wound with clean funeral linen / matching grave soil to the parish map”. Keepers distinguish its function from the Character's own abilities: “knows which old stone will hold a ward through one storm”.
- **Cost Or Drawback:** Using “a bone-handled field saw stamped with a monastery seal” well requires the keeper's judgment; it cannot decide whom to trust. For Orren Crowl, the difficult choice is whether using it could bring about “a blue candle burning in the house where they were born”; “will trade tomorrow's safety for one answer tonight” makes that judgment harder.
- **Rarity:** It is familiar locally but unusual beyond the district.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** The most guarded account of “a bone-handled field saw stamped with a monastery seal” concerns Orren Crowl: “they have been charging the dead for passage they never granted”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a bone-handled field saw stamped with a monastery seal” draws in this connection: “a novice priest keeps removing their name from the death roll”. Who can speak for Orren Crowl's claim? / Evidence concerning “their map was commissioned by a town erased from every parish record” calls the item's custody into question; Orren Crowl must decide what to disclose. / Using “a bone-handled field saw stamped with a monastery seal” might advance “to prove the dead can still keep a promise”, but a witness asks about “they have been charging the dead for passage they never granted” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 04 — dark-fantasy / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Blackthorn stick stays Other; its hidden map does not turn the principal object into a Book. Give the ferrule a clearer physical detail in an edit.

**Frozen Character facts:**

- **Abilities:** can keep a lantern lit in air that snuffs every flame
- **Appearance:** Funeral linen covers one eye and bears a fresh name each dawn.
- **Archetype:** Lantern Warden
- **Backstory Hook:** they grew up in a parish house that appears only during funerals
- **Desire:** to buy back one winter of safety for the village that banished them
- **Fear:** that mercy will arrive one name too late
- **Flaw:** records debts with greater care than kindness
- **Name:** Mara Vell
- **Personality:** asks the dead person's name before hearing the accusation / protects strangers before trusted friends
- **Relationship Hook:** a rival bell-keeper knows the warden's true birth name
- **Role:** witness paid to remember the dead
- **Secret:** they know which elder is buried under the wrong name
- **Signature Gear:** a blackthorn stick with a map hidden in its ferrule
- **Skills:** setting a bone by lantern light / cutting a safe path through a collapsed ossuary
- **Title:** The Widow's Last Lantern

**Proposed Item Candidate:**

- **Name:** Grave Promise
- **Item Profile:** other
- **Appearance:** A personal interpretation of “a blackthorn stick with a map hidden in its ferrule”. The repaired edges retain thorn stitching and iron rivets. Its carrying arrangement is recognizable beside Mara Vell, described as “Funeral linen covers one eye and bears a fresh name each dawn.”.
- **Purpose:** Keep “a blackthorn stick with a map hidden in its ferrule” ready for Mara Vell's work as “witness paid to remember the dead”; its keeper hopes it can help with “to buy back one winter of safety for the village that banished them”.
- **Properties:** Handling marks distinguish this interpretation of “a blackthorn stick with a map hidden in its ferrule” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “setting a bone by lantern light / cutting a safe path through a collapsed ossuary”; ownership does not grant those talents.
- **Origin:** Made by the funeral workshops beneath the drowned abbey. A surviving commission identifies its intended keeper as Mara Vell, “The Widow's Last Lantern”, and describes the requested object as “a blackthorn stick with a map hidden in its ferrule”.
- **History:** It disappeared during a coronation held behind locked doors. A later custody note connects it with this account of Mara Vell: “they grew up in a parish house that appears only during funerals”. The note needs corroboration.
- **Power Or Function:** It makes the described personal object available for its keeper's work; the account of the gear sets its practical limits. Mara Vell uses “a blackthorn stick with a map hidden in its ferrule” alongside “setting a bone by lantern light / cutting a safe path through a collapsed ossuary”. Keepers distinguish its function from the Character's own abilities: “can keep a lantern lit in air that snuffs every flame”.
- **Cost Or Drawback:** Maintaining “a blackthorn stick with a map hidden in its ferrule” takes time its keeper might need elsewhere. For Mara Vell, the difficult choice is whether using it could bring about “that mercy will arrive one name too late”; “records debts with greater care than kindness” makes that judgment harder.
- **Rarity:** Custodians exchange examples privately rather than through public markets.
- **Condition:** Apparently ready for use, though a keeper recommends a patient inspection.
- **Secret:** A concealed custody note links “a blackthorn stick with a map hidden in its ferrule” to this secret about Mara Vell: “they know which elder is buried under the wrong name”. It does not reveal who wrote the note.
- **Story Hooks:** A dispute over “a blackthorn stick with a map hidden in its ferrule” draws in this connection: “a rival bell-keeper knows the warden's true birth name”. Who can speak for Mara Vell's claim? / Evidence concerning “they grew up in a parish house that appears only during funerals” calls the item's custody into question; Mara Vell must decide what to disclose. / Using “a blackthorn stick with a map hidden in its ferrule” might advance “to buy back one winter of safety for the village that banished them”, but a witness asks about “they know which elder is buried under the wrong name” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 05 — pirate / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Boarding hook supports line work and rough-water skill. Tighten the repeated mutiny-island desire.

**Frozen Character facts:**

- **Abilities:** can steer a skiff through a cannon-smoke blind
- **Appearance:** Their coat carries empty buttons for people lost at sea.
- **Archetype:** Mutiny-Script Quartermaster
- **Backstory Hook:** they survived a wreck with no broken planks
- **Desire:** to find the island that appears only on mutiny night
- **Fear:** that the sea will collect a promise made by someone else
- **Flaw:** cannot leave a dishonest bargain unfinished
- **Name:** Nella Rook
- **Personality:** keeps a joke ready for the person who has none / knows exactly who has not eaten
- **Relationship Hook:** a rival quartermaster keeps their share in a locked chest
- **Role:** go-between for captains who cannot meet
- **Secret:** their map tattoo ends at a port they deny visiting
- **Signature Gear:** a folding boarding hook etched with old crew marks
- **Skills:** splicing a mast in rough water / finding fresh water beneath a coral shelf
- **Title:** The Third Bell

**Proposed Item Candidate:**

- **Name:** Tide Dividend
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a folding boarding hook etched with old crew marks”. Its carrying wrap echoes amber varnish and rope impressions. Its carrying arrangement is recognizable beside Nella Rook, described as “Their coat carries empty buttons for people lost at sea.”.
- **Purpose:** Help Nella Rook carry out “go-between for captains who cannot meet”, while pursuing “to find the island that appears only on mutiny night”. The commission centers on “a folding boarding hook etched with old crew marks”.
- **Properties:** Handling marks distinguish this interpretation of “a folding boarding hook etched with old crew marks” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “splicing a mast in rough water / finding fresh water beneath a coral shelf”; ownership does not grant those talents.
- **Origin:** Made by a harbor workshop maintaining illicit night signals. A surviving commission identifies its intended keeper as Nella Rook, “The Third Bell”, and describes the requested object as “a folding boarding hook etched with old crew marks”.
- **History:** It was carried ashore after a vessel vanished from its mooring. A later custody note connects it with this account of Nella Rook: “they survived a wreck with no broken planks”. The note needs corroboration.
- **Power Or Function:** It helps its keeper catch, hold, or tend lines using practiced handling. Nella Rook uses “a folding boarding hook etched with old crew marks” alongside “splicing a mast in rough water / finding fresh water beneath a coral shelf”. Keepers distinguish its function from the Character's own abilities: “can steer a skiff through a cannon-smoke blind”.
- **Cost Or Drawback:** Maintaining “a folding boarding hook etched with old crew marks” takes time its keeper might need elsewhere. For Nella Rook, the difficult choice is whether using it could bring about “that the sea will collect a promise made by someone else”; “cannot leave a dishonest bargain unfinished” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** One prior keeper recorded “their map tattoo ends at a port they deny visiting” beside Nella Rook's name. The record was hidden in the wrapping of “a folding boarding hook etched with old crew marks”.
- **Story Hooks:** Using “a folding boarding hook etched with old crew marks” might advance “to find the island that appears only on mutiny night”, but a witness asks about “their map tattoo ends at a port they deny visiting” first. / A dispute over “a folding boarding hook etched with old crew marks” draws in this connection: “a rival quartermaster keeps their share in a locked chest”. Who can speak for Nella Rook's claim? / Evidence concerning “they survived a wreck with no broken planks” calls the item's custody into question; Nella Rook must decide what to disclose.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 06 — pirate / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Tide table is a written Book-profile object carried in a coat; the name Lantern Levy is figurative. An edit can explain the carrying arrangement.

**Frozen Character facts:**

- **Abilities:** can hear a drowned ship's bell through solid rock
- **Appearance:** A chart is tattooed across one shoulder, with a blank space where the island should be.
- **Archetype:** The Drowned Navigator
- **Backstory Hook:** their first crew left them on an island that moved
- **Desire:** to recover a ship's log before a rival sells its final page
- **Fear:** hearing their own voice answer from below the hull
- **Flaw:** turns every disagreement into a vote they can win
- **Name:** Kit Rook
- **Personality:** never accepts the first version of a bargain / laughs when a plan becomes impossible
- **Relationship Hook:** a navy clerk keeps forging their death certificate
- **Role:** ship's cook who keeps the crew's secret vote
- **Secret:** their map tattoo ends at a port they deny visiting
- **Signature Gear:** a tide table inked inside a waterproof coat
- **Skills:** splicing a mast in rough water / keeping a damaged compass true with a sliver of iron
- **Title:** Red Wake's Witness

**Proposed Item Candidate:**

- **Name:** Lantern Levy
- **Item Profile:** book
- **Appearance:** A personal interpretation of “a tide table inked inside a waterproof coat”. The sheltered recesses preserve patched canvas and silver wire. Its carrying arrangement is recognizable beside Kit Rook, described as “A chart is tattooed across one shoulder, with a blank space where the island should be.”.
- **Purpose:** Support the practical work behind “The Drowned Navigator”: Kit Rook's “a tide table inked inside a waterproof coat” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a tide table inked inside a waterproof coat” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “splicing a mast in rough water / keeping a damaged compass true with a sliver of iron”; ownership does not grant those talents.
- **Origin:** Made by a harbor workshop maintaining illicit night signals. A surviving commission identifies its intended keeper as Kit Rook, “Red Wake's Witness”, and describes the requested object as “a tide table inked inside a waterproof coat”.
- **History:** It survived a raid that extinguished the harbor’s signal fires. A later custody note connects it with this account of Kit Rook: “their first crew left them on an island that moved”. The note needs corroboration.
- **Power Or Function:** It keeps written material available for reading, comparison, and annotation; it cannot establish the truth of an entry. Kit Rook uses “a tide table inked inside a waterproof coat” alongside “splicing a mast in rough water / keeping a damaged compass true with a sliver of iron”. Keepers distinguish its function from the Character's own abilities: “can hear a drowned ship's bell through solid rock”.
- **Cost Or Drawback:** Using “a tide table inked inside a waterproof coat” well requires the keeper's judgment; it cannot decide whom to trust. For Kit Rook, the difficult choice is whether using it could bring about “hearing their own voice answer from below the hull”; “turns every disagreement into a vote they can win” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Apparently ready for use, though a keeper recommends a patient inspection.
- **Secret:** One prior keeper recorded “their map tattoo ends at a port they deny visiting” beside Kit Rook's name. The record was hidden in the wrapping of “a tide table inked inside a waterproof coat”.
- **Story Hooks:** Using “a tide table inked inside a waterproof coat” might advance “to recover a ship's log before a rival sells its final page”, but a witness asks about “their map tattoo ends at a port they deny visiting” first. / A dispute over “a tide table inked inside a waterproof coat” draws in this connection: “a navy clerk keeps forging their death certificate”. Who can speak for Kit Rook's claim? / Evidence concerning “their first crew left them on an island that moved” calls the item's custody into question; Kit Rook must decide what to disclose.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 07 — pirate / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Boarding axe stays Tool because the source explicitly says rigging, not bodies. Line work remains practical rather than a combat statistic.

**Frozen Character facts:**

- **Abilities:** can taste a coming squall on a brass coin
- **Appearance:** Their coat carries empty buttons for people lost at sea.
- **Archetype:** Wreck-Lantern Diver
- **Backstory Hook:** their first mutiny began over who would carry an injured enemy
- **Desire:** to give the crew a vote that cannot be bought
- **Fear:** a calm sea reflecting a ship that is not overhead
- **Flaw:** mistakes a rescued enemy for a future ally
- **Name:** Rafe Marlowe
- **Personality:** counts the crew before counting the cargo / knows exactly who has not eaten
- **Relationship Hook:** a rival quartermaster keeps their share in a locked chest
- **Role:** pilot trusted only in fog
- **Secret:** they changed the course that stranded their former captain
- **Signature Gear:** a boarding axe balanced for rigging, not bodies
- **Skills:** diving by sound through wreckage / splicing a mast in rough water
- **Title:** The Third Bell

**Proposed Item Candidate:**

- **Name:** Tide Dividend
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a boarding axe balanced for rigging, not bodies”. Its carrying wrap echoes amber varnish and rope impressions. Its carrying arrangement is recognizable beside Rafe Marlowe, described as “Their coat carries empty buttons for people lost at sea.”.
- **Purpose:** Keep “a boarding axe balanced for rigging, not bodies” ready for Rafe Marlowe's work as “pilot trusted only in fog”; its keeper hopes it can help with “to give the crew a vote that cannot be bought”.
- **Properties:** Handling marks distinguish this interpretation of “a boarding axe balanced for rigging, not bodies” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “diving by sound through wreckage / splicing a mast in rough water”; ownership does not grant those talents.
- **Origin:** Made by the repair benches of a floating pirate market. A surviving commission identifies its intended keeper as Rafe Marlowe, “The Third Bell”, and describes the requested object as “a boarding axe balanced for rigging, not bodies”.
- **History:** It was held as collateral during a mutiny trial. A later custody note connects it with this account of Rafe Marlowe: “their first mutiny began over who would carry an injured enemy”. The note needs corroboration.
- **Power Or Function:** It helps its keeper catch, hold, or tend lines using practiced handling. Rafe Marlowe uses “a boarding axe balanced for rigging, not bodies” alongside “diving by sound through wreckage / splicing a mast in rough water”. Keepers distinguish its function from the Character's own abilities: “can taste a coming squall on a brass coin”.
- **Cost Or Drawback:** Using “a boarding axe balanced for rigging, not bodies” well requires the keeper's judgment; it cannot decide whom to trust. For Rafe Marlowe, the difficult choice is whether using it could bring about “a calm sea reflecting a ship that is not overhead”; “mistakes a rescued enemy for a future ally” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Well preserved beneath a worn protective wrapping.
- **Secret:** The most guarded account of “a boarding axe balanced for rigging, not bodies” concerns Rafe Marlowe: “they changed the course that stranded their former captain”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a boarding axe balanced for rigging, not bodies” draws in this connection: “a rival quartermaster keeps their share in a locked chest”. Who can speak for Rafe Marlowe's claim? / Evidence concerning “their first mutiny began over who would carry an injured enemy” calls the item's custody into question; Rafe Marlowe must decide what to disclose. / Using “a boarding axe balanced for rigging, not bodies” might advance “to give the crew a vote that cannot be bought”, but a witness asks about “they changed the course that stranded their former captain” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 08 — pirate / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Flintlock remains Weapon; a cook owning one is an authored source fact, not a role contradiction. Tighten the political-purpose wording.

**Frozen Character facts:**

- **Abilities:** can follow a ship by its wake after sunset
- **Appearance:** A torn sail is worn as a sash, patched with flags from vanished crews.
- **Archetype:** Tide-Court Interpreter
- **Backstory Hook:** they learned navigation from a prisoner who could not see the sky
- **Desire:** to find the island that appears only on mutiny night
- **Fear:** being named captain by people who expect obedience
- **Flaw:** keeps sailing because shore means settling accounts
- **Name:** Kit Marlowe
- **Personality:** counts the crew before counting the cargo / knows exactly who has not eaten
- **Relationship Hook:** a rival quartermaster keeps their share in a locked chest
- **Role:** ship's cook who keeps the crew's secret vote
- **Secret:** they changed the course that stranded their former captain
- **Signature Gear:** a flintlock with one silvered shot reserved for the captain
- **Skills:** diving by sound through wreckage / finding fresh water beneath a coral shelf
- **Title:** The Last Share on Deck

**Proposed Item Candidate:**

- **Name:** Tide Dividend
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a flintlock with one silvered shot reserved for the captain”. The repaired edges retain dark teak and green copper. Its carrying arrangement is recognizable beside Kit Marlowe, described as “A torn sail is worn as a sash, patched with flags from vanished crews.”.
- **Purpose:** Help Kit Marlowe carry out “ship's cook who keeps the crew's secret vote”, while pursuing “to find the island that appears only on mutiny night”. The commission centers on “a flintlock with one silvered shot reserved for the captain”.
- **Properties:** Handling marks distinguish this interpretation of “a flintlock with one silvered shot reserved for the captain” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “diving by sound through wreckage / finding fresh water beneath a coral shelf”; ownership does not grant those talents.
- **Origin:** Made by a workshop financed by pearl divers. A surviving commission identifies its intended keeper as Kit Marlowe, “The Last Share on Deck”, and describes the requested object as “a flintlock with one silvered shot reserved for the captain”.
- **History:** It paid for a voyage whose chart was later confiscated. A later custody note connects it with this account of Kit Marlowe: “they learned navigation from a prisoner who could not see the sky”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Kit Marlowe uses “a flintlock with one silvered shot reserved for the captain” alongside “diving by sound through wreckage / finding fresh water beneath a coral shelf”. Keepers distinguish its function from the Character's own abilities: “can follow a ship by its wake after sunset”.
- **Cost Or Drawback:** Maintaining “a flintlock with one silvered shot reserved for the captain” takes time its keeper might need elsewhere. For Kit Marlowe, the difficult choice is whether using it could bring about “being named captain by people who expect obedience”; “keeps sailing because shore means settling accounts” makes that judgment harder.
- **Rarity:** It is familiar locally but unusual beyond the district.
- **Condition:** Apparently ready for use, though a keeper recommends a patient inspection.
- **Secret:** The most guarded account of “a flintlock with one silvered shot reserved for the captain” concerns Kit Marlowe: “they changed the course that stranded their former captain”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a flintlock with one silvered shot reserved for the captain” draws in this connection: “a rival quartermaster keeps their share in a locked chest”. Who can speak for Kit Marlowe's claim? / Evidence concerning “they learned navigation from a prisoner who could not see the sky” calls the item's custody into question; Kit Marlowe must decide what to disclose. / Using “a flintlock with one silvered shot reserved for the captain” might advance “to find the island that appears only on mutiny night”, but a witness asks about “they changed the course that stranded their former captain” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 09 — steampunk / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Pocket gauge stays Tool; the union seal is decoration rather than access authority. Readings need interpretation, matching the inspector.

**Frozen Character facts:**

- **Abilities:** can quiet a boiler by matching its rhythm with a wrench
- **Appearance:** A pressure dial sits where a coat button should be.
- **Archetype:** Clockwork Union Advocate
- **Backstory Hook:** they survived a lift collapse by counting the gears in the dark
- **Desire:** to prove the city's clock is losing one citizen each hour
- **Fear:** becoming useful enough that nobody asks what they want
- **Flaw:** cannot leave a locked control untouched
- **Name:** Mina Cogswell
- **Personality:** keeps a pocket watch that runs backward during arguments / keeps a ledger of favors owed by powerful people
- **Relationship Hook:** a clock singer sends one wrong note when danger is near
- **Role:** inspector certifying machines built from confiscated homes
- **Secret:** they know why the public clock skips the night shift
- **Signature Gear:** a pocket gauge sealed with a union stamp
- **Skills:** tracing a steam leak by its pattern in soot / reading a shift schedule hidden in a hymn wheel
- **Title:** The Unlicensed Spring

**Proposed Item Candidate:**

- **Name:** Pressure Covenant
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a pocket gauge sealed with a union stamp”. The sheltered recesses preserve verdigris copper and waxed wool. Its carrying arrangement is recognizable beside Mina Cogswell, described as “A pressure dial sits where a coat button should be.”.
- **Purpose:** Support the practical work behind “Clockwork Union Advocate”: Mina Cogswell's “a pocket gauge sealed with a union stamp” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a pocket gauge sealed with a union stamp” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “tracing a steam leak by its pattern in soot / reading a shift schedule hidden in a hymn wheel”; ownership does not grant those talents.
- **Origin:** Made by the engineers of an obsolete pumping station. A surviving commission identifies its intended keeper as Mina Cogswell, “The Unlicensed Spring”, and describes the requested object as “a pocket gauge sealed with a union stamp”.
- **History:** It was auctioned when the station’s district lost water service. A later custody note connects it with this account of Mina Cogswell: “they survived a lift collapse by counting the gears in the dark”. The note needs corroboration.
- **Power Or Function:** It presents a reading that its keeper must interpret and check against other evidence. Mina Cogswell uses “a pocket gauge sealed with a union stamp” alongside “tracing a steam leak by its pattern in soot / reading a shift schedule hidden in a hymn wheel”. Keepers distinguish its function from the Character's own abilities: “can quiet a boiler by matching its rhythm with a wrench”.
- **Cost Or Drawback:** Maintaining “a pocket gauge sealed with a union stamp” takes time its keeper might need elsewhere. For Mina Cogswell, the difficult choice is whether using it could bring about “becoming useful enough that nobody asks what they want”; “cannot leave a locked control untouched” makes that judgment harder.
- **Rarity:** Custodians exchange examples privately rather than through public markets.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** Someone recognizes “a pocket gauge sealed with a union stamp” because of “they know why the public clock skips the night shift”, a fact Mina Cogswell has kept out of the public account.
- **Story Hooks:** A dispute over “a pocket gauge sealed with a union stamp” draws in this connection: “a clock singer sends one wrong note when danger is near”. Who can speak for Mina Cogswell's claim? / Evidence concerning “they survived a lift collapse by counting the gears in the dark” calls the item's custody into question; Mina Cogswell must decide what to disclose. / Using “a pocket gauge sealed with a union stamp” might advance “to prove the city's clock is losing one citizen each hour”, but a witness asks about “they know why the public clock skips the night shift” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 10 — steampunk / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Pepperbox remains Weapon; union-courier ownership and clock investigation can coexist. Clarify the practical protection role in a short edit.

**Frozen Character facts:**

- **Abilities:** can quiet a boiler by matching its rhythm with a wrench
- **Appearance:** Copper dust settles in the seams of a coat repaired with railway tickets.
- **Archetype:** Boiler Choir Tuner
- **Backstory Hook:** they were raised in a boarding house built around a steam main
- **Desire:** to find the hour missing from the public clock
- **Fear:** being praised for making exploitation quieter
- **Flaw:** takes machinery apart before securing the room
- **Name:** Mina Cogswell
- **Personality:** keeps a ledger of favors owed by powerful people / hates waste more than failure
- **Relationship Hook:** a supervisor keeps paying their medical bills anonymously
- **Role:** union courier through private lift shafts
- **Secret:** their brass hand is powered by a name they cannot remember
- **Signature Gear:** a compact pepperbox stamped with a factory serial
- **Skills:** making a replacement gear from a tram token / reading a shift schedule hidden in a hymn wheel
- **Title:** The Unlicensed Spring

**Proposed Item Candidate:**

- **Name:** Pressure Covenant
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a compact pepperbox stamped with a factory serial”. The sheltered recesses preserve verdigris copper and waxed wool. Its carrying arrangement is recognizable beside Mina Cogswell, described as “Copper dust settles in the seams of a coat repaired with railway tickets.”.
- **Purpose:** Help Mina Cogswell carry out “union courier through private lift shafts”, while pursuing “to find the hour missing from the public clock”. The commission centers on “a compact pepperbox stamped with a factory serial”.
- **Properties:** Handling marks distinguish this interpretation of “a compact pepperbox stamped with a factory serial” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “making a replacement gear from a tram token / reading a shift schedule hidden in a hymn wheel”; ownership does not grant those talents.
- **Origin:** Made by a railway workshop removed from the network. A surviving commission identifies its intended keeper as Mina Cogswell, “The Unlicensed Spring”, and describes the requested object as “a compact pepperbox stamped with a factory serial”.
- **History:** It crossed picket lines concealed in a repair shipment. A later custody note connects it with this account of Mina Cogswell: “they were raised in a boarding house built around a steam main”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Mina Cogswell uses “a compact pepperbox stamped with a factory serial” alongside “making a replacement gear from a tram token / reading a shift schedule hidden in a hymn wheel”. Keepers distinguish its function from the Character's own abilities: “can quiet a boiler by matching its rhythm with a wrench”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a compact pepperbox stamped with a factory serial” can make its use unreliable until inspected. For Mina Cogswell, the difficult choice is whether using it could bring about “being praised for making exploitation quieter”; “takes machinery apart before securing the room” makes that judgment harder.
- **Rarity:** Its form is known; documented examples are seldom offered for trade.
- **Condition:** Apparently ready for use, though a keeper recommends a patient inspection.
- **Secret:** The most guarded account of “a compact pepperbox stamped with a factory serial” concerns Mina Cogswell: “their brass hand is powered by a name they cannot remember”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a compact pepperbox stamped with a factory serial” draws in this connection: “a supervisor keeps paying their medical bills anonymously”. Who can speak for Mina Cogswell's claim? / Evidence concerning “they were raised in a boarding house built around a steam main” calls the item's custody into question; Mina Cogswell must decide what to disclose. / Using “a compact pepperbox stamped with a factory serial” might advance “to find the hour missing from the public clock”, but a witness asks about “their brass hand is powered by a name they cannot remember” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 11 — steampunk / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Pepperbox supports the inspector without acquiring mechanism-sensing ability. Clarify how its custody connects to the rejected engine plans.

**Frozen Character facts:**

- **Abilities:** can feel when a mechanism is about to repeat a mistake
- **Appearance:** A pressure dial sits where a coat button should be.
- **Archetype:** The Brass-Bound Runaway
- **Backstory Hook:** they survived a lift collapse by counting the gears in the dark
- **Desire:** to find the hour missing from the public clock
- **Fear:** becoming useful enough that nobody asks what they want
- **Flaw:** cannot leave a locked control untouched
- **Name:** Ada Pike
- **Personality:** hates waste more than failure / asks workers before reading the owner's manual
- **Relationship Hook:** a clock singer sends one wrong note when danger is near
- **Role:** inspector certifying machines built from confiscated homes
- **Secret:** the factory's newest engine was built from their own rejected plans
- **Signature Gear:** a compact pepperbox stamped with a factory serial
- **Skills:** tracing a steam leak by its pattern in soot / making a replacement gear from a tram token
- **Title:** The Valve That Would Not Close

**Proposed Item Candidate:**

- **Name:** Glass Petition
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a compact pepperbox stamped with a factory serial”. The sheltered recesses preserve verdigris copper and waxed wool. Its carrying arrangement is recognizable beside Ada Pike, described as “A pressure dial sits where a coat button should be.”.
- **Purpose:** Support the practical work behind “The Brass-Bound Runaway”: Ada Pike's “a compact pepperbox stamped with a factory serial” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a compact pepperbox stamped with a factory serial” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “tracing a steam leak by its pattern in soot / making a replacement gear from a tram token”; ownership does not grant those talents.
- **Origin:** Made by a clockmakers’ mutual-aid workshop. A surviving commission identifies its intended keeper as Ada Pike, “The Valve That Would Not Close”, and describes the requested object as “a compact pepperbox stamped with a factory serial”.
- **History:** It was salvaged after a forced landing at a closed station. A later custody note connects it with this account of Ada Pike: “they survived a lift collapse by counting the gears in the dark”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Ada Pike uses “a compact pepperbox stamped with a factory serial” alongside “tracing a steam leak by its pattern in soot / making a replacement gear from a tram token”. Keepers distinguish its function from the Character's own abilities: “can feel when a mechanism is about to repeat a mistake”.
- **Cost Or Drawback:** Using “a compact pepperbox stamped with a factory serial” well requires the keeper's judgment; it cannot decide whom to trust. For Ada Pike, the difficult choice is whether using it could bring about “becoming useful enough that nobody asks what they want”; “cannot leave a locked control untouched” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Its outer surfaces were recently cleaned, with older marks deliberately left intact.
- **Secret:** The most guarded account of “a compact pepperbox stamped with a factory serial” concerns Ada Pike: “the factory's newest engine was built from their own rejected plans”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a compact pepperbox stamped with a factory serial” draws in this connection: “a clock singer sends one wrong note when danger is near”. Who can speak for Ada Pike's claim? / Evidence concerning “they survived a lift collapse by counting the gears in the dark” calls the item's custody into question; Ada Pike must decide what to disclose. / Using “a compact pepperbox stamped with a factory serial” might advance “to find the hour missing from the public clock”, but a witness asks about “the factory's newest engine was built from their own rejected plans” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 12 — steampunk / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Notebook clasp is Tool, not the notebook itself. Its pipe-key use fits the clockmaker and flywheel skills; tighten the carrying description.

**Frozen Character facts:**

- **Abilities:** can feel when a mechanism is about to repeat a mistake
- **Appearance:** A brass ear trumpet is engraved with names of people ignored.
- **Archetype:** Boiler Choir Tuner
- **Backstory Hook:** their first invention was patented by a person who could not read
- **Desire:** to publish the factory's true readings before inspection day
- **Fear:** the sound of a familiar voice inside a sealed gear case
- **Flaw:** assumes every failure has one accountable operator
- **Name:** Bram Kettle
- **Personality:** keeps a ledger of favors owed by powerful people / asks workers before reading the owner's manual
- **Relationship Hook:** a union singer knows who built the forbidden engine
- **Role:** clockmaker summoned when public clocks disagree
- **Secret:** they know why the public clock skips the night shift
- **Signature Gear:** a brass notebook clasp that doubles as a pipe key
- **Skills:** balancing a flywheel by ear / tracing a steam leak by its pattern in soot
- **Title:** Keeper of the Quiet Engine

**Proposed Item Candidate:**

- **Name:** Valve Testament
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a brass notebook clasp that doubles as a pipe key”. The sheltered recesses preserve verdigris copper and waxed wool. Its carrying arrangement is recognizable beside Bram Kettle, described as “A brass ear trumpet is engraved with names of people ignored.”.
- **Purpose:** Give Bram Kettle a dependable companion for “to publish the factory's true readings before inspection day”, shaped around the familiar “a brass notebook clasp that doubles as a pipe key”.
- **Properties:** Handling marks distinguish this interpretation of “a brass notebook clasp that doubles as a pipe key” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “balancing a flywheel by ear / tracing a steam leak by its pattern in soot”; ownership does not grant those talents.
- **Origin:** Made by the engineers of an obsolete pumping station. A surviving commission identifies its intended keeper as Bram Kettle, “Keeper of the Quiet Engine”, and describes the requested object as “a brass notebook clasp that doubles as a pipe key”.
- **History:** It was salvaged after a forced landing at a closed station. A later custody note connects it with this account of Bram Kettle: “their first invention was patented by a person who could not read”. The note needs corroboration.
- **Power Or Function:** It secures the keeper's carried material and supports the additional use described in the gear account. Bram Kettle uses “a brass notebook clasp that doubles as a pipe key” alongside “balancing a flywheel by ear / tracing a steam leak by its pattern in soot”. Keepers distinguish its function from the Character's own abilities: “can feel when a mechanism is about to repeat a mistake”.
- **Cost Or Drawback:** Carrying “a brass notebook clasp that doubles as a pipe key” makes the keeper recognizable to people who know the object. For Bram Kettle, the difficult choice is whether using it could bring about “the sound of a familiar voice inside a sealed gear case”; “assumes every failure has one accountable operator” makes that judgment harder.
- **Rarity:** Few survive outside the workshop’s own circle.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** Someone recognizes “a brass notebook clasp that doubles as a pipe key” because of “they know why the public clock skips the night shift”, a fact Bram Kettle has kept out of the public account.
- **Story Hooks:** A dispute over “a brass notebook clasp that doubles as a pipe key” draws in this connection: “a union singer knows who built the forbidden engine”. Who can speak for Bram Kettle's claim? / Evidence concerning “their first invention was patented by a person who could not read” calls the item's custody into question; Bram Kettle must decide what to disclose. / Using “a brass notebook clasp that doubles as a pipe key” might advance “to publish the factory's true readings before inspection day”, but a witness asks about “they know why the public clock skips the night shift” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 13 — cyberpunk / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Smart pistol does not recover its deleted targeting model; the Character's listening ability remains separate. Tighten the neighborhood-purpose quote.

**Frozen Character facts:**

- **Abilities:** can run a silent diagnostic through nearby street sensors
- **Appearance:** Their prosthetic fingertips carry scratches from old physical keys.
- **Archetype:** Memory-Bank Salvager
- **Backstory Hook:** they learned code from repair notes left inside a public lift
- **Desire:** to make one neighborhood invisible to its rent algorithm
- **Fear:** a clean log with no record of the people harmed
- **Flaw:** trusts a clever exploit more than a frightened witness
- **Name:** Mika Imani
- **Personality:** shares a tool before explaining its risk / keeps a backup plan on paper
- **Relationship Hook:** a former client keeps sending encrypted birthday candles
- **Role:** data broker returning stolen records without charging the subject
- **Secret:** they sold a false profile to protect the person it copied
- **Signature Gear:** a smart pistol with its targeting model deleted
- **Skills:** spotting a deepfake by its mismatched room echo / spoofing a transit gate with a paper transfer
- **Title:** Witness to the Last Offline Hour

**Proposed Item Candidate:**

- **Name:** Consent Receipt
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a smart pistol with its targeting model deleted”. The repaired edges retain ceramic composite and hand-painted circuit motifs. Its carrying arrangement is recognizable beside Mika Imani, described as “Their prosthetic fingertips carry scratches from old physical keys.”.
- **Purpose:** Give Mika Imani a dependable companion for “to make one neighborhood invisible to its rent algorithm”, shaped around the familiar “a smart pistol with its targeting model deleted”.
- **Properties:** Handling marks distinguish this interpretation of “a smart pistol with its targeting model deleted” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “spotting a deepfake by its mismatched room echo / spoofing a transit gate with a paper transfer”; ownership does not grant those talents.
- **Origin:** Made by a hardware cooperative in a condemned tower. A surviving commission identifies its intended keeper as Mika Imani, “Witness to the Last Offline Hour”, and describes the requested object as “a smart pistol with its targeting model deleted”.
- **History:** It was leased under a contract that erased the makers’ names. A later custody note connects it with this account of Mika Imani: “they learned code from repair notes left inside a public lift”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Mika Imani uses “a smart pistol with its targeting model deleted” alongside “spotting a deepfake by its mismatched room echo / spoofing a transit gate with a paper transfer”. Keepers distinguish its function from the Character's own abilities: “can run a silent diagnostic through nearby street sensors”.
- **Cost Or Drawback:** Maintaining “a smart pistol with its targeting model deleted” takes time its keeper might need elsewhere. For Mika Imani, the difficult choice is whether using it could bring about “a clean log with no record of the people harmed”; “trusts a clever exploit more than a frightened witness” makes that judgment harder.
- **Rarity:** Few survive outside the workshop’s own circle.
- **Condition:** Its outer surfaces were recently cleaned, with older marks deliberately left intact.
- **Secret:** The most guarded account of “a smart pistol with its targeting model deleted” concerns Mika Imani: “they sold a false profile to protect the person it copied”. A second witness could confirm or dispute it.
- **Story Hooks:** Evidence concerning “they learned code from repair notes left inside a public lift” calls the item's custody into question; Mika Imani must decide what to disclose. / Using “a smart pistol with its targeting model deleted” might advance “to make one neighborhood invisible to its rent algorithm”, but a witness asks about “they sold a false profile to protect the person it copied” first. / A dispute over “a smart pistol with its targeting model deleted” draws in this connection: “a former client keeps sending encrypted birthday candles”. Who can speak for Mika Imani's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 14 — cyberpunk / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Auditor's pistol retains the source gear rather than pretending to be an auditing computer. The voice-licensing link is a hook, not an Item power.

**Frozen Character facts:**

- **Abilities:** can find a dead zone by listening for overlapping alerts
- **Appearance:** A cracked retinal display shows a different bus route in each eye.
- **Archetype:** Memory-Bank Salvager
- **Backstory Hook:** they lived a month in an account the city marked deceased
- **Desire:** to learn which version of their voice the city licensed
- **Fear:** losing the one memory that cannot be copied
- **Flaw:** sells access before checking who else is listening
- **Name:** Eli Cho
- **Personality:** asks whose consent is missing from the contract / keeps a paper address book for people who disappear online
- **Relationship Hook:** their sibling's voice appears in unauthorized navigation prompts
- **Role:** auditor of predictive-policing scores
- **Secret:** they sold a false profile to protect the person it copied
- **Signature Gear:** a smart pistol with its targeting model deleted
- **Skills:** recovering a transit pass from a damaged kiosk cache / spotting a deepfake by its mismatched room echo
- **Title:** The Last Clean Copy

**Proposed Item Candidate:**

- **Name:** Ghost Warranty
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a smart pistol with its targeting model deleted”. The repaired edges retain ceramic composite and hand-painted circuit motifs. Its carrying arrangement is recognizable beside Eli Cho, described as “A cracked retinal display shows a different bus route in each eye.”.
- **Purpose:** Help Eli Cho carry out “auditor of predictive-policing scores”, while pursuing “to learn which version of their voice the city licensed”. The commission centers on “a smart pistol with its targeting model deleted”.
- **Properties:** Handling marks distinguish this interpretation of “a smart pistol with its targeting model deleted” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “recovering a transit pass from a damaged kiosk cache / spotting a deepfake by its mismatched room echo”; ownership does not grant those talents.
- **Origin:** Made by a repair stall beneath an elevated data trunk. A surviving commission identifies its intended keeper as Eli Cho, “The Last Clean Copy”, and describes the requested object as “a smart pistol with its targeting model deleted”.
- **History:** It was returned under a warranty the issuer denies selling. A later custody note connects it with this account of Eli Cho: “they lived a month in an account the city marked deceased”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Eli Cho uses “a smart pistol with its targeting model deleted” alongside “recovering a transit pass from a damaged kiosk cache / spotting a deepfake by its mismatched room echo”. Keepers distinguish its function from the Character's own abilities: “can find a dead zone by listening for overlapping alerts”.
- **Cost Or Drawback:** Maintaining “a smart pistol with its targeting model deleted” takes time its keeper might need elsewhere. For Eli Cho, the difficult choice is whether using it could bring about “losing the one memory that cannot be copied”; “sells access before checking who else is listening” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** The most guarded account of “a smart pistol with its targeting model deleted” concerns Eli Cho: “they sold a false profile to protect the person it copied”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a smart pistol with its targeting model deleted” draws in this connection: “their sibling's voice appears in unauthorized navigation prompts”. Who can speak for Eli Cho's claim? / Evidence concerning “they lived a month in an account the city marked deceased” calls the item's custody into question; Eli Cho must decide what to disclose. / Using “a smart pistol with its targeting model deleted” might advance “to learn which version of their voice the city licensed”, but a witness asks about “they sold a false profile to protect the person it copied” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 15 — cyberpunk / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Signal sniffer and clinic-device skills are directly relevant. Hidden sleeve remains in the gear account; shorten the custody prose.

**Frozen Character facts:**

- **Abilities:** can run a silent diagnostic through nearby street sensors
- **Appearance:** A dead screen is stitched into their sleeve as a neighborhood map.
- **Archetype:** Roofline Clinic Runner
- **Backstory Hook:** they rebuilt a clinic terminal from discarded parts
- **Desire:** to hide clinic patients from the billing model
- **Fear:** a clean log with no record of the people harmed
- **Flaw:** keeps evidence even when it puts a friend at risk
- **Name:** Jun Sato
- **Personality:** remembers faces better than passwords / shares bandwidth before food
- **Relationship Hook:** their sibling's voice appears in unauthorized navigation prompts
- **Role:** night courier for an unlicensed clinic
- **Secret:** they sold a false profile to protect the person it copied
- **Signature Gear:** a pocket signal sniffer hidden in a transit-card sleeve
- **Skills:** building a clean device from discarded clinic parts / recovering a transit pass from a damaged kiosk cache
- **Title:** The Cache Nobody Claimed

**Proposed Item Candidate:**

- **Name:** Ghost Warranty
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a pocket signal sniffer hidden in a transit-card sleeve”. The repaired edges retain ceramic composite and hand-painted circuit motifs. Its carrying arrangement is recognizable beside Jun Sato, described as “A dead screen is stitched into their sleeve as a neighborhood map.”.
- **Purpose:** Support the practical work behind “Roofline Clinic Runner”: Jun Sato's “a pocket signal sniffer hidden in a transit-card sleeve” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a pocket signal sniffer hidden in a transit-card sleeve” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “building a clean device from discarded clinic parts / recovering a transit pass from a damaged kiosk cache”; ownership does not grant those talents.
- **Origin:** Made by the neighborhood escrow printers. A surviving commission identifies its intended keeper as Jun Sato, “The Cache Nobody Claimed”, and describes the requested object as “a pocket signal sniffer hidden in a transit-card sleeve”.
- **History:** It was returned under a warranty the issuer denies selling. A later custody note connects it with this account of Jun Sato: “they rebuilt a clinic terminal from discarded parts”. The note needs corroboration.
- **Power Or Function:** It lets its keeper receive or inspect signals through the described apparatus. Jun Sato uses “a pocket signal sniffer hidden in a transit-card sleeve” alongside “building a clean device from discarded clinic parts / recovering a transit pass from a damaged kiosk cache”. Keepers distinguish its function from the Character's own abilities: “can run a silent diagnostic through nearby street sensors”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a pocket signal sniffer hidden in a transit-card sleeve” can make its use unreliable until inspected. For Jun Sato, the difficult choice is whether using it could bring about “a clean log with no record of the people harmed”; “keeps evidence even when it puts a friend at risk” makes that judgment harder.
- **Rarity:** Its form is known; documented examples are seldom offered for trade.
- **Condition:** Ready for its intended use; its protective wrapping has been carefully repaired.
- **Secret:** The most guarded account of “a pocket signal sniffer hidden in a transit-card sleeve” concerns Jun Sato: “they sold a false profile to protect the person it copied”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a pocket signal sniffer hidden in a transit-card sleeve” draws in this connection: “their sibling's voice appears in unauthorized navigation prompts”. Who can speak for Jun Sato's claim? / Evidence concerning “they rebuilt a clinic terminal from discarded parts” calls the item's custody into question; Jun Sato must decide what to disclose. / Using “a pocket signal sniffer hidden in a transit-card sleeve” might advance “to hide clinic patients from the billing model”, but a witness asks about “they sold a false profile to protect the person it copied” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 16 — cyberpunk / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Broker's smart pistol remains a personal weapon, not a memory-editing device. The old-identity theme is coherent; tighten repeated quotation.

**Frozen Character facts:**

- **Abilities:** can trace a memory edit through a pause in a familiar story
- **Appearance:** Their prosthetic fingertips carry scratches from old physical keys.
- **Archetype:** Memory-Bank Salvager
- **Backstory Hook:** they lived a month in an account the city marked deceased
- **Desire:** to learn which version of their voice the city licensed
- **Fear:** a service notification addressed to who they used to be
- **Flaw:** stores evidence in one place because every backup feels like betrayal
- **Name:** Tariq Kade
- **Personality:** keeps a backup plan on paper / keeps a paper address book for people who disappear online
- **Relationship Hook:** a former client keeps sending encrypted birthday candles
- **Role:** broker of stolen minutes
- **Secret:** they sold a false profile to protect the person it copied
- **Signature Gear:** a smart pistol with its targeting model deleted
- **Skills:** soldering a wet neural socket / spoofing a transit gate with a paper transfer
- **Title:** Ghost in the Queue

**Proposed Item Candidate:**

- **Name:** Street Escrow
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a smart pistol with its targeting model deleted”. Its carrying wrap echoes reclaimed alloy and deliberately mismatched panels. Its carrying arrangement is recognizable beside Tariq Kade, described as “Their prosthetic fingertips carry scratches from old physical keys.”.
- **Purpose:** Keep “a smart pistol with its targeting model deleted” ready for Tariq Kade's work as “broker of stolen minutes”; its keeper hopes it can help with “to learn which version of their voice the city licensed”.
- **Properties:** Handling marks distinguish this interpretation of “a smart pistol with its targeting model deleted” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “soldering a wet neural socket / spoofing a transit gate with a paper transfer”; ownership does not grant those talents.
- **Origin:** Made by a repair stall beneath an elevated data trunk. A surviving commission identifies its intended keeper as Tariq Kade, “Ghost in the Queue”, and describes the requested object as “a smart pistol with its targeting model deleted”.
- **History:** It survived a recall enforced by private security. A later custody note connects it with this account of Tariq Kade: “they lived a month in an account the city marked deceased”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Tariq Kade uses “a smart pistol with its targeting model deleted” alongside “soldering a wet neural socket / spoofing a transit gate with a paper transfer”. Keepers distinguish its function from the Character's own abilities: “can trace a memory edit through a pause in a familiar story”.
- **Cost Or Drawback:** Carrying “a smart pistol with its targeting model deleted” makes the keeper recognizable to people who know the object. For Tariq Kade, the difficult choice is whether using it could bring about “a service notification addressed to who they used to be”; “stores evidence in one place because every backup feels like betrayal” makes that judgment harder.
- **Rarity:** Repair copies exist, while originals with intact provenance are difficult to find.
- **Condition:** Apparently ready for use, though a keeper recommends a patient inspection.
- **Secret:** One prior keeper recorded “they sold a false profile to protect the person it copied” beside Tariq Kade's name. The record was hidden in the wrapping of “a smart pistol with its targeting model deleted”.
- **Story Hooks:** Evidence concerning “they lived a month in an account the city marked deceased” calls the item's custody into question; Tariq Kade must decide what to disclose. / Using “a smart pistol with its targeting model deleted” might advance “to learn which version of their voice the city licensed”, but a witness asks about “they sold a false profile to protect the person it copied” first. / A dispute over “a smart pistol with its targeting model deleted” draws in this connection: “a former client keeps sending encrypted birthday candles”. Who can speak for Tariq Kade's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 17 — nordic-myth / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Seal-bone needle stays Tool, not a credential. Stitching and repairs fit storekeeping; no boundary-stone hearing transfers to the Item.

**Frozen Character facts:**

- **Abilities:** can hear a boundary stone answer once each winter
- **Appearance:** A cloak pin is shaped like a fish from a sea that no longer freezes.
- **Archetype:** Boundary-Stone Keeper
- **Backstory Hook:** their first voyage returned with a passenger no one invited
- **Desire:** to stop a feud passing to unborn children
- **Fear:** a feast where nobody remembers who invited them
- **Flaw:** waits for a witness when a friend needs an answer
- **Name:** Sten Varg
- **Personality:** keeps a place at the fire for an enemy / offers food before asking which side a guest serves
- **Relationship Hook:** a rival heir keeps the other half of their oath-ring
- **Role:** keeper of winter stores and old debts
- **Secret:** they crossed the fjord under their enemy's name
- **Signature Gear:** a seal-bone needle for mending sails and winter clothes
- **Skills:** repairing a shield with bone pegs / reading weather from sea-bird flight
- **Title:** Bearer of the Unfinished Verse

**Proposed Item Candidate:**

- **Name:** Winter Oath
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a seal-bone needle for mending sails and winter clothes”. Its carrying wrap echoes warm ochre and carved antler trim. Its carrying arrangement is recognizable beside Sten Varg, described as “A cloak pin is shaped like a fish from a sea that no longer freezes.”.
- **Purpose:** Support the practical work behind “Boundary-Stone Keeper”: Sten Varg's “a seal-bone needle for mending sails and winter clothes” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a seal-bone needle for mending sails and winter clothes” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “repairing a shield with bone pegs / reading weather from sea-bird flight”; ownership does not grant those talents.
- **Origin:** Made by a kin-house rebuilt after a longhouse fire. A surviving commission identifies its intended keeper as Sten Varg, “Bearer of the Unfinished Verse”, and describes the requested object as “a seal-bone needle for mending sails and winter clothes”.
- **History:** It returned from a pilgrimage whose travelers never came home. A later custody note connects it with this account of Sten Varg: “their first voyage returned with a passenger no one invited”. The note needs corroboration.
- **Power Or Function:** It helps its keeper carry out fine stitching and repairs with practiced hands. Sten Varg uses “a seal-bone needle for mending sails and winter clothes” alongside “repairing a shield with bone pegs / reading weather from sea-bird flight”. Keepers distinguish its function from the Character's own abilities: “can hear a boundary stone answer once each winter”.
- **Cost Or Drawback:** Carrying “a seal-bone needle for mending sails and winter clothes” makes the keeper recognizable to people who know the object. For Sten Varg, the difficult choice is whether using it could bring about “a feast where nobody remembers who invited them”; “waits for a witness when a friend needs an answer” makes that judgment harder.
- **Rarity:** Its form is known; documented examples are seldom offered for trade.
- **Condition:** Well preserved beneath a worn protective wrapping.
- **Secret:** One prior keeper recorded “they crossed the fjord under their enemy's name” beside Sten Varg's name. The record was hidden in the wrapping of “a seal-bone needle for mending sails and winter clothes”.
- **Story Hooks:** Evidence concerning “their first voyage returned with a passenger no one invited” calls the item's custody into question; Sten Varg must decide what to disclose. / Using “a seal-bone needle for mending sails and winter clothes” might advance “to stop a feud passing to unborn children”, but a witness asks about “they crossed the fjord under their enemy's name” first. / A dispute over “a seal-bone needle for mending sails and winter clothes” draws in this connection: “a rival heir keeps the other half of their oath-ring”. Who can speak for Sten Varg's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 18 — nordic-myth / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Whetstone supports tending a shipwright's working edge. Oath/saga custody is plausible; clarify which tool it normally maintains.

**Frozen Character facts:**

- **Abilities:** can hear a boundary stone answer once each winter
- **Appearance:** A seal-skin mantle is fastened with a coin from a kingdom that never existed.
- **Archetype:** The Birch-Woken
- **Backstory Hook:** they once carried a rival clan's dead through a storm
- **Desire:** to settle an oath before the fjord freezes over
- **Fear:** that an inherited vow will outlive everyone it was meant to serve
- **Flaw:** treats a public challenge as stronger than a private plea
- **Name:** Eirik Varg
- **Personality:** laughs at danger but not at grief / remembers who was missing from the last feast
- **Relationship Hook:** a skald edits their name out of every victory song
- **Role:** shipwright repairing vessels named in old songs
- **Secret:** they have been changing the saga so a dead rival is remembered kindly
- **Signature Gear:** a whetstone carved from the first harbor wall
- **Skills:** mending a longship seam with pine pitch and wool / reciting a lineage without naming its betrayer
- **Title:** Voice at the Whale Road

**Proposed Item Candidate:**

- **Name:** Sunless Gift
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a whetstone carved from the first harbor wall”. The sheltered recesses preserve dark stone and silver knotwork. Its carrying arrangement is recognizable beside Eirik Varg, described as “A seal-skin mantle is fastened with a coin from a kingdom that never existed.”.
- **Purpose:** Support the practical work behind “The Birch-Woken”: Eirik Varg's “a whetstone carved from the first harbor wall” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a whetstone carved from the first harbor wall” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “mending a longship seam with pine pitch and wool / reciting a lineage without naming its betrayer”; ownership does not grant those talents.
- **Origin:** Made by the craftspeople serving a winter assembly. A surviving commission identifies its intended keeper as Eirik Varg, “Voice at the Whale Road”, and describes the requested object as “a whetstone carved from the first harbor wall”.
- **History:** It was traded for a place on the last boat before the freeze. A later custody note connects it with this account of Eirik Varg: “they once carried a rival clan's dead through a storm”. The note needs corroboration.
- **Power Or Function:** It gives its keeper a surface for tending a working edge. Eirik Varg uses “a whetstone carved from the first harbor wall” alongside “mending a longship seam with pine pitch and wool / reciting a lineage without naming its betrayer”. Keepers distinguish its function from the Character's own abilities: “can hear a boundary stone answer once each winter”.
- **Cost Or Drawback:** Maintaining “a whetstone carved from the first harbor wall” takes time its keeper might need elsewhere. For Eirik Varg, the difficult choice is whether using it could bring about “that an inherited vow will outlive everyone it was meant to serve”; “treats a public challenge as stronger than a private plea” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Apparently ready for use, though a keeper recommends a patient inspection.
- **Secret:** A concealed custody note links “a whetstone carved from the first harbor wall” to this secret about Eirik Varg: “they have been changing the saga so a dead rival is remembered kindly”. It does not reveal who wrote the note.
- **Story Hooks:** Using “a whetstone carved from the first harbor wall” might advance “to settle an oath before the fjord freezes over”, but a witness asks about “they have been changing the saga so a dead rival is remembered kindly” first. / A dispute over “a whetstone carved from the first harbor wall” draws in this connection: “a skald edits their name out of every victory song”. Who can speak for Eirik Varg's claim? / Evidence concerning “they once carried a rival clan's dead through a storm” calls the item's custody into question; Eirik Varg must decide what to disclose.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 19 — nordic-myth / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Trade pistol in Nordic myth is already an explicit source fact; the contextual path preserves it. Guest-right and feud prevention form tension, not automatic aggression.

**Frozen Character facts:**

- **Abilities:** can hear a broken oath in a house beam's creak
- **Appearance:** Their mittens carry a thread from every household they visited.
- **Archetype:** Whale-Road Skald
- **Backstory Hook:** two households each claim the other abandoned them
- **Desire:** to stop a feud passing to unborn children
- **Fear:** their own voice making a promise they never chose
- **Flaw:** treats silence as agreement
- **Name:** Liv Berg
- **Personality:** asks for witnesses before making a promise / laughs at danger but not at grief
- **Relationship Hook:** a rival singer refuses the verse about their first meeting
- **Role:** keeper of guest-right at a hall shared by rival clans
- **Secret:** they were the only witness to an oath sworn by the wrong sibling
- **Signature Gear:** a trade pistol taken from a southern raider
- **Skills:** reading weather from sea-bird flight / remembering a lineage through its shared debts
- **Title:** The Hearth's Other Witness

**Proposed Item Candidate:**

- **Name:** Fjord Promise
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a trade pistol taken from a southern raider”. The sheltered recesses preserve dark stone and silver knotwork. Its carrying arrangement is recognizable beside Liv Berg, described as “Their mittens carry a thread from every household they visited.”.
- **Purpose:** Help Liv Berg carry out “keeper of guest-right at a hall shared by rival clans”, while pursuing “to stop a feud passing to unborn children”. The commission centers on “a trade pistol taken from a southern raider”.
- **Properties:** Handling marks distinguish this interpretation of “a trade pistol taken from a southern raider” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “reading weather from sea-bird flight / remembering a lineage through its shared debts”; ownership does not grant those talents.
- **Origin:** Made by a shore workshop trading with distant jarls. A surviving commission identifies its intended keeper as Liv Berg, “The Hearth's Other Witness”, and describes the requested object as “a trade pistol taken from a southern raider”.
- **History:** It returned from a pilgrimage whose travelers never came home. A later custody note connects it with this account of Liv Berg: “two households each claim the other abandoned them”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Liv Berg uses “a trade pistol taken from a southern raider” alongside “reading weather from sea-bird flight / remembering a lineage through its shared debts”. Keepers distinguish its function from the Character's own abilities: “can hear a broken oath in a house beam's creak”.
- **Cost Or Drawback:** Maintaining “a trade pistol taken from a southern raider” takes time its keeper might need elsewhere. For Liv Berg, the difficult choice is whether using it could bring about “their own voice making a promise they never chose”; “treats silence as agreement” makes that judgment harder.
- **Rarity:** Custodians exchange examples privately rather than through public markets.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** The most guarded account of “a trade pistol taken from a southern raider” concerns Liv Berg: “they were the only witness to an oath sworn by the wrong sibling”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a trade pistol taken from a southern raider” draws in this connection: “a rival singer refuses the verse about their first meeting”. Who can speak for Liv Berg's claim? / Evidence concerning “two households each claim the other abandoned them” calls the item's custody into question; Liv Berg must decide what to disclose. / Using “a trade pistol taken from a southern raider” might advance “to stop a feud passing to unborn children”, but a witness asks about “they were the only witness to an oath sworn by the wrong sibling” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 20 — nordic-myth / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Iron pot supports hearth mediation and provisioning. Names of absent guests and the founding-song secret agree; tighten the hook prose.

**Frozen Character facts:**

- **Abilities:** can hear a broken oath in a house beam's creak
- **Appearance:** A cloak pin is shaped like a fish from a sea that no longer freezes.
- **Archetype:** The Birch-Woken
- **Backstory Hook:** they returned from a winter crossing with one extra passenger and no footprints
- **Desire:** to settle an oath before the fjord freezes over
- **Fear:** the old boundary stones agreeing with one another
- **Flaw:** waits for a witness when a friend needs an answer
- **Name:** Liv Berg
- **Personality:** keeps a place at the fire for an enemy / remembers who was missing from the last feast
- **Relationship Hook:** a rival heir keeps the other half of their oath-ring
- **Role:** mediator between rival hearths
- **Secret:** they know which verse was removed from the clan's founding song
- **Signature Gear:** a small iron pot engraved with absent guests' names
- **Skills:** mending a longship seam with pine pitch and wool / repairing a shield with bone pegs
- **Title:** Last Guest of the Longhouse

**Proposed Item Candidate:**

- **Name:** Hearth Witness
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a small iron pot engraved with absent guests' names”. The repaired edges retain ash wood and braided wool. Its carrying arrangement is recognizable beside Liv Berg, described as “A cloak pin is shaped like a fish from a sea that no longer freezes.”.
- **Purpose:** Help Liv Berg carry out “mediator between rival hearths”, while pursuing “to settle an oath before the fjord freezes over”. The commission centers on “a small iron pot engraved with absent guests' names”.
- **Properties:** Handling marks distinguish this interpretation of “a small iron pot engraved with absent guests' names” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “mending a longship seam with pine pitch and wool / repairing a shield with bone pegs”; ownership does not grant those talents.
- **Origin:** Made by the seasonal workshops beside a frozen fjord. A surviving commission identifies its intended keeper as Liv Berg, “Last Guest of the Longhouse”, and describes the requested object as “a small iron pot engraved with absent guests' names”.
- **History:** It was pledged during a dispute over shelter rights. A later custody note connects it with this account of Liv Berg: “they returned from a winter crossing with one extra passenger and no footprints”. The note needs corroboration.
- **Power Or Function:** It holds the keeper's provisions; its condition limits what it can safely retain. Liv Berg uses “a small iron pot engraved with absent guests' names” alongside “mending a longship seam with pine pitch and wool / repairing a shield with bone pegs”. Keepers distinguish its function from the Character's own abilities: “can hear a broken oath in a house beam's creak”.
- **Cost Or Drawback:** Carrying “a small iron pot engraved with absent guests' names” makes the keeper recognizable to people who know the object. For Liv Berg, the difficult choice is whether using it could bring about “the old boundary stones agreeing with one another”; “waits for a witness when a friend needs an answer” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Ready for its intended use; its protective wrapping has been carefully repaired.
- **Secret:** Someone recognizes “a small iron pot engraved with absent guests' names” because of “they know which verse was removed from the clan's founding song”, a fact Liv Berg has kept out of the public account.
- **Story Hooks:** A dispute over “a small iron pot engraved with absent guests' names” draws in this connection: “a rival heir keeps the other half of their oath-ring”. Who can speak for Liv Berg's claim? / Evidence concerning “they returned from a winter crossing with one extra passenger and no footprints” calls the item's custody into question; Liv Berg must decide what to disclose. / Using “a small iron pot engraved with absent guests' names” might advance “to settle an oath before the fjord freezes over”, but a witness asks about “they know which verse was removed from the clan's founding song” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 21 — post-apocalyptic / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Receiver keeps its missing dial; the function does not repair it implicitly. Inspection and maintenance limits should be foregrounded in an edit.

**Frozen Character facts:**

- **Abilities:** can hear a dry pump's failing valve across a compound
- **Appearance:** One boot is wrapped in tire rubber, the other in a faded school scarf.
- **Archetype:** Water-Ration Mediator
- **Backstory Hook:** they crossed the salt flats following a school timetable
- **Desire:** to find the school-bus route beneath the new road
- **Fear:** being the only survivor who knows the route is unsafe
- **Flaw:** keeps walking instead of asking a settlement for help
- **Name:** Mercy Dust
- **Personality:** shares the last clean cup before asking who brought it / keeps jokes for the long walk home
- **Relationship Hook:** a roof-school child saves them a place on each visit
- **Role:** courier carrying seed samples between guarded roofs
- **Secret:** they know who silenced the old siren
- **Signature Gear:** a hand-cranked receiver with a missing dial
- **Skills:** reading storm dust against a broken highway sign / marking a safe route by the noon shade
- **Title:** The Road's Spare Key

**Proposed Item Candidate:**

- **Name:** Seed Witness
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a hand-cranked receiver with a missing dial”. The sheltered recesses preserve sealed fiber panels and green thread. Its carrying arrangement is recognizable beside Mercy Dust, described as “One boot is wrapped in tire rubber, the other in a faded school scarf.”.
- **Purpose:** Keep “a hand-cranked receiver with a missing dial” ready for Mercy Dust's work as “courier carrying seed samples between guarded roofs”; its keeper hopes it can help with “to find the school-bus route beneath the new road”.
- **Properties:** Handling marks distinguish this interpretation of “a hand-cranked receiver with a missing dial” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “reading storm dust against a broken highway sign / marking a safe route by the noon shade”; ownership does not grant those talents.
- **Origin:** Made by a settlement workshop near a seed vault. A surviving commission identifies its intended keeper as Mercy Dust, “The Road's Spare Key”, and describes the requested object as “a hand-cranked receiver with a missing dial”.
- **History:** It survived a shutdown blamed on exhausted fuel. A later custody note connects it with this account of Mercy Dust: “they crossed the salt flats following a school timetable”. The note needs corroboration.
- **Power Or Function:** It lets its keeper receive or inspect signals through the described apparatus. Mercy Dust uses “a hand-cranked receiver with a missing dial” alongside “reading storm dust against a broken highway sign / marking a safe route by the noon shade”. Keepers distinguish its function from the Character's own abilities: “can hear a dry pump's failing valve across a compound”.
- **Cost Or Drawback:** Maintaining “a hand-cranked receiver with a missing dial” takes time its keeper might need elsewhere. For Mercy Dust, the difficult choice is whether using it could bring about “being the only survivor who knows the route is unsafe”; “keeps walking instead of asking a settlement for help” makes that judgment harder.
- **Rarity:** Its form is known; documented examples are seldom offered for trade.
- **Condition:** Its outer surfaces were recently cleaned, with older marks deliberately left intact.
- **Secret:** The most guarded account of “a hand-cranked receiver with a missing dial” concerns Mercy Dust: “they know who silenced the old siren”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a hand-cranked receiver with a missing dial” draws in this connection: “a roof-school child saves them a place on each visit”. Who can speak for Mercy Dust's claim? / Evidence concerning “they crossed the salt flats following a school timetable” calls the item's custody into question; Mercy Dust must decide what to disclose. / Using “a hand-cranked receiver with a missing dial” might advance “to find the school-bus route beneath the new road”, but a witness asks about “they know who silenced the old siren” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 22 — post-apocalyptic / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Courier's rifle supplies a personal weapon, not clean-water testing or radio resurrection. Water-delivery custody fits the source stakes.

**Frozen Character facts:**

- **Abilities:** can coax one dead receiver to broadcast for a single minute
- **Appearance:** A child's red mitten is tied around a cracked water gauge.
- **Archetype:** The Glassland Stray
- **Backstory Hook:** they grew up where the evacuation siren never sounded
- **Desire:** to bring two water crews together before the dry season
- **Fear:** a full cistern with no one willing to test the water
- **Flaw:** hoards useful information like spare batteries
- **Name:** Sol Dust
- **Personality:** measures a promise by whether it can be carried / counts supplies without making people feel counted
- **Relationship Hook:** a radio operator keeps names they asked not to broadcast
- **Role:** courier carrying seed samples between guarded roofs
- **Secret:** they divert one clean-water barrel to a rival camp
- **Signature Gear:** a bolt-action rifle wrapped in a child's drawing
- **Skills:** testing well water with copper and a clean jar / marking a safe route by the noon shade
- **Title:** Last Voice on Channel Nine

**Proposed Item Candidate:**

- **Name:** Water Promise
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a bolt-action rifle wrapped in a child's drawing”. The sheltered recesses preserve sealed fiber panels and green thread. Its carrying arrangement is recognizable beside Sol Dust, described as “A child's red mitten is tied around a cracked water gauge.”.
- **Purpose:** Support the practical work behind “The Glassland Stray”: Sol Dust's “a bolt-action rifle wrapped in a child's drawing” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a bolt-action rifle wrapped in a child's drawing” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “testing well water with copper and a clean jar / marking a safe route by the noon shade”; ownership does not grant those talents.
- **Origin:** Made by a settlement workshop near a seed vault. A surviving commission identifies its intended keeper as Sol Dust, “Last Voice on Channel Nine”, and describes the requested object as “a bolt-action rifle wrapped in a child's drawing”.
- **History:** It circulated as a pledge for a promised water delivery. A later custody note connects it with this account of Sol Dust: “they grew up where the evacuation siren never sounded”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Sol Dust uses “a bolt-action rifle wrapped in a child's drawing” alongside “testing well water with copper and a clean jar / marking a safe route by the noon shade”. Keepers distinguish its function from the Character's own abilities: “can coax one dead receiver to broadcast for a single minute”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a bolt-action rifle wrapped in a child's drawing” can make its use unreliable until inspected. For Sol Dust, the difficult choice is whether using it could bring about “a full cistern with no one willing to test the water”; “hoards useful information like spare batteries” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Ready for its intended use; its protective wrapping has been carefully repaired.
- **Secret:** The most guarded account of “a bolt-action rifle wrapped in a child's drawing” concerns Sol Dust: “they divert one clean-water barrel to a rival camp”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a bolt-action rifle wrapped in a child's drawing” draws in this connection: “a radio operator keeps names they asked not to broadcast”. Who can speak for Sol Dust's claim? / Evidence concerning “they grew up where the evacuation siren never sounded” calls the item's custody into question; Sol Dust must decide what to disclose. / Using “a bolt-action rifle wrapped in a child's drawing” might advance “to bring two water crews together before the dry season”, but a witness asks about “they divert one clean-water barrel to a rival camp” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 23 — post-apocalyptic / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Seed tin is Resource because its authored contents are seed; sealed road metal is not an access seal. A short edit can distinguish contents from container.

**Frozen Character facts:**

- **Abilities:** can hear a dry pump's failing valve across a compound
- **Appearance:** Medicine labels patch their mask, though the medicines are gone.
- **Archetype:** Water-Ration Mediator
- **Backstory Hook:** their first pump repair saved a settlement that later barred them
- **Desire:** to reconnect two settlements before their shared pump runs dry
- **Fear:** a full tank with no one willing to drink first
- **Flaw:** makes ration decisions before hearing from those who wait
- **Name:** Tavi North
- **Personality:** measures a promise by whether it can be carried / offers a tool before advice
- **Relationship Hook:** a pump mechanic keeps repairing the road marker they keep stealing
- **Role:** broker between settlements that share one well
- **Secret:** they divert one clean-water barrel to a rival camp
- **Signature Gear:** a seed tin sealed with old road sign metal
- **Skills:** repairing a wind pump with bicycle chain and boot lace / testing well water with copper and a clean jar
- **Title:** The Waterline Witness

**Proposed Item Candidate:**

- **Name:** Shelter Receipt
- **Item Profile:** resource
- **Appearance:** A personal interpretation of “a seed tin sealed with old road sign metal”. A deliberate maker’s pattern uses sand-scored metal and faded yellow paint. Its carrying arrangement is recognizable beside Tavi North, described as “Medicine labels patch their mask, though the medicines are gone.”.
- **Purpose:** Give Tavi North a dependable companion for “to reconnect two settlements before their shared pump runs dry”, shaped around the familiar “a seed tin sealed with old road sign metal”.
- **Properties:** Handling marks distinguish this interpretation of “a seed tin sealed with old road sign metal” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “repairing a wind pump with bicycle chain and boot lace / testing well water with copper and a clean jar”; ownership does not grant those talents.
- **Origin:** Made by a shelter workshop reclaiming evacuation supplies. A surviving commission identifies its intended keeper as Tavi North, “The Waterline Witness”, and describes the requested object as “a seed tin sealed with old road sign metal”.
- **History:** It was exchanged for seeds later declared unusable. A later custody note connects it with this account of Tavi North: “their first pump repair saved a settlement that later barred them”. The note needs corroboration.
- **Power Or Function:** It keeps the described material available for the keeper's work; quality and supply remain limited. Tavi North uses “a seed tin sealed with old road sign metal” alongside “repairing a wind pump with bicycle chain and boot lace / testing well water with copper and a clean jar”. Keepers distinguish its function from the Character's own abilities: “can hear a dry pump's failing valve across a compound”.
- **Cost Or Drawback:** Using “a seed tin sealed with old road sign metal” well requires the keeper's judgment; it cannot decide whom to trust. For Tavi North, the difficult choice is whether using it could bring about “a full tank with no one willing to drink first”; “makes ration decisions before hearing from those who wait” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Well preserved beneath a worn protective wrapping.
- **Secret:** One prior keeper recorded “they divert one clean-water barrel to a rival camp” beside Tavi North's name. The record was hidden in the wrapping of “a seed tin sealed with old road sign metal”.
- **Story Hooks:** A dispute over “a seed tin sealed with old road sign metal” draws in this connection: “a pump mechanic keeps repairing the road marker they keep stealing”. Who can speak for Tavi North's claim? / Evidence concerning “their first pump repair saved a settlement that later barred them” calls the item's custody into question; Tavi North must decide what to disclose. / Using “a seed tin sealed with old road sign metal” might advance “to reconnect two settlements before their shared pump runs dry”, but a witness asks about “they divert one clean-water barrel to a rival camp” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 24 — post-apocalyptic / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Receiver remains Tool despite the school-map wrapping. Signal use and the Character's one-minute radio ability are explicitly distinguished.

**Frozen Character facts:**

- **Abilities:** can coax one dead receiver to broadcast for a single minute
- **Appearance:** Medicine labels patch their mask, though the medicines are gone.
- **Archetype:** Signal-Tower Salvager
- **Backstory Hook:** they guided a convoy through a dust storm by following a buried rail line
- **Desire:** to find a safe place where children can learn without taking watch
- **Fear:** a full tank with no one willing to drink first
- **Flaw:** will repair a machine before asking who it excludes
- **Name:** Mica Wells
- **Personality:** keeps a quiet seat for people who return late / keeps jokes for the long walk home
- **Relationship Hook:** a caravan child knows the route they erased from their map
- **Role:** scout who marks safe roofs in chalk
- **Secret:** the radio tower answers using their childhood nickname
- **Signature Gear:** a hand-cranked receiver wrapped in a school map
- **Skills:** building a clean-water filter from charcoal and cloth / repairing a wind pump with bicycle chain and boot lace
- **Title:** The Tin Roof's Last Guest

**Proposed Item Candidate:**

- **Name:** Water Promise
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a hand-cranked receiver wrapped in a school map”. The sheltered recesses preserve sealed fiber panels and green thread. Its carrying arrangement is recognizable beside Mica Wells, described as “Medicine labels patch their mask, though the medicines are gone.”.
- **Purpose:** Give Mica Wells a dependable companion for “to find a safe place where children can learn without taking watch”, shaped around the familiar “a hand-cranked receiver wrapped in a school map”.
- **Properties:** Handling marks distinguish this interpretation of “a hand-cranked receiver wrapped in a school map” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “building a clean-water filter from charcoal and cloth / repairing a wind pump with bicycle chain and boot lace”; ownership does not grant those talents.
- **Origin:** Made by a settlement workshop near a seed vault. A surviving commission identifies its intended keeper as Mica Wells, “The Tin Roof's Last Guest”, and describes the requested object as “a hand-cranked receiver wrapped in a school map”.
- **History:** It was traded during the closing of a refugee shelter. A later custody note connects it with this account of Mica Wells: “they guided a convoy through a dust storm by following a buried rail line”. The note needs corroboration.
- **Power Or Function:** It lets its keeper receive or inspect signals through the described apparatus. Mica Wells uses “a hand-cranked receiver wrapped in a school map” alongside “building a clean-water filter from charcoal and cloth / repairing a wind pump with bicycle chain and boot lace”. Keepers distinguish its function from the Character's own abilities: “can coax one dead receiver to broadcast for a single minute”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a hand-cranked receiver wrapped in a school map” can make its use unreliable until inspected. For Mica Wells, the difficult choice is whether using it could bring about “a full tank with no one willing to drink first”; “will repair a machine before asking who it excludes” makes that judgment harder.
- **Rarity:** Few survive outside the workshop’s own circle.
- **Condition:** Well preserved beneath a worn protective wrapping.
- **Secret:** One prior keeper recorded “the radio tower answers using their childhood nickname” beside Mica Wells's name. The record was hidden in the wrapping of “a hand-cranked receiver wrapped in a school map”.
- **Story Hooks:** Evidence concerning “they guided a convoy through a dust storm by following a buried rail line” calls the item's custody into question; Mica Wells must decide what to disclose. / Using “a hand-cranked receiver wrapped in a school map” might advance “to find a safe place where children can learn without taking watch”, but a witness asks about “the radio tower answers using their childhood nickname” first. / A dispute over “a hand-cranked receiver wrapped in a school map” draws in this connection: “a caravan child knows the route they erased from their map”. Who can speak for Mica Wells's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 25 — occult / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Key roll remains Key Item without guaranteeing entry or recognition. Missing-street embroidery and archive custody are coherent; tighten the note.

**Frozen Character facts:**

- **Abilities:** can tell when a room has been used for a promise
- **Appearance:** Their umbrella drips upward when a stranger says their name.
- **Archetype:** Séance Room Janitor
- **Backstory Hook:** their first interview was with a witness who remembered their future
- **Desire:** to return a missing person's file to the family it keeps visiting
- **Fear:** being recognized by a stranger who cannot see them
- **Flaw:** mistakes a pattern for a promise
- **Name:** Ruth Hale
- **Personality:** asks permission before entering a strange room / keeps a straight face until the kettle answers
- **Relationship Hook:** a landlord insists their room has been vacant for years
- **Role:** records clerk for complaints submitted by the missing
- **Secret:** their handwriting appears on returned photographs
- **Signature Gear:** a velvet key roll embroidered with vanished street names
- **Skills:** matching a key to scratches hidden under paint / recognizing a séance by dust left around its chairs
- **Title:** The Name in the Margin

**Proposed Item Candidate:**

- **Name:** Veiled Testament
- **Item Profile:** key-item
- **Appearance:** A personal interpretation of “a velvet key roll embroidered with vanished street names”. The repaired edges retain pale bone pigment and stitched linen. Its carrying arrangement is recognizable beside Ruth Hale, described as “Their umbrella drips upward when a stranger says their name.”.
- **Purpose:** Keep “a velvet key roll embroidered with vanished street names” ready for Ruth Hale's work as “records clerk for complaints submitted by the missing”; its keeper hopes it can help with “to return a missing person's file to the family it keeps visiting”.
- **Properties:** Handling marks distinguish this interpretation of “a velvet key roll embroidered with vanished street names” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “matching a key to scratches hidden under paint / recognizing a séance by dust left around its chairs”; ownership does not grant those talents.
- **Origin:** Made by the craftsmen of a shuttered mirror arcade. A surviving commission identifies its intended keeper as Ruth Hale, “The Name in the Margin”, and describes the requested object as “a velvet key roll embroidered with vanished street names”.
- **History:** It was removed before an archive room was sealed. A later custody note connects it with this account of Ruth Hale: “their first interview was with a witness who remembered their future”. The note needs corroboration.
- **Power Or Function:** It presents the described key or credential; access still depends on compatible locks or a recipient's recognition. Ruth Hale uses “a velvet key roll embroidered with vanished street names” alongside “matching a key to scratches hidden under paint / recognizing a séance by dust left around its chairs”. Keepers distinguish its function from the Character's own abilities: “can tell when a room has been used for a promise”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a velvet key roll embroidered with vanished street names” can make its use unreliable until inspected. For Ruth Hale, the difficult choice is whether using it could bring about “being recognized by a stranger who cannot see them”; “mistakes a pattern for a promise” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Its outer surfaces were recently cleaned, with older marks deliberately left intact.
- **Secret:** Someone recognizes “a velvet key roll embroidered with vanished street names” because of “their handwriting appears on returned photographs”, a fact Ruth Hale has kept out of the public account.
- **Story Hooks:** A dispute over “a velvet key roll embroidered with vanished street names” draws in this connection: “a landlord insists their room has been vacant for years”. Who can speak for Ruth Hale's claim? / Evidence concerning “their first interview was with a witness who remembered their future” calls the item's custody into question; Ruth Hale must decide what to disclose. / Using “a velvet key roll embroidered with vanished street names” might advance “to return a missing person's file to the family it keeps visiting”, but a witness asks about “their handwriting appears on returned photographs” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 26 — occult / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Key blank has only conditional compatible-access use; no guaranteed courthouse access is asserted. An edit can specify whether it still needs cutting.

**Frozen Character facts:**

- **Abilities:** can tell when a room has been used for a promise
- **Appearance:** Their coat pockets are lined with appointment cards for people nobody remembers.
- **Archetype:** Missing-Persons Archivist
- **Backstory Hook:** they inherited an archive whose pages were still being written
- **Desire:** to learn why the registry lists their childhood address
- **Fear:** a visitor leaving with one more shadow
- **Flaw:** keeps a case open because an ending feels like evidence
- **Name:** Lucien Bell
- **Personality:** keeps a straight face until the kettle answers / asks permission before entering a strange room
- **Relationship Hook:** a medium refuses to read the name inside their coat
- **Role:** interviewer asking witnesses what the room forgot
- **Secret:** the first name in the archive belongs to someone who has not been born
- **Signature Gear:** a key blank cut from a demolished courthouse
- **Skills:** recognizing a séance by dust left around its chairs / opening a sealed door without damaging its frame
- **Title:** The Fourth Knock

**Proposed Item Candidate:**

- **Name:** Quiet Witness
- **Item Profile:** key-item
- **Appearance:** A personal interpretation of “a key blank cut from a demolished courthouse”. Its carrying wrap echoes indigo dye and concealed gold leaf. Its carrying arrangement is recognizable beside Lucien Bell, described as “Their coat pockets are lined with appointment cards for people nobody remembers.”.
- **Purpose:** Keep “a key blank cut from a demolished courthouse” ready for Lucien Bell's work as “interviewer asking witnesses what the room forgot”; its keeper hopes it can help with “to learn why the registry lists their childhood address”.
- **Properties:** Handling marks distinguish this interpretation of “a key blank cut from a demolished courthouse” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “recognizing a séance by dust left around its chairs / opening a sealed door without damaging its frame”; ownership does not grant those talents.
- **Origin:** Made by the craftsmen of a shuttered mirror arcade. A surviving commission identifies its intended keeper as Lucien Bell, “The Fourth Knock”, and describes the requested object as “a key blank cut from a demolished courthouse”.
- **History:** It crossed estates during a contested inheritance. A later custody note connects it with this account of Lucien Bell: “they inherited an archive whose pages were still being written”. The note needs corroboration.
- **Power Or Function:** It presents the described key or credential; access still depends on compatible locks or a recipient's recognition. Lucien Bell uses “a key blank cut from a demolished courthouse” alongside “recognizing a séance by dust left around its chairs / opening a sealed door without damaging its frame”. Keepers distinguish its function from the Character's own abilities: “can tell when a room has been used for a promise”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a key blank cut from a demolished courthouse” can make its use unreliable until inspected. For Lucien Bell, the difficult choice is whether using it could bring about “a visitor leaving with one more shadow”; “keeps a case open because an ending feels like evidence” makes that judgment harder.
- **Rarity:** It is familiar locally but unusual beyond the district.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** Someone recognizes “a key blank cut from a demolished courthouse” because of “the first name in the archive belongs to someone who has not been born”, a fact Lucien Bell has kept out of the public account.
- **Story Hooks:** Evidence concerning “they inherited an archive whose pages were still being written” calls the item's custody into question; Lucien Bell must decide what to disclose. / Using “a key blank cut from a demolished courthouse” might advance “to learn why the registry lists their childhood address”, but a witness asks about “the first name in the archive belongs to someone who has not been born” first. / A dispute over “a key blank cut from a demolished courthouse” draws in this connection: “a medium refuses to read the name inside their coat”. Who can speak for Lucien Bell's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 27 — occult / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Appointment cards stay Other rather than acquiring arbitrary ritual signaling powers. A short edit can state their everyday scheduling use more directly.

**Frozen Character facts:**

- **Abilities:** can hear a lie repeated by the walls after everyone leaves
- **Appearance:** A brass key is pinned inside a lapel, though no lock claims it.
- **Archetype:** Séance Room Janitor
- **Backstory Hook:** they inherited keys from a relative whose house had no doors
- **Desire:** to close the door that has appeared in three unrelated basements
- **Fear:** a telephone ringing inside a sealed evidence bag
- **Flaw:** mistakes a pattern for a promise
- **Name:** Jonah Bell
- **Personality:** takes meticulous notes and trusts none of them / asks before touching an object left by the missing
- **Relationship Hook:** a night porter signs them into a hotel that burned down
- **Role:** records clerk for complaints submitted by the missing
- **Secret:** a locked room at home still sends rent receipts
- **Signature Gear:** blank appointment cards held by a funeral clip
- **Skills:** cataloguing voices from a damaged reel-to-reel / opening a sealed door without damaging its frame
- **Title:** Custodian of the Second Appointment

**Proposed Item Candidate:**

- **Name:** Veiled Testament
- **Item Profile:** other
- **Appearance:** A personal interpretation of “blank appointment cards held by a funeral clip”. The sheltered recesses preserve smoked glass and tiny pewter stars. Its carrying arrangement is recognizable beside Jonah Bell, described as “A brass key is pinned inside a lapel, though no lock claims it.”.
- **Purpose:** Help Jonah Bell carry out “records clerk for complaints submitted by the missing”, while pursuing “to close the door that has appeared in three unrelated basements”. The commission centers on “blank appointment cards held by a funeral clip”.
- **Properties:** Handling marks distinguish this interpretation of “blank appointment cards held by a funeral clip” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “cataloguing voices from a damaged reel-to-reel / opening a sealed door without damaging its frame”; ownership does not grant those talents.
- **Origin:** Made by the makers serving a house of ritual thresholds. A surviving commission identifies its intended keeper as Jonah Bell, “Custodian of the Second Appointment”, and describes the requested object as “blank appointment cards held by a funeral clip”.
- **History:** It was seized during an inquiry into altered reflections. A later custody note connects it with this account of Jonah Bell: “they inherited keys from a relative whose house had no doors”. The note needs corroboration.
- **Power Or Function:** It makes the described personal object available for its keeper's work; the account of the gear sets its practical limits. Jonah Bell uses “blank appointment cards held by a funeral clip” alongside “cataloguing voices from a damaged reel-to-reel / opening a sealed door without damaging its frame”. Keepers distinguish its function from the Character's own abilities: “can hear a lie repeated by the walls after everyone leaves”.
- **Cost Or Drawback:** Using “blank appointment cards held by a funeral clip” well requires the keeper's judgment; it cannot decide whom to trust. For Jonah Bell, the difficult choice is whether using it could bring about “a telephone ringing inside a sealed evidence bag”; “mistakes a pattern for a promise” makes that judgment harder.
- **Rarity:** Few survive outside the workshop’s own circle.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** A concealed custody note links “blank appointment cards held by a funeral clip” to this secret about Jonah Bell: “a locked room at home still sends rent receipts”. It does not reveal who wrote the note.
- **Story Hooks:** A dispute over “blank appointment cards held by a funeral clip” draws in this connection: “a night porter signs them into a hotel that burned down”. Who can speak for Jonah Bell's claim? / Evidence concerning “they inherited keys from a relative whose house had no doors” calls the item's custody into question; Jonah Bell must decide what to disclose. / Using “blank appointment cards held by a funeral clip” might advance “to close the door that has appeared in three unrelated basements”, but a witness asks about “a locked room at home still sends rent receipts” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 28 — occult / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Notebook remains Book and keeps its authored refusal of false names. The output does not transfer wall-listening ability; tighten redundant source quotes.

**Frozen Character facts:**

- **Abilities:** can hear a lie repeated by the walls after everyone leaves
- **Appearance:** One eye waters whenever a door opens onto the wrong floor.
- **Archetype:** Missing-Persons Archivist
- **Backstory Hook:** they indexed records from a town with no postal code
- **Desire:** to return a recording to the person who has not made it
- **Fear:** a familiar knock from inside a locked drawer
- **Flaw:** mistakes a pattern for a promise
- **Name:** Ruth Hale
- **Personality:** asks permission before entering a strange room / takes meticulous notes and trusts none of them
- **Relationship Hook:** a detective sends a case number when their reflection vanishes
- **Role:** guide for clients who will not give a legal name
- **Secret:** their reflection leaves a room several seconds late
- **Signature Gear:** a pocket notebook that refuses to hold a false name
- **Skills:** cataloguing voices from a damaged reel-to-reel / recognizing a false séance by the room's dust
- **Title:** The Name Behind the Glass

**Proposed Item Candidate:**

- **Name:** Veiled Testament
- **Item Profile:** book
- **Appearance:** A personal interpretation of “a pocket notebook that refuses to hold a false name”. The sheltered recesses preserve smoked glass and tiny pewter stars. Its carrying arrangement is recognizable beside Ruth Hale, described as “One eye waters whenever a door opens onto the wrong floor.”.
- **Purpose:** Give Ruth Hale a dependable companion for “to return a recording to the person who has not made it”, shaped around the familiar “a pocket notebook that refuses to hold a false name”.
- **Properties:** Handling marks distinguish this interpretation of “a pocket notebook that refuses to hold a false name” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “cataloguing voices from a damaged reel-to-reel / recognizing a false séance by the room's dust”; ownership does not grant those talents.
- **Origin:** Made by the workshops behind a séance parlor. A surviving commission identifies its intended keeper as Ruth Hale, “The Name Behind the Glass”, and describes the requested object as “a pocket notebook that refuses to hold a false name”.
- **History:** It was seized during an inquiry into altered reflections. A later custody note connects it with this account of Ruth Hale: “they indexed records from a town with no postal code”. The note needs corroboration.
- **Power Or Function:** It keeps written material available for reading, comparison, and annotation; it cannot establish the truth of an entry. Ruth Hale uses “a pocket notebook that refuses to hold a false name” alongside “cataloguing voices from a damaged reel-to-reel / recognizing a false séance by the room's dust”. Keepers distinguish its function from the Character's own abilities: “can hear a lie repeated by the walls after everyone leaves”.
- **Cost Or Drawback:** Using “a pocket notebook that refuses to hold a false name” well requires the keeper's judgment; it cannot decide whom to trust. For Ruth Hale, the difficult choice is whether using it could bring about “a familiar knock from inside a locked drawer”; “mistakes a pattern for a promise” makes that judgment harder.
- **Rarity:** It is familiar locally but unusual beyond the district.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** One prior keeper recorded “their reflection leaves a room several seconds late” beside Ruth Hale's name. The record was hidden in the wrapping of “a pocket notebook that refuses to hold a false name”.
- **Story Hooks:** A dispute over “a pocket notebook that refuses to hold a false name” draws in this connection: “a detective sends a case number when their reflection vanishes”. Who can speak for Ruth Hale's claim? / Evidence concerning “they indexed records from a town with no postal code” calls the item's custody into question; Ruth Hale must decide what to disclose. / Using “a pocket notebook that refuses to hold a false name” might advance “to return a recording to the person who has not made it”, but a witness asks about “their reflection leaves a room several seconds late” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 29 — monster-hunter / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Hunting rifle respects the spared-quarry inscription and humane intent; it does not grant beast-calming. Clarify that possession does not require killing.

**Frozen Character facts:**

- **Abilities:** can hold a frightened beast still long enough to understand it
- **Appearance:** A leather sleeve is patched with tags from quarries they released.
- **Archetype:** Witness-First Exorcist
- **Backstory Hook:** their first license was signed by a person who later became the quarry
- **Desire:** to prove a so-called monster guards the water source
- **Fear:** a fresh track that circles their own camp from inside
- **Flaw:** waits for a second witness while the first is in danger
- **Name:** Anja Dahl
- **Personality:** remembers every animal's name / asks what the creature protected before counting the damage
- **Relationship Hook:** a rival hunter sends warning whistles before crossing their ground
- **Role:** broker who keeps villages from selling the wrong carcass
- **Secret:** their shadow carries a set of claws no lamp can explain
- **Signature Gear:** a short hunting rifle marked with the names of spared quarries
- **Skills:** calming a village after a false alarm / setting a cage that closes only when the animal is safe
- **Title:** The Hunter Who Waited

**Proposed Item Candidate:**

- **Name:** Trail Pledge
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a short hunting rifle marked with the names of spared quarries”. Its carrying wrap echoes charred leather and dark green pigment. Its carrying arrangement is recognizable beside Anja Dahl, described as “A leather sleeve is patched with tags from quarries they released.”.
- **Purpose:** Support the practical work behind “Witness-First Exorcist”: Anja Dahl's “a short hunting rifle marked with the names of spared quarries” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a short hunting rifle marked with the names of spared quarries” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “calming a village after a false alarm / setting a cage that closes only when the animal is safe”; ownership does not grant those talents.
- **Origin:** Made by a workshop studying seasonal migrations. A surviving commission identifies its intended keeper as Anja Dahl, “The Hunter Who Waited”, and describes the requested object as “a short hunting rifle marked with the names of spared quarries”.
- **History:** It traveled with a survey crew whose findings were suppressed. A later custody note connects it with this account of Anja Dahl: “their first license was signed by a person who later became the quarry”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Anja Dahl uses “a short hunting rifle marked with the names of spared quarries” alongside “calming a village after a false alarm / setting a cage that closes only when the animal is safe”. Keepers distinguish its function from the Character's own abilities: “can hold a frightened beast still long enough to understand it”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a short hunting rifle marked with the names of spared quarries” can make its use unreliable until inspected. For Anja Dahl, the difficult choice is whether using it could bring about “a fresh track that circles their own camp from inside”; “waits for a second witness while the first is in danger” makes that judgment harder.
- **Rarity:** Custodians exchange examples privately rather than through public markets.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** A concealed custody note links “a short hunting rifle marked with the names of spared quarries” to this secret about Anja Dahl: “their shadow carries a set of claws no lamp can explain”. It does not reveal who wrote the note.
- **Story Hooks:** Using “a short hunting rifle marked with the names of spared quarries” might advance “to prove a so-called monster guards the water source”, but a witness asks about “their shadow carries a set of claws no lamp can explain” first. / A dispute over “a short hunting rifle marked with the names of spared quarries” draws in this connection: “a rival hunter sends warning whistles before crossing their ground”. Who can speak for Anja Dahl's claim? / Evidence concerning “their first license was signed by a person who later became the quarry” calls the item's custody into question; Anja Dahl must decide what to disclose.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 30 — monster-hunter / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Cage stays Tool despite map lining; enclosure and inspection fit a sanctuary keeper. Custody hook supports the altered-license secret.

**Frozen Character facts:**

- **Abilities:** can hold a frightened beast still long enough to understand it
- **Appearance:** Their field glasses are scratched by something that never approached.
- **Archetype:** Carcass-Tax Inspector
- **Backstory Hook:** they learned field medicine from the creature's former keeper
- **Desire:** to prove a so-called monster guards the water source
- **Fear:** finding a familiar name carved into the quarry's den
- **Flaw:** keeps a specimen after its owner asks for it back
- **Name:** Mirelle Kestrel
- **Personality:** asks what the creature protected before counting the damage / keeps a spare blanket for witnesses
- **Relationship Hook:** a village healer keeps treating the creature they are licensed to kill
- **Role:** keeper of a sanctuary for beasts named in old licenses
- **Secret:** they have been altering the license ledger to protect a village's sacred predator
- **Signature Gear:** a folding cage lined with old campaign maps
- **Skills:** calming a village after a false alarm / casting a track in wet plaster
- **Title:** Keeper of the Empty Cage

**Proposed Item Candidate:**

- **Name:** Molt Testament
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a folding cage lined with old campaign maps”. The repaired edges retain packed clay glaze and tough fiber. Its carrying arrangement is recognizable beside Mirelle Kestrel, described as “Their field glasses are scratched by something that never approached.”.
- **Purpose:** Give Mirelle Kestrel a dependable companion for “to prove a so-called monster guards the water source”, shaped around the familiar “a folding cage lined with old campaign maps”.
- **Properties:** Handling marks distinguish this interpretation of “a folding cage lined with old campaign maps” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “calming a village after a false alarm / casting a track in wet plaster”; ownership does not grant those talents.
- **Origin:** Made by a camp repairing gear near a burrow network. A surviving commission identifies its intended keeper as Mirelle Kestrel, “Keeper of the Empty Cage”, and describes the requested object as “a folding cage lined with old campaign maps”.
- **History:** It was pledged during a hunt abandoned near a breeding ground. A later custody note connects it with this account of Mirelle Kestrel: “they learned field medicine from the creature's former keeper”. The note needs corroboration.
- **Power Or Function:** It provides a portable enclosure; its latch and lining still require inspection before use. Mirelle Kestrel uses “a folding cage lined with old campaign maps” alongside “calming a village after a false alarm / casting a track in wet plaster”. Keepers distinguish its function from the Character's own abilities: “can hold a frightened beast still long enough to understand it”.
- **Cost Or Drawback:** Carrying “a folding cage lined with old campaign maps” makes the keeper recognizable to people who know the object. For Mirelle Kestrel, the difficult choice is whether using it could bring about “finding a familiar name carved into the quarry's den”; “keeps a specimen after its owner asks for it back” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** A concealed custody note links “a folding cage lined with old campaign maps” to this secret about Mirelle Kestrel: “they have been altering the license ledger to protect a village's sacred predator”. It does not reveal who wrote the note.
- **Story Hooks:** Evidence concerning “they learned field medicine from the creature's former keeper” calls the item's custody into question; Mirelle Kestrel must decide what to disclose. / Using “a folding cage lined with old campaign maps” might advance “to prove a so-called monster guards the water source”, but a witness asks about “they have been altering the license ledger to protect a village's sacred predator” first. / A dispute over “a folding cage lined with old campaign maps” draws in this connection: “a village healer keeps treating the creature they are licensed to kill”. Who can speak for Mirelle Kestrel's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 31 — monster-hunter / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Inside-opening cage remains an enclosure with a latch to inspect, not a guaranteed escape-proof trap. The source's cautious interviewing role fits.

**Frozen Character facts:**

- **Abilities:** can tell a fresh molt from a false skin by touch
- **Appearance:** A field coat is patched with hide that does not match any known animal.
- **Archetype:** Molt-Pattern Tracker
- **Backstory Hook:** their first hunt ended when the quarry returned a lost child's shoe
- **Desire:** to learn who baits creatures toward the village
- **Fear:** mistaking a frightened person for a dangerous beast
- **Flaw:** waits for perfect proof while someone is in danger
- **Name:** Mirelle Dahl
- **Personality:** asks what the creature protected before counting the damage / asks what the creature protects before asking what it killed
- **Relationship Hook:** a village healer keeps treating the creature they are licensed to kill
- **Role:** licensed hunter who interviews witnesses before loading a weapon
- **Secret:** they have been altering the license ledger to protect a village's sacred predator
- **Signature Gear:** a field cage whose latch opens from the inside
- **Skills:** casting a track in wet plaster / calming a panicked witness with field triage
- **Title:** The Witness Beyond the Fence

**Proposed Item Candidate:**

- **Name:** Scale Witness
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a field cage whose latch opens from the inside”. A deliberate maker’s pattern uses feather-pattern enamel and polished bone. Its carrying arrangement is recognizable beside Mirelle Dahl, described as “A field coat is patched with hide that does not match any known animal.”.
- **Purpose:** Keep “a field cage whose latch opens from the inside” ready for Mirelle Dahl's work as “licensed hunter who interviews witnesses before loading a weapon”; its keeper hopes it can help with “to learn who baits creatures toward the village”.
- **Properties:** Handling marks distinguish this interpretation of “a field cage whose latch opens from the inside” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “casting a track in wet plaster / calming a panicked witness with field triage”; ownership does not grant those talents.
- **Origin:** Made by a workshop studying seasonal migrations. A surviving commission identifies its intended keeper as Mirelle Dahl, “The Witness Beyond the Fence”, and describes the requested object as “a field cage whose latch opens from the inside”.
- **History:** It traveled with a survey crew whose findings were suppressed. A later custody note connects it with this account of Mirelle Dahl: “their first hunt ended when the quarry returned a lost child's shoe”. The note needs corroboration.
- **Power Or Function:** It provides a portable enclosure; its latch and lining still require inspection before use. Mirelle Dahl uses “a field cage whose latch opens from the inside” alongside “casting a track in wet plaster / calming a panicked witness with field triage”. Keepers distinguish its function from the Character's own abilities: “can tell a fresh molt from a false skin by touch”.
- **Cost Or Drawback:** Using “a field cage whose latch opens from the inside” well requires the keeper's judgment; it cannot decide whom to trust. For Mirelle Dahl, the difficult choice is whether using it could bring about “mistaking a frightened person for a dangerous beast”; “waits for perfect proof while someone is in danger” makes that judgment harder.
- **Rarity:** Custodians exchange examples privately rather than through public markets.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** A concealed custody note links “a field cage whose latch opens from the inside” to this secret about Mirelle Dahl: “they have been altering the license ledger to protect a village's sacred predator”. It does not reveal who wrote the note.
- **Story Hooks:** A dispute over “a field cage whose latch opens from the inside” draws in this connection: “a village healer keeps treating the creature they are licensed to kill”. Who can speak for Mirelle Dahl's claim? / Evidence concerning “their first hunt ended when the quarry returned a lost child's shoe” calls the item's custody into question; Mirelle Dahl must decide what to disclose. / Using “a field cage whose latch opens from the inside” might advance “to learn who baits creatures toward the village”, but a witness asks about “they have been altering the license ledger to protect a village's sacred predator” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 32 — monster-hunter / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Cage keeps its maps as lining and does not become a Book. Examiner/negotiated-retreat history supports nonlethal use; tighten the quotes.

**Frozen Character facts:**

- **Abilities:** can tell a fresh molt from a false skin by touch
- **Appearance:** Their field glasses are scratched by something that never approached.
- **Archetype:** Carcass-Tax Inspector
- **Backstory Hook:** their first license was signed by a person who later became the quarry
- **Desire:** to prove the quarry has been guarding the village boundary
- **Fear:** becoming the only person who can recognize the next molt
- **Flaw:** keeps dangerous trophies to remember the cost
- **Name:** Anja Rusk
- **Personality:** asks what the creature protects before asking what it killed / keeps a spare blanket for witnesses
- **Relationship Hook:** a rival hunter leaves a warning marker instead of taking the quarry
- **Role:** examiner recording what a creature leaves untouched
- **Secret:** they reported a kill that was actually a negotiated retreat
- **Signature Gear:** a folding cage lined with old campaign maps
- **Skills:** casting a track in wet plaster / calming a village after a false alarm
- **Title:** Keeper of the Empty Cage

**Proposed Item Candidate:**

- **Name:** Scale Witness
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a folding cage lined with old campaign maps”. The repaired edges retain packed clay glaze and tough fiber. Its carrying arrangement is recognizable beside Anja Rusk, described as “Their field glasses are scratched by something that never approached.”.
- **Purpose:** Keep “a folding cage lined with old campaign maps” ready for Anja Rusk's work as “examiner recording what a creature leaves untouched”; its keeper hopes it can help with “to prove the quarry has been guarding the village boundary”.
- **Properties:** Handling marks distinguish this interpretation of “a folding cage lined with old campaign maps” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “casting a track in wet plaster / calming a village after a false alarm”; ownership does not grant those talents.
- **Origin:** Made by a workshop supporting migrating hunt camps. A surviving commission identifies its intended keeper as Anja Rusk, “Keeper of the Empty Cage”, and describes the requested object as “a folding cage lined with old campaign maps”.
- **History:** It passed between crews after an unexplained tunnel collapse. A later custody note connects it with this account of Anja Rusk: “their first license was signed by a person who later became the quarry”. The note needs corroboration.
- **Power Or Function:** It provides a portable enclosure; its latch and lining still require inspection before use. Anja Rusk uses “a folding cage lined with old campaign maps” alongside “casting a track in wet plaster / calming a village after a false alarm”. Keepers distinguish its function from the Character's own abilities: “can tell a fresh molt from a false skin by touch”.
- **Cost Or Drawback:** Maintaining “a folding cage lined with old campaign maps” takes time its keeper might need elsewhere. For Anja Rusk, the difficult choice is whether using it could bring about “becoming the only person who can recognize the next molt”; “keeps dangerous trophies to remember the cost” makes that judgment harder.
- **Rarity:** Repair copies exist, while originals with intact provenance are difficult to find.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** One prior keeper recorded “they reported a kill that was actually a negotiated retreat” beside Anja Rusk's name. The record was hidden in the wrapping of “a folding cage lined with old campaign maps”.
- **Story Hooks:** A dispute over “a folding cage lined with old campaign maps” draws in this connection: “a rival hunter leaves a warning marker instead of taking the quarry”. Who can speak for Anja Rusk's claim? / Evidence concerning “their first license was signed by a person who later became the quarry” calls the item's custody into question; Anja Rusk must decide what to disclose. / Using “a folding cage lined with old campaign maps” might advance “to prove the quarry has been guarding the village boundary”, but a witness asks about “they reported a kill that was actually a negotiated retreat” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 33 — rogue-assassin / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Pistol supports an exit-plan courier without granting disguise or alibi-reading abilities. Void contract and protected target form a useful custody hook.

**Frozen Character facts:**

- **Abilities:** can spot a staged alibi by the missing inconvenience
- **Appearance:** A plain gray coat is tailored with pockets for other people's signatures.
- **Archetype:** Ledger-Page Infiltrator
- **Backstory Hook:** their first contract paid them to protect its named target
- **Desire:** to leave the city without making someone else take the blame
- **Fear:** a room with only one exit and no visible lock
- **Flaw:** keeps a secret after it has stopped protecting anyone
- **Name:** Vera Quill
- **Personality:** keeps one honest answer ready / checks who pays for a promise before making one
- **Relationship Hook:** a guild clerk edits their old score by one point each month
- **Role:** courier carrying exit plans instead of messages
- **Secret:** their reflection still performs an unfinished disguise
- **Signature Gear:** a compact pistol wrapped in a contract marked void
- **Skills:** refitting a coat with two stitches and a borrowed pin / copying a seal from its pressure mark on paper
- **Title:** The Empty Glove

**Proposed Item Candidate:**

- **Name:** Alley Covenant
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a compact pistol wrapped in a contract marked void”. A deliberate maker’s pattern uses indigo cloth and weathered bronze. Its carrying arrangement is recognizable beside Vera Quill, described as “A plain gray coat is tailored with pockets for other people's signatures.”.
- **Purpose:** Help Vera Quill carry out “courier carrying exit plans instead of messages”, while pursuing “to leave the city without making someone else take the blame”. The commission centers on “a compact pistol wrapped in a contract marked void”.
- **Properties:** Handling marks distinguish this interpretation of “a compact pistol wrapped in a contract marked void” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “refitting a coat with two stitches and a borrowed pin / copying a seal from its pressure mark on paper”; ownership does not grant those talents.
- **Origin:** Made by the craftspeople beneath a closed customs office. A surviving commission identifies its intended keeper as Vera Quill, “The Empty Glove”, and describes the requested object as “a compact pistol wrapped in a contract marked void”.
- **History:** It crossed checkpoints hidden in legitimate repair work. A later custody note connects it with this account of Vera Quill: “their first contract paid them to protect its named target”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Vera Quill uses “a compact pistol wrapped in a contract marked void” alongside “refitting a coat with two stitches and a borrowed pin / copying a seal from its pressure mark on paper”. Keepers distinguish its function from the Character's own abilities: “can spot a staged alibi by the missing inconvenience”.
- **Cost Or Drawback:** Carrying “a compact pistol wrapped in a contract marked void” makes the keeper recognizable to people who know the object. For Vera Quill, the difficult choice is whether using it could bring about “a room with only one exit and no visible lock”; “keeps a secret after it has stopped protecting anyone” makes that judgment harder.
- **Rarity:** It is familiar locally but unusual beyond the district.
- **Condition:** Its outer surfaces were recently cleaned, with older marks deliberately left intact.
- **Secret:** One prior keeper recorded “their reflection still performs an unfinished disguise” beside Vera Quill's name. The record was hidden in the wrapping of “a compact pistol wrapped in a contract marked void”.
- **Story Hooks:** Using “a compact pistol wrapped in a contract marked void” might advance “to leave the city without making someone else take the blame”, but a witness asks about “their reflection still performs an unfinished disguise” first. / A dispute over “a compact pistol wrapped in a contract marked void” draws in this connection: “a guild clerk edits their old score by one point each month”. Who can speak for Vera Quill's claim? / Evidence concerning “their first contract paid them to protect its named target” calls the item's custody into question; Vera Quill must decide what to disclose.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 34 — rogue-assassin / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Reversible scarf remains Other and keeps the two-name disguise cue. A short edit can make its clothing function more concrete.

**Frozen Character facts:**

- **Abilities:** can throw a blade to cut a bell rope without striking the bell
- **Appearance:** Their gloves carry faint chalk marks from doors they chose not to open.
- **Archetype:** Witness-Sparing Cutpurse
- **Backstory Hook:** their first escape route led through a room full of people they had wronged
- **Desire:** to leave the guild with one witness still willing to speak
- **Fear:** the guild recognizing their handwriting in a warning
- **Flaw:** turns a simple request into a loyalty test
- **Name:** Nox Moth
- **Personality:** lets a silence finish before deciding it is a threat / checks who pays for a promise before making one
- **Relationship Hook:** a watch officer keeps leaving the same window unlocked
- **Role:** accountant tracking contracts never accepted
- **Secret:** they left a witness alive who now believes the wrong person saved them
- **Signature Gear:** a reversible scarf stitched with two different names
- **Skills:** finding the quiet stairwell in a crowded building / refitting a coat with two stitches and a borrowed pin
- **Title:** The Empty Glove

**Proposed Item Candidate:**

- **Name:** Dusk Witness
- **Item Profile:** other
- **Appearance:** A personal interpretation of “a reversible scarf stitched with two different names”. Its carrying wrap echoes matte ceramic and hand-filed copper. Its carrying arrangement is recognizable beside Nox Moth, described as “Their gloves carry faint chalk marks from doors they chose not to open.”.
- **Purpose:** Keep “a reversible scarf stitched with two different names” ready for Nox Moth's work as “accountant tracking contracts never accepted”; its keeper hopes it can help with “to leave the guild with one witness still willing to speak”.
- **Properties:** Handling marks distinguish this interpretation of “a reversible scarf stitched with two different names” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “finding the quiet stairwell in a crowded building / refitting a coat with two stitches and a borrowed pin”; ownership does not grant those talents.
- **Origin:** Made by the artisans maintaining a neutral meeting house. A surviving commission identifies its intended keeper as Nox Moth, “The Empty Glove”, and describes the requested object as “a reversible scarf stitched with two different names”.
- **History:** It was left behind when negotiators vanished. A later custody note connects it with this account of Nox Moth: “their first escape route led through a room full of people they had wronged”. The note needs corroboration.
- **Power Or Function:** It makes the described personal object available for its keeper's work; the account of the gear sets its practical limits. Nox Moth uses “a reversible scarf stitched with two different names” alongside “finding the quiet stairwell in a crowded building / refitting a coat with two stitches and a borrowed pin”. Keepers distinguish its function from the Character's own abilities: “can throw a blade to cut a bell rope without striking the bell”.
- **Cost Or Drawback:** Carrying “a reversible scarf stitched with two different names” makes the keeper recognizable to people who know the object. For Nox Moth, the difficult choice is whether using it could bring about “the guild recognizing their handwriting in a warning”; “turns a simple request into a loyalty test” makes that judgment harder.
- **Rarity:** It is familiar locally but unusual beyond the district.
- **Condition:** Ready for its intended use; its protective wrapping has been carefully repaired.
- **Secret:** The most guarded account of “a reversible scarf stitched with two different names” concerns Nox Moth: “they left a witness alive who now believes the wrong person saved them”. A second witness could confirm or dispute it.
- **Story Hooks:** A dispute over “a reversible scarf stitched with two different names” draws in this connection: “a watch officer keeps leaving the same window unlocked”. Who can speak for Nox Moth's claim? / Evidence concerning “their first escape route led through a room full of people they had wronged” calls the item's custody into question; Nox Moth must decide what to disclose. / Using “a reversible scarf stitched with two different names” might advance “to leave the guild with one witness still willing to speak”, but a witness asks about “they left a witness alive who now believes the wrong person saved them” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 35 — rogue-assassin / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Seal kit stays Tool, not automatic permission. Precise marking fits copying seals and reading ledgers; its lack of house crest is retained.

**Frozen Character facts:**

- **Abilities:** can throw a blade to cut a bell rope without striking the bell
- **Appearance:** Their polished shoes carry brick dust from an old escape route.
- **Archetype:** Ledger-Page Infiltrator
- **Backstory Hook:** they escaped a locked house by following a child who lived there
- **Desire:** to recover a ledger page that proves who ordered the old betrayal
- **Fear:** the guild recognizing their handwriting in a warning
- **Flaw:** keeps a contingency for every friend and calls it care
- **Name:** Rin Quill
- **Personality:** checks who pays for a promise before making one / refuses a job described as simple
- **Relationship Hook:** a guild clerk edits their old score by one point each month
- **Role:** courier for people who cannot appear together
- **Secret:** they have been falsifying their own guild score to keep a rival employed
- **Signature Gear:** a wax seal kit whose stamp carries no house crest
- **Skills:** opening a ledger without disturbing its dust / copying a seal from its pressure mark on paper
- **Title:** Last Through the Door

**Proposed Item Candidate:**

- **Name:** Alley Covenant
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a wax seal kit whose stamp carries no house crest”. The sheltered recesses preserve faded red lacquer and stitched velvet. Its carrying arrangement is recognizable beside Rin Quill, described as “Their polished shoes carry brick dust from an old escape route.”.
- **Purpose:** Support the practical work behind “Ledger-Page Infiltrator”: Rin Quill's “a wax seal kit whose stamp carries no house crest” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a wax seal kit whose stamp carries no house crest” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “opening a ledger without disturbing its dust / copying a seal from its pressure mark on paper”; ownership does not grant those talents.
- **Origin:** Made by the artisans maintaining a neutral meeting house. A surviving commission identifies its intended keeper as Rin Quill, “Last Through the Door”, and describes the requested object as “a wax seal kit whose stamp carries no house crest”.
- **History:** It was pledged during a failed exchange of safe-conduct papers. A later custody note connects it with this account of Rin Quill: “they escaped a locked house by following a child who lived there”. The note needs corroboration.
- **Power Or Function:** It supports precise manipulation or marking; compatible hardware and the keeper's skill remain necessary. Rin Quill uses “a wax seal kit whose stamp carries no house crest” alongside “opening a ledger without disturbing its dust / copying a seal from its pressure mark on paper”. Keepers distinguish its function from the Character's own abilities: “can throw a blade to cut a bell rope without striking the bell”.
- **Cost Or Drawback:** Using “a wax seal kit whose stamp carries no house crest” well requires the keeper's judgment; it cannot decide whom to trust. For Rin Quill, the difficult choice is whether using it could bring about “the guild recognizing their handwriting in a warning”; “keeps a contingency for every friend and calls it care” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Ready for its intended use; its protective wrapping has been carefully repaired.
- **Secret:** A concealed custody note links “a wax seal kit whose stamp carries no house crest” to this secret about Rin Quill: “they have been falsifying their own guild score to keep a rival employed”. It does not reveal who wrote the note.
- **Story Hooks:** Evidence concerning “they escaped a locked house by following a child who lived there” calls the item's custody into question; Rin Quill must decide what to disclose. / Using “a wax seal kit whose stamp carries no house crest” might advance “to recover a ledger page that proves who ordered the old betrayal”, but a witness asks about “they have been falsifying their own guild score to keep a rival employed” first. / A dispute over “a wax seal kit whose stamp carries no house crest” draws in this connection: “a guild clerk edits their old score by one point each month”. Who can speak for Rin Quill's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 36 — rogue-assassin / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Lock shim remains Tool and needs compatible hardware and skill. It does not make its accountant keeper a perfect lockpicker; tighten the purpose.

**Frozen Character facts:**

- **Abilities:** can remember a lock's sound after hearing it once
- **Appearance:** A narrow scar crosses the palm where a contract once tried to bind them.
- **Archetype:** Ledger-Page Infiltrator
- **Backstory Hook:** they inherited a contract that renews whenever they tell the truth
- **Desire:** to recover the key to the guild's sealed accounts
- **Fear:** the guild recognizing their handwriting in a warning
- **Flaw:** leaves before hearing how a rescued person feels
- **Name:** Sami Rook
- **Personality:** learns the servant's name before the target's / checks who pays for a promise before making one
- **Relationship Hook:** a guild accountant knows which jobs they falsified
- **Role:** accountant tracking contracts never accepted
- **Secret:** a contract in their pocket names no client but them
- **Signature Gear:** a lock shim cut from a theater ticket
- **Skills:** refitting a coat with two stitches and a borrowed pin / finding the quiet stairwell in a crowded building
- **Title:** The Door Left Open

**Proposed Item Candidate:**

- **Name:** Last Courtesy
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a lock shim cut from a theater ticket”. The sheltered recesses preserve faded red lacquer and stitched velvet. Its carrying arrangement is recognizable beside Sami Rook, described as “A narrow scar crosses the palm where a contract once tried to bind them.”.
- **Purpose:** Help Sami Rook carry out “accountant tracking contracts never accepted”, while pursuing “to recover the key to the guild's sealed accounts”. The commission centers on “a lock shim cut from a theater ticket”.
- **Properties:** Handling marks distinguish this interpretation of “a lock shim cut from a theater ticket” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “refitting a coat with two stitches and a borrowed pin / finding the quiet stairwell in a crowded building”; ownership does not grant those talents.
- **Origin:** Made by a backstreet workshop serving anonymous clients. A surviving commission identifies its intended keeper as Sami Rook, “The Door Left Open”, and describes the requested object as “a lock shim cut from a theater ticket”.
- **History:** It was pledged during a failed exchange of safe-conduct papers. A later custody note connects it with this account of Sami Rook: “they inherited a contract that renews whenever they tell the truth”. The note needs corroboration.
- **Power Or Function:** It supports precise manipulation or marking; compatible hardware and the keeper's skill remain necessary. Sami Rook uses “a lock shim cut from a theater ticket” alongside “refitting a coat with two stitches and a borrowed pin / finding the quiet stairwell in a crowded building”. Keepers distinguish its function from the Character's own abilities: “can remember a lock's sound after hearing it once”.
- **Cost Or Drawback:** Using “a lock shim cut from a theater ticket” well requires the keeper's judgment; it cannot decide whom to trust. For Sami Rook, the difficult choice is whether using it could bring about “the guild recognizing their handwriting in a warning”; “leaves before hearing how a rescued person feels” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** The most guarded account of “a lock shim cut from a theater ticket” concerns Sami Rook: “a contract in their pocket names no client but them”. A second witness could confirm or dispute it.
- **Story Hooks:** Using “a lock shim cut from a theater ticket” might advance “to recover the key to the guild's sealed accounts”, but a witness asks about “a contract in their pocket names no client but them” first. / A dispute over “a lock shim cut from a theater ticket” draws in this connection: “a guild accountant knows which jobs they falsified”. Who can speak for Sami Rook's claim? / Evidence concerning “they inherited a contract that renews whenever they tell the truth” calls the item's custody into question; Sami Rook must decide what to disclose.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 37 — contemporary-weird / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Remote retains the extra-elevator-button anomaly; response needs testing. Municipal service context is relevant without new hidden abilities.

**Frozen Character facts:**

- **Abilities:** can get a human answer from any municipal phone tree
- **Appearance:** Their lanyard holds a key for a building removed from the city map.
- **Archetype:** Substitute for an Unlisted Neighbor
- **Backstory Hook:** they once took the wrong bus and arrived at a family photograph
- **Desire:** to finish one ordinary repair request before it becomes a mystery
- **Fear:** a voicemail from a number they have not been assigned yet
- **Flaw:** takes a complaint home because the office closed too soon
- **Name:** Miles Hsu
- **Personality:** records a caller's preferred name even when forms have no field / apologizes to the elevator before pressing the wrong floor
- **Relationship Hook:** a neighbor leaves groceries outside a door that is not there
- **Role:** after-hours clerk for buildings with duplicate addresses
- **Secret:** they once filed a complaint against their present home
- **Signature Gear:** a universal remote that operates one elevator button too many
- **Skills:** finding a person through a chain of misdelivered mail / getting a straight answer from a municipal switchboard
- **Title:** The Last Caller on Line Two

**Proposed Item Candidate:**

- **Name:** Night Warranty
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a universal remote that operates one elevator button too many”. The sheltered recesses preserve orange plastic and damp paper seals. Its carrying arrangement is recognizable beside Miles Hsu, described as “Their lanyard holds a key for a building removed from the city map.”.
- **Purpose:** Support the practical work behind “Substitute for an Unlisted Neighbor”: Miles Hsu's “a universal remote that operates one elevator button too many” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a universal remote that operates one elevator button too many” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “finding a person through a chain of misdelivered mail / getting a straight answer from a municipal switchboard”; ownership does not grant those talents.
- **Origin:** Made by a lost-property workshop beneath a railway concourse. A surviving commission identifies its intended keeper as Miles Hsu, “The Last Caller on Line Two”, and describes the requested object as “a universal remote that operates one elevator button too many”.
- **History:** It appeared in a replacement shipment ordered by a retired employee. A later custody note connects it with this account of Miles Hsu: “they once took the wrong bus and arrived at a family photograph”. The note needs corroboration.
- **Power Or Function:** It offers controls whose response must be tested on the intended device. Miles Hsu uses “a universal remote that operates one elevator button too many” alongside “finding a person through a chain of misdelivered mail / getting a straight answer from a municipal switchboard”. Keepers distinguish its function from the Character's own abilities: “can get a human answer from any municipal phone tree”.
- **Cost Or Drawback:** Using “a universal remote that operates one elevator button too many” well requires the keeper's judgment; it cannot decide whom to trust. For Miles Hsu, the difficult choice is whether using it could bring about “a voicemail from a number they have not been assigned yet”; “takes a complaint home because the office closed too soon” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Ready for its intended use; its protective wrapping has been carefully repaired.
- **Secret:** Someone recognizes “a universal remote that operates one elevator button too many” because of “they once filed a complaint against their present home”, a fact Miles Hsu has kept out of the public account.
- **Story Hooks:** A dispute over “a universal remote that operates one elevator button too many” draws in this connection: “a neighbor leaves groceries outside a door that is not there”. Who can speak for Miles Hsu's claim? / Evidence concerning “they once took the wrong bus and arrived at a family photograph” calls the item's custody into question; Miles Hsu must decide what to disclose. / Using “a universal remote that operates one elevator button too many” might advance “to finish one ordinary repair request before it becomes a mystery”, but a witness asks about “they once filed a complaint against their present home” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 38 — contemporary-weird / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Paper map remains Book despite coffee-ring decoration. Address/plumbing skills and tenant investigation agree; shorten repetitive quotations.

**Frozen Character facts:**

- **Abilities:** can tell a street was renamed by how locals give directions
- **Appearance:** A rain-speckled bus timetable is stitched inside their jacket.
- **Archetype:** Late-Night Lost-Property Host
- **Backstory Hook:** they once took the wrong bus and arrived at a family photograph
- **Desire:** to find the passenger who keeps riding a route after its last stop
- **Fear:** a voicemail from a number they have not been assigned yet
- **Flaw:** takes a complaint home because the office closed too soon
- **Name:** Rae Moreno
- **Personality:** records a caller's preferred name even when forms have no field / brings snacks to meetings nobody remembers scheduling
- **Relationship Hook:** a bus driver waves whenever the route skips a stop
- **Role:** caseworker assigned to complaints from future tenants
- **Secret:** they have been signing for packages addressed to a tenant nobody can see
- **Signature Gear:** a paper map annotated with coffee rings and closed storefronts
- **Skills:** finding an apartment through the sound of its plumbing / finding a person through a chain of misdelivered mail
- **Title:** The Last Caller on Line Two

**Proposed Item Candidate:**

- **Name:** Waiting Room Bond
- **Item Profile:** book
- **Appearance:** A personal interpretation of “a paper map annotated with coffee rings and closed storefronts”. Its carrying wrap echoes frosted resin and a crooked inspection stamp. Its carrying arrangement is recognizable beside Rae Moreno, described as “A rain-speckled bus timetable is stitched inside their jacket.”.
- **Purpose:** Keep “a paper map annotated with coffee rings and closed storefronts” ready for Rae Moreno's work as “caseworker assigned to complaints from future tenants”; its keeper hopes it can help with “to find the passenger who keeps riding a route after its last stop”.
- **Properties:** Handling marks distinguish this interpretation of “a paper map annotated with coffee rings and closed storefronts” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “finding an apartment through the sound of its plumbing / finding a person through a chain of misdelivered mail”; ownership does not grant those talents.
- **Origin:** Made by a municipal workshop at the end of a demolished street. A surviving commission identifies its intended keeper as Rae Moreno, “The Last Caller on Line Two”, and describes the requested object as “a paper map annotated with coffee rings and closed storefronts”.
- **History:** It was exchanged during an eviction nobody remembers signing. A later custody note connects it with this account of Rae Moreno: “they once took the wrong bus and arrived at a family photograph”. The note needs corroboration.
- **Power Or Function:** It keeps written material available for reading, comparison, and annotation; it cannot establish the truth of an entry. Rae Moreno uses “a paper map annotated with coffee rings and closed storefronts” alongside “finding an apartment through the sound of its plumbing / finding a person through a chain of misdelivered mail”. Keepers distinguish its function from the Character's own abilities: “can tell a street was renamed by how locals give directions”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a paper map annotated with coffee rings and closed storefronts” can make its use unreliable until inspected. For Rae Moreno, the difficult choice is whether using it could bring about “a voicemail from a number they have not been assigned yet”; “takes a complaint home because the office closed too soon” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Protected by a replacement carrying case that leaves the original maker’s marks visible.
- **Secret:** Someone recognizes “a paper map annotated with coffee rings and closed storefronts” because of “they have been signing for packages addressed to a tenant nobody can see”, a fact Rae Moreno has kept out of the public account.
- **Story Hooks:** A dispute over “a paper map annotated with coffee rings and closed storefronts” draws in this connection: “a bus driver waves whenever the route skips a stop”. Who can speak for Rae Moreno's claim? / Evidence concerning “they once took the wrong bus and arrived at a family photograph” calls the item's custody into question; Rae Moreno must decide what to disclose. / Using “a paper map annotated with coffee rings and closed storefronts” might advance “to find the passenger who keeps riding a route after its last stop”, but a witness asks about “they have been signing for packages addressed to a tenant nobody can see” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 39 — contemporary-weird / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Street map remains Book; temporary addresses support the caseworker. Bus foreknowledge remains the Character's ability.

**Frozen Character facts:**

- **Abilities:** can hear the next stop announced before the bus arrives
- **Appearance:** A bus transfer peeks from a pocket stitched shut from outside.
- **Archetype:** Late-Night Lost-Property Host
- **Backstory Hook:** they were hired to close a case that had been open longer than the city
- **Desire:** to return a library book checked out by the building itself
- **Fear:** a voicemail from a number they have not been assigned yet
- **Flaw:** keeps helping after the official case has been closed
- **Name:** Tomas Bell
- **Personality:** brings snacks to meetings nobody remembers scheduling / asks a direct question after everyone else changes the subject
- **Relationship Hook:** a bus driver waits one minute at the stop they claim is closed
- **Role:** caseworker for complaints the city form cannot name
- **Secret:** they have been signing for packages addressed to a tenant nobody can see
- **Signature Gear:** a paper street map with handwritten temporary addresses
- **Skills:** finding a person through a chain of misdelivered mail / finding an apartment through the sound of its plumbing
- **Title:** Keeper of the Extra Copy

**Proposed Item Candidate:**

- **Name:** Lost Property Due
- **Item Profile:** book
- **Appearance:** A personal interpretation of “a paper street map with handwritten temporary addresses”. Its carrying wrap echoes frosted resin and a crooked inspection stamp. Its carrying arrangement is recognizable beside Tomas Bell, described as “A bus transfer peeks from a pocket stitched shut from outside.”.
- **Purpose:** Keep “a paper street map with handwritten temporary addresses” ready for Tomas Bell's work as “caseworker for complaints the city form cannot name”; its keeper hopes it can help with “to return a library book checked out by the building itself”.
- **Properties:** Handling marks distinguish this interpretation of “a paper street map with handwritten temporary addresses” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “finding a person through a chain of misdelivered mail / finding an apartment through the sound of its plumbing”; ownership does not grant those talents.
- **Origin:** Made by the maintenance room of a bus depot with an unlisted platform. A surviving commission identifies its intended keeper as Tomas Bell, “Keeper of the Extra Copy”, and describes the requested object as “a paper street map with handwritten temporary addresses”.
- **History:** It was claimed by someone whose signature is on a missing-person poster. A later custody note connects it with this account of Tomas Bell: “they were hired to close a case that had been open longer than the city”. The note needs corroboration.
- **Power Or Function:** It keeps written material available for reading, comparison, and annotation; it cannot establish the truth of an entry. Tomas Bell uses “a paper street map with handwritten temporary addresses” alongside “finding a person through a chain of misdelivered mail / finding an apartment through the sound of its plumbing”. Keepers distinguish its function from the Character's own abilities: “can hear the next stop announced before the bus arrives”.
- **Cost Or Drawback:** A damaged or unfamiliar part of “a paper street map with handwritten temporary addresses” can make its use unreliable until inspected. For Tomas Bell, the difficult choice is whether using it could bring about “a voicemail from a number they have not been assigned yet”; “keeps helping after the official case has been closed” makes that judgment harder.
- **Rarity:** Repair copies exist, while originals with intact provenance are difficult to find.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** One prior keeper recorded “they have been signing for packages addressed to a tenant nobody can see” beside Tomas Bell's name. The record was hidden in the wrapping of “a paper street map with handwritten temporary addresses”.
- **Story Hooks:** Using “a paper street map with handwritten temporary addresses” might advance “to return a library book checked out by the building itself”, but a witness asks about “they have been signing for packages addressed to a tenant nobody can see” first. / A dispute over “a paper street map with handwritten temporary addresses” draws in this connection: “a bus driver waits one minute at the stop they claim is closed”. Who can speak for Tomas Bell's claim? / Evidence concerning “they were hired to close a case that had been open longer than the city” calls the item's custody into question; Tomas Bell must decide what to disclose.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 40 — contemporary-weird / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Thermos remains Tool despite its stamped obsolete stop. Holding provisions fits a bus dispatcher; a brief edit can connect it to rainy shifts.

**Frozen Character facts:**

- **Abilities:** can find the missing floor by counting elevator pauses
- **Appearance:** Their raincoat is dry except for one sleeve smelling of a lake.
- **Archetype:** Municipal Oddity Clerk
- **Backstory Hook:** they once took the wrong bus and arrived at a family photograph
- **Desire:** to finish one ordinary repair request before it becomes a mystery
- **Fear:** a reminder for an appointment they already attended
- **Flaw:** keeps the peace by agreeing to needless meetings
- **Name:** Nia Okafor
- **Personality:** apologizes to automated voices / brings snacks to meetings nobody remembers scheduling
- **Relationship Hook:** a bus driver waves whenever the route skips a stop
- **Role:** bus dispatcher for a route appearing only in rain
- **Secret:** their old number appears on the city's emergency form
- **Signature Gear:** a thermos stamped with a bus stop that closed years ago
- **Skills:** finding an apartment through the sound of its plumbing / getting a straight answer from a municipal switchboard
- **Title:** Keeper of the Extra Copy

**Proposed Item Candidate:**

- **Name:** Apartment Promise
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a thermos stamped with a bus stop that closed years ago”. A deliberate maker’s pattern uses mint-green paint and bent aluminum trim. Its carrying arrangement is recognizable beside Nia Okafor, described as “Their raincoat is dry except for one sleeve smelling of a lake.”.
- **Purpose:** Keep “a thermos stamped with a bus stop that closed years ago” ready for Nia Okafor's work as “bus dispatcher for a route appearing only in rain”; its keeper hopes it can help with “to finish one ordinary repair request before it becomes a mystery”.
- **Properties:** Handling marks distinguish this interpretation of “a thermos stamped with a bus stop that closed years ago” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “finding an apartment through the sound of its plumbing / getting a straight answer from a municipal switchboard”; ownership does not grant those talents.
- **Origin:** Made by a lost-property workshop beneath a railway concourse. A surviving commission identifies its intended keeper as Nia Okafor, “Keeper of the Extra Copy”, and describes the requested object as “a thermos stamped with a bus stop that closed years ago”.
- **History:** It was exchanged during an eviction nobody remembers signing. A later custody note connects it with this account of Nia Okafor: “they once took the wrong bus and arrived at a family photograph”. The note needs corroboration.
- **Power Or Function:** It holds the keeper's provisions; its condition limits what it can safely retain. Nia Okafor uses “a thermos stamped with a bus stop that closed years ago” alongside “finding an apartment through the sound of its plumbing / getting a straight answer from a municipal switchboard”. Keepers distinguish its function from the Character's own abilities: “can find the missing floor by counting elevator pauses”.
- **Cost Or Drawback:** Using “a thermos stamped with a bus stop that closed years ago” well requires the keeper's judgment; it cannot decide whom to trust. For Nia Okafor, the difficult choice is whether using it could bring about “a reminder for an appointment they already attended”; “keeps the peace by agreeing to needless meetings” makes that judgment harder.
- **Rarity:** Its form is known; documented examples are seldom offered for trade.
- **Condition:** Its outer surfaces were recently cleaned, with older marks deliberately left intact.
- **Secret:** A concealed custody note links “a thermos stamped with a bus stop that closed years ago” to this secret about Nia Okafor: “their old number appears on the city's emergency form”. It does not reveal who wrote the note.
- **Story Hooks:** A dispute over “a thermos stamped with a bus stop that closed years ago” draws in this connection: “a bus driver waves whenever the route skips a stop”. Who can speak for Nia Okafor's claim? / Evidence concerning “they once took the wrong bus and arrived at a family photograph” calls the item's custody into question; Nia Okafor must decide what to disclose. / Using “a thermos stamped with a bus stop that closed years ago” might advance “to finish one ordinary repair request before it becomes a mystery”, but a witness asks about “their old number appears on the city's emergency form” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 41 — chaos-blend / source seed 0 / action seed 100

**Classification:** usable with minor edit.

**Review:** Chaos carbine preserves constellation and calendar/archive intention. Cyberpunk escrow-printer origin is mixed secondary imagery; clarify the workshop's manufacture in a minor edit.

**Frozen Character facts:**

- **Abilities:** can recognize a familiar voice in a whale song
- **Appearance:** A fur-lined pressure suit carries salt crystals in its brass seams.
- **Archetype:** Wasteland Oath Cartographer
- **Backstory Hook:** they wintered at a station whose trains came from other years
- **Desire:** to find who moves the calendar in the northern camp
- **Fear:** a map that adds their home only after it is destroyed
- **Flaw:** assigns uncertainty to someone else
- **Name:** Sable Northstar
- **Personality:** can bargain with a rival without pretending to like them / trusts tools but listens to weather
- **Relationship Hook:** a railway singer omits their first crossing from the saga
- **Role:** archivist tracking ruins that appear in different seasons
- **Secret:** they erased one port from every map to keep two worlds apart
- **Signature Gear:** a wheel-lock carbine etched with a constellation
- **Skills:** navigating by stars reflected in a dry well / repairing a field radio with a ship's bell spring
- **Title:** Cartographer of Borrowed Years

**Proposed Item Candidate:**

- **Name:** Last Shift
- **Item Profile:** weapon
- **Appearance:** A personal interpretation of “a wheel-lock carbine etched with a constellation”. A deliberate maker’s pattern uses sand-scored metal and faded yellow paint. Its carrying arrangement is recognizable beside Sable Northstar, described as “A fur-lined pressure suit carries salt crystals in its brass seams.”.
- **Purpose:** Support the practical work behind “Wasteland Oath Cartographer”: Sable Northstar's “a wheel-lock carbine etched with a constellation” must serve a person, not merely display their title.
- **Properties:** Handling marks distinguish this interpretation of “a wheel-lock carbine etched with a constellation” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “navigating by stars reflected in a dry well / repairing a field radio with a ship's bell spring”; ownership does not grant those talents.
- **Origin:** Made by the neighborhood escrow printers. A surviving commission identifies its intended keeper as Sable Northstar, “Cartographer of Borrowed Years”, and describes the requested object as “a wheel-lock carbine etched with a constellation”.
- **History:** It was leased under a contract that erased the makers’ names. A later custody note connects it with this account of Sable Northstar: “they wintered at a station whose trains came from other years”. The note needs corroboration.
- **Power Or Function:** It serves as the described personal weapon; handling, aim, and consequences depend on the keeper. Sable Northstar uses “a wheel-lock carbine etched with a constellation” alongside “navigating by stars reflected in a dry well / repairing a field radio with a ship's bell spring”. Keepers distinguish its function from the Character's own abilities: “can recognize a familiar voice in a whale song”.
- **Cost Or Drawback:** Using “a wheel-lock carbine etched with a constellation” well requires the keeper's judgment; it cannot decide whom to trust. For Sable Northstar, the difficult choice is whether using it could bring about “a map that adds their home only after it is destroyed”; “assigns uncertainty to someone else” makes that judgment harder.
- **Rarity:** It is uncommon because its makers accept only witnessed commissions.
- **Condition:** Well preserved beneath a worn protective wrapping.
- **Secret:** One prior keeper recorded “they erased one port from every map to keep two worlds apart” beside Sable Northstar's name. The record was hidden in the wrapping of “a wheel-lock carbine etched with a constellation”.
- **Story Hooks:** Evidence concerning “they wintered at a station whose trains came from other years” calls the item's custody into question; Sable Northstar must decide what to disclose. / Using “a wheel-lock carbine etched with a constellation” might advance “to find who moves the calendar in the northern camp”, but a witness asks about “they erased one port from every map to keep two worlds apart” first. / A dispute over “a wheel-lock carbine etched with a constellation” draws in this connection: “a railway singer omits their first crossing from the saga”. Who can speak for Sable Northstar's claim?

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 42 — chaos-blend / source seed 1 / action seed 101

**Classification:** usable with minor edit.

**Review:** Chaos shadow-depth gauge gives a reading requiring interpretation, not a numerical game rule. Hunt-camp custody and eclipse contracts fit mixed-world travel.

**Frozen Character facts:**

- **Abilities:** can align a brass prosthetic with a damaged navigation array
- **Appearance:** Their map case is stitched from an old lighthouse uniform.
- **Archetype:** Saga-Engine Salvager
- **Backstory Hook:** they crossed the desert carrying a drowned kingdom's court seal
- **Desire:** to learn whether the compass points to a place or person
- **Fear:** a radio carrying tomorrow's voice
- **Flaw:** turns uncertainty into a route someone else must follow
- **Name:** Sable Ash
- **Personality:** asks who is missing before counting arrivals / checks who will bear the cost before celebrating a clever plan
- **Relationship Hook:** a radio operator warns them as a future self
- **Role:** witness for contracts signed during eclipses
- **Secret:** the compass points to a promise not yet made
- **Signature Gear:** a gauge measuring the depth of shadows
- **Skills:** mending sailcloth with wire from a signal mast / navigating by stars reflected in a dry well
- **Title:** Keeper of the Brass Tide

**Proposed Item Candidate:**

- **Name:** Hollow Coronation
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a gauge measuring the depth of shadows”. Its carrying wrap echoes weathered leather and ash-grey lacquer. Its carrying arrangement is recognizable beside Sable Ash, described as “Their map case is stitched from an old lighthouse uniform.”.
- **Purpose:** Help Sable Ash carry out “witness for contracts signed during eclipses”, while pursuing “to learn whether the compass points to a place or person”. The commission centers on “a gauge measuring the depth of shadows”.
- **Properties:** Handling marks distinguish this interpretation of “a gauge measuring the depth of shadows” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “mending sailcloth with wire from a signal mast / navigating by stars reflected in a dry well”; ownership does not grant those talents.
- **Origin:** Made by a workshop supporting migrating hunt camps. A surviving commission identifies its intended keeper as Sable Ash, “Keeper of the Brass Tide”, and describes the requested object as “a gauge measuring the depth of shadows”.
- **History:** It was pledged during a hunt abandoned near a breeding ground. A later custody note connects it with this account of Sable Ash: “they crossed the desert carrying a drowned kingdom's court seal”. The note needs corroboration.
- **Power Or Function:** It presents a reading that its keeper must interpret and check against other evidence. Sable Ash uses “a gauge measuring the depth of shadows” alongside “mending sailcloth with wire from a signal mast / navigating by stars reflected in a dry well”. Keepers distinguish its function from the Character's own abilities: “can align a brass prosthetic with a damaged navigation array”.
- **Cost Or Drawback:** Using “a gauge measuring the depth of shadows” well requires the keeper's judgment; it cannot decide whom to trust. For Sable Ash, the difficult choice is whether using it could bring about “a radio carrying tomorrow's voice”; “turns uncertainty into a route someone else must follow” makes that judgment harder.
- **Rarity:** Its form is known; documented examples are seldom offered for trade.
- **Condition:** Travel-worn outer packaging protects an apparently usable object.
- **Secret:** One prior keeper recorded “the compass points to a promise not yet made” beside Sable Ash's name. The record was hidden in the wrapping of “a gauge measuring the depth of shadows”.
- **Story Hooks:** A dispute over “a gauge measuring the depth of shadows” draws in this connection: “a radio operator warns them as a future self”. Who can speak for Sable Ash's claim? / Evidence concerning “they crossed the desert carrying a drowned kingdom's court seal” calls the item's custody into question; Sable Ash must decide what to disclose. / Using “a gauge measuring the depth of shadows” might advance “to learn whether the compass points to a place or person”, but a witness asks about “the compass points to a promise not yet made” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 43 — chaos-blend / source seed 2 / action seed 102

**Classification:** usable with minor edit.

**Review:** Chaos bell remains Artifact while its fitted receiver supplies signal use. Across-century tower repair and lighthouse history bind the mixture coherently.

**Frozen Character facts:**

- **Abilities:** can point a lighthouse beam through solid stone
- **Appearance:** A brass hearing aid plays a saga when a train passes.
- **Archetype:** Tide-Court Signal Witch
- **Backstory Hook:** they rebuilt a lighthouse from three different eras
- **Desire:** to guide a refugee convoy through a sea that now crosses the desert
- **Fear:** that a successful crossing will strand someone on the wrong side of time
- **Flaw:** turns rescue into debt without noticing
- **Name:** Mina Ash
- **Personality:** offers compromises with their cost written plainly / checks who will bear the cost before celebrating a clever plan
- **Relationship Hook:** a convoy child asks them to map a moving home
- **Role:** repairer of signal towers that broadcast across centuries
- **Secret:** one name belongs to a person beneath the desert tide
- **Signature Gear:** a ship's bell fitted with a radio receiver
- **Skills:** negotiating passage between court and rail union / keeping a convoy ledger across competing calendars
- **Title:** Cartographer of Borrowed Years

**Proposed Item Candidate:**

- **Name:** Quiet Substitution
- **Item Profile:** artifact
- **Appearance:** A personal interpretation of “a ship's bell fitted with a radio receiver”. The sheltered recesses preserve orange plastic and damp paper seals. Its carrying arrangement is recognizable beside Mina Ash, described as “A brass hearing aid plays a saga when a train passes.”.
- **Purpose:** Help Mina Ash carry out “repairer of signal towers that broadcast across centuries”, while pursuing “to guide a refugee convoy through a sea that now crosses the desert”. The commission centers on “a ship's bell fitted with a radio receiver”.
- **Properties:** Handling marks distinguish this interpretation of “a ship's bell fitted with a radio receiver” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “negotiating passage between court and rail union / keeping a convoy ledger across competing calendars”; ownership does not grant those talents.
- **Origin:** Made by a repair club in a building without a ground floor. A surviving commission identifies its intended keeper as Mina Ash, “Cartographer of Borrowed Years”, and describes the requested object as “a ship's bell fitted with a radio receiver”.
- **History:** It was exchanged during an eviction nobody remembers signing. A later custody note connects it with this account of Mina Ash: “they rebuilt a lighthouse from three different eras”. The note needs corroboration.
- **Power Or Function:** It lets its keeper receive or inspect signals through the described apparatus. Mina Ash uses “a ship's bell fitted with a radio receiver” alongside “negotiating passage between court and rail union / keeping a convoy ledger across competing calendars”. Keepers distinguish its function from the Character's own abilities: “can point a lighthouse beam through solid stone”.
- **Cost Or Drawback:** Maintaining “a ship's bell fitted with a radio receiver” takes time its keeper might need elsewhere. For Mina Ash, the difficult choice is whether using it could bring about “that a successful crossing will strand someone on the wrong side of time”; “turns rescue into debt without noticing” makes that judgment harder.
- **Rarity:** It is familiar locally but unusual beyond the district.
- **Condition:** Well preserved beneath a worn protective wrapping.
- **Secret:** Someone recognizes “a ship's bell fitted with a radio receiver” because of “one name belongs to a person beneath the desert tide”, a fact Mina Ash has kept out of the public account.
- **Story Hooks:** A dispute over “a ship's bell fitted with a radio receiver” draws in this connection: “a convoy child asks them to map a moving home”. Who can speak for Mina Ash's claim? / Evidence concerning “they rebuilt a lighthouse from three different eras” calls the item's custody into question; Mina Ash must decide what to disclose. / Using “a ship's bell fitted with a radio receiver” might advance “to guide a refugee convoy through a sea that now crosses the desert”, but a witness asks about “one name belongs to a person beneath the desert tide” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.

### 44 — chaos-blend / source seed 3 / action seed 103

**Classification:** usable with minor edit.

**Review:** Chaos weather vane gives an indication requiring judgment, not guaranteed truth about promises. Fjord pilgrimage/oath travel agree; tighten the mixed-era prose.

**Frozen Character facts:**

- **Abilities:** can point a lighthouse beam through solid stone
- **Appearance:** Their prosthetic hand is carved with a family oath and fitted with a radio dial.
- **Archetype:** Saga-Engine Salvager
- **Backstory Hook:** they crossed the desert carrying a drowned kingdom's court seal
- **Desire:** to settle an oath between people living in different versions of the same year
- **Fear:** a map that adds their home only after it is destroyed
- **Flaw:** collects witnesses but forgets to ask what they need
- **Name:** Kest Ash
- **Personality:** can bargain with a rival without pretending to like them / checks who will bear the cost before celebrating a clever plan
- **Relationship Hook:** a rival captain keeps the other half of a machine they built together
- **Role:** repairer of signal towers that broadcast across centuries
- **Secret:** their compass points toward a person who has not met them
- **Signature Gear:** a weather vane pointing toward the nearest broken promise
- **Skills:** identifying a false star in a dry well / reading tide marks on a railway bridge
- **Title:** Last Passenger on the Whale Road

**Proposed Item Candidate:**

- **Name:** Blackout Bond
- **Item Profile:** tool
- **Appearance:** A personal interpretation of “a weather vane pointing toward the nearest broken promise”. Its carrying wrap echoes reclaimed alloy and deliberately mismatched panels. Its carrying arrangement is recognizable beside Kest Ash, described as “Their prosthetic hand is carved with a family oath and fitted with a radio dial.”.
- **Purpose:** Help Kest Ash carry out “repairer of signal towers that broadcast across centuries”, while pursuing “to settle an oath between people living in different versions of the same year”. The commission centers on “a weather vane pointing toward the nearest broken promise”.
- **Properties:** Handling marks distinguish this interpretation of “a weather vane pointing toward the nearest broken promise” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “identifying a false star in a dry well / reading tide marks on a railway bridge”; ownership does not grant those talents.
- **Origin:** Made by the seasonal workshops beside a frozen fjord. A surviving commission identifies its intended keeper as Kest Ash, “Last Passenger on the Whale Road”, and describes the requested object as “a weather vane pointing toward the nearest broken promise”.
- **History:** It returned from a pilgrimage whose travelers never came home. A later custody note connects it with this account of Kest Ash: “they crossed the desert carrying a drowned kingdom's court seal”. The note needs corroboration.
- **Power Or Function:** It offers a directional indication whose meaning the keeper must assess. Kest Ash uses “a weather vane pointing toward the nearest broken promise” alongside “identifying a false star in a dry well / reading tide marks on a railway bridge”. Keepers distinguish its function from the Character's own abilities: “can point a lighthouse beam through solid stone”.
- **Cost Or Drawback:** Carrying “a weather vane pointing toward the nearest broken promise” makes the keeper recognizable to people who know the object. For Kest Ash, the difficult choice is whether using it could bring about “a map that adds their home only after it is destroyed”; “collects witnesses but forgets to ask what they need” makes that judgment harder.
- **Rarity:** Its form is known; documented examples are seldom offered for trade.
- **Condition:** Apparently ready for use, though a keeper recommends a patient inspection.
- **Secret:** A concealed custody note links “a weather vane pointing toward the nearest broken promise” to this secret about Kest Ash: “their compass points toward a person who has not met them”. It does not reveal who wrote the note.
- **Story Hooks:** A dispute over “a weather vane pointing toward the nearest broken promise” draws in this connection: “a rival captain keeps the other half of a machine they built together”. Who can speak for Kest Ash's claim? / Evidence concerning “they crossed the desert carrying a drowned kingdom's court seal” calls the item's custody into question; Kest Ash must decide what to disclose. / Using “a weather vane pointing toward the nearest broken promise” might advance “to settle an oath between people living in different versions of the same year”, but a witness asks about “their compass points toward a person who has not met them” first.

**Proposed relationship:** Character → Owns → Item; Generated Candidate, one stored edge, inverse Owned by.
