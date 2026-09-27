# Mission 03 — Final Quest Review Set

**Audit sample set:** 52 deterministic Quest candidates from the final Mission 03 run. Every entry includes the complete serialized Quest entity (including its envelope), plus the v3 clue and briefing fields. Root seed, selected thread, axis settings, and constraints are included for reproduction. Neutral axes are all 50; any non-neutral override is called out in the sample heading.

**Manual review method:** I read each complete generated output against the closeout rubric: actionable objective, meaningful stakes, relevant giver and threat, useful clue, situation-changing complication, contextual reward, logical consequence, moral pressure, grammar, preset identity, contradictions, template joins, repetition, and usefulness to a writer. Ratings are reviewer screening judgments; no player playtest is represented here.

**Review totals:** 45 USABLE AS-IS; 7 USABLE WITH MINOR EDITING; 0 WEAK / SUBSTANTIAL REWRITE. Each minor-edit case includes its primary reason. The constrained briefing punctuation issue found in the first read-through was fixed and the affected samples were regenerated before these ratings.

## Sample 1 — dark-fantasy / seed 0

- Thread: `winter-lanterns`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a hospice keeper who has burned the last healing oil: a pass has vanished from the map whenever the watchtower flame is lit",
  "clue": "A line of warm footprints ends at a sealed tomb",
  "quest": {
    "complications": [
      "The map records travelers who have not yet decided to leave"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "dark-fantasy"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "4fa59aa1-2177-3ded-fbd2-6e066b64428a",
        "parent_generation_id": null
      },
      "id": "4fa59aa1-2177-3ded-fbd2-6e066b64428a",
      "name": "A Beacon for the Uncounted",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:dark-fantasy",
        "thread:winter-lanterns"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The valley survives the winter while the watchtower claims the abandoned homes"
    ],
    "follow_up_hooks": [
      "The route map now shows a road leading south into summer"
    ],
    "locations": [
      "the charcoal road beneath the ruined watchtower"
    ],
    "moral_pressure": "Save the hospice now by abandoning the hamlets that depend on the beacon",
    "objective": "Find what is feeding the curse and break its hold before nightfall",
    "opposition": "the frost rider collecting names from every closed door",
    "patron": "a hospice keeper who has burned the last healing oil",
    "premise": "a pass has vanished from the map whenever the watchtower flame is lit",
    "quest_type": "scarcity",
    "rewards": [
      "A winter-safe route known only to the old convoy guides"
    ],
    "stakes": "The hospice patients will freeze before the convoy can reach the pass",
    "twist": "The frost-bound knight is the last survivor of the first village the beacon failed"
  }
}
```

## Sample 2 — dark-fantasy / seed 1

- Thread: `winter-lanterns`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a hospice keeper who has burned the last healing oil: the winter road is safe only for travelers the village has expelled",
  "clue": "A line of warm footprints ends at a sealed tomb",
  "quest": {
    "complications": [
      "The dead village's hearths are burning with fuel taken from the living"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "dark-fantasy"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "d7a4b1d9-b330-f58c-70fd-138e80df7003",
        "parent_generation_id": null
      },
      "id": "d7a4b1d9-b330-f58c-70fd-138e80df7003",
      "name": "A Hearth Owed to the Dark",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:dark-fantasy",
        "thread:winter-lanterns"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The wards are warm, but the convoy must spend the night beyond the pass"
    ],
    "follow_up_hooks": [
      "A spring thaw reveals a second beacon beneath the hospice floor"
    ],
    "locations": [
      "the charcoal road beneath the ruined watchtower"
    ],
    "moral_pressure": "Save the hospice now by abandoning the hamlets that depend on the beacon",
    "objective": "Carry the unlit beacon through the pass and discover which route it refuses",
    "opposition": "the village warden who made the bargain and now denies it",
    "patron": "a hospice keeper who has burned the last healing oil",
    "premise": "the winter road is safe only for travelers the village has expelled",
    "quest_type": "survival",
    "rewards": [
      "A lantern wick that burns only when someone offers shelter freely"
    ],
    "stakes": "Each night the curse lasts, one more household wakes unable to feel warmth",
    "twist": "The route disappears to keep the frost rider from following the convoy home"
  }
}
```

## Sample 3 — dark-fantasy / seed 2

- Thread: `winter-lanterns`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a hospice keeper who has burned the last healing oil: a pass has vanished from the map whenever the watchtower flame is lit",
  "clue": "A frost pattern on the ward window matches the vanished road",
  "quest": {
    "complications": [
      "The curse retreats from the hospice but grows stronger in every rescued house"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "dark-fantasy"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "3553695c-38b2-2576-e8d2-33958d102407",
        "parent_generation_id": null
      },
      "id": "3553695c-38b2-2576-e8d2-33958d102407",
      "name": "The Night the Pass Went Dark",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:dark-fantasy",
        "thread:winter-lanterns"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The beacon is restored, but the hospice must ration its healing oil"
    ],
    "follow_up_hooks": [
      "A convoy map marks another beacon that went dark on the same night"
    ],
    "locations": [
      "the charcoal road beneath the ruined watchtower"
    ],
    "moral_pressure": "Saving the beacon requires leaving the hospice keeper's own house unheated",
    "objective": "Choose a safe fuel source before the next storm seals the charcoal road",
    "opposition": "a frost-bound knight who guards the beacon's fuel",
    "patron": "a hospice keeper who has burned the last healing oil",
    "premise": "a pass has vanished from the map whenever the watchtower flame is lit",
    "quest_type": "survival",
    "rewards": [
      "A coal that can keep one chosen household warm through any storm"
    ],
    "stakes": "The hospice patients will freeze before the convoy can reach the pass",
    "twist": "The frost-bound knight is the last survivor of the first village the beacon failed"
  }
}
```

## Sample 4 — pirate / seed 0

- Thread: `drowned-roll`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a harbor widow whose house appears on no surviving chart: a ship's crew exists in the ledger but no living sailor remembers sailing with them",
  "clue": "A berth number is carved into the courthouse foundation",
  "quest": {
    "complications": [
      "Every recovered claim adds another street to the submerged map"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "pirate"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "8de0f361-23b0-574b-fdf1-517774ae7748",
        "parent_generation_id": null
      },
      "id": "8de0f361-23b0-574b-fdf1-517774ae7748",
      "name": "A Claim Without a Shore",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:pirate",
        "thread:drowned-roll"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The district's families regain their claim while the magistrate loses office"
    ],
    "follow_up_hooks": [
      "The recovered chart marks a safe channel through tomorrow's storm"
    ],
    "locations": [
      "a floating court anchored above the buried market"
    ],
    "moral_pressure": "Return the port deed to its heirs or keep it to protect the harbor from annexation",
    "objective": "Compare the submerged street plan with the auction rolls before the claims are sold",
    "opposition": "the harbor judge who refuses to recognize anyone absent from the roll",
    "patron": "a harbor widow whose house appears on no surviving chart",
    "premise": "a ship's crew exists in the ledger but no living sailor remembers sailing with them",
    "quest_type": "debt",
    "rewards": [
      "A harbor charter recognizing homes reclaimed from the tide"
    ],
    "stakes": "The harbor loses legal claim to its own shoreline if the ledger is false",
    "twist": "The wreck's cargo is the harbor's original title deed"
  }
}
```

## Sample 5 — pirate / seed 1

- Thread: `drowned-roll`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "the captain's sister, who still pays the crew's port fees: a salvage claim will transfer a sunken neighborhood to the ship that sank it",
  "clue": "A berth number is carved into the courthouse foundation",
  "quest": {
    "complications": [
      "The census page changes whenever a claimant says their own name"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "pirate"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "a4cac1ad-f2ef-d65b-aed4-1e0d134e7b2b",
        "parent_generation_id": null
      },
      "id": "a4cac1ad-f2ef-d65b-aed4-1e0d134e7b2b",
      "name": "The Drowned Crew's Due",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:pirate",
        "thread:drowned-roll"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The heirs recover their homes, and the harbor must repay the salvage families"
    ],
    "follow_up_hooks": [
      "A second bell answers from a ship listed as never launched"
    ],
    "locations": [
      "the customs archive sealed behind the high-water mark"
    ],
    "moral_pressure": "Ringing the bell saves the families but reveals the captain's treason",
    "objective": "Recover the census seal from the prison hulk before the tide turns",
    "opposition": "a mutinous pilot holding the only safe chart",
    "patron": "the captain's sister, who still pays the crew's port fees",
    "premise": "a salvage claim will transfer a sunken neighborhood to the ship that sank it",
    "quest_type": "debt",
    "rewards": [
      "A harbor charter recognizing homes reclaimed from the tide"
    ],
    "stakes": "The listed crew will be condemned as smugglers unless their voyage is proven",
    "twist": "The drowned crew refuses rescue until the living return its stolen pay"
  }
}
```

## Sample 6 — pirate / seed 2

- Thread: `drowned-roll`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a rescued sailor carrying a census page in a bottle: the harbor court is selling berths to people who have not drowned yet",
  "clue": "The customs seal bears salt from a tide that has not yet come",
  "quest": {
    "complications": [
      "The bell is still rung nightly by the ghost crew who believe the district never sank"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "pirate"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "368a47af-1557-4cb2-c07c-58851974bd0e",
        "parent_generation_id": null
      },
      "id": "368a47af-1557-4cb2-c07c-58851974bd0e",
      "name": "The Census Below the Bell",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:pirate",
        "thread:drowned-roll"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The crew keeps its names but the harbor court seizes the wreck"
    ],
    "follow_up_hooks": [
      "A second bell answers from a ship listed as never launched"
    ],
    "locations": [
      "the tide-locked customs vault"
    ],
    "moral_pressure": "Return the port deed to its heirs or keep it to protect the harbor from annexation",
    "objective": "Find the living crew named in the ledger and establish who owns their berth",
    "opposition": "a salvage captain selling the same wreck to three competing families",
    "patron": "a rescued sailor carrying a census page in a bottle",
    "premise": "the harbor court is selling berths to people who have not drowned yet",
    "quest_type": "crew",
    "rewards": [
      "A salvage share held in trust for the crew's families"
    ],
    "stakes": "Erasure voids the families' right to recover their dead or their homes",
    "twist": "The census office was sunk deliberately to keep the drowned from voting"
  }
}
```

## Sample 7 — steampunk / seed 0

