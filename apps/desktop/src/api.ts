import { invoke } from '@tauri-apps/api/core';

export type FieldValue = string | string[];
export type ForgePayload =
  | { kind: 'character_candidate'; value: CharacterCandidate }
  | { kind: 'quest_candidate'; value: QuestCandidate }
  | { kind: 'location_candidate'; value: LocationCandidate }
  | { kind: 'faction_candidate'; value: FactionCandidate }
  | { kind: 'item_candidate'; value: ItemCandidate }
  | { kind: 'creature_candidate'; value: CreatureCandidate }
  | { kind: 'character'; value: Character }
  | { kind: 'quest'; value: Quest }
  | { kind: 'location'; value: Location }
  | { kind: 'faction'; value: Faction }
  | { kind: 'item'; value: Item }
  | { kind: 'creature'; value: Creature };
export type CharacterCandidate = {
  character: Character;
  dna: ForgeDNA;
  field_origins: Record<string, string>;
};
export type QuestCandidate = {
  quest: Quest;
  dna: ForgeDNA;
  field_origins: Record<string, string>;
  clue?: string | null;
  briefing?: string | null;
};
export type LocationCandidate = {
  location: Location;
  dna: ForgeDNA;
  field_origins: Record<string, string>;
};
export type FactionCandidate = {
  faction: Faction;
  dna: ForgeDNA;
  field_origins: Record<string, string>;
};
export type Faction = {
  envelope: Envelope;
  faction_profile: FactionProfile;
  identity: string;
  ideology: string;
  primary_goal: string;
  methods: string;
  culture: string;
  resources: string;
  public_face: string;
  hidden_truth: string;
  symbols: string;
  reputation: string;
  story_hooks: string[];
};
export type FactionProfile =
  | 'guild'
  | 'order'
  | 'cult'
  | 'government'
  | 'kingdom'
  | 'military'
  | 'corporation'
  | 'gang'
  | 'rebellion'
  | 'religious-order'
  | 'tribe'
  | 'secret-society'
  | 'other';
export type ItemCandidate = {
  item: Item;
  dna: ForgeDNA;
  field_origins: Record<string, string>;
};
export type Item = {
  envelope: Envelope;
  item_profile: ItemProfile;
  appearance: string;
  purpose: string;
  properties: string;
  origin: string;
  history: string;
  power_or_function: string;
  cost_or_drawback: string;
  rarity: string;
  condition: string;
  secret: string;
  story_hooks: string[];
};
export type ItemProfile =
  | 'weapon'
  | 'armor'
  | 'artifact'
  | 'consumable'
  | 'tool'
  | 'key-item'
  | 'resource'
  | 'treasure'
  | 'book'
  | 'quest-item'
  | 'other';
export type CreatureCandidate = {
  creature: Creature;
  dna: ForgeDNA;
  field_origins: Record<string, string>;
};
export type Creature = {
  envelope: Envelope;
  creature_profile: CreatureProfile;
  appearance: string;
  habitat: string;
  behavior: string;
  ecology: string;
  threat: string;
  abilities_summary: string;
  weakness: string;
  origin: string;
  signs_of_presence: string;
  loot_or_resource: string;
  secret: string;
  story_hooks: string[];
};
export type CreatureProfile =
  | 'beast'
  | 'monster'
  | 'undead'
  | 'construct'
  | 'aberration'
  | 'spirit'
  | 'elemental'
  | 'humanoid-creature'
  | 'swarm'
  | 'boss'
  | 'other';
