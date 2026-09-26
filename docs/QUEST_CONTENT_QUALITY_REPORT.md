# Quest Forge Content Quality Audit — Mission 03

## Method

`quest_quality_audit` generates sequential seeds from zero for each canonical preset: 256 seeds for ten presets and 1,024 for Chaos Blend. Candidate signatures cover the fourteen public Quest values; similarity compares exact equality across those fourteen values, treating ordered text lists as whole fields. A controlled-axis pass compares 256 paired Cyberpunk seeds at axis levels 0 and 100 while holding all other inputs fixed.

These exact-equality measurements do not identify semantic near-clones and cannot establish subjective quality. A field with only three variants can still contribute to a large combinatorial candidate count.

## Measurements

Results from the final sweep are listed below. The audit executable is `cargo run -q -p forge-app-core --example quest_quality_audit`.

| Preset | Seeds | Unique candidates | Exact duplicates | Pair count | Mean matching fields | Maximum | Pairs ≥8 | Pairs ≥10 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| dark-fantasy | 256 | 256 | 0 | 32,640 | 4.668 | 12 | 1,844 | 136 |
| pirate | 256 | 256 | 0 | 32,640 | 4.673 | 12 | 1,963 | 145 |
| steampunk | 256 | 256 | 0 | 32,640 | 4.666 | 13 | 1,924 | 135 |
| cyberpunk | 256 | 256 | 0 | 32,640 | 4.684 | 12 | 1,901 | 125 |
| nordic-myth | 256 | 256 | 0 | 32,640 | 4.664 | 12 | 1,874 | 121 |
| post-apocalyptic | 256 | 256 | 0 | 32,640 | 4.662 | 13 | 1,842 | 154 |
| occult | 256 | 256 | 0 | 32,640 | 4.669 | 12 | 1,888 | 113 |
| monster-hunter | 256 | 256 | 0 | 32,640 | 4.668 | 12 | 1,862 | 121 |
| rogue-assassin | 256 | 256 | 0 | 32,640 | 4.669 | 12 | 1,936 | 133 |
| contemporary-weird | 256 | 256 | 0 | 32,640 | 4.668 | 13 | 1,846 | 131 |
| Chaos Blend | 1,024 | 1,024 | 0 | 523,776 | 4.668 | 13 | 30,149 | 2,083 |

## Field-level diversity

The audit records per-value uniqueness for title, objective, stakes, opposition, twist, and location approach; each has three observed values in each preset sample. The other fields are included in pair similarity but are not individually reported by this prototype audit, another unmet requirement. These small measured pools explain why unique full candidates do not imply credible field-level diversity. Pairwise exact similarity averages 4.66–4.68 of 14 fields; in Chaos Blend, 30,149/523,776 pairs match at least eight fields and 2,083 match at least ten. This is a material clone-risk signal despite zero exact full duplicates.

## Chaos Blend

Chaos Blend uses an explicitly authored hybrid pressure involving port law, civic records, and machine authority. It is structurally valid in generation samples. Its current hybrid is a single fixed preset identity; it does not combine per-quest components from independently selected universes, so the required broad, meaningful blend is not established.

## Controlled axes

Each of the eight Cyberpunk axis comparisons changed the candidate signature for 256/256 paired seeds. Because every axis is included in ForgeDNA stream derivation, output changes alone can arise from a changed stream. They are not sufficient evidence that each axis has a deliberate semantic influence. Only Bleak, Epic, and Chaos have direct generation branches in this implementation; the other axis effects remain unproven.

## Representative sample review

Seed 42 was generated and manually reviewed for all eleven presets. Every sample has an identifiable local pressure, an objective, and an adverse result. However, repeated legal-record framing and generic fill phrases often override the preset logic; multiple samples have awkward grammar or mismatched stakes. This does not meet the human quality gate.

| Preset | Sample pressure and action | Review |
|---|---|---|
| Dark fantasy | Valley condemned at the next bell; recover an iron witness seal from the ash court. | Clear fantasy locale, but the legal-hearing framing feels imported. |
| Pirate | Drowned crew erased from port rolls by tomorrow's tide; recover the brass tide-bell. | Strong tide/port identity; the default recovery wording remains generic. |
| Steampunk | District loses heat when a regulator is condemned; authenticate its core. | Industrial pressure is clear; repeated “when” construction is clumsy. |
| Cyberpunk | Corporation overwrites 400 identities at midnight; authenticate a consent ledger. | Strong identity/surveillance pressure; opposition's forgery testimony conflicts with its access control role. |
| Nordic myth | Three villages lose protection when a cairn moves; carry the boundary stone to a hearing. | Mythic object and stakes are present, but the public hearing is an awkward genre fit. |
| Post-apocalyptic | Lower camp loses water at the next ration count; authenticate an aquifer map. | Resource pressure fits; the map's purpose is not tied to controlling the pump. |
| Occult | Patients vanish from hospital records; recover a ledger copy. | Institutional weirdness is present; the dead clerk as forgery witness lacks setup. |
| Monster hunter | Brood migration threatens a market town; authenticate a shed scale. | Threat fits; the evidence objective does not directly stop the migration. |
| Rogue assassin | Framed courier faces execution when a ledger opens; expose a false contract. | Covert intrigue is present; the lawful hearing language weakens the premise. |
| Contemporary weird | Apartments disappear from tax rolls before eviction; prove occupancy. | Mundane civic system plus impossible event fits well; threat remains abstract. |
| Chaos Blend | City will classify a living district as myth; recover a tide-powered memory chart. | Three thematic systems combine legibly, but the blend is one fixed preset theme rather than broad per-quest mixing. |

Repeated artifacts such as “ledger entry,” “official account,” “public hearing,” and “lawful route” are visible across unrelated presets. That reuse is a template-clone concern even where exact field comparisons differ.

## Version decision and limitations

Quest recipes default to content version 2. Content version 1 is rejected for Quest generation because Mission 02 introduced no historical Quest corpus to replay. Character versions 1 and 2 remain untouched. Quest content is currently embedded in Rust source rather than in a declarative versioned corpus, which prevents content review and safe content-only evolution. Exact clone metrics do not account for paraphrase similarity, and user-supplied constraints are not fully enforced.