- Thread: `clockwork-inheritance`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "the inventor's former partner, excluded from every patent: the registry will erase a living heir when the clock strikes twelve",
  "clue": "A gear bears a repair mark made after the inventor's recorded death",
  "quest": {
    "complications": [
      "The clock's maintenance record is written in adjustments only its workers can read"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "steampunk"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "510c885b-b1f2-a62f-c404-53e481e555ba",
        "parent_generation_id": null
      },
      "id": "510c885b-b1f2-a62f-c404-53e481e555ba",
      "name": "The Clockmaker's Unclaimed Hour",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:steampunk",
        "thread:clockwork-inheritance"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The will is voided and the workers must find new housing"
    ],
    "follow_up_hooks": [
      "The automaton's repair log names a workshop on a street not yet built"
    ],
    "locations": [
      "the guild's apprenticeship hall, locked since its master disappeared"
    ],
    "moral_pressure": "Let the workers inherit the workshop even though they concealed its testimony",
    "objective": "Stop the registry from dismantling the automaton before its testimony is recorded",
    "opposition": "the guild judge who can freeze every workshop named in the will",
    "patron": "the inventor's former partner, excluded from every patent",
    "premise": "the registry will erase a living heir when the clock strikes twelve",
    "quest_type": "identity",
    "rewards": [
      "The inventor's tool set and authority to maintain the civic time network"
    ],
    "stakes": "The city may lose the patent rights that keep its bridges operating",
    "twist": "The maker intended the automaton to inherit so it could protect the workers"
  }
}
```

## Sample 8 — steampunk / seed 1

- Thread: `clockwork-inheritance`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a municipal judge whose family owns the workshop lease: the city will dismantle its oldest automaton to settle a patent dispute",
  "clue": "The probate stamp predates the will by exactly one day",
  "quest": {
    "complications": [
      "The will's ink appears only while the clockwork heart is running"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "steampunk"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "bb19b600-c0d4-02e5-aacc-b26d796c9d89",
        "parent_generation_id": null
      },
      "id": "bb19b600-c0d4-02e5-aacc-b26d796c9d89",
      "name": "The Heir in the Gearbox",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:steampunk",
        "thread:clockwork-inheritance"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The workers inherit the shop and accept responsibility for every clock it serves"
    ],
    "follow_up_hooks": [
      "A public clock strikes an hour that appears in no local calendar"
    ],
    "locations": [
      "the inventor's rented room preserved inside a factory timepiece"
    ],
    "moral_pressure": "Recognize the automaton and deny the human heir the only inheritance promised",
    "objective": "Find the maker's final instruction and establish who the will recognizes",
    "opposition": "the executor who has wound the automaton to reject all human claims",
    "patron": "a municipal judge whose family owns the workshop lease",
    "premise": "the city will dismantle its oldest automaton to settle a patent dispute",
    "quest_type": "automation",
    "rewards": [
      "The workshop and its designs placed in a workers' trust"
    ],
    "stakes": "The workers lose their shop and wages to an heir who has never entered it",
    "twist": "The inventor designed the will to transfer the patent to the workers collectively"
  }
}
```

## Sample 9 — steampunk / seed 2

- Thread: `foundry-pressure`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "the district's heat collector, whose meter runs backward: a patent claim has made the public waterworks private property",
  "clue": "The clock loses seven seconds whenever the district's pressure rises",
  "quest": {
    "complications": [
      "A second valve can prevent the rupture only by flooding the guild archive"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "steampunk"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "ad6aff65-454c-a80c-f6c3-e0192e56f83b",
        "parent_generation_id": null
      },
      "id": "ad6aff65-454c-a80c-f6c3-e0192e56f83b",
      "name": "The Pressure in the Public Clock",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:steampunk",
        "thread:foundry-pressure"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The infirmary is safe, though the flooded archive destroys the guild's claims"
    ],
    "follow_up_hooks": [
      "The regulator ticks once for every civic engine that has not yet failed"
    ],
    "locations": [
      "the guild's patent vault beneath the workers' infirmary"
    ],
    "moral_pressure": "Saving the workers requires destroying the patent that could heat the whole city",
    "objective": "Trace the clock regulator's hidden line and isolate it without stopping the public works",
    "opposition": "the foundry owner who controls the only licensed repair tools",
    "patron": "the district's heat collector, whose meter runs backward",
    "premise": "a patent claim has made the public waterworks private property",
    "quest_type": "labor",
    "rewards": [
      "A pressure key that safely opens any civic relief valve"
    ],
    "stakes": "A pressure failure could collapse the canal locks and strand the winter grain",
    "twist": "The inspector falsified the readings to stop an unsafe test ordered by the owner"
  }
}
```

## Sample 10 — cyberpunk / seed 0

- Thread: `identity-override`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a transit medic whose legal identity belongs to a stranger: the registry is issuing one person's access credentials to an entire household",
  "clue": "A school attendance log contains memories no enrolled child recalls",
  "quest": {
    "complications": [
      "The evidence archive is hosted on a network whose power comes from the affected district"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "cyberpunk"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "6f74789c-e7c5-6f2d-142b-b3aeed1d907b",
        "parent_generation_id": null
      },
      "id": "6f74789c-e7c5-6f2d-142b-b3aeed1d907b",
      "name": "A Face the Registry Rejected",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:cyberpunk",
        "thread:identity-override"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The false credentials are withdrawn, leaving the shelter to negotiate its own legal status"
    ],
    "follow_up_hooks": [
      "The suspended model continues assigning scores through the transit gates"
    ],
    "locations": [
      "the corporation's identity escrow server"
    ],
    "moral_pressure": "The daemon can save everyone if one resident agrees to be erased permanently",
    "objective": "Find which memories were used to train the eligibility model and return control to their owners",
    "opposition": "a corporate privacy lawyer holding the only unaltered consent record",
    "patron": "a transit medic whose legal identity belongs to a stranger",
    "premise": "the registry is issuing one person's access credentials to an entire household",
    "quest_type": "identity",
    "rewards": [
      "A signed registry correction recognized by transit, housing, and clinic systems"
    ],
    "stakes": "A district-wide identity lockout will strand patients away from their clinics",
    "twist": "The dead commuter licensed their identity to support dependents after death"
  }
}
```

## Sample 11 — cyberpunk / seed 1

- Thread: `identity-override`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a memory broker who wants one client returned to their own past: the city's identity score has reassigned a neighborhood to people who do not exist",
  "clue": "A checksum changes only when a living resident speaks their chosen name",
  "quest": {
    "complications": [
      "The ledger is encrypted with the memories of the first resident overwritten"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "cyberpunk"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "515d1d76-badf-bf79-636a-a9c7a22e9929",
        "parent_generation_id": null
      },
      "id": "515d1d76-badf-bf79-636a-a9c7a22e9929",
      "name": "Four Hundred Ghost Accounts",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:cyberpunk",
        "thread:identity-override"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The overwrite is delayed, but the clinic begins tracking every person who entered"
    ],
    "follow_up_hooks": [
      "The ledger lists a second city where the same overwrite is scheduled"
    ],
    "locations": [
      "the corporation's identity escrow server"
    ],
    "moral_pressure": "Save the shelter's access by letting the false profile remain in control",
    "objective": "Trace the false credentials to their point of approval before the district's access window closes",
    "opposition": "a corporate privacy lawyer holding the only unaltered consent record",
    "patron": "a memory broker who wants one client returned to their own past",
    "premise": "the city's identity score has reassigned a neighborhood to people who do not exist",
    "quest_type": "identity",
    "rewards": [
      "A signed registry correction recognized by transit, housing, and clinic systems"
    ],
    "stakes": "The affected residents lose housing, wages, and access to their own medical records",
    "twist": "The memory clinic used the stolen recollections to make the model more accurate"
  }
}
```

## Sample 12 — cyberpunk / seed 2

- Thread: `identity-override`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a transit medic whose legal identity belongs to a stranger: the registry is issuing one person's access credentials to an entire household",
  "clue": "A checksum changes only when a living resident speaks their chosen name",
  "quest": {
    "complications": [
      "The evidence archive is hosted on a network whose power comes from the affected district"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "cyberpunk"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "bd9c9154-165a-a765-7b59-8e6f0242bbd2",
        "parent_generation_id": null
      },
      "id": "bd9c9154-165a-a765-7b59-8e6f0242bbd2",
      "name": "The Last Name in the Queue",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:cyberpunk",
        "thread:identity-override"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The dead commuter's dependents keep their rights and inherit the profile's outstanding debts"
    ],
    "follow_up_hooks": [
      "The suspended model continues assigning scores through the transit gates"
    ],
    "locations": [
      "a sealed municipal memory clinic"
    ],
    "moral_pressure": "Restore the stolen memories even though some owners chose to forget them",
    "objective": "Find which memories were used to train the eligibility model and return control to their owners",
    "opposition": "the identity vendor's recovery daemon, which flags witnesses as duplicates",
    "patron": "a transit medic whose legal identity belongs to a stranger",
    "premise": "the registry is issuing one person's access credentials to an entire household",
    "quest_type": "identity",
    "rewards": [
      "A public appeal key that bypasses automated identity scoring"
    ],
    "stakes": "The training archive will erase the only memories that can prove the model is biased",
    "twist": "The clerk has been approving appeals through an unrecorded paper queue"
  }
}
```

## Sample 13 — nordic-myth / seed 0

- Thread: `saga-debt`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a skald's apprentice who found a verse naming the living: a dead ruler's debt has been assigned to every child born this year",
  "clue": "A melody remembered by the oldest goat-herd matches the missing meter",
  "quest": {
    "complications": [
      "The missing stanza is remembered only by a child not yet named in the saga"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "nordic-myth"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "c497825b-ca67-1c84-b659-e9a411f7073f",
        "parent_generation_id": null
      },
      "id": "c497825b-ca67-1c84-b659-e9a411f7073f",
      "name": "A Winter Account in Song",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:nordic-myth",
        "thread:saga-debt"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The child is freed from the account while the village loses its claim to the old stores"
    ],
    "follow_up_hooks": [
      "The listening stone repeats a debt owed by a village across the sea"
    ],
    "locations": [
      "the listening stone beside a lake that keeps spoken promises"
    ],
    "moral_pressure": "Clear the heir by keeping the original debtor's betrayal secret",
    "objective": "Identify the original debtor before the winter stores are divided",
    "opposition": "the store keeper who has been paying the wrong family for years",
    "patron": "a skald's apprentice who found a verse naming the living",
    "premise": "a dead ruler's debt has been assigned to every child born this year",
    "quest_type": "winter",
    "rewards": [
      "A listening-stone token that preserves one promise without changing its words"
    ],
    "stakes": "The village's founding account will erase the kin who kept it alive",
    "twist": "The debt is a promise to return a stolen child, not a payment owed"
  }
}
```

## Sample 14 — nordic-myth / seed 1

- Thread: `saga-debt`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a foster parent who has kept a child's origin secret: winter will claim the village unless the missing saga verse is restored",
  "clue": "The listening stone repeats one line in the voice of the dead ruler",
  "quest": {
    "complications": [
      "The store count matches the false account exactly"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "nordic-myth"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "57e7b771-dfc6-9a9d-f8e8-eac973c3f2aa",
        "parent_generation_id": null
      },
      "id": "57e7b771-dfc6-9a9d-f8e8-eac973c3f2aa",
      "name": "A Verse Missing from the Fire",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:nordic-myth",
        "thread:saga-debt"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The children are spared while the rival skald claims the saga's honor"
    ],
    "follow_up_hooks": [
      "The listening stone repeats a debt owed by a village across the sea"
    ],
    "locations": [
      "the listening stone beside a lake that keeps spoken promises"
    ],
    "moral_pressure": "Let the child hear the truth even though it may divide their kin",
    "objective": "Compare the sung versions and find when the inheritance verse was changed",
    "opposition": "the inheritance speaker whose authority depends on the disputed verse",
    "patron": "a foster parent who has kept a child's origin secret",
    "premise": "winter will claim the village unless the missing saga verse is restored",
    "quest_type": "debt",
    "rewards": [
      "A name-stone that can remember any promise spoken before it"
    ],
    "stakes": "The children are marked for service to the dead ruler when they come of age",
    "twist": "The dead ruler assigned the debt to protect the child from a living claimant"
  }
}
```