export type Character = {
  envelope: Envelope;
  aliases: string[];
  title: string;
  archetype: string;
  role: string;
  appearance: string;
  faction_hint: string;
  signature_gear: string;
  personality: string[];
  desire: string;
  fear: string;
  flaw: string;
  secret: string;
  skills: string[];
  abilities: string[];
  relationship_hook: string;
  backstory_hook: string;
  story_hooks: string[];
};
export type Quest = {
  envelope: Envelope;
  quest_type: string;
  premise: string;
  patron: string;
  objective: string;
  stakes: string;
  opposition: string;
  complications: string[];
  twist: string;
  moral_pressure: string;
  locations: string[];
  rewards: string[];
  failure_consequences: string[];
  follow_up_hooks: string[];
};
export type Location = {
  envelope: Envelope;
  location_profile:
    | 'settlement'
    | 'district'
    | 'building'
    | 'dungeon'
    | 'ruins'
    | 'wilderness'
    | 'landmark'
    | 'interior'
    | 'site'
    | 'other';
  summary: string;
  visual_identity: string;
  atmosphere: string;
  purpose: string;
  history: string;
  inhabitants_summary: string;
  notable_features: string[];
  danger: string;
  secret: string;
  story_hooks: string[];
};
export type Envelope = {
  id: string;
  project_id: string;
  entity_type:
    | 'character'
    | 'quest'
    | 'location'
    | 'faction'
    | 'item'
    | 'creature';
  origin?: string;
  name: string;
  canon_state: 'draft' | 'candidate' | 'canon' | 'archived';
  favorite: boolean;
  created_at_unix_ms: number;
  updated_at_unix_ms: number;
  tags: string[];
  generation_provenance: { dna: ForgeDNA; action: string } | null;
};
export type ForgeDNA = {
  generator_version?: number;
  content_version?: number;
  recipe: {
    kind?: 'character' | 'quest' | 'location' | 'faction' | 'item' | 'creature';
    root_seed: string;
    universe_preset: string;
    creative_axes: { levels: Record<Axis, number> };
    constraints: Record<string, string>;
    field_controls: Record<string, { nonce: number; locked: boolean }>;
  };
};
export type Axis =
  | 'bleak'
  | 'mythic'
  | 'chaos'
  | 'epic'
  | 'ambiguity'
  | 'surreal'
  | 'technology'
  | 'mysticism';
export type Project = {
  name: string;
  path: string;
  id: string;
  createdAtUnixMs: number;
  updatedAtUnixMs: number;
};
export type RecentProject = {
  name: string;
  path: string;
  lastOpenedAtUnixMs: number;
  exists: boolean;
};
export type EntityRecord = { entity: ForgePayload; writeVersion: number };
export type ForgeRelationship = {
  id: string;
  project_id: string;
  relationship_type: 'born_in' | 'member_of' | 'owns' | 'opposed_by';
  source_id: string;
  target_id: string;
  state: 'candidate' | 'canon' | 'archived';
  origin: 'manual' | 'generated' | 'imported';
  created_at_unix_ms: number;
  updated_at_unix_ms: number;
};
export type ConnectionView = {
  relationship: ForgeRelationship;
  writeVersion: number;
  role: 'source' | 'target';
  relatedId: string;
  relatedType: Envelope['entity_type'];
  relatedName: string | null;
  relatedAvailability: 'available' | 'archived' | 'missing';
  definition: {
    key: ForgeRelationship['relationship_type'];
    relationshipType: ForgeRelationship['relationship_type'];
    source: Envelope['entity_type'];
    target: Envelope['entity_type'];
    directed: boolean;
    sourcePhrase: string;
    inversePhrase: string;
  };
};
export type RelationshipRevision = {
  revisionNumber: number;
  reason: string;
  relationship: ForgeRelationship;
  createdAtUnixMs: number;
};
export type HometownContextSnapshot = {
  format_version: number;
  context_recipe_id: string;
  context_recipe_version: number;
  source_entity_id: string;
  source_write_version: number;
  source_canon_state: string;
  source_type: 'character';
  source_generation_kind: string;
  source_generator_version: number;
  source_content_version: number;
  facts: {
    name: string;
    title: string;
    archetype: string;
    role: string;
    faction_hint: string;
    backstory_hook: string;
    desire: string;
    secret: string;
  };
  inherited_preset: string;
  inherited_creative_axes: { levels: Record<Axis, number> };
  source_body_hash: string;
  context_hash: string;
};
export type ForgeResultBundle = {
  primary: LocationCandidate;
  supporting: never[];
  relationship: ForgeRelationship;
  recipe: {
    format_version: number;
    action_id: string;
    action_version: number;
    context_recipe_id: string;
    context_recipe_version: number;
    context_snapshot: HometownContextSnapshot;
    action_seed: number;
    location_dna: ForgeDNA;
  };
  result: {
    format_version: number;
    primary_output_id: string;
    primary_output_type: 'location';
    relationship_ids: string[];
    relationship_type: 'born_in';
    source_entity_id: string;
    warnings: string[];
  };
};
export type ForgeBundleSaveResponse = {
  location: EntityRecord;
  relationship: ForgeRelationship;
  eventId: number;
};
export type FactionContextSnapshot = Omit<HometownContextSnapshot, 'facts'> & {
  facts: HometownContextSnapshot['facts'] & {
    personality: string[];
    fear: string;
    flaw: string;
    relationship_hook: string;
  };
};
export type FactionResultBundle = Omit<
  ForgeResultBundle,
  'primary' | 'recipe' | 'result'
