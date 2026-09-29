import { invoke } from '@tauri-apps/api/core';

export type FieldValue = string | string[];
export type ForgePayload =
  | { kind: 'character_candidate'; value: CharacterCandidate }
  | { kind: 'quest_candidate'; value: QuestCandidate }
  | { kind: 'location_candidate'; value: LocationCandidate }
  | { kind: 'character'; value: Character }
  | { kind: 'quest'; value: Quest }
  | { kind: 'location'; value: Location };
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
  entity_type: 'character' | 'quest' | 'location';
  name: string;
  canon_state: 'draft' | 'candidate' | 'canon' | 'archived';
  favorite: boolean;
  created_at_unix_ms: number;
  updated_at_unix_ms: number;
  tags: string[];
  generation_provenance: { dna: ForgeDNA; action: string } | null;
};
export type ForgeDNA = {
  recipe: {
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

export const api = {
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