## Sample 15 — nordic-myth / seed 2

- Thread: `boundary-oath`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "an oath-speaker whose village is no longer named in the pact: one village's graves are being counted as the other village's debt",
  "clue": "The boundary stones are warm on the side where the dead were carried",
  "quest": {
    "complications": [
      "The oath answers only to the descendant who broke it"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "nordic-myth"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "15ad97d7-dd12-1cd0-e38b-42c996738a1d",
        "parent_generation_id": null
      },
      "id": "15ad97d7-dd12-1cd0-e38b-42c996738a1d",
      "name": "A Boundary Kept by No One",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:nordic-myth",
        "thread:boundary-oath"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The dead are returned to the common ground while the feud claim remains unresolved"
    ],
    "follow_up_hooks": [
      "The ferry keeper receives an oath token from beyond the mountain"
    ],
    "locations": [
      "a ferry landing that belongs to whichever village feeds its keeper"
    ],
    "moral_pressure": "Honor the old hospitality promise even though neither village can spare a winter meal",
    "objective": "Find who was sheltered under the original oath and let both villages hear their account",
    "opposition": "the spirit host who enforces hospitality without distinguishing invitation from trespass",
    "patron": "an oath-speaker whose village is no longer named in the pact",
    "premise": "one village's graves are being counted as the other village's debt",
    "quest_type": "protection",
    "rewards": [
      "A guest token that grants shelter at any hearth bound by the old pact"
    ],
    "stakes": "A false boundary ruling will put the burial ground under a feud claim",
    "twist": "The boundary keeper moved the cairn to prevent the feud speaker from claiming the dead"
  }
}
```

## Sample 16 — post-apocalyptic / seed 0

- Thread: `seed-vault`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a grower whose family name was removed from the seed register: the council will burn the last viable seed stock to prevent a disputed claim",
  "clue": "The tending notes include a water ration code from a vanished settlement",
  "quest": {
    "complications": [
      "The seed register lists the marshal's family as the rightful keeper"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "post-apocalyptic"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "4f5a2470-7543-be15-1c15-7ebde41b6a83",
        "parent_generation_id": null
      },
      "id": "4f5a2470-7543-be15-1c15-7ebde41b6a83",
      "name": "The Seed Vault's Claim",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:post-apocalyptic",
        "thread:seed-vault"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The disputed crop survives while the greenhouse's current food supply is spent"
    ],
    "follow_up_hooks": [
      "The vault requests a germination report from a community that has not been founded"
    ],
    "locations": [
      "an orchard planted around the shell of a weather station"
    ],
    "moral_pressure": "Save the pollinators by sacrificing a harvest that could feed the outer camp",
    "objective": "Recover the original seed register and prove who can open the vault",
    "opposition": "a seed broker holding the only legal claim to the vault's oldest stock",
    "patron": "a grower whose family name was removed from the seed register",
    "premise": "the council will burn the last viable seed stock to prevent a disputed claim",
    "quest_type": "inheritance",
    "rewards": [
      "A route to a settlement that can grow the crop without engine heat"
    ],
    "stakes": "One settlement could control the region's food supply through a single crop claim",
    "twist": "The raiders want the crop because their own children are starving"
  }
}
```

## Sample 17 — post-apocalyptic / seed 1

- Thread: `water-allocation`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "the lower camp's pump mechanic: one block's ration has been counted against every neighboring household",
  "clue": "The tower log records an increase every time the settlement sleeps",
  "quest": {
    "complications": [
      "The flow meter works only while the tower's backup battery is failing"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "post-apocalyptic"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "f462ab6b-42ea-3e61-a4a2-bce7313f4447",
        "parent_generation_id": null
      },
      "id": "f462ab6b-42ea-3e61-a4a2-bce7313f4447",
      "name": "The Valve at Mile Zero",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:post-apocalyptic",
        "thread:water-allocation"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The hidden reserve is revealed and residents must decide how to divide it"
    ],
    "follow_up_hooks": [
      "The fountain runs clear once, then displays a ration count for an unknown district"
    ],
    "locations": [
      "the canal gate where three camps share one working valve"
    ],
    "moral_pressure": "Reopening the pump saves the camp only if the clinic gives up its reserve",
    "objective": "Trace the diverted pipe and restore water to the lower camp",
    "opposition": "a ration boss who controls the pump keys and the public count",
    "patron": "the lower camp's pump mechanic",
    "premise": "one block's ration has been counted against every neighboring household",
    "quest_type": "governance",
    "rewards": [
      "A pressure key that lets residents verify future ration readings"
    ],
    "stakes": "Without water, the camp must abandon its sick and its seed stores",
    "twist": "The convoy boss diverted water to keep a neighboring camp alive"
  }
}
```

## Sample 18 — post-apocalyptic / seed 2

- Thread: `water-allocation`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a caravan host asked to divide water between guests and residents: one block's ration has been counted against every neighboring household",
  "clue": "The ration marks repeat a pattern found on the old aquifer map",
  "quest": {
    "complications": [
      "The hidden outlet serves a neighboring camp that has no other source"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "post-apocalyptic"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "bc935136-ff3d-0b11-60a1-2dc43f241dae",
        "parent_generation_id": null
      },
      "id": "bc935136-ff3d-0b11-60a1-2dc43f241dae",
      "name": "The Last Clean Share",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:post-apocalyptic",
        "thread:water-allocation"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The hidden reserve is revealed and residents must decide how to divide it"
    ],
    "follow_up_hooks": [
      "A neighboring camp sends its own pump key under the new compact"
    ],
    "locations": [
      "an underground reservoir reached through a collapsed transit platform"
    ],
    "moral_pressure": "Expose the clinic's false count and risk treatment for undocumented patients",
    "objective": "Open the canal gate without flooding the camp built beneath it",
    "opposition": "a repair crew paid to keep the hidden outlet running",
    "patron": "a caravan host asked to divide water between guests and residents",
    "premise": "one block's ration has been counted against every neighboring household",
    "quest_type": "scarcity",
    "rewards": [
      "A season's clean water allocation shared by both camps"
    ],
    "stakes": "Patients lose treatment if the clinic cannot clean its equipment",
    "twist": "The council's count excludes refugees because the original system cannot register them"
  }
}
```

## Sample 19 — occult / seed 0

- Thread: `missing-register`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a records clerk whose employment file has vanished: patients vanish from official records before the dawn bell",
  "clue": "A carbon copy bears a timestamp from the hour the building was evacuated",
  "quest": {
    "complications": [
      "The night archivist appears in a photograph taken before they were born"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "occult"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "4af692dc-deed-f6f6-4afe-e74c3513180c",
        "parent_generation_id": null
      },
      "id": "4af692dc-deed-f6f6-4afe-e74c3513180c",
      "name": "Receipt for an Unremembered Name",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:occult",
        "thread:missing-register"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The patients return, but the hospital must publicly admit the ward existed"
    ],
    "follow_up_hooks": [
      "The recovered file contains a map to an office removed from the building plan"
    ],
    "locations": [
      "a hospital records desk staffed only after the last patient leaves"
    ],
    "moral_pressure": "Restore the patients' names even though the archive will identify the clerk who removed them",
    "objective": "Find the carbon copy and restore the missing patients' names before dawn",
    "opposition": "the hospital director whose treatment license depends on the impossible order",
    "patron": "a records clerk whose employment file has vanished",
    "premise": "patients vanish from official records before the dawn bell",
    "quest_type": "haunting",
    "rewards": [
      "A certified copy of the register that records who authorized each absence"
    ],
    "stakes": "The new names are replacing living patients one entry at a time",
    "twist": "The dead clerk is keeping the patients hidden from something that follows names"
  }
}
```

## Sample 20 — occult / seed 1

- Thread: `borrowed-hour`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE WITH MINOR EDITING** — Bus-passenger and factory-hour threads sit beside an otherwise municipal clock case; tie the missing interval to the transit delay.

```json
{
  "briefing": "a shift steward whose timecard shows a shift they have not worked: a public clock has begun charging people time for delays it creates",
  "clue": "The clock has a thirteenth hand that moves only in a mirror",
  "quest": {
    "complications": [
      "Returning the hour will make every event inside it happen again"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "occult"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "df475302-f18c-1476-3d2d-b26476f887ab",
        "parent_generation_id": null
      },
      "id": "df475302-f18c-1476-3d2d-b26476f887ab",
      "name": "A Shift the Bell Forgot",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:occult",
        "thread:borrowed-hour"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "Workers recover their pay, but the factory loses the hour that kept its line running"
    ],
    "follow_up_hooks": [
      "The apprentice receives an inspection appointment signed by their missing mentor"
    ],
    "locations": [
      "the municipal time office where clocks are inspected for legal accuracy"
    ],
    "moral_pressure": "Name the town's promise publicly and bind residents who never agreed to it",
    "objective": "Follow the repeating bus passenger through the missing interval",
    "opposition": "the appointment clerk who schedules hearings outside ordinary time",
    "patron": "a shift steward whose timecard shows a shift they have not worked",
    "premise": "a public clock has begun charging people time for delays it creates",
    "quest_type": "time",
    "rewards": [
      "A timecard ledger that lets workers reclaim unrecorded hours"
    ],
    "stakes": "A delayed bus could strand patients after the clinic closes",
    "twist": "The missing hour was taken to hide a murder that has not happened yet"
  }
}
```

## Sample 21 — occult / seed 2

- Thread: `missing-register`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE WITH MINOR EDITING** — The heir-finding objective competes with the premise about unborn names and an intentionally missing identity; clarify which record is being adjudicated.

```json
{
  "briefing": "a records clerk whose employment file has vanished: the hospital's basement register is adding names of people not yet born",
  "clue": "A carbon copy records the same pulse under two different names",
  "quest": {
    "complications": [
      "The register accepts ink only from someone whose name is already missing"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "occult"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "7d67fb3c-bf8a-90bc-b5de-4e524ddc9be3",
        "parent_generation_id": null
      },
      "id": "7d67fb3c-bf8a-90bc-b5de-4e524ddc9be3",
      "name": "Receipt for an Unremembered Name",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:occult",
        "thread:missing-register"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The inheritance remains unclaimed while the family must disclose its hidden witness"
    ],
    "follow_up_hooks": [
      "The carbon copy lists a second hospital with no building on the city map"
    ],
    "locations": [
      "the closed ward behind the hospital chapel"
    ],
    "moral_pressure": "Expose the patient's former identity even though they chose to leave it behind",
    "objective": "Identify the rightful heir without submitting the missing name to the archive",
    "opposition": "a family solicitor using the false entry to seize a ward's home",
    "patron": "a records clerk whose employment file has vanished",
    "premise": "the hospital's basement register is adding names of people not yet born",
    "quest_type": "haunting",
    "rewards": [
      "A legal notice recognized by every office still using the old index"
    ],
    "stakes": "The disputed inheritance will pass to whoever the archive recognizes first",
    "twist": "The archivist's name was removed to protect them from the register"
  }
}
```

## Sample 22 — monster-hunter / seed 0

- Thread: `brood-migration`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE WITH MINOR EDITING** — The hidden migration route and well-poisoning stakes need one stronger evidence link so bait placement clearly addresses the water hazard.

```json
{
  "briefing": "a village trapper who has stopped setting snares near the river: a trophy hunter has hidden the only safe route past the nest",
  "clue": "The claw marks stop where an old fence has been repaired from the wrong side",
  "quest": {
    "complications": [
      "The safe trail crosses a field where the creature has already laid eggs"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "monster-hunter"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "8a8aee99-871f-30cc-45f7-da130dff1bfb",
        "parent_generation_id": null
      },
      "id": "8a8aee99-871f-30cc-45f7-da130dff1bfb",
      "name": "Bait for the Wrong Season",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:monster-hunter",
        "thread:brood-migration"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The wells are cured while the healer must close the clinic to avoid spreading venom"
    ],
    "follow_up_hooks": [
      "The larger predator leaves tracks leading toward the wardens' lodge"
    ],
    "locations": [
      "the market town's drained well"
    ],
    "moral_pressure": "Protect the brood's route even though it cuts through the village's only pasture",
    "objective": "Test a nonlethal bait away from the settlement before the brood reaches the river",
    "opposition": "a trophy hunter hiding the creature's trail",
    "patron": "a village trapper who has stopped setting snares near the river",
    "premise": "a trophy hunter has hidden the only safe route past the nest",
    "quest_type": "creature",
    "rewards": [
      "A cure for the venom already in the wells"
    ],
    "stakes": "The brood will poison the town's wells if it crosses the pass",
    "twist": "The brood is fleeing a larger predator already inside the town"
  }
}
```

## Sample 23 — monster-hunter / seed 1

- Thread: `brood-migration`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a poacher offering a map of bait sites they refuse to revisit: the next migration will carry the brood into a market town",
  "clue": "The venom appears in water upstream from the creature's trail",
  "quest": {
    "complications": [
      "The safe bait works on adults but draws the young away from food"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "monster-hunter"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "e4a2111c-052e-96fe-d316-e2205a1d36f4",
        "parent_generation_id": null
      },
      "id": "e4a2111c-052e-96fe-d316-e2205a1d36f4",
      "name": "The Trail That Avoided the Salt",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:monster-hunter",
        "thread:brood-migration"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The wells are cured while the healer must close the clinic to avoid spreading venom"
    ],
    "follow_up_hooks": [
      "A tracker finds fresh marks above the pass after the migration is declared over"
    ],
    "locations": [
      "a salt marsh where the usual prey have moved inland"
    ],
    "moral_pressure": "Publish the migration route so farms can prepare, knowing trophy hunters will learn where the brood nests",
    "objective": "Find the venom source and prepare a cure before the wells are used",
    "opposition": "a landholder who keeps the migration route fenced for private pasture",
    "patron": "a poacher offering a map of bait sites they refuse to revisit",
    "premise": "the next migration will carry the brood into a market town",
    "quest_type": "migration",
    "rewards": [
      "A seasonal migration map agreed to by trackers and settlement wardens"
    ],
    "stakes": "The healer's patients will die if the next molt contaminates the stream",
    "twist": "The brood is fleeing a larger predator already inside the town"
  }
}
```