> & {
  primary: FactionCandidate;
  recipe: Omit<
    ForgeResultBundle['recipe'],
    'location_dna' | 'context_snapshot'
  > & {
    target_dna: ForgeDNA;
    context_snapshot: FactionContextSnapshot;
  };
  result: Omit<
    ForgeResultBundle['result'],
    'primary_output_type' | 'relationship_type'
  > & {
    primary_output_type: 'faction';
    relationship_type: 'member_of';
  };
};
export type FactionBundleSaveResponse = {
  faction: EntityRecord;
  relationship: ForgeRelationship;
  eventId: number;
};
export type SignatureItemContextSnapshot = Omit<
  HometownContextSnapshot,
  'facts'
> & {
  facts: Omit<HometownContextSnapshot['facts'], 'faction_hint'> & {
    appearance: string;
    personality: string[];
    fear: string;
    flaw: string;
    signature_gear: string;
    skills: string[];
    abilities: string[];
    relationship_hook: string;
  };
};
export type SignatureItemResultBundle = Omit<
  FactionResultBundle,
  'primary' | 'recipe' | 'result'
> & {
  primary: ItemCandidate;
  recipe: Omit<FactionResultBundle['recipe'], 'context_snapshot'> & {
    context_snapshot: SignatureItemContextSnapshot;
  };
  result: Omit<
    FactionResultBundle['result'],
    'primary_output_type' | 'relationship_type'
  > & { primary_output_type: 'item'; relationship_type: 'owns' };
};
export type SignatureItemBundleSaveResponse = {
  item: EntityRecord;
  relationship: ForgeRelationship;
  eventId: number;
};
export type HistoryRecord = {
  runId: string;
  entityId: string | null;
  action: string;
  createdAtUnixMs: number;
};
export type Revision = {
  revisionNumber: number;
  reason: string;
  entity: ForgePayload;
  createdAtUnixMs: number;
};
export type UserSettings = { version: 1; reducedMotion: boolean };
export type WorkingState = {
  entity: ForgePayload | null;
  canUndo: boolean;
  canRedo: boolean;
};
export type UiError =
  | { kind?: string; message?: string; expected?: number; actual?: number }
  | string;

export const presets = [
  ['dark-fantasy', 'Dark Fantasy'],
  ['pirate', 'Pirate'],
  ['steampunk', 'Steampunk'],
  ['cyberpunk', 'Cyberpunk'],
  ['nordic-myth', 'Nordic Myth'],
  ['post-apocalyptic', 'Post-Apocalyptic'],
  ['occult', 'Occult'],
  ['monster-hunter', 'Monster Hunter'],
  ['rogue-assassin', 'Rogue / Assassin'],
  ['contemporary-weird', 'Contemporary Weird'],
  ['chaos-blend', 'Chaos Blend'],
] as const;

export const axisMeta: {
  id: Axis;
  label: string;
  low: string;
  high: string;
  hint: string;
}[] = [
  {
    id: 'bleak',
    label: 'Bleakness',
    low: 'Hopeful',
    high: 'Bleak',
    hint: 'Influences how hopeful or bleak a result may feel.',
  },
  {
    id: 'mythic',
    label: 'Mythic',
    low: 'Grounded',
    high: 'Mythic',
    hint: 'Influences how legendary or grounded a result may feel.',
  },
  {
    id: 'chaos',
    label: 'Chaos',
    low: 'Ordered',
    high: 'Chaotic',
    hint: 'Influences how orderly or unpredictable a result may feel.',
  },
  {
    id: 'epic',
    label: 'Scale',
    low: 'Intimate',
    high: 'Epic',
    hint: 'Influences the scale and reach of the result.',
  },
  {
    id: 'ambiguity',
    label: 'Morality',
    low: 'Clear',
    high: 'Ambiguous',
    hint: 'Influences how clear-cut or morally uncertain a result may feel.',
  },
  {
    id: 'surreal',
    label: 'Strangeness',
    low: 'Familiar',
    high: 'Surreal',
    hint: 'Influences how familiar or unusual a result may feel.',
  },
  {
    id: 'technology',
    label: 'Technology',
    low: 'Primitive',
    high: 'Advanced',
    hint: 'Influences the technology present in the result.',
  },
  {
    id: 'mysticism',
    label: 'Mysticism',
    low: 'None',
    high: 'Dominant',
    hint: 'Influences the presence of mysticism in the result.',
  },
];