## Sample 24 — monster-hunter / seed 2

- Thread: `false-trophy`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a ranger whose warning about the species was dismissed as a mistake: a celebrated trophy does not match the creature responsible for recent attacks",
  "clue": "A track cast shows the animal turned away from the livestock pens",
  "quest": {
    "complications": [
      "The beast is protecting a village from a predator no one has seen"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "monster-hunter"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "480569de-e21b-c8ba-f0a8-8204c132c081",
        "parent_generation_id": null
      },
      "id": "480569de-e21b-c8ba-f0a8-8204c132c081",
      "name": "The Hunter's False Head",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:monster-hunter",
        "thread:false-trophy"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The bounty is withdrawn and the lodge must fund a survey of the real range"
    ],
    "follow_up_hooks": [
      "The camera records a second creature following the beast"
    ],
    "locations": [
      "the lodge trophy hall"
    ],
    "moral_pressure": "Collect the bounty for the victims or expose that the hunt targeted the wrong animal",
    "objective": "Compare the trophy with the attack-site evidence before the lodge certifies its report",
    "opposition": "a rival hunter destroying evidence to claim the bounty",
    "patron": "a ranger whose warning about the species was dismissed as a mistake",
    "premise": "a celebrated trophy does not match the creature responsible for recent attacks",
    "quest_type": "reputation",
    "rewards": [
      "A lodge bounty redirected to the settlement's patrol"
    ],
    "stakes": "A protective order will be lifted if the trophy is accepted as proof",
    "twist": "The trophy was assembled from remains of two different animals"
  }
}
```

## Sample 25 — rogue-assassin / seed 0

- Thread: `missing-informant`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a courier carrying a ledger page they were paid not to read: the target's route has been sold to a rival before the escort arrives",
  "clue": "The route ledger shows a detour added after the target left",
  "quest": {
    "complications": [
      "The informant wants to stay missing until both patrons expose themselves"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "rogue-assassin"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "43573c7b-fa93-6129-496c-5259df4546e0",
        "parent_generation_id": null
      },
      "id": "43573c7b-fa93-6129-496c-5259df4546e0",
      "name": "The Informant Who Wasn't There",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:rogue-assassin",
        "thread:missing-informant"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The false extraction is corrected and the missing person remains legally unaccounted for"
    ],
    "follow_up_hooks": [
      "A new route appears in the ledger in handwriting no courier recognizes"
    ],
    "locations": [
      "a counting house where informants rent identities by the week"
    ],
    "moral_pressure": "Return the secret to the patron who paid for it or let the informant decide its use",
    "objective": "Reconstruct the informant's route from access logs before the surveillance window resets",
    "opposition": "a rival escort who knows every fallback route",
    "patron": "a courier carrying a ledger page they were paid not to read",
    "premise": "the target's route has been sold to a rival before the escort arrives",
    "quest_type": "witness",
    "rewards": [
      "A clean identity and exit papers controlled by the witness"
    ],
    "stakes": "The target's capture gives the rival control of the network's safe houses",
    "twist": "The rival is moving the target away from a contract already signed by your patron"
  }
}
```

## Sample 26 — rogue-assassin / seed 1

- Thread: `missing-informant`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a lookout who insists the informant was never recruited: an informant disappears from every network record the night before a purge",
  "clue": "A dead drop was opened with a key assigned to a missing courier",
  "quest": {
    "complications": [
      "The informant's cipher is built from a poem only the target remembers"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "rogue-assassin"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "154ff391-fa89-c8ea-d328-87181591820c",
        "parent_generation_id": null
      },
      "id": "154ff391-fa89-c8ea-d328-87181591820c",
      "name": "The Informant Who Left No Door",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:rogue-assassin",
        "thread:missing-informant"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The patrons are exposed while the informant keeps the secret they traded"
    ],
    "follow_up_hooks": [
      "The cipher names a child who has just been recruited"
    ],
    "locations": [
      "a safe apartment registered to a patron who denies owning it"
    ],
    "moral_pressure": "The target will testify only if the bodyguard's betrayal stays hidden",
    "objective": "Find which patron owns the borrowed identity and secure the witness's release",
    "opposition": "the lookout who erased the informant's records",
    "patron": "a lookout who insists the informant was never recruited",
    "premise": "an informant disappears from every network record the night before a purge",
    "quest_type": "witness",
    "rewards": [
      "The target's testimony, which can cancel the purge order"
    ],
    "stakes": "A false extraction record could put every safehouse on the guild's map",
    "twist": "The witness is protecting the handler who originally sold their identity"
  }
}
```

## Sample 27 — rogue-assassin / seed 2

- Thread: `false-contract`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "the old network's handler, whose own seal appears on the order: a forged contract has hired several crews to eliminate people who never met",
  "clue": "The contract ink matches the notary's public forms, not the guild copy",
  "quest": {
    "complications": [
      "The duplicate carries the courier's real fingerprint"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "rogue-assassin"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "b333084a-d43a-16f6-0d93-675b3bbe093e",
        "parent_generation_id": null
      },
      "id": "b333084a-d43a-16f6-0d93-675b3bbe093e",
      "name": "Payment Due Before the Job",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:rogue-assassin",
        "thread:false-contract"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The target survives but the network loses its most valuable safe house"
    ],
    "follow_up_hooks": [
      "A witness asks for protection from the crew that helped them escape"
    ],
    "locations": [
      "a contract exchange hidden in the back room of a public bath"
    ],
    "moral_pressure": "Save the witness by letting the target disappear before their name is cleared",
    "objective": "Extract the witness through an unmonitored route before the broker closes the safehouse",
    "opposition": "the target's fixer, who wants the ledger before it reaches the notary",
    "patron": "the old network's handler, whose own seal appears on the order",
    "premise": "a forged contract has hired several crews to eliminate people who never met",
    "quest_type": "contract",
    "rewards": [
      "The safehouse lease transferred to its keeper without guild debt"
    ],
    "stakes": "If the ledger is destroyed, the patron can deny hiring anyone",
    "twist": "The target hired the courier to fake the contract and escape the network"
  }
}
```

## Sample 28 — contemporary-weird / seed 0

- Thread: `route-that-stops`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a night driver whose bus returns with one more passenger than it boarded: the hospital's late-shift attendance depends on a stop omitted from every public map",
  "clue": "The lost-property desk has collected the same umbrella after every unlisted stop",
  "quest": {
    "complications": [
      "The original map includes a stop at the organizer's childhood address"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "contemporary-weird"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "03355bc4-f583-2f50-062a-5e3ca439229d",
        "parent_generation_id": null
      },
      "id": "03355bc4-f583-2f50-062a-5e3ca439229d",
      "name": "A Route Number the City Retired",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:contemporary-weird",
        "thread:route-that-stops"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The stop is restored to the map while one passenger remains missing"
    ],
    "follow_up_hooks": [
      "A new schedule adds a stop beside the transit director's childhood home"
    ],
    "locations": [
      "the last stop on the number 8 bus"
    ],
    "moral_pressure": "Publishing the passenger log will find the missing people but expose their private routes",
    "objective": "Compare the driver's paper log with the route system before the schedule rolls over",
    "opposition": "the route itself, which returns to the stop whenever someone names it",
    "patron": "a night driver whose bus returns with one more passenger than it boarded",
    "premise": "the hospital's late-shift attendance depends on a stop omitted from every public map",
    "quest_type": "neighborhood",
    "rewards": [
      "A protected night route and the driver's unedited passenger log"
    ],
    "stakes": "The authority may charge passengers for trips the city refuses to recognize",
    "twist": "The stop is a temporary shelter created by passengers for people who miss the last connection"
  }
}
```

## Sample 29 — contemporary-weird / seed 1

- Thread: `route-that-stops`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a bus driver whose route has gained an unlisted stop: the hospital's late-shift attendance depends on a stop omitted from every public map",
  "clue": "Hospital timecards show arrivals before the bus is scheduled to reach the campus",
  "quest": {
    "complications": [
      "The driver can record the stop only on a paper form the authority stopped accepting"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "contemporary-weird"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "51567651-f5c1-02b0-7448-7c42ee79fe4c",
        "parent_generation_id": null
      },
      "id": "51567651-f5c1-02b0-7448-7c42ee79fe4c",
      "name": "A Route Number the City Retired",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:contemporary-weird",
        "thread:route-that-stops"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The stop is restored to the map while one passenger remains missing"
    ],
    "follow_up_hooks": [
      "A passenger appears on the log with tomorrow's date and your name"
    ],
    "locations": [
      "a shelter that appears only on the night driver's inspection list"
    ],
    "moral_pressure": "Return the extra minute to the driver or let the authority claim it as system error",
    "objective": "Find the original route map before the block is removed from service",
    "opposition": "a budget director closing routes whose passengers cannot be counted",
    "patron": "a bus driver whose route has gained an unlisted stop",
    "premise": "the hospital's late-shift attendance depends on a stop omitted from every public map",
    "quest_type": "transit",
    "rewards": [
      "A transit pass valid at a stop the city has not built"
    ],
    "stakes": "The authority may charge passengers for trips the city refuses to recognize",
    "twist": "The missing stop appears only for passengers who have lost someone"
  }
}
```

## Sample 30 — contemporary-weird / seed 6

- Thread: `vanishing-floor`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a tenants' association treasurer receiving rent from an unknown account: the city will evict an entire floor after its apartments disappear from the tax roll",
  "clue": "A fire-door inspection tag is dated after the stairwell was removed",
  "quest": {
    "complications": [
      "The fire plan includes a stairwell that opens only during a scheduled drill"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000006",
            "universe_preset": "contemporary-weird"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "768a2530-281f-b137-af0e-4595bd5f732a",
        "parent_generation_id": null
      },
      "id": "768a2530-281f-b137-af0e-4595bd5f732a",
      "name": "Tenants on the Unapproved Level",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:contemporary-weird",
        "thread:vanishing-floor"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The eviction is stopped while apartment zero remains outside the official map"
    ],
    "follow_up_hooks": [
      "A new maintenance ticket is filed from the sealed service landing"
    ],
    "locations": [
      "the housing board's basement hearing room"
    ],
    "moral_pressure": "Delay the demolition for a level that may be unsafe to occupy",
    "objective": "Verify the demolition signatures before the contractor removes the stairwell",
    "opposition": "a municipal clerk whose software lists one extra unit",
    "patron": "a tenants' association treasurer receiving rent from an unknown account",
    "premise": "the city will evict an entire floor after its apartments disappear from the tax roll",
    "quest_type": "municipal",
    "rewards": [
      "A stamped occupancy register recognized by the assessor"
    ],
    "stakes": "The missing tenants lose their legal address and every service tied to it",
    "twist": "The complaints are coming from a unit used to test the building's new acoustic sensors"
  }
}
```

## Sample 31 — chaos-blend / seed 0

- Thread: `mythic-salvage`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE WITH MINOR EDITING** — Several independently mixed choices split attention between a memory chart and a convoy ration dispute; tighten the location/title around the selected convoy objective.

```json
{
  "briefing": "a water runner carrying a debt token shared by two rival settlements: the old water authority is withholding rations until a settlement honors a promise made before the collapse",
  "clue": "The storm signal repeats a ration code from before the sea withdrew",
  "quest": {
    "complications": [
      "Repairing the pump will erase the machine's record of who took the last reserve"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000000",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "ebf57b10-7caf-a0aa-b118-51738b377494",
        "parent_generation_id": null
      },
      "id": "ebf57b10-7caf-a0aa-b118-51738b377494",
      "name": "A Chart Under Oath Law",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The pump is restored and the upper settlement must surrender its winter reserve"
    ],
    "follow_up_hooks": [
      "The convoy receives a repair request from a pump beyond the old shoreline"
    ],
    "locations": [
      "a harbor court's data room wired through the breakwater"
    ],
    "moral_pressure": "Send the remaining water to the refugee convoy even if the settlements that honored the oath go dry",
    "objective": "Reach the signal tower and guide the convoy past the drowned boundary stones",
    "opposition": "a settlement elder willing to sacrifice the convoy to preserve the old boundary",
    "patron": "a water runner carrying a debt token shared by two rival settlements",
    "premise": "the old water authority is withholding rations until a settlement honors a promise made before the collapse",
    "quest_type": "post-apocalyptic",
    "rewards": [
      "A tide ledger that records consent separately from ownership"
    ],
    "stakes": "The refugees may lose their only marked route if the stone is taken as salvage",
    "twist": "The chart's missing district is the only place where copied identities remain private"
  }
}
```

## Sample 32 — chaos-blend / seed 1

- Thread: `mythic-salvage`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a cairn keeper who has been repairing the convoy's signal gear: a storm signal has summoned a convoy to a harbor erased from every map",
  "clue": "A carved boundary stone bears the same serial mark as the pump",
  "quest": {
    "complications": [
      "The convoy carries the only witness who can interpret the ration code, but their family claims them as an heir"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000001",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "fa0d3b09-ac76-6cdc-3cfd-783b0db092b3",
        "parent_generation_id": null
      },
      "id": "fa0d3b09-ac76-6cdc-3cfd-783b0db092b3",
      "name": "Memory Fees at the Harbor Gate",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The pass remains open, but the settlements must maintain it together"
    ],
    "follow_up_hooks": [
      "The harbor court receives a claim filed under the name of its own security daemon"
    ],
    "locations": [
      "the rusted signal tower at the winter port"
    ],
    "moral_pressure": "Let the machine save everyone only if one resident gives up their original memory",
    "objective": "Repair the convoy's water system without crossing the cairn before the guardian answers",
    "opposition": "a sworn guardian who mistakes the salvage crew for oath-breakers",
    "patron": "a cairn keeper who has been repairing the convoy's signal gear",
    "premise": "a storm signal has summoned a convoy to a harbor erased from every map",
    "quest_type": "mythic",
    "rewards": [
      "A tide ledger that records consent separately from ownership"
    ],
    "stakes": "A broken compact could make the machine reserve every remaining drop for one settlement",
    "twist": "The convoy is carrying the only witness who remembers the harbor before it sank"
  }
}
```

## Sample 33 — chaos-blend / seed 2

- Thread: `tide-memory-archive`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE WITH MINOR EDITING** — The registry and copied-memory spine is interrupted by pump-vow language; connect that pump record to the harbor claim or select a closer complication.

```json
{
  "briefing": "a synthetic pilot carrying a family debt in its navigation cache: the city will classify a living district as an abandoned myth at sunrise",
  "clue": "The oracle's ledger contains a tide reading from a sea that no longer exists",
  "quest": {
    "complications": [
      "The pump accepts only a vow spoken in a language no living settler remembers"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000002",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "756a068a-d9d2-7809-c0ef-3ecd98c71965",
        "parent_generation_id": null
      },
      "id": "756a068a-d9d2-7809-c0ef-3ecd98c71965",
      "name": "A Chart Under Oath Law",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The district is restored, but every copied memory becomes public evidence"
    ],
    "follow_up_hooks": [
      "A municipal boundary marker appears on the sea floor beside the old port"
    ],
    "locations": [
      "the dry reservoir beneath the boundary cairn"
    ],
    "moral_pressure": "Publish the oracle's records to prove consent was stolen and reveal private memories",
    "objective": "Recover the brass memory chart and prove the district is still inhabited",
    "opposition": "a salvage broker who can enforce ancient harbor claims through modern property systems",
    "patron": "a synthetic pilot carrying a family debt in its navigation cache",
    "premise": "the city will classify a living district as an abandoned myth at sunrise",
    "quest_type": "surreal",
    "rewards": [
      "A shared water charter and a pump that can run without fuel"
    ],
    "stakes": "The archive may preserve the port only by making every resident's memory public",
    "twist": "The chart was made by the district's residents to erase the corporation that owned them"
  }
}
```

## Sample 34 — chaos-blend / seed 7

- Thread: `mythic-salvage`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a salvage captain who has found a living village on the old sea floor: a migration convoy is carrying winter stores across a boundary the dead still enforce",
  "clue": "The storm signal repeats a ration code from before the sea withdrew",
  "quest": {
    "complications": [
      "The pump accepts only a vow spoken in a language no living settler remembers"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000007",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "7d06a4c0-79a2-9f57-0100-0b77fb0032a4",
        "parent_generation_id": null
      },
      "id": "7d06a4c0-79a2-9f57-0100-0b77fb0032a4",
      "name": "A Harbor with Two Histories",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The pass remains open, but the settlements must maintain it together"
    ],
    "follow_up_hooks": [
      "The harbor court receives a debt notice from a machine that has not been assembled"
    ],
    "locations": [
      "the customs kiosk where copied memories are declared as cargo"
    ],
    "moral_pressure": "Honor the old harbor debt even though the court uses it to claim modern property",
    "objective": "Recover the pump's oath key and divide the remaining water before the ration count",
    "opposition": "a salvage broker claiming the boundary stones under an old transport charter",
    "patron": "a salvage captain who has found a living village on the old sea floor",
    "premise": "a migration convoy is carrying winter stores across a boundary the dead still enforce",
    "quest_type": "mythic",
    "rewards": [
      "An oath-backed safe passage for anyone carrying an unlicensed identity"
    ],
    "stakes": "A broken compact could make the machine reserve every remaining drop for one settlement",
    "twist": "The machine's oath protects the water from a contaminant buried under the cairn"
  }
}
```

## Sample 35 — chaos-blend / seed 42

- Thread: `mythic-salvage`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a water runner carrying a debt token shared by two rival settlements: a migration convoy is carrying winter stores across a boundary the dead still enforce",
  "clue": "A carved boundary stone bears the same serial mark as the pump",
  "quest": {
    "complications": [
      "The oath token belongs to a family whose copied memories disagree about its wording"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000002a",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "629b8c35-9428-e992-27c5-5636d6b4f2a8",
        "parent_generation_id": null
      },
      "id": "629b8c35-9428-e992-27c5-5636d6b4f2a8",
      "name": "A Chart Under Oath Law",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The settlements share the water while the guardian keeps the pump key"
    ],
    "follow_up_hooks": [
      "A ration engine sends a winter count to a village that disappeared before the collapse"
    ],
    "locations": [
      "the ration hall built around a stone that records each winter share"
    ],
    "moral_pressure": "Guide the convoy only by admitting the village broke its promise",
    "objective": "Find the original ration compact and determine which settlement has kept its terms",
    "opposition": "a sworn guardian who mistakes the salvage crew for oath-breakers",
    "patron": "a water runner carrying a debt token shared by two rival settlements",
    "premise": "a migration convoy is carrying winter stores across a boundary the dead still enforce",
    "quest_type": "mythic",
    "rewards": [
      "A boundary marker that records each settlement's contribution to the route"
    ],
    "stakes": "The refugees may lose their only marked route if the stone is taken as salvage",
    "twist": "The machine's oath protects the water from a contaminant buried under the cairn"
  }
}
```