export type VisualRole =
  | 'portrait'
  | 'full_body'
  | 'character_sheet'
  | 'establishing_shot'
  | 'environment_concept'
  | 'interior_concept'
  | 'emblem'
  | 'banner'
  | 'uniform_concept'
  | 'item_concept'
  | 'inventory_icon'
  | 'prop_sheet'
  | 'creature_concept'
  | 'anatomy_detail';
export type ArtDirection = {
  medium_style: string;
  palette: string;
  materials: string;
  camera_composition: string;
  atmosphere: string;
  avoid: string;
};
export type VisualFact = { field: string; values: string[] };
export type VisualRecipeDefinition = {
  recipe_id: string;
  version: number;
  source_type: string;
  role: VisualRole;
  composition_intent: string;
  aspect_intent: 'portrait' | 'landscape' | 'square';
  transparent_background_meaningful: boolean;
  environment_present: boolean;
};
export type VisualGenerationBrief = {
  format_version: number;
  subject_classification: {
    subject_kind:
      | 'human'
      | 'humanoid'
      | 'creature'
      | 'beast'
      | 'object'
      | 'location'
      | 'faction_symbolic'
      | 'unknown';
    species_mode: 'explicit' | 'inferred-default';
    literalization_risk: 'low' | 'medium' | 'high';
    humanoid_guardrail: boolean;
  };
  prompt_semantics: {
    identity: string[];
    flavor_symbolism: string[];
    negative_guardrails: string[];
  };
  safeguards_applied: string[];
  negative_prompt_text: string;
  context: {
    format_version: number;
    visual_recipe_id: string;
    visual_recipe_version: number;
    role: VisualRole;
    facts: VisualFact[];
    art_direction: ArtDirection;
    context_hash: string;
    source: {
      entity_id: string;
      project_id: string;
      entity_type: string;
      write_version: number;
      canon_state: string;
      canon_revision: number | null;
      source_body_hash: string;
      generation: {
        generation_kind: string;
        generator_version: number;
        content_version: number;
        preset: string;
        creative_axes: { levels: Record<Axis, number> };
      } | null;
    };
  };
  subject_description: string;
  must_include: VisualFact[];
  supporting_details: VisualFact[];
  composition_intent: string;
  aspect_intent: string;
  background_intent: string;
  transparent_background_meaningful: boolean;
  avoid: string[];
  consistency_notes: string[];
  prompt_text: string;
  brief_hash: string;
};
export type ComfySettings = {
  checkpoint: string;
  seed: number;
  width: number;
  height: number;
  steps: number;
  cfg: number;
  sampler: string;
  scheduler: string;
};
export type ComfyDiscovery = {
  state: 'unavailable' | 'no_model' | 'ready';
  message: string;
  server_url: string;
  server_version: string | null;
  checkpoints: string[];
  samplers: string[];
  schedulers: string[];
  missing_nodes: string[];
};
export type ComfyResult = {
  provider: 'comfyui_local';
  server_version: string | null;
  brief_hash: string;
  prompt_id: string;
  settings: ComfySettings;
  output: { filename: string; subfolder: string; type: string };
  mime_type: string;
  byte_length: number;
  content_blake3: string;
  completed_unix_ms: number;
  warnings: string[];
};
export type ComfyStatus = {
  state: 'queued' | 'running' | 'completed' | 'failed';
  prompt_id: string;
  stale: boolean;
  message: string;
  result: ComfyResult | null;
};
export const api = {
  comfyResume: (entityId: string) =>
    invoke<ComfyStatus | null>('comfyui_resume', { entityId }),
  comfyCheck: (url: string) => invoke<ComfyDiscovery>('comfyui_check', { url }),
  comfyGenerate: (url: string, briefHash: string, settings: ComfySettings) =>
    invoke<ComfyStatus>('comfyui_generate', { url, briefHash, settings }),
  comfyStatus: (promptId: string) =>
    invoke<ComfyStatus>('comfyui_generation_status', { promptId }),
  comfyImage: (promptId: string) =>
    invoke<number[]>('comfyui_image', { promptId }),
  comfySave: (promptId: string, path: string) =>
    invoke<void>('save_visual_image', { promptId, path }),
  comfyDiscard: (promptId: string) =>
    invoke<void>('discard_visual_image', { promptId }),
  visualRecipes: (entityType: string) =>
    invoke<VisualRecipeDefinition[]>('get_visual_recipes', { entityType }),
  compileVisualBrief: (id: string, role: VisualRole, direction: ArtDirection) =>
    invoke<VisualGenerationBrief>('compile_visual_brief_command', {
      id,
      role,
      direction,
    }),
  visualBriefStale: (briefHash: string) =>
    invoke<boolean>('visual_brief_stale', { briefHash }),
  exportVisualBrief: (
    briefHash: string,
    path: string,
    format: 'json' | 'markdown',
  ) => invoke<void>('export_visual_brief_command', { briefHash, path, format }),
  project: () => invoke<Project | null>('get_project'),
  defaultProjectDirectory: () => invoke<string>('default_project_directory'),
  recentProjects: () => invoke<RecentProject[]>('recent_projects'),
  forgetRecentProject: (path: string) =>
    invoke<void>('forget_recent_project', { path }),
  createProject: (directory: string, name: string) =>
    invoke<Project>('create_project', { directory, name }),
  openProject: (path: string) => invoke<Project>('open_project', { path }),
  closeProject: () => invoke<void>('close_project'),
  exportProject: (path: string) =>
    invoke<void>('export_project_command', { path }),
  createBackup: (path: string) =>
    invoke<void>('create_backup_command', { path }),
  importProject: (archivePath: string, destinationDirectory: string) =>
    invoke<Project>('import_project_command', {
      archivePath,
      destinationDirectory,
    }),
  restoreBackup: (archivePath: string) =>
    invoke<void>('restore_backup_command', { archivePath }),
  exportEntity: (id: string, path: string, format: 'json' | 'markdown') =>
    invoke<void>('export_entity_command', { id, path, format }),
  forgeCharacter: (
    preset: string,
    seed: number,
    axes: Record<Axis, number>,
    constraints: Record<string, string>,
  ) =>
    invoke<ForgePayload>('forge_character', {
      preset,
      seed,
      axes,
      constraints,
    }),
  forgeQuest: (
    preset: string,
    seed: number,
    axes: Record<Axis, number>,
    constraints: Record<string, string>,
  ) => invoke<ForgePayload>('forge_quest', { preset, seed, axes, constraints }),
  forgeLocation: (preset: string, seed: number, axes: Record<Axis, number>) =>
    invoke<ForgePayload>('forge_location', { preset, seed, axes }),
  forgeFaction: (preset: string, seed: number, axes: Record<Axis, number>) =>
    invoke<ForgePayload>('forge_faction', { preset, seed, axes }),
  forgeItem: (preset: string, seed: number, axes: Record<Axis, number>) =>
    invoke<ForgePayload>('forge_item', { preset, seed, axes }),
  forgeCreature: (preset: string, seed: number, axes: Record<Axis, number>) =>
    invoke<ForgePayload>('forge_creature', { preset, seed, axes }),
  forgeCharacterSignatureItem: (sourceId: string, seed: number) =>
    invoke<SignatureItemResultBundle>('forge_character_signature_item', {
      sourceId,
      seed,
    }),
  saveSignatureItemBundle: () =>
    invoke<SignatureItemBundleSaveResponse>('save_signature_item_bundle'),
  forgeCharacterFaction: (sourceId: string, seed: number) =>
    invoke<FactionResultBundle>('forge_character_faction', { sourceId, seed }),
  saveFactionBundle: () =>
    invoke<FactionBundleSaveResponse>('save_faction_bundle'),
  forgeHometown: (sourceId: string, seed: number) =>
    invoke<ForgeResultBundle>('forge_hometown', { sourceId, seed }),
  saveForgeBundle: () => invoke<ForgeBundleSaveResponse>('save_forge_bundle'),
  discardForgeBundle: () => invoke<void>('discard_forge_bundle'),
  relationshipsForEntity: (id: string) =>
    invoke<ConnectionView[]>('list_relationships_for_entity', { id }),
  promoteRelationship: (id: string, expectedVersion: number) =>
    invoke<number>('promote_relationship', { id, expectedVersion }),
  archiveRelationship: (id: string, expectedVersion: number) =>
    invoke<number>('archive_relationship', { id, expectedVersion }),
  relationshipRevisions: (id: string) =>
    invoke<RelationshipRevision[]>('list_relationship_revisions', { id }),
  setLock: (field: string, locked: boolean) =>
    invoke<ForgePayload>('set_field_lock', { field, locked }),
  reroll: (field: string) => invoke<ForgePayload>('reroll_field', { field }),
  rerollUnlocked: () => invoke<ForgePayload>('reroll_unlocked'),
  editField: (field: string, value: FieldValue) =>
    invoke<ForgePayload>('edit_field', { field, value }),
  saveCandidate: () => invoke<EntityRecord>('save_candidate'),
  listEntities: () => invoke<EntityRecord[]>('list_entities'),
  getEntity: (id: string) => invoke<EntityRecord | null>('get_entity', { id }),
  favorite: (id: string, favorite: boolean, expectedVersion: number) =>
    invoke<EntityRecord>('toggle_favorite', { id, favorite, expectedVersion }),
  promote: (id: string, expectedVersion: number) =>
    invoke<EntityRecord>('promote_to_canon', { id, expectedVersion }),
  history: (id?: string) =>
    invoke<HistoryRecord[]>('list_history', { id: id ?? null }),
  snapshot: (runId: string) =>
    invoke<ForgePayload | null>('get_history_snapshot', { runId }),
  revisions: (id: string) => invoke<Revision[]>('get_canon_revisions', { id }),
  restoreHistoryToWorkingCopy: (runId: string) =>
    invoke<WorkingState>('use_history_as_working_copy', { runId }),
  loadCanonRevisionWorkingCopy: (id: string, revisionNumber: number) =>
    invoke<WorkingState>('use_canon_revision_as_working_copy', {
      id,
      revisionNumber,
    }),
  undo: () => invoke<WorkingState>('undo_session'),
  redo: () => invoke<WorkingState>('redo_session'),
  reviseCanon: (reason: string) =>
    invoke<EntityRecord>('revise_canon', { reason }),
  userSettings: () => invoke<UserSettings>('get_user_settings'),
  updateUserSettings: (settings: UserSettings) =>
    invoke<UserSettings>('update_user_settings', { settings }),
};