## Sample 36 — chaos-blend / seed 99

- Thread: `tide-memory-archive`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "the harbor oracle, which has begun deleting living districts: the city will classify a living district as an abandoned myth at sunrise",
  "clue": "A brass chart records footsteps from a district missing from the city's memory",
  "quest": {
    "complications": [
      "The archive's tide schedule was calibrated to a coastline that no longer exists"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000063",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "2b7ad4a9-33c0-8be8-fd82-267e63d7bd80",
        "parent_generation_id": null
      },
      "id": "2b7ad4a9-33c0-8be8-fd82-267e63d7bd80",
      "name": "A Harbor with Two Histories",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The district is restored, but every copied memory becomes public evidence"
    ],
    "follow_up_hooks": [
      "The convoy receives a repair request from a pump beyond the old shoreline"
    ],
    "locations": [
      "the rusted signal tower at the winter port"
    ],
    "moral_pressure": "The oath-speaker can guide the convoy only by admitting the village broke its promise",
    "objective": "Recover the brass memory chart and prove the district is still inhabited",
    "opposition": "a salvage broker who can enforce ancient harbor claims through modern property systems",
    "patron": "the harbor oracle, which has begun deleting living districts",
    "premise": "the city will classify a living district as an abandoned myth at sunrise",
    "quest_type": "technology",
    "rewards": [
      "A ration key recognized by both the machine and the oath-speaking families"
    ],
    "stakes": "The archive may preserve the port only by making every resident's memory public",
    "twist": "The port oracle is preserving evidence that the harbor court ordered erased"
  }
}
```

## Sample 37 — chaos-blend / seed 127

- Thread: `mythic-salvage`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE WITH MINOR EDITING** — The selected title promises memory fees while the premise centers on ration withholding; align the title with the authority dispute.

```json
{
  "briefing": "a salvage captain who has found a living village on the old sea floor: the old water authority is withholding rations until a settlement honors a promise made before the collapse",
  "clue": "The storm signal repeats a ration code from before the sea withdrew",
  "quest": {
    "complications": [
      "The archive's tide gate opens during the same hour the district vanishes"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000007f",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "c05ceb63-b89f-6548-8749-313b8e83fe9a",
        "parent_generation_id": null
      },
      "id": "c05ceb63-b89f-6548-8749-313b8e83fe9a",
      "name": "Memory Fees at the Harbor Gate",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The disputed reserve is divided and the ration engine loses authority over the convoy"
    ],
    "follow_up_hooks": [
      "The harbor court receives a claim filed under the name of its own security daemon"
    ],
    "locations": [
      "a harbor court's data room wired through the breakwater"
    ],
    "moral_pressure": "Send the remaining water to the refugee convoy even if the settlements that honored the oath go dry",
    "objective": "Compare the convoy manifest with the cairn marks and establish who may use the pass",
    "opposition": "the water authority's ration engine, enforcing a treaty no survivor can read",
    "patron": "a salvage captain who has found a living village on the old sea floor",
    "premise": "the old water authority is withholding rations until a settlement honors a promise made before the collapse",
    "quest_type": "mysticism",
    "rewards": [
      "A consent register separating memory access from harbor property rights"
    ],
    "stakes": "A broken compact could make the machine reserve every remaining drop for one settlement",
    "twist": "The security daemon is enforcing the harbor's original rescue oath against the court"
  }
}
```

## Sample 38 — chaos-blend / seed 255

- Thread: `tide-memory-archive`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a salvage clerk trying to return an identity listed as cargo: the archive is charging residents a memory fee to preserve their addresses",
  "clue": "The oracle's ledger contains a tide reading from a sea that no longer exists",
  "quest": {
    "complications": [
      "Repairing the pump will erase the machine's record of who took the last reserve"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "00000000000000ff",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "0f5d31ee-a450-7b89-52dd-b3966f66b409",
        "parent_generation_id": null
      },
      "id": "0f5d31ee-a450-7b89-52dd-b3966f66b409",
      "name": "The Ration Oath at the Iron Cairn",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The memory fee is voided and the archive must accept testimony from unregistered residents"
    ],
    "follow_up_hooks": [
      "The guardian leaves the pass and follows the convoy toward a new shoreline"
    ],
    "locations": [
      "the ration hall built around a stone that records each winter share"
    ],
    "moral_pressure": "Restoring the district exposes residents whose identities they chose to bury",
    "objective": "Find a living witness who can testify to the port's boundary before the archive seals it",
    "opposition": "a union of chart smugglers and civic automata",
    "patron": "a salvage clerk trying to return an identity listed as cargo",
    "premise": "the archive is charging residents a memory fee to preserve their addresses",
    "quest_type": "ambiguity",
    "rewards": [
      "A boundary marker that records each settlement's contribution to the route"
    ],
    "stakes": "An unchallenged salvage claim will let the court auction occupied homes",
    "twist": "The ration engine is following a promise to leave water for travelers, not residents"
  }
}
```

## Sample 39 — chaos-blend / seed 512

- Thread: `tide-memory-archive`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a chart smuggler whose memory was copied by the port machine: the port oracle is selling identity copies under a harbor debt law no living clerk remembers",
  "clue": "A customs stamp appears on a memory copy before its owner entered the port",
  "quest": {
    "complications": [
      "The archive's tide schedule was calibrated to a coastline that no longer exists"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000200",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "9919f4e5-2d6f-316e-d971-7d05b283cd9d",
        "parent_generation_id": null
      },
      "id": "9919f4e5-2d6f-316e-d971-7d05b283cd9d",
      "name": "A Signal from the Last Cairn",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The sale is halted while the court must hear residents under their chosen names"
    ],
    "follow_up_hooks": [
      "A new cairn appears beside the road with a ration count for next winter"
    ],
    "locations": [
      "a harbor court's data room wired through the breakwater"
    ],
    "moral_pressure": "Let the machine save everyone only if one resident gives up their original memory",
    "objective": "Find a living witness who can testify to the port's boundary before the archive seals it",
    "opposition": "the harbor court's security daemon, which controls the archive tide gates",
    "patron": "a chart smuggler whose memory was copied by the port machine",
    "premise": "the port oracle is selling identity copies under a harbor debt law no living clerk remembers",
    "quest_type": "memory",
    "rewards": [
      "A claim ticket honored by both harbor law and machine code"
    ],
    "stakes": "An unchallenged salvage claim will let the court auction occupied homes",
    "twist": "The oracle is preserving the district by hiding it from the city's network"
  }
}
```

## Sample 40 — chaos-blend / seed 1023

- Thread: `tide-memory-archive`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{}`
- Review: **USABLE WITH MINOR EDITING** — The boundary-witness objective and the water-contamination twist pull in different directions; connect the contamination evidence to the archive hearing.

```json
{
  "briefing": "a chart smuggler whose memory was copied by the port machine: the port oracle is selling identity copies to the highest bidder",
  "clue": "A brass chart records footsteps from a district missing from the city's memory",
  "quest": {
    "complications": [
      "The pump accepts only a vow spoken in a language no living settler remembers"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "00000000000003ff",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "a20558c6-b474-323b-1e49-dd66080ebf37",
        "parent_generation_id": null
      },
      "id": "a20558c6-b474-323b-1e49-dd66080ebf37",
      "name": "Memory Fees at the Harbor Gate",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The sale is halted while the court must hear residents under their chosen names"
    ],
    "follow_up_hooks": [
      "The chart marks a second living district beneath the harbor court"
    ],
    "locations": [
      "the customs kiosk where copied memories are declared as cargo"
    ],
    "moral_pressure": "The machine can save everyone only if one resident gives up their original memory",
    "objective": "Recover the brass memory chart and prove the district is still inhabited",
    "opposition": "a harbor assessor who treats copied identities as transferable property",
    "patron": "a chart smuggler whose memory was copied by the port machine",
    "premise": "the port oracle is selling identity copies to the highest bidder",
    "quest_type": "memory",
    "rewards": [
      "A consent register separating memory access from harbor property rights"
    ],
    "stakes": "An unchallenged salvage claim will let the court auction occupied homes",
    "twist": "The machine's oath protects the water from a contaminant buried under the cairn"
  }
}
```

## Sample 41 — chaos-blend / seed 42

- Thread: `mythic-salvage`
- Axis overrides: `{"mythic": 0}`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a convoy mechanic whose repair manual contains an ancestral boundary song: the convoy's salvage claim includes a boundary stone still used to guide refugees",
  "clue": "The pump's maintenance log counts one more settlement than the oath names",
  "quest": {
    "complications": [
      "The court will hear the claim only from a witness whose memory has been copied"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 0,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000002a",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "9bb16cf3-ec12-5273-9357-cae407ad2b0e",
        "parent_generation_id": null
      },
      "id": "9bb16cf3-ec12-5273-9357-cae407ad2b0e",
      "name": "The Port That Audited Its Ghosts",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The convoy reaches shore, but the boundary oath now includes every survivor"
    ],
    "follow_up_hooks": [
      "A ration engine sends a winter count to a village that disappeared before the collapse"
    ],
    "locations": [
      "the ration hall built around a stone that records each winter share"
    ],
    "moral_pressure": "Let the machine save everyone only if one resident gives up their original memory",
    "objective": "Trace the disputed ration count to its source before either settlement opens the reserve",
    "opposition": "a settlement elder willing to sacrifice the convoy to preserve the old boundary",
    "patron": "a convoy mechanic whose repair manual contains an ancestral boundary song",
    "premise": "the convoy's salvage claim includes a boundary stone still used to guide refugees",
    "quest_type": "bleak",
    "rewards": [
      "A tide ledger that records consent separately from ownership"
    ],
    "stakes": "A false ration count could leave both settlements without enough water to plant",
    "twist": "The port oracle is preserving evidence that the harbor court ordered erased"
  }
}
```

## Sample 42 — chaos-blend / seed 42

- Thread: `mythic-salvage`
- Axis overrides: `{"mythic": 100}`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a convoy mechanic whose repair manual contains an ancestral boundary song: a migration convoy is carrying winter stores across a boundary the dead still enforce",
  "clue": "The storm signal repeats a ration code from before the sea withdrew",
  "quest": {
    "complications": [
      "The cairn records the last ration promise in a script the pump can read but no witness can speak"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 100,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000002a",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "5d651e0b-e966-0d78-fc70-b7dc68315063",
        "parent_generation_id": null
      },
      "id": "5d651e0b-e966-0d78-fc70-b7dc68315063",
      "name": "A Chart Under Oath Law",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The disputed reserve is divided and the ration engine loses authority over the convoy"
    ],
    "follow_up_hooks": [
      "The guardian leaves the pass and follows the convoy toward a new shoreline"
    ],
    "locations": [
      "a salvage court built around an old water authority terminal"
    ],
    "moral_pressure": "Reveal who took the last reserve even though the oath protects that person's descendants",
    "objective": "Trace the disputed ration count to its source before either settlement opens the reserve",
    "opposition": "a settlement elder willing to sacrifice the convoy to preserve the old boundary",
    "patron": "a convoy mechanic whose repair manual contains an ancestral boundary song",
    "premise": "a migration convoy is carrying winter stores across a boundary the dead still enforce",
    "quest_type": "post-apocalyptic",
    "rewards": [
      "A claim ticket honored by both harbor law and machine code"
    ],
    "stakes": "The guardian may turn the convoy back and leave its winter stores beyond the pass",
    "twist": "The salvage claim was filed by the refugees to keep the broker from taking the pass"
  }
}
```

## Sample 43 — chaos-blend / seed 42

- Thread: `mythic-salvage`
- Axis overrides: `{"chaos": 0}`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a water runner carrying a debt token shared by two rival settlements: a migration convoy is carrying winter stores across a boundary the dead still enforce",
  "clue": "A repair tag is tied with the same knot used to mark winter graves",
  "quest": {
    "complications": [
      "The convoy's water tank bears a seal that belongs to a settlement erased from the map"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 0,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000002a",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "0de2382b-3e2c-0c25-a8b5-490ec45247ed",
        "parent_generation_id": null
      },
      "id": "0de2382b-3e2c-0c25-a8b5-490ec45247ed",
      "name": "The Oath at the Dry Harbor",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The pass remains open, but the settlements must maintain it together"
    ],
    "follow_up_hooks": [
      "The convoy carries a cairn stone carved with a settlement that has not been founded"
    ],
    "locations": [
      "the migration road where a convoy's route crosses a standing-stone line"
    ],
    "moral_pressure": "Send the remaining water to the refugee convoy even if the settlements that honored the oath go dry",
    "objective": "Reach the signal tower and guide the convoy past the drowned boundary stones",
    "opposition": "a settlement elder willing to sacrifice the convoy to preserve the old boundary",
    "patron": "a water runner carrying a debt token shared by two rival settlements",
    "premise": "a migration convoy is carrying winter stores across a boundary the dead still enforce",
    "quest_type": "post-apocalyptic",
    "rewards": [
      "A repair ledger that records every settlement's contribution to the shared reserve"
    ],
    "stakes": "The refugees may lose their only marked route if the stone is taken as salvage",
    "twist": "The guardian is keeping the route open for a settlement that has not reached the pass"
  }
}
```

## Sample 44 — chaos-blend / seed 42

- Thread: `tide-memory-archive`
- Axis overrides: `{"chaos": 100}`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a salvage clerk trying to return an identity listed as cargo: a tide ledger has inherited an oath to protect a port the city erased",
  "clue": "A harbor receipt bears both a machine checksum and a witness mark from the old court",
  "quest": {
    "complications": [
      "The convoy carries the only witness who can interpret the ration code, but their family claims them as an heir"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 100,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000002a",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "c3bf9eaf-7049-e1a9-e76a-a466ab4344c1",
        "parent_generation_id": null
      },
      "id": "c3bf9eaf-7049-e1a9-e76a-a466ab4344c1",
      "name": "The Convoy's Boundary Claim",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The memory fee is voided and the archive must accept testimony from unregistered residents"
    ],
    "follow_up_hooks": [
      "A ration engine sends a winter count to a village that disappeared before the collapse"
    ],
    "locations": [
      "the rusted signal tower at the winter port"
    ],
    "moral_pressure": "Reveal who took the last reserve even though the oath protects that person's descendants",
    "objective": "Find a living witness who can testify to the port's boundary before the archive seals it",
    "opposition": "the harbor court's security daemon, which controls the archive tide gates",
    "patron": "a salvage clerk trying to return an identity listed as cargo",
    "premise": "a tide ledger has inherited an oath to protect a port the city erased",
    "quest_type": "technology",
    "rewards": [
      "A shared water charter and a pump that can run without fuel"
    ],
    "stakes": "The harbor will surrender a living district as salvage if no one challenges the classification",
    "twist": "The salvage claim was filed by the refugees to keep the broker from taking the pass"
  }
}
```

## Sample 45 — chaos-blend / seed 42

- Thread: `mythic-salvage`
- Axis overrides: `{"epic": 0}`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a salvage captain who has found a living village on the old sea floor: a storm signal has summoned a convoy to a harbor erased from every map",
  "clue": "A repair tag is tied with the same knot used to mark winter graves",
  "quest": {
    "complications": [
      "The oath token belongs to a family whose copied memories disagree about its wording"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 0,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000002a",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "5febfac5-8fac-adda-89c0-4a97da5e81c8",
        "parent_generation_id": null
      },
      "id": "5febfac5-8fac-adda-89c0-4a97da5e81c8",
      "name": "A Signal from the Last Cairn",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:mythic-salvage"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The pass remains open, but the settlements must maintain it together"
    ],
    "follow_up_hooks": [
      "The guardian leaves the pass and follows the convoy toward a new shoreline"
    ],
    "locations": [
      "the migration road where a convoy's route crosses a standing-stone line"
    ],
    "moral_pressure": "Share water even if the upper settlement must give up its winter reserve",
    "objective": "Trace the disputed ration count to its source before either settlement opens the reserve",
    "opposition": "a sworn guardian who mistakes the salvage crew for oath-breakers",
    "patron": "a salvage captain who has found a living village on the old sea floor",
    "premise": "a storm signal has summoned a convoy to a harbor erased from every map",
    "quest_type": "oath",
    "rewards": [
      "A shared water charter and a pump that can run without fuel"
    ],
    "stakes": "The guardian may turn the convoy back and leave its winter stores beyond the pass",
    "twist": "The salvage claim was filed by the refugees to keep the broker from taking the pass"
  }
}
```

## Sample 46 — chaos-blend / seed 42

- Thread: `tide-memory-archive`
- Axis overrides: `{"epic": 100}`
- Constraints: `{}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a synthetic pilot carrying a family debt in its navigation cache: the port oracle is selling identity copies under a harbor debt law no living clerk remembers",
  "clue": "The tide clock advances whenever a resident declines to identify themselves",
  "quest": {
    "complications": [
      "Repairing the pump will erase the machine's record of who took the last reserve"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {},
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 100,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000002a",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "d154c17c-525a-ade8-ec8a-8008328f89ad",
        "parent_generation_id": null
      },
      "id": "d154c17c-525a-ade8-ec8a-8008328f89ad",
      "name": "The Chart That Remembered",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The oracle keeps its archive, but the harbor loses the right to trade stored memories"
    ],
    "follow_up_hooks": [
      "A municipal boundary marker appears on the sea floor beside the old port"
    ],
    "locations": [
      "a convoy weigh station where the boundary markers have been reused as road signs"
    ],
    "moral_pressure": "Let the machine save everyone only if one resident gives up their original memory",
    "objective": "Copy the oracle's tide ledger before the next harbor census",
    "opposition": "the oracle's compliance process, which treats every unregistered memory as stolen cargo",
    "patron": "a synthetic pilot carrying a family debt in its navigation cache",
    "premise": "the port oracle is selling identity copies under a harbor debt law no living clerk remembers",
    "quest_type": "pirate",
    "rewards": [
      "A harbor appeal writ that can suspend one identity transfer"
    ],
    "stakes": "Residents may lose the right to refuse copies made from their own memories",
    "twist": "The convoy is carrying the only witness who remembers the harbor before it sank"
  }
}
```

## Sample 47 — dark-fantasy / seed 101

- Thread: `winter-lanterns`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{"inspiration": "a bell that rings below the river"}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a shepherd whose flock has returned from the dead village: the beacon is warming an abandoned settlement while the occupied hospice freezes. Inspiration: a bell that rings below the river",
  "clue": "The beacon's fuel bears the hospice keeper's family crest",
  "quest": {
    "complications": [
      "The hospice can be warmed only by closing the pass to the convoy"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {
              "inspiration": "a bell that rings below the river"
            },
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000065",
            "universe_preset": "dark-fantasy"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "49fda756-6ead-1d2e-b1f1-6b1e462b33b4",
        "parent_generation_id": null
      },
      "id": "49fda756-6ead-1d2e-b1f1-6b1e462b33b4",
      "name": "The Night the Pass Went Dark",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:dark-fantasy",
        "thread:winter-lanterns"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The beacon is restored, but the hospice must ration its healing oil"
    ],
    "follow_up_hooks": [
      "The warm footprints continue beyond the valley boundary"
    ],
    "locations": [
      "the frost-locked hospice at Crow Pass"
    ],
    "moral_pressure": "Restore the dead village's warmth by taking fuel from families already short of it",
    "objective": "Follow the heat trail to its source and reopen the hospice wards before dusk",
    "opposition": "a fuel merchant selling the hospice's stolen heat back to the village",
    "patron": "a shepherd whose flock has returned from the dead village",
    "premise": "the beacon is warming an abandoned settlement while the occupied hospice freezes",
    "quest_type": "survival",
    "rewards": [
      "A lantern wick that burns only when someone offers shelter freely"
    ],
    "stakes": "The hospice patients will freeze before the convoy can reach the pass",
    "twist": "The lantern's bargain was made by the first hospice patients, not its keepers"
  }
}
```

## Sample 48 — pirate / seed 202