export function errorMessage(error: UiError): {
  message: string;
  stale: boolean;
} {
  if (typeof error === 'string') return { message: error, stale: false };
  if (error.kind === 'stale_write')
    return {
      message:
        'This item changed since you opened it. Reload the latest version before saving again.',
      stale: true,
    };
  const messages: Record<string, string> = {
    unsupported_schema:
      'This project was created by a newer app version and cannot be opened here.',
    corrupt_project:
      'The project data could not be read safely. Your saved data has not been overwritten.',
    validation:
      error.message ??
      'Some of the chosen values are not valid. Review the form and try again.',
    storage: error.message ?? 'The project could not be saved or opened.',
    no_project: 'Open or create a project first.',
    project_exists:
      'A project with that name already exists in this folder. Choose another name or location.',
    not_found:
      'Character Forge couldn’t find that project file. Check its location or forget the stale recent entry.',
    invalid_project:
      'That file is not a Character Forge project. Choose an existing project file, or use Import Project for a portable project.',
    invalid_archive:
      'This is not a valid Forge project. The existing project was left unchanged.',
    unsupported_format:
      'This project was created by a newer unsupported format.',
    integrity: 'The project archive failed its integrity check.',
    unsafe_archive: 'This project file contains an unsafe path.',
  };
  return {
    message:
      messages[error.kind ?? ''] ??
      error.message ??
      'Something went wrong. Please try again.',
    stale: false,
  };
}