- Thread: `mutiny-debt`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{"inspiration": "a witness who remembers tomorrow"}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a ship's surgeon keeping the crew's unpaid names: the naval court will execute the navigator before hearing why the brig changed course. Inspiration: a witness who remembers tomorrow",
  "clue": "A tide stain on the order proves it was aboard before the ship left port",
  "quest": {
    "complications": [
      "The chart is written in a code that changes with the tide"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {
              "inspiration": "a witness who remembers tomorrow"
            },
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "00000000000000ca",
            "universe_preset": "pirate"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "867c116e-796e-24ad-a914-da389dacf1a9",
        "parent_generation_id": null
      },
      "id": "867c116e-796e-24ad-a914-da389dacf1a9",
      "name": "A Chart with Two Masters",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:pirate",
        "thread:mutiny-debt"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The brig is released to its crew and the court loses its claim to the cargo"
    ],
    "follow_up_hooks": [
      "The harbor receives a commission addressed to its elected tide keeper"
    ],
    "locations": [
      "a tide cave where the crew held its first vote"
    ],
    "moral_pressure": "Clear the navigator and reveal the crew member who changed course to save captives",
    "objective": "Find who pledged the crew's shares and recover the original contract",
    "opposition": "a naval auditor who can condemn the brig without hearing its crew",
    "patron": "a ship's surgeon keeping the crew's unpaid names",
    "premise": "the naval court will execute the navigator before hearing why the brig changed course",
    "quest_type": "mutiny",
    "rewards": [
      "The recovered prize shares and a berth chosen by the crew"
    ],
    "stakes": "The verdict could hang sailors who never received the disputed order",
    "twist": "The crew voted to change course to rescue a ship the captain had abandoned"
  }
}
```

## Sample 49 — occult / seed 303

- Thread: `borrowed-hour`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{"inspiration": "an unclaimed key from the registry"}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "an apprentice clock inspector whose mentor has lost an hour: a public clock has begun charging people time for delays it creates. Inspiration: an unclaimed key from the registry",
  "clue": "A funeral notice is dated tomorrow but carries today's signature",
  "quest": {
    "complications": [
      "The clock can be stopped only by certifying it as accurate"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {
              "inspiration": "an unclaimed key from the registry"
            },
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000012f",
            "universe_preset": "occult"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "ea62407b-cb48-7441-eaa1-7eb13eb46b47",
        "parent_generation_id": null
      },
      "id": "ea62407b-cb48-7441-eaa1-7eb13eb46b47",
      "name": "A Bell for the Unburied",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:occult",
        "thread:borrowed-hour"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The public clock is certified, forcing the town to account for every missing minute"
    ],
    "follow_up_hooks": [
      "A courthouse bell rings once for every minute the town has returned"
    ],
    "locations": [
      "the flooded platform beneath Mercy Street"
    ],
    "moral_pressure": "Preserve the night route by letting the workers continue paying its hidden cost",
    "objective": "Compare the workers' timecards with the public clock before the next shift change",
    "opposition": "a timekeeper who profits from every unrecorded minute",
    "patron": "an apprentice clock inspector whose mentor has lost an hour",
    "premise": "a public clock has begun charging people time for delays it creates",
    "quest_type": "ritual",
    "rewards": [
      "A written appointment that guarantees its holder will be heard at the next midnight"
    ],
    "stakes": "A delayed bus could strand patients after the clinic closes",
    "twist": "The passenger is carrying the missing interval to a place where time has stopped"
  }
}
```

## Sample 50 — monster-hunter / seed 404

- Thread: `false-trophy`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{"inspiration": "a migration trail through winter glass"}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a taxidermist asked to certify a trophy they did not prepare: the lodge is selling a dangerous species as harmless to protect its hunting license. Inspiration: a migration trail through winter glass",
  "clue": "The trophy's tooth wear matches a scavenger, not the reported hunter",
  "quest": {
    "complications": [
      "The beast is protecting a village from a predator no one has seen"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {
              "inspiration": "a migration trail through winter glass"
            },
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "0000000000000194",
            "universe_preset": "monster-hunter"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "c191bed2-f472-98b3-b79a-0bb321d438c5",
        "parent_generation_id": null
      },
      "id": "c191bed2-f472-98b3-b79a-0bb321d438c5",
      "name": "Proof of the Last Stag",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:monster-hunter",
        "thread:false-trophy"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The hunter is cleared, but the ranger service must publish its failed report"
    ],
    "follow_up_hooks": [
      "A second trophy appears with the same export date and a different species label"
    ],
    "locations": [
      "the ravine where the last horned beast was seen"
    ],
    "moral_pressure": "The lodge keeper will clear the tracker only if the village becomes a sanctuary",
    "objective": "Compare the trophy with the attack-site evidence before the lodge certifies its report",
    "opposition": "the real predator, feeding near the route used by livestock herders",
    "patron": "a taxidermist asked to certify a trophy they did not prepare",
    "premise": "the lodge is selling a dangerous species as harmless to protect its hunting license",
    "quest_type": "evidence",
    "rewards": [
      "A verified field report that can protect the actual nesting range"
    ],
    "stakes": "A protective order will be lifted if the trophy is accepted as proof",
    "twist": "The beast is real but has been using the forged trophy as a decoy"
  }
}
```

## Sample 51 — contemporary-weird / seed 505

- Thread: `route-that-stops`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{"inspiration": "a maintenance ticket for a missing floor"}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "a neighborhood organizer whose meeting place is missing from every map: passengers are filing valid expense claims for a stop the transit authority insists does not exist. Inspiration: a maintenance ticket for a missing floor",
  "clue": "The lost-property desk has collected the same umbrella after every unlisted stop",
  "quest": {
    "complications": [
      "The stop appears only when no passenger requests it by name"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {
              "inspiration": "a maintenance ticket for a missing floor"
            },
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "00000000000001f9",
            "universe_preset": "contemporary-weird"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "ad4c5cf3-9faf-03a2-fccb-55db7440defc",
        "parent_generation_id": null
      },
      "id": "ad4c5cf3-9faf-03a2-fccb-55db7440defc",
      "name": "The Bus Schedule with One Extra Minute",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:contemporary-weird",
        "thread:route-that-stops"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The stop is restored to the map while one passenger remains missing"
    ],
    "follow_up_hooks": [
      "A reimbursement arrives from a route that was canceled before the city was founded"
    ],
    "locations": [
      "a shelter that appears only on the night driver's inspection list"
    ],
    "moral_pressure": "The driver asks you to leave their child off the corrected count",
    "objective": "Ride the route, record the missing stop, and bring proof to the transit hearing",
    "opposition": "the route itself, which returns to the stop whenever someone names it",
    "patron": "a neighborhood organizer whose meeting place is missing from every map",
    "premise": "passengers are filing valid expense claims for a stop the transit authority insists does not exist",
    "quest_type": "memory",
    "rewards": [
      "A protected night-service order that recognizes the disputed stop without publishing its coordinates"
    ],
    "stakes": "Night-shift staff will lose their only reliable connection to the hospital",
    "twist": "The route planner requested the stop years ago to reach a patient who has since recovered"
  }
}
```

## Sample 52 — chaos-blend / seed 606

- Thread: `tide-memory-archive`
- Axis overrides: `none; all axes neutral at 50`
- Constraints: `{"inspiration": "a tide chart signed by a machine"}`
- Review: **USABLE AS-IS**

```json
{
  "briefing": "the harbor oracle, which has begun deleting living districts: the city will classify a living district as an abandoned myth at sunrise. Inspiration: a tide chart signed by a machine",
  "clue": "The tide clock advances whenever a resident declines to identify themselves",
  "quest": {
    "complications": [
      "The cairn records the last ration promise in a script the pump can read but no witness can speak"
    ],
    "envelope": {
      "canon_state": "candidate",
      "created_at_unix_ms": 0,
      "entity_type": "quest",
      "favorite": false,
      "generation_provenance": {
        "action": "forge",
        "dna": {
          "content_version": 3,
          "generator_version": 1,
          "recipe": {
            "constraints": {
              "inspiration": "a tide chart signed by a machine"
            },
            "creative_axes": {
              "levels": {
                "ambiguity": 50,
                "bleak": 50,
                "chaos": 50,
                "epic": 50,
                "mysticism": 50,
                "mythic": 50,
                "surreal": 50,
                "technology": 50
              }
            },
            "dependencies": {
              "02-title": [
                "01-frame"
              ],
              "03-type": [
                "01-frame"
              ],
              "04-premise": [
                "01-frame"
              ],
              "05-patron": [
                "01-frame"
              ],
              "06-objective": [
                "01-frame"
              ],
              "07-stakes": [
                "01-frame"
              ],
              "08-opposition": [
                "01-frame"
              ],
              "09-complications": [
                "01-frame"
              ],
              "10-twist": [
                "01-frame"
              ],
              "11-moral-pressure": [
                "01-frame"
              ],
              "12-locations": [
                "01-frame"
              ],
              "13-rewards": [
                "01-frame"
              ],
              "14-failure": [
                "01-frame"
              ],
              "15-follow-up": [
                "01-frame"
              ],
              "16-clue": [
                "01-frame"
              ],
              "17-briefing": [
                "01-frame",
                "04-premise",
                "05-patron"
              ]
            },
            "field_controls": {
              "01-frame": {
                "locked": false,
                "nonce": 0
              },
              "02-title": {
                "locked": false,
                "nonce": 0
              },
              "03-type": {
                "locked": false,
                "nonce": 0
              },
              "04-premise": {
                "locked": false,
                "nonce": 0
              },
              "05-patron": {
                "locked": false,
                "nonce": 0
              },
              "06-objective": {
                "locked": false,
                "nonce": 0
              },
              "07-stakes": {
                "locked": false,
                "nonce": 0
              },
              "08-opposition": {
                "locked": false,
                "nonce": 0
              },
              "09-complications": {
                "locked": false,
                "nonce": 0
              },
              "10-twist": {
                "locked": false,
                "nonce": 0
              },
              "11-moral-pressure": {
                "locked": false,
                "nonce": 0
              },
              "12-locations": {
                "locked": false,
                "nonce": 0
              },
              "13-rewards": {
                "locked": false,
                "nonce": 0
              },
              "14-failure": {
                "locked": false,
                "nonce": 0
              },
              "15-follow-up": {
                "locked": false,
                "nonce": 0
              },
              "16-clue": {
                "locked": false,
                "nonce": 0
              },
              "17-briefing": {
                "locked": false,
                "nonce": 0
              }
            },
            "kind": "quest",
            "recipe_version": 1,
            "root_seed": "000000000000025e",
            "universe_preset": "chaos-blend"
          },
          "schema_version": 1
        },
        "generated_at_unix_ms": 0,
        "generation_id": "cea4bf48-d751-6860-6d34-32ccb06ac1b9",
        "parent_generation_id": null
      },
      "id": "cea4bf48-d751-6860-6d34-32ccb06ac1b9",
      "name": "The Archive Owed a Tide",
      "notes": null,
      "origin": "generated",
      "project_id": "07070707-0707-0707-0707-070707070707",
      "schema_version": 1,
      "tags": [
        "preset:chaos-blend",
        "thread:tide-memory-archive"
      ],
      "updated_at_unix_ms": 0
    },
    "failure_consequences": [
      "The sale is halted while the court must hear residents under their chosen names"
    ],
    "follow_up_hooks": [
      "The convoy carries a cairn stone carved with a settlement that has not been founded"
    ],
    "locations": [
      "the harbor's submerged claims office beneath the tide clock"
    ],
    "moral_pressure": "Restore the district and expose residents whose identities they chose to bury",
    "objective": "Trace the memory fees to the court that authorized them and challenge the debt",
    "opposition": "the oracle's compliance process, which treats every unregistered memory as stolen cargo",
    "patron": "the harbor oracle, which has begun deleting living districts",
    "premise": "the city will classify a living district as an abandoned myth at sunrise",
    "quest_type": "tide",
    "rewards": [
      "A safe route to the village beneath the old shoreline"
    ],
    "stakes": "The archive may preserve the port only by making every resident's memory public",
    "twist": "The oracle is preserving the district by hiding it from the city's network"
  }
}
```
