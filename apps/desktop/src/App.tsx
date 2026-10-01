import { useCallback, useEffect, useRef, useState } from 'react';
import {
  type Axis,
  api,
  axisMeta,
  type Character,
  type Creature,
  type CreatureProfile,
  type EntityRecord,
  errorMessage,
  type Faction,
  type FactionProfile,
  type FactionResultBundle,
  type FieldValue,
  type ForgePayload,
  type ForgeResultBundle,
  type HistoryRecord,
  type Item,
  type ItemProfile,
  type Project,
  presets,
  type Quest,
  type RecentProject,
  type Revision,
  type UiError,
  type UserSettings,
  type WorkingState,
} from './api';
import { Connections } from './Connections';

type FurtherReview =
  | { kind: 'hometown'; bundle: ForgeResultBundle }
  | { kind: 'faction'; bundle: FactionResultBundle };

type Section =
  | 'forge'
  | 'characters'
  | 'quests'
  | 'locations'
  | 'factions'
  | 'items'
  | 'creatures'
  | 'canon'
  | 'history'
  | 'project';
type Mode =
  | 'character'
  | 'quest'
  | 'location'
  | 'faction'
  | 'item'
  | 'creature';
type ProjectStartMode = 'create' | 'open' | 'import';
type Field = {
  key: string;
  label: string;
  value: FieldValue;
  list?: boolean;
  options?: { id: string; label: string }[];
};
const blankAxes = (): Record<Axis, number> =>
  Object.fromEntries(axisMeta.map(({ id }) => [id, 50])) as Record<
    Axis,
    number
  >;
const labels = Object.fromEntries(presets) as Record<string, string>;
function required<T>(value: T | null | undefined, label: string): T {
  if (value == null)
    throw new Error(`Missing ${label} in the saved project data.`);
  return value;
}

function payloadEntity(payload: ForgePayload) {
  const raw = payload.value as unknown as Record<string, unknown>;
  if (payload.kind === 'character_candidate') {
    const wrapper = raw as unknown as import('./api').CharacterCandidate;
    return {
      envelope: wrapper.character.envelope,
      character: wrapper.character,
      dna: wrapper.dna,
      field_origins: wrapper.field_origins,
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'quest_candidate') {
    const wrapper = raw as unknown as import('./api').QuestCandidate;
    return {
      envelope: wrapper.quest.envelope,
      quest: wrapper.quest,
      dna: wrapper.dna,
      field_origins: wrapper.field_origins,
      clue: wrapper.clue,
      briefing: wrapper.briefing,
    };
  }
  if (payload.kind === 'location_candidate') {
    const wrapper = raw as unknown as import('./api').LocationCandidate;
    return {
      envelope: wrapper.location.envelope,
      location: wrapper.location,
      dna: wrapper.dna,
      field_origins: wrapper.field_origins,
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'faction_candidate') {
    const wrapper = raw as unknown as import('./api').FactionCandidate;
    return {
      envelope: wrapper.faction.envelope,
      faction: wrapper.faction,
      dna: wrapper.dna,
      field_origins: wrapper.field_origins,
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'faction') {
    const faction = raw as unknown as Faction;
    return {
      envelope: faction.envelope,
      faction,
      dna: faction.envelope.generation_provenance?.dna,
      field_origins: {},
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'item_candidate') {
    const wrapper = raw as unknown as import('./api').ItemCandidate;
    return {
      envelope: wrapper.item.envelope,
      item: wrapper.item,
      dna: wrapper.dna,
      field_origins: wrapper.field_origins,
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'creature_candidate') {
    const wrapper = raw as unknown as import('./api').CreatureCandidate;
    return {
      envelope: wrapper.creature.envelope,
      creature: wrapper.creature,
      dna: wrapper.dna,
      field_origins: wrapper.field_origins,
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'item') {
    const item = raw as unknown as Item;
    return {
      envelope: item.envelope,
      item,
      dna: item.envelope.generation_provenance?.dna,
      field_origins: {},
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'creature') {
    const creature = raw as unknown as Creature;
    return {
      envelope: creature.envelope,
      creature,
      dna: creature.envelope.generation_provenance?.dna,
      field_origins: {},
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'location') {
    const location = raw as unknown as import('./api').Location;
    return {
      envelope: location.envelope,
      location,
      dna: location.envelope.generation_provenance?.dna,
      field_origins: {},
      clue: undefined,
      briefing: undefined,
    };
  }
  if (payload.kind === 'character') {
    const character = raw as unknown as Character;
    return {
      envelope: character.envelope,
      character,
      dna: character.envelope.generation_provenance?.dna,
      field_origins: {},
      clue: undefined,
      briefing: undefined,
    };
  }
  const quest = raw as unknown as Quest;
  return {
    envelope: quest.envelope,
    quest,
    dna: quest.envelope.generation_provenance?.dna,
    field_origins: {},
    clue: undefined,
    briefing: undefined,
  };
}
function isCharacter(payload: ForgePayload) {
  return payload.kind === 'character_candidate' || payload.kind === 'character';
}
function isLocation(payload: ForgePayload) {
  return payload.kind === 'location_candidate' || payload.kind === 'location';
}
function isQuest(payload: ForgePayload) {
  return payload.kind === 'quest_candidate' || payload.kind === 'quest';
}
function isFaction(payload: ForgePayload) {
  return payload.kind === 'faction_candidate' || payload.kind === 'faction';
}
function isItem(payload: ForgePayload) {
  return payload.kind === 'item_candidate' || payload.kind === 'item';
}
function isCreature(payload: ForgePayload) {
  return payload.kind === 'creature_candidate' || payload.kind === 'creature';
}
function payloadTitle(payload: ForgePayload) {
  const value = payloadEntity(payload);
  if (isCharacter(payload))
    return required(value.character, 'Character payload').title;
  if (isLocation(payload))
    return required(value.location, 'Location payload').envelope.name;
  if (isFaction(payload))
    return required(value.faction, 'Faction payload').envelope.name;
  if (isItem(payload))
    return required(value.item, 'Item payload').envelope.name;
  if (isCreature(payload))
    return required(value.creature, 'Creature payload').envelope.name;
  return required(value.quest, 'Quest payload').envelope.name;
}
function payloadFields(payload: ForgePayload): Field[] {
  if (isCharacter(payload)) {
    const c = required(payloadEntity(payload).character, 'Character payload');
    return [
      { key: 'name', label: 'Name', value: c.envelope.name },
      { key: 'title', label: 'Title', value: c.title },
      { key: 'archetype', label: 'Archetype', value: c.archetype },
      { key: 'role', label: 'Role', value: c.role },
      { key: 'appearance', label: 'Appearance', value: c.appearance },
      {
        key: 'personality',
        label: 'Personality',
        value: c.personality,
        list: true,
      },
      { key: 'desire', label: 'Desire', value: c.desire },
      { key: 'fear', label: 'Fear', value: c.fear },
      { key: 'flaw', label: 'Flaw', value: c.flaw },
      { key: 'secret', label: 'Secret', value: c.secret },
      { key: 'skills', label: 'Skills', value: c.skills, list: true },
      { key: 'abilities', label: 'Abilities', value: c.abilities, list: true },
      { key: 'faction_hint', label: 'Faction', value: c.faction_hint },
      {
        key: 'signature_gear',
        label: 'Signature gear',
        value: c.signature_gear,
      },
      {
        key: 'relationship_hook',
        label: 'Relationship hook',
        value: c.relationship_hook,
      },
      {
        key: 'backstory_hook',
        label: 'Backstory hook',
        value: c.backstory_hook,
      },
      {
        key: 'story_hooks',
        label: 'Story hooks',
        value: c.story_hooks,
        list: true,
      },
    ];
  }
  const value = payloadEntity(payload);
  if (isLocation(payload)) {
    const l = required(value.location, 'Location payload');
    return [
      {
        key: 'profile',
        label: 'Location profile',
        value: l.location_profile,
      },
      { key: 'name', label: 'Name', value: l.envelope.name },
      {
        key: 'visual_identity',
        label: 'Visual identity',
        value: l.visual_identity,
      },
      { key: 'atmosphere', label: 'Atmosphere', value: l.atmosphere },
      { key: 'purpose', label: 'Purpose', value: l.purpose },
      { key: 'summary', label: 'Summary', value: l.summary },
      { key: 'history', label: 'History', value: l.history },
      {
        key: 'inhabitants_summary',
        label: 'Inhabitants',
        value: l.inhabitants_summary,
      },
      {
        key: 'notable_features',
        label: 'Notable features',
        value: l.notable_features,
        list: true,
      },
      { key: 'danger', label: 'Danger', value: l.danger },
      { key: 'secret', label: 'Secret', value: l.secret },
      {
        key: 'story_hooks',
        label: 'Story hooks',
        value: l.story_hooks,
        list: true,
      },
    ];
  }
  if (isFaction(payload)) {
    const f = required(value.faction, 'Faction payload');
    const profiles: { id: FactionProfile; label: string }[] = [
      { id: 'guild', label: 'Guild' },
      { id: 'order', label: 'Order' },
      { id: 'cult', label: 'Cult' },
      { id: 'government', label: 'Government' },
      { id: 'kingdom', label: 'Kingdom' },
      { id: 'military', label: 'Military' },
      { id: 'corporation', label: 'Corporation' },
      { id: 'gang', label: 'Gang' },
      { id: 'rebellion', label: 'Rebellion' },
      { id: 'religious-order', label: 'Religious Order' },
      { id: 'tribe', label: 'Tribe' },
      { id: 'secret-society', label: 'Secret Society' },
      { id: 'other', label: 'Other' },
    ];
    return [
      { key: 'name', label: 'Name', value: f.envelope.name },
      {
        key: 'faction_profile',
        label: 'Faction Profile',
        value: f.faction_profile,
        options: profiles,
      },
      { key: 'identity', label: 'Identity', value: f.identity },
      { key: 'ideology', label: 'Ideology', value: f.ideology },
      { key: 'primary_goal', label: 'Primary Goal', value: f.primary_goal },
      { key: 'methods', label: 'Methods', value: f.methods },
      { key: 'culture', label: 'Culture', value: f.culture },
      { key: 'resources', label: 'Resources', value: f.resources },
      { key: 'public_face', label: 'Public Face', value: f.public_face },
      { key: 'hidden_truth', label: 'Hidden Truth', value: f.hidden_truth },
      { key: 'symbols', label: 'Symbols', value: f.symbols },
      { key: 'reputation', label: 'Reputation', value: f.reputation },
      {
        key: 'story_hooks',
        label: 'Story Hooks',
        value: f.story_hooks,
        list: true,
      },
    ];
  }
  if (isItem(payload)) {
    const f = required(value.item, 'Item payload');
    const profiles: { id: ItemProfile; label: string }[] = [
      { id: 'weapon', label: 'Weapon' },
      { id: 'armor', label: 'Armor' },
      { id: 'artifact', label: 'Artifact' },
      { id: 'consumable', label: 'Consumable' },
      { id: 'tool', label: 'Tool' },
      { id: 'key-item', label: 'Key Item' },
      { id: 'resource', label: 'Resource' },
      { id: 'treasure', label: 'Treasure' },
      { id: 'book', label: 'Book' },
      { id: 'quest-item', label: 'Quest Item' },
      { id: 'other', label: 'Other' },
    ];
    return [
      { key: 'name', label: 'Name', value: f.envelope.name },
      {
        key: 'item_profile',
        label: 'Item Profile',
        value: f.item_profile,
        options: profiles,
      },
      { key: 'appearance', label: 'Appearance', value: f.appearance },
      { key: 'purpose', label: 'Purpose', value: f.purpose },
      { key: 'properties', label: 'Properties', value: f.properties },
      { key: 'origin', label: 'Origin', value: f.origin },
      { key: 'history', label: 'History', value: f.history },
      {
        key: 'power_or_function',
        label: 'Power or Function',
        value: f.power_or_function,
      },
      {
        key: 'cost_or_drawback',
        label: 'Cost or Drawback',
        value: f.cost_or_drawback,
      },
      { key: 'rarity', label: 'Rarity', value: f.rarity },
      { key: 'condition', label: 'Condition', value: f.condition },
      { key: 'secret', label: 'Secret', value: f.secret },
      {
        key: 'story_hooks',
        label: 'Story Hooks',
        value: f.story_hooks,
        list: true,
      },
    ];
  }
  if (isCreature(payload)) {
    const f = required(value.creature, 'Creature payload');
    const profiles: { id: CreatureProfile; label: string }[] = [
      { id: 'beast', label: 'Beast' },
      { id: 'monster', label: 'Monster' },
      { id: 'undead', label: 'Undead' },
      { id: 'construct', label: 'Construct' },
      { id: 'aberration', label: 'Aberration' },
      { id: 'spirit', label: 'Spirit' },
      { id: 'elemental', label: 'Elemental' },
      { id: 'humanoid-creature', label: 'Humanoid Creature' },
      { id: 'swarm', label: 'Swarm' },
      { id: 'boss', label: 'Boss' },
      { id: 'other', label: 'Other' },
    ];
    return [
      { key: 'name', label: 'Name', value: f.envelope.name },
      {
        key: 'creature_profile',
        label: 'Creature Profile',
        value: f.creature_profile,
        options: profiles,
      },
      { key: 'appearance', label: 'Appearance', value: f.appearance },
      { key: 'habitat', label: 'Habitat', value: f.habitat },
      { key: 'behavior', label: 'Behavior', value: f.behavior },
      { key: 'ecology', label: 'Ecology', value: f.ecology },
      { key: 'threat', label: 'Threat', value: f.threat },
      {
        key: 'abilities_summary',
        label: 'Abilities Summary',
        value: f.abilities_summary,
      },
      {
        key: 'weakness',
        label: 'Weakness',
        value: f.weakness,
      },
      { key: 'origin', label: 'Origin', value: f.origin },
      {
        key: 'signs_of_presence',
        label: 'Signs of Presence',
        value: f.signs_of_presence,
      },
      {
        key: 'loot_or_resource',
        label: 'Loot or Resource',
        value: f.loot_or_resource,
      },
      { key: 'secret', label: 'Secret', value: f.secret },
      {
        key: 'story_hooks',
        label: 'Story Hooks',
        value: f.story_hooks,
        list: true,
      },
    ];
  }
  const q = required(value.quest, 'Quest payload');
  const fields: Field[] = [
    { key: 'title', label: 'Title', value: q.envelope.name },
    {
      key: 'briefing',
      label: 'Briefing',
      value: value.briefing ?? '',
    },
    { key: 'premise', label: 'Premise', value: q.premise },
    { key: 'locations', label: 'Location', value: q.locations, list: true },
    { key: 'patron', label: 'Quest giver', value: q.patron },
    { key: 'objective', label: 'Objective', value: q.objective },
    { key: 'stakes', label: 'Stakes', value: q.stakes },
    { key: 'opposition', label: 'Threat', value: q.opposition },
    {
      key: 'clue',
      label: 'Clue / condition',
      value: value.clue ?? '',
    },
    {
      key: 'complications',
      label: 'Complications',
      value: q.complications,
      list: true,
    },
    { key: 'twist', label: 'Twist', value: q.twist },
    { key: 'rewards', label: 'Reward', value: q.rewards, list: true },
    {
      key: 'failure',
      label: 'Failure consequence',
      value: q.failure_consequences,
      list: true,
    },
    { key: 'moral_pressure', label: 'Moral pressure', value: q.moral_pressure },
    {
      key: 'follow_up',
      label: 'Follow-up hooks',
      value: q.follow_up_hooks,
      list: true,
    },
  ];
  return fields;
}
function fieldsOf(
  payload: ForgePayload,
): Record<string, { nonce: number; locked: boolean }> {
  return payloadEntity(payload).dna?.recipe.field_controls ?? {};
}
function fieldControl(payload: ForgePayload, field: string) {
  const suffix = `-${field.replaceAll('_', '-')}`;
  const key = Object.keys(fieldsOf(payload)).find(
    (value) => value === suffix || value.endsWith(suffix),
  );
  return key ? fieldsOf(payload)[key] : undefined;
}
function originOf(payload: ForgePayload, field: string) {
  const key = field.replaceAll('_', '-');
  const control = Object.keys(fieldsOf(payload)).find((value) =>
    value.endsWith(`-${key}`),
  );
  return payload.kind.endsWith('candidate') && control
    ? (payloadEntity(payload).field_origins as Record<string, string>)[control]
    : undefined;
}
function displayField(value: FieldValue) {
  return Array.isArray(value) ? value.join('\n') : value;
}
function dateLabel(value: number) {
  return value ? new Date(value).toLocaleString() : 'Just now';
}

export function App() {
  const [project, setProject] = useState<Project | null>(null);
  const [section, setSection] = useState<Section>('forge');
  const [mode, setMode] = useState<Mode>('character');
  const [preset, setPreset] = useState('dark-fantasy');
  const [seed, setSeed] = useState('42');
  const [axes, setAxes] = useState<Record<Axis, number>>(blankAxes);
  const [constraints, setConstraints] = useState<Record<string, string>>({});
  const [candidate, setCandidate] = useState<ForgePayload | null>(null);
  const [dirty, setDirty] = useState(false);
  const [records, setRecords] = useState<EntityRecord[]>([]);
  const [history, setHistory] = useState<HistoryRecord[]>([]);
  const [revisions, setRevisions] = useState<Revision[]>([]);
  const [selectedRecord, setSelectedRecord] = useState<EntityRecord | null>(
    null,
  );
  const [furtherReview, setFurtherReview] = useState<FurtherReview | null>(
    null,
  );
  const [furtherPending, setFurtherPending] = useState(false);
  const [furtherSeed, setFurtherSeed] = useState('42');
  const [furtherSourceId, setFurtherSourceId] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const [projectName, setProjectName] = useState('My World');
  const [projectDirectory, setProjectDirectory] = useState('');
  const [openProjectPath, setOpenProjectPath] = useState('');
  const [projectStartMode, setProjectStartMode] =
    useState<ProjectStartMode>('create');
  const [recentProjects, setRecentProjects] = useState<RecentProject[]>([]);
  const [importArchivePath, setImportArchivePath] = useState('');
  const [importDirectory, setImportDirectory] = useState('');
  const [exportPath, setExportPath] = useState('');
  const [backupPath, setBackupPath] = useState('');
  const [restorePath, setRestorePath] = useState('');
  const [editing, setEditing] = useState<string | null>(null);
  const [editValue, setEditValue] = useState('');
  const [snapshot, setSnapshot] = useState<ForgePayload | null>(null);
  const [canUndo, setCanUndo] = useState(false);
  const [canRedo, setCanRedo] = useState(false);
  const [settings, setSettings] = useState<UserSettings>({
    version: 1,
    reducedMotion: false,
  });
  const [selectedRevision, setSelectedRevision] = useState<Revision | null>(
    null,
  );
  const [selectedHistory, setSelectedHistory] = useState<HistoryRecord | null>(
    null,
  );
  const [revisionBase, setRevisionBase] = useState<EntityRecord | null>(null);
  const historySnapshotRequest = useRef(0);
  const [revisionTarget, setRevisionTarget] = useState(false);
  const navigateToSection = (next: Section) => {
    historySnapshotRequest.current += 1;
    setSnapshot(null);
    setSelectedHistory(null);
    setSection(next);
  };

  const clearWorkingView = useCallback(() => {
    historySnapshotRequest.current += 1;
    setCandidate(null);
    setDirty(false);
    setCanUndo(false);
    setCanRedo(false);
    setRevisionTarget(false);
    setSelectedRevision(null);
    setSelectedHistory(null);
    setSnapshot(null);
    setRevisionBase(null);
    setFurtherReview(null);
    setFurtherPending(false);
    setFurtherSourceId(null);
  }, []);

  const setProjectLocationDefaults = useCallback((active: Project) => {
    const parts = active.path.split(/[\\/]/);
    parts.pop();
    const separator = active.path.includes('\\') ? '\\' : '/';
    const directory = parts.join(separator) || separator;
    const basename =
      active.path
        .split(/[\\/]/)
        .pop()
        ?.replace(/\.sqlite3$/i, '') || active.name;
    const portableDefault = `${directory}${separator}${basename}.forgeproject`;
    const backupDefault = `${directory}${separator}${basename}.backup.forgeproject`;
    setProjectDirectory(directory);
    setImportDirectory(directory);
    setExportPath(portableDefault);
    setBackupPath(backupDefault);
    setRestorePath(backupDefault);
  }, []);

  const refreshProject = useCallback(async () => {
    setLoading(true);
    try {
      const active = await api.project();
      setProject(active);
      if (active) {
        setProjectName(active.name);
        setProjectLocationDefaults(active);
      }
    } catch (reason) {
      setError(errorMessage(reason as UiError).message);
    } finally {
      setLoading(false);
    }
  }, [setProjectLocationDefaults]);
  useEffect(() => {
    void refreshProject();
  }, [refreshProject]);
  useEffect(() => {
    void api
      .userSettings()
      .then(setSettings)
      .catch((reason: unknown) =>
        setError(errorMessage(reason as UiError).message),
      );
  }, []);
  useEffect(() => {
    document.documentElement.dataset.reducedMotion = String(
      settings.reducedMotion,
    );
  }, [settings.reducedMotion]);

  const refreshRecentProjects = useCallback(async () => {
    try {
      setRecentProjects(await api.recentProjects());
    } catch (reason) {
      setError(errorMessage(reason as UiError).message);
    }
  }, []);
  useEffect(() => {
    void api
      .defaultProjectDirectory()
      .then((directory) => {
        setProjectDirectory((current) => current || directory);
        setImportDirectory((current) => current || directory);
      })
      .catch((reason: unknown) =>
        setError(errorMessage(reason as UiError).message),
      );
    void refreshRecentProjects();
  }, [refreshRecentProjects]);

  const refreshRecords = useCallback(async () => {
    if (!project) return;
    setLoading(true);
    try {
      const next = await api.listEntities();
      setRecords(next);
    } catch (reason) {
      setError(errorMessage(reason as UiError).message);
    } finally {
      setLoading(false);
    }
  }, [project]);
  useEffect(() => {
    if (project) void refreshRecords();
  }, [project, refreshRecords]);
  useEffect(() => {
    if (
      project &&
      [
        'characters',
        'quests',
        'locations',
        'factions',
        'items',
        'creatures',
        'canon',
      ].includes(section)
    )
      void refreshRecords();
  }, [project, section, refreshRecords]);

  const run = useCallback(
    async <T,>(operation: () => Promise<T>, onSuccess: (value: T) => void) => {
      setBusy(true);
      setError('');
      setNotice('');
      try {
        onSuccess(await operation());
      } catch (reason) {
        setError(errorMessage(reason as UiError).message);
      } finally {
        setBusy(false);
      }
    },
    [],
  );
  const createProject = () =>
    void run(
      () => api.createProject(projectDirectory, projectName),
      (value) => {
        setProject(value);
        setProjectLocationDefaults(value);
        navigateToSection('forge');
        clearWorkingView();
        setNotice('Project created and ready to forge.');
        void refreshRecentProjects();
      },
    );
  const openProject = () =>
    void run(
      () => api.openProject(openProjectPath),
      (value) => {
        setProject(value);
        setProjectLocationDefaults(value);
        navigateToSection('forge');
        clearWorkingView();
        setSelectedRecord(null);
        setNotice('Project opened.');
        void refreshRecentProjects();
      },
    );
  const exportProject = () =>
    void run(
      () => api.exportProject(exportPath),
      () => setNotice('Project exported successfully.'),
    );
  const createBackup = () =>
    void run(
      () => api.createBackup(backupPath),
      () => setNotice('Backup created successfully.'),
    );
  const importProject = () =>
    void run(
      () => api.importProject(importArchivePath, importDirectory),
      (value) => {
        setProject(value);
        setProjectLocationDefaults(value);
        navigateToSection('forge');
        clearWorkingView();
        setSelectedRecord(null);
        setNotice('Project imported and opened.');
        void refreshRecentProjects();
      },
    );
  const openRecentProject = (recent: RecentProject) => {
    if (!recent.exists) {
      setProjectStartMode('open');
      setOpenProjectPath(recent.path);
      setError(
        `“${recent.name}” couldn’t be found at its last known location. Locate the project file or forget this entry.`,
      );
      return;
    }
    void run(
      () => api.openProject(recent.path),
      (value) => {
        setProject(value);
        setProjectLocationDefaults(value);
        navigateToSection('forge');
        clearWorkingView();
        setSelectedRecord(null);
        setNotice('Project opened.');
        void refreshRecentProjects();
      },
    );
  };
  const forgetRecentProject = (recent: RecentProject) =>
    void run(
      () => api.forgetRecentProject(recent.path),
      () => {
        setRecentProjects((items) =>
          items.filter((item) => item.path !== recent.path),
        );
        setNotice(
          'Removed from Recent Projects. The project file was not deleted.',
        );
      },
    );
  const restoreBackup = () => {
    if (
      !window.confirm(
        'Restore this backup over the open project? A recoverable copy will be kept while restore is checked.',
      )
    )
      return;
    void run(
      () => api.restoreBackup(restorePath),
      () => {
        clearWorkingView();
        setSelectedRecord(null);
        setNotice('Backup restored successfully.');
        void refreshRecords();
      },
    );
  };
  const exportEntity = (record: EntityRecord, format: 'json' | 'markdown') => {
    const extension = format === 'json' ? 'json' : 'md';
    const path = window.prompt(
      `Save ${format.toUpperCase()} export as`,
      `${payloadTitle(record.entity)}.${extension}`,
    );
    if (!path?.trim()) return;
    void run(
      () => api.exportEntity(entityId(record.entity), path.trim(), format),
      () =>
        setNotice(`${format === 'json' ? 'JSON' : 'Markdown'} export saved.`),
    );
  };
  const forge = () => {
    const numericSeed = Number(seed);
    if (!Number.isSafeInteger(numericSeed) || numericSeed < 0) {
      setError('Enter a whole-number seed from 0 to 9,007,199,254,740,991.');
      return;
    }
    const activeConstraints: Record<string, string> = {};
    if (mode === 'character') {
      for (const key of [
        'must_be_non_human',
        'no_magic',
        'no_firearms',
        'urban_only',
      ])
        if (constraints[key]) activeConstraints[key] = 'true';
      if (constraints.inspiration?.trim())
        activeConstraints.inspiration = constraints.inspiration.trim();
    } else if (mode === 'quest' && constraints.inspiration?.trim())
      activeConstraints.inspiration = constraints.inspiration.trim();
    void run(
      () =>
        mode === 'character'
          ? api.forgeCharacter(preset, numericSeed, axes, activeConstraints)
          : mode === 'quest'
            ? api.forgeQuest(preset, numericSeed, axes, activeConstraints)
            : mode === 'location'
              ? api.forgeLocation(preset, numericSeed, axes)
              : mode === 'faction'
                ? api.forgeFaction(preset, numericSeed, axes)
                : mode === 'item'
                  ? api.forgeItem(preset, numericSeed, axes)
                  : api.forgeCreature(preset, numericSeed, axes),
      (value) => {
        setFurtherReview(null);
        setFurtherPending(false);
        setFurtherSourceId(null);
        setCandidate(value);
        setCanUndo(false);
        setCanRedo(false);
        setDirty(true);
        setSelectedRecord(null);
        setNotice(`${modeLabel(mode)} forged from seed ${numericSeed}.`);
      },
    );
  };
  const updateCandidate = (
    operation: () => Promise<ForgePayload>,
    success?: string,
  ) =>
    void run(operation, (value) => {
      setCandidate(value);
      setCanUndo(true);
      setCanRedo(false);
      setDirty(true);
      if (success) setNotice(success);
    });
  const current = candidate ?? selectedRecord?.entity ?? null;
  const candidateSaved = candidate
    ? records.find((record) => entityId(record.entity) === entityId(candidate))
    : null;
  const canShape =
    current?.kind.endsWith('candidate') &&
    payloadEntity(current).envelope.canon_state !== 'canon';
  const allFields = current ? payloadFields(current) : [];
  const currentDna = current ? payloadEntity(current).dna : undefined;
  const label = current
    ? currentDna
      ? (labels[currentDna.recipe.universe_preset] ??
        currentDna.recipe.universe_preset)
      : 'Saved Character has no generation recipe'
    : labels[preset];
  const canonRecords = records.filter(
    (record) => payloadEntity(record.entity).envelope.canon_state === 'canon',
  );

  const save = async () => {
    if (furtherPending && furtherReview) {
      await run(
        async () =>
          furtherReview.kind === 'faction'
            ? (await api.saveFactionBundle()).faction
            : (await api.saveForgeBundle()).location,
        (saved) => {
          setRecords((old) =>
            old.some((row) => entityId(row.entity) === entityId(saved.entity))
              ? old.map((row) =>
                  entityId(row.entity) === entityId(saved.entity) ? saved : row,
                )
              : [saved, ...old],
          );
          setSelectedRecord(saved);
          setCandidate(saved.entity);
          setDirty(false);
          setFurtherPending(false);
          void api.listEntities().then(setRecords);
          setNotice(
            furtherReview.kind === 'faction'
              ? 'Faction and member_of Candidate relationship saved together.'
              : 'Hometown Location and born_in Candidate relationship saved together.',
          );
        },
      );
      return;
    }
    if (revisionTarget) {
      if (!candidate) return;
      const savedCanon = revisionBase?.entity;
      const oldFields = new Map(
        (savedCanon ? payloadFields(savedCanon) : []).map((field) => [
          field.key,
          field,
        ]),
      );
      const changes = payloadFields(candidate).flatMap((field) => {
        const oldField = oldFields.get(field.key);
        if (
          !oldField ||
          JSON.stringify(oldField.value) === JSON.stringify(field.value)
        )
          return [];
        const concise = (value: FieldValue) => {
          const normalized = displayField(value).replaceAll('\n', ' ');
          return normalized.length > 120
            ? `${normalized.slice(0, 117)}…`
            : normalized;
        };
        const oldValue = concise(oldField.value);
        const newValue = concise(field.value);
        return [`${field.label}: “${oldValue}” → “${newValue}”`];
      });
      const visibleChanges = changes.slice(0, 8);
      if (changes.length > visibleChanges.length)
        visibleChanges.push(
          `…and ${changes.length - visibleChanges.length} more changed fields.`,
        );
      const confirmation = [
        'Make this working copy the new current Canon?',
        'The existing current Canon will be preserved in revision history.',
        '',
        ...(visibleChanges.length > 0
          ? ['Changes from current Canon:', ...visibleChanges]
          : ['No structured field differences from current Canon.']),
      ].join('\n');
      if (!window.confirm(confirmation)) return;
      const reason = window.prompt('Why are you revising this Canon item?');
      if (!reason?.trim()) return;
      await run(
        () => api.reviseCanon(reason.trim()),
        (saved) => {
          setRecords((old) =>
            old.map((row) =>
              entityId(row.entity) === entityId(saved.entity) ? saved : row,
            ),
          );
          setSelectedRecord(saved);
          setCandidate(saved.entity);
          setDirty(false);
          setRevisionTarget(false);
          setRevisionBase(null);
          setNotice(
            'Canon revision saved. The prior revision remains available.',
          );
          void api.revisions(entityId(saved.entity)).then(setRevisions);
        },
      );
      return;
    }
    await run(
      () => api.saveCandidate(),
      (saved) => {
        setRecords((old) =>
          old.some((row) => entityId(row.entity) === entityId(saved.entity))
            ? old.map((row) =>
                entityId(row.entity) === entityId(saved.entity) ? saved : row,
              )
            : [saved, ...old],
        );
        setSelectedRecord(saved);
        setDirty(false);
        setNotice('Saved to this project.');
      },
    );
  };
  const selectRecord = async (record: EntityRecord) => {
    setRevisionTarget(false);
    setRevisionBase(null);
    await run(
      () => api.getEntity(entityId(record.entity)),
      (fresh) => {
        if (fresh) {
          setFurtherReview(null);
          setFurtherPending(false);
          setFurtherSourceId(null);
          setSelectedRecord(fresh);
          setCandidate(fresh.entity);
          setDirty(false);
          setCanUndo(false);
          setCanRedo(false);
          setRevisionTarget(false);
          setSelectedRevision(null);
          setRevisions([]);
          void api
            .revisions(entityId(fresh.entity))
            .then(setRevisions)
            .catch(() => undefined);
        }
      },
    );
  };
  const promote = (record: EntityRecord) => {
    const name = payloadTitle(record.entity);
    if (
      !window.confirm(
        `Promote “${name}” to Canon? This marks this saved version as established project material. Its history is preserved.`,
      )
    )
      return;
    void run(
      () => api.promote(entityId(record.entity), record.writeVersion),
      (updated) => {
        setRecords((old) =>
          old.map((row) =>
            entityId(row.entity) === entityId(updated.entity) ? updated : row,
          ),
        );
        setSelectedRecord(updated);
        setNotice(
          'Promoted to Canon. The previous state is preserved as a revision.',
        );
        void api.revisions(entityId(updated.entity)).then(setRevisions);
      },
    );
  };
  const favorite = (record: EntityRecord) =>
    void run(
      () =>
        api.favorite(
          entityId(record.entity),
          !payloadEntity(record.entity).envelope.favorite,
          record.writeVersion,
        ),
      (updated) =>
        setRecords((old) =>
          old.map((row) =>
            entityId(row.entity) === entityId(updated.entity) ? updated : row,
          ),
        ),
    );
  const forgeFurtherFrom = (
    record: EntityRecord,
    kind: FurtherReview['kind'],
  ) => {
    const actionSeed = Number(furtherSeed);
    if (!Number.isSafeInteger(actionSeed) || actionSeed < 0) {
      setError('Enter a non-negative whole number for the action seed.');
      return;
    }
    const sourceId = entityId(record.entity);
    void run(
      async (): Promise<FurtherReview> =>
        kind === 'faction'
          ? {
              kind,
              bundle: await api.forgeCharacterFaction(sourceId, actionSeed),
            }
          : { kind, bundle: await api.forgeHometown(sourceId, actionSeed) },
      (review) => {
        setFurtherSourceId(sourceId);
        setFurtherReview(review);
        setFurtherPending(true);
        setCandidate(
          review.kind === 'faction'
            ? { kind: 'faction_candidate', value: review.bundle.primary }
            : { kind: 'location_candidate', value: review.bundle.primary },
        );
        setMode(review.kind === 'faction' ? 'faction' : 'location');
        setSelectedRecord(null);
        setDirty(true);
        setCanUndo(false);
        setCanRedo(false);
        setRevisionTarget(false);
        navigateToSection('forge');
        setNotice(
          review.kind === 'faction'
            ? 'Review this Faction and its proposed member_of relationship before saving the bundle.'
            : 'Review this Hometown and its proposed born_in relationship before saving the bundle.',
        );
      },
    );
  };
  const forgeHometownFrom = (record: EntityRecord) =>
    forgeFurtherFrom(record, 'hometown');
  const forgeFactionFrom = (record: EntityRecord) =>
    forgeFurtherFrom(record, 'faction');
  const discardHometown = () => {
    void run(
      () => api.discardForgeBundle(),
      () => {
        setFurtherReview(null);
        setFurtherPending(false);
        setFurtherSourceId(null);
        clearWorkingView();
        setNotice(
          furtherReview?.kind === 'faction'
            ? 'Unsaved Faction bundle discarded.'
            : 'Unsaved Hometown bundle discarded.',
        );
        navigateToSection('characters');
      },
    );
  };
  const loadHistory = useCallback(() => {
    if (!project) return;
    setBusy(true);
    setError('');
    api
      .history()
      .then(setHistory)
      .catch((reason: unknown) =>
        setError(errorMessage(reason as UiError).message),
      )
      .finally(() => setBusy(false));
  }, [project]);
  useEffect(() => {
    if (project && section === 'history') loadHistory();
  }, [project, section, loadHistory]);
  const showSnapshot = (item: HistoryRecord) => {
    if (item.runId) {
      const requestId = ++historySnapshotRequest.current;
      setSelectedHistory(item);
      setSnapshot(null);
      setBusy(true);
      setError('');
      api
        .snapshot(item.runId)
        .then((value) => {
          if (requestId === historySnapshotRequest.current) setSnapshot(value);
        })
        .catch((reason: unknown) => {
          if (requestId === historySnapshotRequest.current)
            setError(errorMessage(reason as UiError).message);
        })
        .finally(() => {
          if (requestId === historySnapshotRequest.current) setBusy(false);
        });
    }
  };
  const applyWorkingState = (state: WorkingState, message: string) => {
    if (state.entity) setCandidate(state.entity);
    setCanUndo(state.canUndo);
    setCanRedo(state.canRedo);
    setDirty(true);
    setSelectedRecord(null);
    navigateToSection('forge');
    setSnapshot(null);
    setNotice(message);
  };
  const restoreHistoryToWorking = (item: HistoryRecord) => {
    if (
      dirty &&
      !window.confirm(
        'Replace the current unsaved working session? Saved history and Canon will remain unchanged.',
      )
    )
      return;
    const canonRecord = item.entityId
      ? records.find(
          (record) =>
            entityId(record.entity) === item.entityId &&
            payloadEntity(record.entity).envelope.canon_state === 'canon',
        )
      : undefined;
    void run(
      () => api.restoreHistoryToWorkingCopy(item.runId),
      (state) => {
        applyWorkingState(
          state,
          'History snapshot opened as a working copy. Save explicitly when ready.',
        );
        if (canonRecord) {
          setRevisionBase(canonRecord);
          setRevisionTarget(true);
        }
      },
    );
    if (!canonRecord) {
      setRevisionTarget(false);
      setRevisionBase(null);
    }
  };
  const useRevisionAsWorking = (revision: Revision) => {
    if (!selectedRecord) return;
    if (
      dirty &&
      !window.confirm(
        'Replace the current unsaved working session? Saved history and Canon will remain unchanged.',
      )
    )
      return;
    setRevisionBase(selectedRecord);
    void run(
      () =>
        api.loadCanonRevisionWorkingCopy(
          entityId(selectedRecord.entity),
          revision.revisionNumber,
        ),
      (state) => {
        applyWorkingState(
          state,
          'Canon revision opened as working material. Current Canon was not changed.',
        );
        setRevisionTarget(true);
      },
    );
  };
  const undo = useCallback(
    () =>
      void run(api.undo, (state) => {
        if (state.entity) setCandidate(state.entity);
        setCanUndo(state.canUndo);
        setCanRedo(state.canRedo);
        setDirty(true);
      }),
    [run],
  );
  const redo = useCallback(
    () =>
      void run(api.redo, (state) => {
        if (state.entity) setCandidate(state.entity);
        setCanUndo(state.canUndo);
        setCanRedo(state.canRedo);
        setDirty(true);
      }),
    [run],
  );
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      const editingText = target?.matches(
        'input, textarea, select, [contenteditable="true"]',
      );
      if (editingText || !(event.ctrlKey || event.metaKey)) return;
      if (event.key.toLowerCase() === 'z') {
        event.preventDefault();
        if (event.shiftKey) {
          if (canRedo) redo();
        } else if (canUndo) undo();
      } else if (event.key.toLowerCase() === 'y' && canRedo) {
        event.preventDefault();
        redo();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [canUndo, canRedo, undo, redo]);
  const changeLock = (field: Field) => {
    if (!current) return;
    const key = field.key;
    const controlKey = Object.keys(fieldsOf(current)).find((value) =>
      value.endsWith(`-${key.replaceAll('_', '-')}`),
    );
    if (!controlKey) {
      setError('This field does not expose a forge lock.');
      return;
    }
    const nextLocked = !fieldsOf(current)[controlKey]?.locked;
    updateCandidate(
      () => api.setLock(key, nextLocked),
      nextLocked
        ? `${field.label} locked. Rerolls will leave it unchanged.`
        : `${field.label} unlocked.`,
    );
  };
  const startEdit = (field: Field) => {
    setEditing(field.key);
    setEditValue(displayField(field.value));
  };
  const saveEdit = (field: Field) => {
    if (
      field.options &&
      !field.options.some((option) => option.id === editValue)
    )
      return;
    const value: FieldValue = field.list
      ? editValue
          .split('\n')
          .map((line) => line.trim())
          .filter(Boolean)
      : editValue;
    updateCandidate(
      () => api.editField(field.key, value),
      'Edit applied. Save the candidate to keep it in the project.',
    );
    setEditing(null);
  };

  if (loading && !project)
    return (
      <main className="startup-state" aria-live="polite">
        Opening your workspace…
      </main>
    );
  if (!project)
    return (
      <main className="welcome-page">
        <div className="welcome-art">
          <span className="sigil">CF</span>
          <p className="eyebrow">A LOCAL CREATIVE WORKSPACE</p>
          <h1>
            Build a world
            <br />
            one good idea at a time.
          </h1>
          <p>
            Forge Characters and Quests, shape what you love, and keep your
            canon close.
          </p>
        </div>
        <section className="project-card" aria-labelledby="project-heading">
          <p className="eyebrow">YOUR WORKSPACE</p>
          <h2 id="project-heading">Begin with a project</h2>
          <p className="muted">
            Create a world, open work you already have, or bring in a portable
            project.
          </p>
          <div className="project-start-tabs">
            {(
              [
                ['create', 'Create Project'],
                ['open', 'Open Project'],
                ['import', 'Import Project'],
              ] as [ProjectStartMode, string][]
            ).map(([mode, label]) => (
              <button
                type="button"
                aria-pressed={projectStartMode === mode}
                className={projectStartMode === mode ? 'selected' : ''}
                key={mode}
                onClick={() => {
                  setProjectStartMode(mode);
                  setError('');
                  setNotice('');
                }}
              >
                {label}
              </button>
            ))}
          </div>
          {projectStartMode === 'create' && (
            <div className="project-start-form">
              <label htmlFor="new-project-name">Project name</label>
              <input
                id="new-project-name"
                value={projectName}
                onChange={(event) => setProjectName(event.target.value)}
                autoComplete="off"
              />
              <label htmlFor="new-project-directory">Project location</label>
              <input
                id="new-project-directory"
                value={projectDirectory}
                onChange={(event) => setProjectDirectory(event.target.value)}
                placeholder="Documents\\Character Forge"
                autoComplete="off"
              />
              <small className="path-note">
                Choose a folder. Character Forge will name the project file for
                you.
              </small>
              <button
                type="button"
                className="primary full"
                disabled={
                  busy || !projectName.trim() || !projectDirectory.trim()
                }
                onClick={createProject}
              >
                {busy ? 'Creating…' : 'Create Project'}
              </button>
            </div>
          )}
          {projectStartMode === 'open' && (
            <div className="project-start-form">
              <p className="mode-explanation">
                Open an existing local project file. Portable{' '}
                <code>.forgeproject</code> files are imported instead.
              </p>
              <label htmlFor="open-project-path">Existing project file</label>
              <input
                id="open-project-path"
                value={openProjectPath}
                onChange={(event) => setOpenProjectPath(event.target.value)}
                placeholder="C:\\Worlds\\My World.sqlite3"
                autoComplete="off"
              />
              <button
                type="button"
                className="primary full"
                disabled={busy || !openProjectPath.trim()}
                onClick={openProject}
              >
                {busy ? 'Opening…' : 'Open Project'}
              </button>
            </div>
          )}
          {projectStartMode === 'import' && (
            <div className="project-start-form">
              <p className="mode-explanation">
                Import a portable <code>.forgeproject</code> into a folder. Your
                original file stays unchanged.
              </p>
              <label htmlFor="welcome-import-archive">
                Portable project file
              </label>
              <input
                id="welcome-import-archive"
                value={importArchivePath}
                onChange={(event) => setImportArchivePath(event.target.value)}
                placeholder="C:\\Worlds\\My World.forgeproject"
                autoComplete="off"
              />
              <label htmlFor="welcome-import-folder">
                New project location
              </label>
              <input
                id="welcome-import-folder"
                value={importDirectory}
                onChange={(event) => setImportDirectory(event.target.value)}
                placeholder="Documents\\Character Forge"
                autoComplete="off"
              />
              <small className="path-note">
                The imported project keeps its name and gets a matching local
                project file.
              </small>
              <button
                type="button"
                className="primary full"
                disabled={
                  busy || !importArchivePath.trim() || !importDirectory.trim()
                }
                onClick={importProject}
              >
                {busy ? 'Importing…' : 'Import Project'}
              </button>
            </div>
          )}
          <RecentProjectsList
            projects={recentProjects}
            busy={busy}
            onOpen={openRecentProject}
            onForget={forgetRecentProject}
          />
          <Message error={error} notice={notice} />
        </section>
      </main>
    );

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <span className="brand-icon">CF</span>
          <div>
            <strong>Character Forge</strong>
            <small>Quest Forge</small>
          </div>
        </div>
        <p className="nav-caption">WORKSPACE</p>
        <nav aria-label="Main navigation">
          {(
            [
              ['forge', '✦', 'Forge'],
              ['characters', '♙', 'Characters'],
              ['quests', '⚑', 'Quests'],
              ['locations', '⌂', 'Locations'],
              ['factions', '♜', 'Factions'],
              ['items', '◇', 'Items'],
              ['creatures', '♞', 'Creatures'],
              ['canon', '◆', 'Canon'],
              ['history', '◷', 'History'],
              ['project', '▤', 'Project'],
            ] as [Section, string, string][]
          ).map(([id, icon, text]) => (
            <button
              type="button"
              key={id}
              className={`nav-item ${section === id ? 'active' : ''}`}
              onClick={() => {
                navigateToSection(id);
                setSnapshot(null);
              }}
            >
              <span aria-hidden="true">{icon}</span>
              {text}
              {id === 'characters' &&
                records.filter((r) => isCharacter(r.entity)).length > 0 && (
                  <small>
                    {records.filter((r) => isCharacter(r.entity)).length}
                  </small>
                )}
            </button>
          ))}
        </nav>
        <div className="sidebar-bottom">
          <span className="status-dot" />
          Local project<span className="sidebar-project">{project.name}</span>
        </div>
      </aside>
      <main className="main-area">
        <header className="topbar">
          <div>
            <p className="eyebrow">
              {section === 'forge' ? 'CREATIVE WORKSPACE' : 'PROJECT LIBRARY'}
            </p>
            <h1>{sectionTitle(section)}</h1>
          </div>
          <div className="project-chip">
            <span className="status-dot" />
            <div>
              <strong>{project.name}</strong>
              <small>{project.path.split(/[\\/]/).slice(-1)[0]}</small>
            </div>
            <button
              type="button"
              aria-label="Project details"
              title="Project details"
              onClick={() => navigateToSection('project')}
            >
              ⌄
            </button>
          </div>
        </header>
        <Message error={error} notice={notice} />
        {section === 'forge' && (
          <div className="forge-workspace">
            <section className="setup-panel panel">
              <div className="panel-heading">
                <div>
                  <p className="eyebrow">SHAPE THE SEED</p>
                  <h2>Forge settings</h2>
                </div>
              </div>
              <div className="mode-switch">
                {(
                  [
                    'character',
                    'quest',
                    'location',
                    'faction',
                    'item',
                    'creature',
                  ] as Mode[]
                ).map((item) => (
                  <button
                    type="button"
                    key={item}
                    aria-pressed={mode === item}
                    className={mode === item ? 'selected' : ''}
                    onClick={() => setMode(item)}
                  >
                    {modeLabel(item)}
                  </button>
                ))}
              </div>
              <label>
                Universe
                <select
                  value={preset}
                  onChange={(event) => setPreset(event.target.value)}
                >
                  {presets.map(([id, text]) => (
                    <option key={id} value={id}>
                      {text}
                    </option>
                  ))}
                </select>
              </label>
              <label>
                Seed
                <div className="seed-row">
                  <input
                    aria-label="Seed"
                    type="number"
                    min="0"
                    max="9007199254740991"
                    step="1"
                    value={seed}
                    onChange={(event) => setSeed(event.target.value)}
                  />
                  <button
                    type="button"
                    onClick={() =>
                      setSeed(
                        String(
                          Math.floor(Math.random() * Number.MAX_SAFE_INTEGER),
                        ),
                      )
                    }
                  >
                    Randomize
                  </button>
                </div>
              </label>
              <small className="seed-note">
                Reuse this seed with the same settings to reproduce a result.
              </small>
              <details className="axis-details" open>
                <summary>
                  Creative axes{' '}
                  <span className="axis-controls-count">8 controls</span>
                </summary>
                <div className="axis-list">
                  {axisMeta.map((axis) => (
                    <label className="axis" key={axis.id}>
                      <span className="axis-name">
                        {axis.label}
                        <b>{axes[axis.id]}</b>
                      </span>
                      <input
                        type="range"
                        aria-label={axis.label}
                        aria-description={axis.hint}
                        title={axis.hint}
                        min="0"
                        max="100"
                        value={axes[axis.id]}
                        onChange={(event) =>
                          setAxes((old) => ({
                            ...old,
                            [axis.id]: Number(event.target.value),
                          }))
                        }
                      />
                      <span className="axis-ends">
                        <small>{axis.low}</small>
                        <small>{axis.high}</small>
                      </span>
                    </label>
                  ))}
                </div>
              </details>
              {mode === 'character' && (
                <details className="constraint-details">
                  <summary>Character constraints</summary>
                  <div className="check-list">
                    {(
                      [
                        ['must_be_non_human', 'Non-human'],
                        ['no_magic', 'No magic'],
                        ['no_firearms', 'No firearms'],
                        ['urban_only', 'Urban only'],
                      ] as const
                    ).map(([key, label]) => (
                      <label key={key} className="check">
                        <input
                          type="checkbox"
                          checked={Boolean(constraints[key])}
                          onChange={(event) =>
                            setConstraints((old) => ({
                              ...old,
                              [key]: event.target.checked ? 'true' : '',
                            }))
                          }
                        />
                        {label}
                      </label>
                    ))}
                  </div>
                  <label>
                    Inspiration
                    <input
                      value={constraints.inspiration ?? ''}
                      onChange={(event) =>
                        setConstraints((old) => ({
                          ...old,
                          inspiration: event.target.value,
                        }))
                      }
                      placeholder="Optional creative direction"
                    />
                  </label>
                </details>
              )}
              {mode === 'quest' && (
                <label>
                  Inspiration
                  <input
                    value={constraints.inspiration ?? ''}
                    onChange={(event) =>
                      setConstraints((old) => ({
                        ...old,
                        inspiration: event.target.value,
                      }))
                    }
                    placeholder="Optional creative direction"
                  />
                </label>
              )}
              <button
                type="button"
                className="primary forge-button"
                disabled={busy}
                onClick={forge}
              >
                {busy ? 'Forging…' : `Forge ${modeLabel(mode)}`}
                <span className="forge-button-icon" aria-hidden="true">
                  ✦
                </span>
              </button>
              {candidate && (
                <button
                  type="button"
                  className="quiet full"
                  disabled={busy || !canShape}
                  onClick={() =>
                    updateCandidate(
                      api.rerollUnlocked,
                      'All unlocked fields rerolled. Locked fields remain unchanged.',
                    )
                  }
                >
                  Reroll all unlocked fields
                </button>
              )}
            </section>
            <section className="result-panel panel">
              <div className="panel-heading result-heading">
                <div>
                  <p className="eyebrow">
                    {candidate ? 'CURRENT RESULT' : 'YOUR NEXT IDEA'}
                  </p>
                  <h2>
                    {current
                      ? payloadTitle(current)
                      : 'Your next story starts here'}
                  </h2>
                </div>
                {current && (
                  <div className="state-pill">
                    <span>
                      {payloadEntity(current).envelope.canon_state === 'canon'
                        ? '◆ Canon'
                        : dirty && (candidateSaved || selectedRecord)
                          ? '● Changes not saved'
                          : candidateSaved || selectedRecord
                            ? '● Saved candidate'
                            : '○ Unsaved candidate'}
                    </span>
                  </div>
                )}
              </div>
              {!current ? (
                <div className="empty-result">
                  <span className="empty-sigil">✦</span>
                  <h3>A blank page with a spark.</h3>
                  <p>
                    Choose a preset, tune the creative axes, and forge a
                    structured result. Your seed makes it reproducible.
                  </p>
                </div>
              ) : (
                <>
                  {furtherPending && furtherReview && (
                    <section
                      className="panel context-card"
                      aria-label={
                        furtherReview.kind === 'faction'
                          ? 'Faction review'
                          : 'Hometown review'
                      }
                    >
                      <p className="eyebrow">FORGE FURTHER REVIEW</p>
                      <h3>
                        {furtherReview.kind === 'faction'
                          ? 'Forge Their Faction'
                          : 'Hometown'}{' '}
                        · Candidate
                      </h3>
                      <p>
                        Source Character:{' '}
                        {records
                          .filter(
                            (row) => entityId(row.entity) === furtherSourceId,
                          )
                          .map((row) => payloadTitle(row.entity))[0] ??
                          furtherSourceId}
                      </p>
                      <label className="field-control">
                        Action seed
                        <input
                          aria-label={
                            furtherReview.kind === 'faction'
                              ? 'Faction action seed'
                              : 'Hometown action seed'
                          }
                          type="number"
                          min="0"
                          step="1"
                          value={furtherSeed}
                          onChange={(event) =>
                            setFurtherSeed(event.target.value)
                          }
                        />
                      </label>
                      <p>
                        Proposed relationship:{' '}
                        {furtherReview.kind === 'faction'
                          ? 'Member of'
                          : 'Born in'}{' '}
                        {furtherReview.kind === 'faction'
                          ? payloadTitle(
                              candidate ?? {
                                kind: 'faction_candidate',
                                value: furtherReview.bundle.primary,
                              },
                            )
                          : payloadTitle({
                              kind: 'location_candidate',
                              value: furtherReview.bundle.primary,
                            })}
                      </p>
                      <p>
                        Context:{' '}
                        {Object.entries(
                          furtherReview.bundle.recipe.context_snapshot.facts,
                        )
                          .map(([key, value]) => `${key}: ${value}`)
                          .join(' · ')}
                      </p>
                      <p>
                        Generated using action seed{' '}
                        {furtherReview.bundle.recipe.action_seed}
                      </p>
                      <small>
                        Context hash:{' '}
                        {
                          furtherReview.bundle.recipe.context_snapshot
                            .context_hash
                        }
                      </small>
                      <div className="button-row">
                        <button
                          type="button"
                          disabled={busy}
                          onClick={() => {
                            if (furtherSourceId) {
                              const source = records.find(
                                (row) =>
                                  entityId(row.entity) === furtherSourceId,
                              );
                              if (source)
                                forgeFurtherFrom(source, furtherReview.kind);
                            }
                          }}
                        >
                          Forge Again
                        </button>
                        <button
                          type="button"
                          disabled={busy}
                          onClick={discardHometown}
                        >
                          Discard Bundle
                        </button>
                      </div>
                    </section>
                  )}
                  <div className="field-list">
                    {allFields.map((field) => {
                      const control = fieldControl(current, field.key);
                      const locked = Boolean(control?.locked);
                      const modified =
                        originOf(current, field.key) === 'user_modified';
                      return (
                        <article
                          className={`result-field ${locked ? 'locked' : ''}`}
                          key={field.key}
                        >
                          <div className="field-title">
                            <h3>{field.label}</h3>
                            <div className="field-tags">
                              {locked && (
                                <span className="tag lock-tag">Locked</span>
                              )}
                              {modified && (
                                <span className="tag edit-tag">Edited</span>
                              )}
                            </div>
                          </div>
                          {editing === field.key ? (
                            <div className="edit-box">
                              {field.options ? (
                                <select
                                  aria-label={`Edit ${field.label}`}
                                  value={editValue}
                                  onChange={(event) =>
                                    setEditValue(event.target.value)
                                  }
                                >
                                  {field.options.map((option) => (
                                    <option key={option.id} value={option.id}>
                                      {option.label}
                                    </option>
                                  ))}
                                </select>
                              ) : (
                                <textarea
                                  aria-label={`Edit ${field.label}`}
                                  value={editValue}
                                  onChange={(event) =>
                                    setEditValue(event.target.value)
                                  }
                                  onKeyDown={(event) => {
                                    if (event.key === 'Escape')
                                      setEditing(null);
                                  }}
                                />
                              )}
                              <div className="button-row">
                                <button
                                  type="button"
                                  className="primary small"
                                  onClick={() => saveEdit(field)}
                                >
                                  Save edit
                                </button>
                                <button
                                  type="button"
                                  className="small"
                                  onClick={() => setEditing(null)}
                                >
                                  Cancel
                                </button>
                              </div>
                            </div>
                          ) : (
                            <FieldContent field={field} />
                          )}
                          {canShape && (
                            <div className="field-actions">
                              <button
                                type="button"
                                aria-pressed={locked}
                                aria-label={`${locked ? 'Unlock' : 'Lock'} ${field.label}`}
                                className={`icon-button ${locked ? 'pressed' : ''}`}
                                onClick={() => changeLock(field)}
                              >
                                {locked ? '▣' : '▢'}{' '}
                                <span className="icon-button-label">
                                  {locked ? 'Unlock' : 'Lock'}
                                </span>
                              </button>
                              <button
                                type="button"
                                aria-label={`Reroll ${field.label}`}
                                className="icon-button"
                                disabled={locked || busy}
                                title={
                                  locked
                                    ? 'Unlock this field before rerolling'
                                    : `Reroll ${field.label}`
                                }
                                onClick={() =>
                                  updateCandidate(
                                    () => api.reroll(field.key),
                                    `${field.label} rerolled.`,
                                  )
                                }
                              >
                                ↻{' '}
                                <span className="icon-button-label">
                                  Reroll
                                </span>
                              </button>
                              <button
                                type="button"
                                aria-label={`Edit ${field.label}`}
                                className="icon-button"
                                disabled={busy}
                                onClick={() => startEdit(field)}
                              >
                                ✎{' '}
                                <span className="icon-button-label">Edit</span>
                              </button>
                            </div>
                          )}
                        </article>
                      );
                    })}
                  </div>
                </>
              )}
            </section>
            <aside className="context-panel">
              <section className="panel context-card">
                <p className="eyebrow">SESSION</p>
                <h2>{current ? 'Ready to shape' : 'A new beginning'}</h2>
                <p className="muted">
                  {current
                    ? currentDna
                      ? `Seed ${currentDna.recipe.root_seed} · ${label}`
                      : label
                    : 'Your settings stay ready while you explore the project.'}
                </p>
                {current && (
                  <div className="context-stats">
                    <div>
                      <span className="context-stat-label">Preset</span>
                      <strong className="context-stat-value">{label}</strong>
                    </div>
                    <div>
                      <span className="context-stat-label">Seed</span>
                      <strong className="context-stat-value">
                        {currentDna?.recipe.root_seed ?? 'Unavailable'}
                      </strong>
                    </div>
                    <div>
                      <span className="context-stat-label">Fields</span>
                      <strong className="context-stat-value">
                        {allFields.length} structured details
                      </strong>
                    </div>
                  </div>
                )}
                <div className="button-row">
                  <button
                    type="button"
                    disabled={!canUndo || busy}
                    onClick={undo}
                    aria-label="Undo last session change"
                  >
                    Undo
                  </button>
                  <button
                    type="button"
                    disabled={!canRedo || busy}
                    onClick={redo}
                    aria-label="Redo session change"
                  >
                    Redo
                  </button>
                </div>
                <small>
                  Session changes only · Ctrl+Z to undo · Ctrl+Y to redo
                </small>
              </section>
              {current && (
                <section className="panel action-card">
                  <p className="eyebrow">KEEP THIS WORK</p>
                  {furtherPending ? (
                    <>
                      <button
                        type="button"
                        className="primary full"
                        disabled={busy}
                        onClick={save}
                      >
                        Save Bundle
                      </button>
                      <button
                        type="button"
                        className="full"
                        disabled={busy}
                        onClick={discardHometown}
                      >
                        Discard Bundle
                      </button>
                      <small>
                        {furtherReview?.kind === 'faction'
                          ? 'Faction and member_of relationship'
                          : 'Location and born_in relationship'}{' '}
                        save together with frozen provenance.
                      </small>
                    </>
                  ) : (
                    <>
                      <button
                        type="button"
                        className="primary full"
                        disabled={busy || !canShape}
                        onClick={save}
                      >
                        {candidateSaved || selectedRecord
                          ? revisionTarget
                            ? 'Save Canon revision'
                            : 'Save changes'
                          : 'Save candidate'}
                      </button>
                      <button
                        type="button"
                        className="full"
                        disabled={
                          busy ||
                          (!candidateSaved && !selectedRecord) ||
                          payloadEntity(current).envelope.canon_state ===
                            'canon'
                        }
                        onClick={() => {
                          const record = candidateSaved ?? selectedRecord;
                          if (record) promote(record);
                        }}
                      >
                        Promote to canon
                      </button>
                      <button
                        type="button"
                        className="full"
                        disabled={busy || !current}
                        onClick={() => {
                          void run(
                            () => api.history(entityId(current)),
                            setHistory,
                          );
                          navigateToSection('history');
                        }}
                      >
                        View history
                      </button>
                      <small>
                        Changes to saved work are kept when you save.
                      </small>
                    </>
                  )}
                </section>
              )}
              <section className="panel context-card">
                <p className="eyebrow">PROJECT LIBRARY</p>
                <p className="library-count">
                  {records.filter((row) => isCharacter(row.entity)).length}{' '}
                  <span className="library-count-label">Characters</span>
                </p>
                <p className="library-count">
                  {records.filter((row) => isQuest(row.entity)).length}{' '}
                  <span className="library-count-label">Quests</span>
                </p>
                <p className="library-count">
                  {records.filter((row) => isFaction(row.entity)).length}{' '}
                  <span className="library-count-label">Factions</span>
                </p>
                <p className="library-count">
                  {records.filter((row) => isItem(row.entity)).length}{' '}
                  <span className="library-count-label">Items</span>
                </p>
                <p className="library-count">
                  {records.filter((row) => isCreature(row.entity)).length}{' '}
                  <span className="library-count-label">Creatures</span>
                </p>
                <button
                  type="button"
                  className="quiet full"
                  onClick={() =>
                    navigateToSection(
                      mode === 'character'
                        ? 'characters'
                        : mode === 'quest'
                          ? 'quests'
                          : mode === 'location'
                            ? 'locations'
                            : mode === 'faction'
                              ? 'factions'
                              : mode === 'item'
                                ? 'items'
                                : 'creatures',
                    )
                  }
                >
                  Browse{' '}
                  {mode === 'character'
                    ? 'Characters'
                    : mode === 'quest'
                      ? 'Quests'
                      : mode === 'location'
                        ? 'Locations'
                        : mode === 'faction'
                          ? 'Factions'
                          : mode === 'item'
                            ? 'Items'
                            : 'Creatures'}{' '}
                  →
                </button>
              </section>
            </aside>
          </div>
        )}
        {section === 'characters' && (
          <LibraryView
            title="Characters"
            records={records.filter((row) => isCharacter(row.entity))}
            loading={loading}
            empty="No Characters yet. Forge your first Character."
            emptyActionLabel="Forge a Character"
            onEmptyAction={() => {
              setMode('character');
              navigateToSection('forge');
            }}
            onSelect={selectRecord}
            onFavorite={favorite}
            onPromote={promote}
            onExport={exportEntity}
            onForgeHometown={forgeHometownFrom}
            onForgeFaction={forgeFactionFrom}
          />
        )}
        {section === 'quests' && (
          <LibraryView
            title="Quests"
            records={records.filter((row) => isQuest(row.entity))}
            loading={loading}
            empty="No Quests yet. Forge your first Quest."
            emptyActionLabel="Forge a Quest"
            onEmptyAction={() => {
              setMode('quest');
              navigateToSection('forge');
            }}
            onSelect={selectRecord}
            onFavorite={favorite}
            onPromote={promote}
            onExport={exportEntity}
          />
        )}
        {section === 'locations' && (
          <LibraryView
            title="Locations"
            records={records.filter((row) => isLocation(row.entity))}
            loading={loading}
            empty="No Locations yet. Forge your first Location."
            emptyActionLabel="Forge a Location"
            onEmptyAction={() => {
              setMode('location');
              navigateToSection('forge');
            }}
            onSelect={selectRecord}
            onFavorite={favorite}
            onPromote={promote}
            onExport={exportEntity}
          />
        )}
        {section === 'factions' && (
          <LibraryView
            title="Factions"
            records={records.filter((row) => isFaction(row.entity))}
            loading={loading}
            empty="No Factions yet. Forge your first Faction."
            emptyActionLabel="Forge a Faction"
            onEmptyAction={() => {
              setMode('faction');
              navigateToSection('forge');
            }}
            onSelect={selectRecord}
            onFavorite={favorite}
            onPromote={promote}
            onExport={exportEntity}
          />
        )}
        {section === 'items' && (
          <LibraryView
            title="Items"
            records={records.filter((row) => isItem(row.entity))}
            loading={loading}
            empty="No Items yet. Forge your first Item."
            emptyActionLabel="Forge an Item"
            onEmptyAction={() => {
              setMode('item');
              navigateToSection('forge');
            }}
            onSelect={selectRecord}
            onFavorite={favorite}
            onPromote={promote}
            onExport={exportEntity}
          />
        )}
        {section === 'creatures' && (
          <LibraryView
            title="Creatures"
            records={records.filter((row) => isCreature(row.entity))}
            loading={loading}
            empty="No Creatures yet. Forge your first Creature."
            emptyActionLabel="Forge a Creature"
            onEmptyAction={() => {
              setMode('creature');
              navigateToSection('forge');
            }}
            onSelect={selectRecord}
            onFavorite={favorite}
            onPromote={promote}
            onExport={exportEntity}
          />
        )}
        {section === 'canon' && (
          <div className="library-layout">
            <section className="library-list panel">
              <div className="panel-heading">
                <div>
                  <p className="eyebrow">ESTABLISHED TRUTH</p>
                  <h2>Canon</h2>
                </div>
              </div>
              {canonRecords.length === 0 ? (
                <Empty
                  text="Nothing has been promoted to Canon yet. Save a candidate, then choose Promote to Canon when it feels right."
                  actionLabel="Return to Forge"
                  onAction={() => navigateToSection('forge')}
                />
              ) : (
                canonRecords.map((record) => (
                  <RecordRow
                    key={entityId(record.entity)}
                    record={record}
                    selected={
                      selectedRecord !== null &&
                      entityId(selectedRecord.entity) ===
                        entityId(record.entity)
                    }
                    onSelect={selectRecord}
                    onFavorite={favorite}
                    onPromote={promote}
                  />
                ))
              )}
            </section>
            <DetailView
              record={selectedRecord}
              onForgeHometown={forgeHometownFrom}
              onForgeFaction={forgeFactionFrom}
            />
            {selectedRecord && (
              <RevisionPanel
                revisions={revisions}
                selected={selectedRevision}
                onSelect={setSelectedRevision}
                onUse={useRevisionAsWorking}
              />
            )}
          </div>
        )}
        {section === 'history' && (
          <div className="library-layout">
            <section className="library-list panel">
              <div className="panel-heading">
                <div>
                  <p className="eyebrow">CREATIVE PROVENANCE</p>
                  <h2>History</h2>
                </div>
                <button type="button" onClick={loadHistory} disabled={busy}>
                  Refresh
                </button>
              </div>
              {history.length === 0 ? (
                <Empty
                  text="Activity will appear here as you Forge, edit, save, and promote."
                  actionLabel="Return to Forge"
                  onAction={() => navigateToSection('forge')}
                />
              ) : (
                [...history].reverse().map((item) => (
                  <button
                    type="button"
                    className="history-row"
                    key={item.runId}
                    onClick={() => showSnapshot(item)}
                  >
                    <span className="history-mark">◷</span>
                    <span>
                      <strong className="history-action">
                        {friendlyAction(item.action)}
                      </strong>
                      <small className="history-meta">
                        {dateLabel(item.createdAtUnixMs)} ·{' '}
                        {item.entityId
                          ? historyEntityTitle(item.entityId, records)
                          : 'Forge session'}
                      </small>
                    </span>
                    <b>›</b>
                  </button>
                ))
              )}
            </section>
            <section className="detail-panel panel">
              <p className="eyebrow">READ-ONLY HISTORY SNAPSHOT</p>
              {snapshot ? (
                <>
                  <DetailView record={{ entity: snapshot, writeVersion: 0 }} />
                  <button
                    type="button"
                    disabled={busy || !selectedHistory}
                    onClick={() =>
                      selectedHistory &&
                      restoreHistoryToWorking(selectedHistory)
                    }
                  >
                    Use as working copy
                  </button>
                  <small>
                    This creates an editable session copy. History and Canon
                    remain unchanged until you explicitly save or revise.
                  </small>
                </>
              ) : (
                <Empty text="Choose a history entry to inspect its saved snapshot." />
              )}
            </section>
          </div>
        )}
        {section === 'project' && (
          <section className="project-settings panel">
            <p className="eyebrow">LOCAL PROJECT</p>
            <h2>{project.name}</h2>
            <p className="muted">
              This project and its saved work live on this device.
            </p>
            <section
              className="vault-group"
              aria-labelledby="app-settings-heading"
            >
              <p className="eyebrow">APPLICATION</p>
              <h3 id="app-settings-heading">Settings</h3>
              <label className="settings-toggle">
                <input
                  type="checkbox"
                  checked={settings.reducedMotion}
                  onChange={(event) => {
                    const next: UserSettings = {
                      version: 1,
                      reducedMotion: event.target.checked,
                    };
                    void run(() => api.updateUserSettings(next), setSettings);
                  }}
                />
                Reduce motion
              </label>
              <small className="muted">
                Uses the system motion preference unless enabled here. Stored
                locally for this app.
              </small>
            </section>
            <dl>
              <dt>Project file</dt>
              <dd className="project-file-path">{project.path}</dd>
              <dt>Characters</dt>
              <dd>{records.filter((r) => isCharacter(r.entity)).length}</dd>
              <dt>Quests</dt>
              <dd>{records.filter((r) => isQuest(r.entity)).length}</dd>
              <dt>Creatures</dt>
              <dd>{records.filter((r) => isCreature(r.entity)).length}</dd>
              <dt>Items</dt>
              <dd>{records.filter((r) => isItem(r.entity)).length}</dd>
              <dt>Factions</dt>
              <dd>{records.filter((r) => isFaction(r.entity)).length}</dd>
              <dt>Created</dt>
              <dd>{dateLabel(project.createdAtUnixMs)}</dd>
            </dl>
            <div className="project-archive-actions">
              <section
                className="vault-group"
                aria-labelledby="portability-heading"
              >
                <p className="eyebrow">PORTABILITY</p>
                <h3 id="portability-heading">Move a project between places</h3>
                <label htmlFor="export-project-path">Export project to</label>
                <input
                  id="export-project-path"
                  className="project-path-input"
                  value={exportPath}
                  onChange={(event) => setExportPath(event.target.value)}
                  placeholder="C:\\Worlds\\My World.forgeproject"
                />
                <button
                  type="button"
                  className="primary"
                  disabled={busy || !exportPath.trim()}
                  onClick={exportProject}
                >
                  {busy ? 'Exporting…' : 'Export Project'}
                </button>
                <label htmlFor="import-project-file">
                  Import portable project
                </label>
                <input
                  id="import-project-file"
                  className="project-path-input"
                  value={importArchivePath}
                  onChange={(event) => setImportArchivePath(event.target.value)}
                  placeholder="C:\\Worlds\\My World.forgeproject"
                />
                <label htmlFor="import-project-folder">
                  Import into folder
                </label>
                <input
                  id="import-project-folder"
                  className="project-path-input"
                  value={importDirectory}
                  onChange={(event) => setImportDirectory(event.target.value)}
                  placeholder="C:\\Worlds"
                />
                <button
                  type="button"
                  disabled={
                    busy || !importArchivePath.trim() || !importDirectory.trim()
                  }
                  onClick={importProject}
                >
                  {busy ? 'Importing…' : 'Import Project'}
                </button>
              </section>
              <section
                className="vault-group safety-group"
                aria-labelledby="safety-heading"
              >
                <p className="eyebrow">SAFETY</p>
                <h3 id="safety-heading">Keep a backup close</h3>
                <label htmlFor="backup-project-path">Create backup at</label>
                <input
                  id="backup-project-path"
                  className="project-path-input"
                  value={backupPath}
                  onChange={(event) => setBackupPath(event.target.value)}
                  placeholder="C:\\Worlds\\My World.backup.forgeproject"
                />
                <button
                  type="button"
                  disabled={busy || !backupPath.trim()}
                  onClick={createBackup}
                >
                  {busy ? 'Creating backup…' : 'Create Backup'}
                </button>
                <label htmlFor="restore-project-path">
                  Restore from backup file
                </label>
                <input
                  id="restore-project-path"
                  className="project-path-input"
                  value={restorePath}
                  onChange={(event) => setRestorePath(event.target.value)}
                  placeholder="C:\\Worlds\\My World.backup.forgeproject"
                />
                <button
                  type="button"
                  className="danger"
                  disabled={busy || !restorePath.trim()}
                  onClick={restoreBackup}
                >
                  Restore Backup…
                </button>
              </section>
              {busy && <p role="status">Working with project file…</p>}
            </div>
            <button
              type="button"
              className="danger"
              onClick={() =>
                void run(api.closeProject, () => {
                  setProject(null);
                  clearWorkingView();
                  setSelectedRecord(null);
                  setOpenProjectPath('');
                })
              }
            >
              Close project
            </button>
          </section>
        )}
      </main>
    </div>
  );
}

function entityId(entity: ForgePayload) {
  return payloadEntity(entity).envelope.id;
}
function historyEntityTitle(id: string, records: EntityRecord[]) {
  const record = records.find((item) => entityId(item.entity) === id);
  return record ? payloadTitle(record.entity) : 'Saved result';
}
function sectionTitle(section: Section) {
  return (
    {
      forge: 'Forge',
      characters: 'Characters',
      quests: 'Quests',
      locations: 'Locations',
      factions: 'Factions',
      items: 'Items',
      creatures: 'Creatures',
      canon: 'Canon',
      history: 'History',
      project: 'Project',
    } as const
  )[section];
}
function modeLabel(mode: Mode) {
  return {
    character: 'Character',
    quest: 'Quest',
    location: 'Location',
    faction: 'Faction',
    item: 'Item',
    creature: 'Creature',
  }[mode];
}
function friendlyAction(action: string) {
  const labels: Record<string, string> = {
    forge: 'Forged a result',
    reroll_field: 'Rerolled a field',
    reroll_all_unlocked: 'Rerolled unlocked fields',
    manual_edit: 'Edited a result',
    restore_history: 'Restored an earlier result',
    candidate: 'Candidate',
    canon: 'Canon',
  };
  return (
    labels[action] ??
    action
      .replaceAll('_', ' ')
      .replace(/\b\w/g, (letter) => letter.toUpperCase())
  );
}
function Message({ error, notice }: { error: string; notice: string }) {
  return (
    <>
      {error && (
        <div className="message error" role="alert">
          <strong>Couldn’t complete that action</strong>
          <span>{error}</span>
        </div>
      )}
      {notice && !error && (
        <div className="message notice" role="status">
          {notice}
        </div>
      )}
    </>
  );
}
function FieldContent({ field }: { field: Field }) {
  return Array.isArray(field.value) ? (
    <ul className="field-list-values">
      {field.value.map((item) => (
        <li key={item}>{item}</li>
      ))}
    </ul>
  ) : (
    <p className="field-value">
      {(field.options?.find((option) => option.id === field.value)?.label ??
        field.value) ||
        'No text added.'}
    </p>
  );
}
function Empty({
  text,
  actionLabel,
  onAction,
}: {
  text: string;
  actionLabel?: string;
  onAction?: () => void;
}) {
  return (
    <div className="empty-state">
      <span className="empty-state-icon" aria-hidden="true">
        ✦
      </span>
      <p>{text}</p>
      {actionLabel && onAction && (
        <button type="button" className="quiet" onClick={onAction}>
          {actionLabel}
        </button>
      )}
    </div>
  );
}
function RecentProjectsList({
  projects,
  busy,
  onOpen,
  onForget,
}: {
  projects: RecentProject[];
  busy: boolean;
  onOpen: (project: RecentProject) => void;
  onForget: (project: RecentProject) => void;
}) {
  return (
    <section
      className="recent-projects"
      aria-labelledby="recent-projects-heading"
    >
      <div className="recent-projects-heading">
        <h3 id="recent-projects-heading">Recent Projects</h3>
        {projects.length > 0 && <span>{projects.length}</span>}
      </div>
      {projects.length === 0 ? (
        <p className="recent-empty">
          Projects you open will appear here on this device.
        </p>
      ) : (
        <div className="recent-project-list">
          {projects.map((project) => (
            <article className="recent-project-row" key={project.path}>
              <button
                type="button"
                className="recent-project-open"
                disabled={busy}
                onClick={() => onOpen(project)}
              >
                <strong>{project.name}</strong>
                <small>
                  {project.exists
                    ? project.path
                    : 'File not found at this location'}
                </small>
                {project.exists && (
                  <small>
                    Last opened {dateLabel(project.lastOpenedAtUnixMs)}
                  </small>
                )}
              </button>
              <button
                type="button"
                className="recent-forget"
                disabled={busy}
                aria-label={`Forget ${project.name} from Recent Projects`}
                onClick={() => onForget(project)}
              >
                Forget
              </button>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
function RecordRow({
  record,
  selected = false,
  onSelect,
  onFavorite,
  onPromote,
}: {
  record: EntityRecord;
  selected?: boolean;
  onSelect: (record: EntityRecord) => void;
  onFavorite: (record: EntityRecord) => void;
  onPromote: (record: EntityRecord) => void;
}) {
  const entity = payloadEntity(record.entity);
  const recipe = entity.dna?.recipe;
  const character = isCharacter(record.entity);
  const location = isLocation(record.entity);
  const faction = isFaction(record.entity);
  const summary = character
    ? required(entity.character, 'Character payload').archetype
    : location
      ? required(entity.location, 'Location payload').summary
      : faction
        ? required(entity.faction, 'Faction payload').identity
        : isItem(record.entity)
          ? required(entity.item, 'Item payload').purpose
          : isCreature(record.entity)
            ? required(entity.creature, 'Creature payload').threat
            : required(entity.quest, 'Quest payload').objective;
  return (
    <article className="record-row">
      <button
        type="button"
        className={`record-main ${selected ? 'selected' : ''}`}
        aria-pressed={selected}
        onClick={() => onSelect(record)}
      >
        <span className="record-glyph">
          {character
            ? '♙'
            : location
              ? '⌂'
              : faction
                ? '♜'
                : isItem(record.entity)
                  ? '◇'
                  : isCreature(record.entity)
                    ? '♞'
                    : '⚑'}
        </span>
        <span>
          <strong>{payloadTitle(record.entity)}</strong>
          <small>{summary}</small>
          <em>
            {recipe
              ? (labels[recipe.universe_preset] ?? recipe.universe_preset)
              : 'No generation recipe'}{' '}
            · {friendlyAction(entity.envelope.canon_state)}
          </em>
        </span>
      </button>
      <button
        type="button"
        className="star-button"
        aria-label={
          entity.envelope.favorite ? 'Remove favorite' : 'Add favorite'
        }
        onClick={() => onFavorite(record)}
      >
        {entity.envelope.favorite ? '★' : '☆'}
      </button>
      {entity.envelope.canon_state !== 'canon' && (
        <button
          type="button"
          className="small"
          onClick={() => onPromote(record)}
        >
          Promote
        </button>
      )}
    </article>
  );
}
function DetailView({
  record,
  onExport,
  onForgeHometown,
  onForgeFaction,
}: {
  record: EntityRecord | null;
  revisions?: Revision[];
  onExport?: (record: EntityRecord, format: 'json' | 'markdown') => void;
  onForgeHometown?: (record: EntityRecord) => void;
  onForgeFaction?: (record: EntityRecord) => void;
}) {
  if (!record)
    return (
      <section className="detail-panel panel">
        <Empty text="Select a saved item to see the complete structured result." />
      </section>
    );
  const entity = payloadEntity(record.entity);
  const recipe = entity.dna?.recipe;
  const character = isCharacter(record.entity);
  const hometownSupported = Boolean(
    character &&
      record.writeVersion > 0 &&
      ['candidate', 'canon'].includes(entity.envelope.canon_state) &&
      entity.envelope.origin === 'generated' &&
      entity.envelope.generation_provenance?.dna.recipe.kind === 'character' &&
      entity.envelope.generation_provenance.dna.generator_version === 1 &&
      [1, 2].includes(
        entity.envelope.generation_provenance.dna.content_version ?? -1,
      ),
  );
  return (
    <section className="detail-panel panel">
      <div className="panel-heading">
        <div>
          <p className="eyebrow">
            {record.entity.kind.includes('candidate')
              ? 'SAVED CANDIDATE'
              : 'CANON SNAPSHOT'}
          </p>
          <h2>{payloadTitle(record.entity)}</h2>
        </div>
        <span className="state-pill">
          {friendlyAction(payloadEntity(record.entity).envelope.canon_state)}
        </span>
      </div>
      <p className="detail-meta">
        {recipe
          ? `${labels[recipe.universe_preset] ?? recipe.universe_preset} · seed ${recipe.root_seed}`
          : 'No generation provenance'}
      </p>
      {character && onForgeHometown && (
        <div className="button-row">
          <button
            type="button"
            className="primary"
            disabled={!hometownSupported}
            title={
              hometownSupported
                ? 'Create a Settlement Candidate from this saved Character.'
                : 'Forge Hometown requires a saved generated Character with valid generation provenance.'
            }
            onClick={() => onForgeHometown(record)}
          >
            Forge Hometown
          </button>
          {onForgeFaction && (
            <button
              type="button"
              className="primary"
              disabled={!hometownSupported}
              title={
                hometownSupported
                  ? 'Create a Faction Candidate from this saved Character.'
                  : 'Forge Their Faction requires supported generated Character provenance.'
              }
              onClick={() => onForgeFaction(record)}
            >
              Forge Their Faction
            </button>
          )}
          {!hometownSupported && (
            <small>
              Unavailable: this Character has no supported generated ForgeDNA.
            </small>
          )}
        </div>
      )}
      {onExport && (
        <div className="button-row">
          <button type="button" onClick={() => onExport(record, 'json')}>
            Export JSON
          </button>
          <button type="button" onClick={() => onExport(record, 'markdown')}>
            Export Markdown
          </button>
        </div>
      )}
      <div className="detail-fields">
        {payloadFields(record.entity).map((field) => (
          <article key={field.key}>
            <h3>{field.label}</h3>
            <FieldContent field={field} />
            {originOf(record.entity, field.key) === 'user_modified' && (
              <span className="tag edit-tag">Edited</span>
            )}
            {fieldControl(record.entity, field.key)?.locked && (
              <span className="tag lock-tag">Locked</span>
            )}
          </article>
        ))}
      </div>
      {record.writeVersion > 0 && (
        <Connections
          key={`${entity.envelope.project_id}:${entity.envelope.id}`}
          entityId={entity.envelope.id}
        />
      )}
    </section>
  );
}
function RevisionPanel({
  revisions,
  selected,
  onSelect,
  onUse,
}: {
  revisions: Revision[];
  selected: Revision | null;
  onSelect: (revision: Revision) => void;
  onUse: (revision: Revision) => void;
}) {
  return (
    <section className="revision-panel panel">
      <p className="eyebrow">CANON REVISIONS</p>
      {revisions.length ? (
        revisions.map((revision) => (
          <article key={revision.revisionNumber}>
            <strong className="revision-number">
              Revision {revision.revisionNumber}
            </strong>
            <small className="revision-reason">
              {revision.reason} · {dateLabel(revision.createdAtUnixMs)}
            </small>
            <div className="button-row">
              <button
                type="button"
                onClick={() => onSelect(revision)}
                aria-pressed={
                  selected?.revisionNumber === revision.revisionNumber
                }
              >
                Inspect
              </button>
              <button type="button" onClick={() => onUse(revision)}>
                Use as working copy
              </button>
            </div>
          </article>
        ))
      ) : (
        <p className="muted">No canon revisions yet.</p>
      )}
      {selected && (
        <>
          <p className="eyebrow">READ-ONLY CANON REVISION</p>
          <DetailView record={{ entity: selected.entity, writeVersion: 0 }} />
        </>
      )}
    </section>
  );
}
function LibraryView({
  title,
  records,
  loading,
  empty,
  emptyActionLabel,
  onEmptyAction,
  onSelect,
  onFavorite,
  onPromote,
  onExport,
  onForgeHometown,
  onForgeFaction,
}: {
  title: string;
  records: EntityRecord[];
  loading: boolean;
  empty: string;
  emptyActionLabel?: string;
  onEmptyAction?: () => void;
  onSelect: (record: EntityRecord) => void;
  onFavorite: (record: EntityRecord) => void;
  onPromote: (record: EntityRecord) => void;
  onExport: (record: EntityRecord, format: 'json' | 'markdown') => void;
  onForgeHometown?: (record: EntityRecord) => void;
  onForgeFaction?: (record: EntityRecord) => void;
}) {
  const [selected, setSelected] = useState<EntityRecord | null>(null);
  const select = (record: EntityRecord) => {
    setSelected(record);
    onSelect(record);
  };
  return (
    <div className="library-layout">
      <section className="library-list panel">
        <div className="panel-heading">
          <div>
            <p className="eyebrow">PROJECT LIBRARY</p>
            <h2>{title}</h2>
          </div>
          <span className="count-pill">{records.length}</span>
        </div>
        {loading ? (
          <Empty text={`Loading ${title.toLowerCase()}…`} />
        ) : records.length === 0 ? (
          <Empty
            text={empty}
            actionLabel={emptyActionLabel}
            onAction={onEmptyAction}
          />
        ) : (
          records.map((record) => (
            <RecordRow
              key={entityId(record.entity)}
              record={record}
              selected={
                selected !== null &&
                entityId(selected.entity) === entityId(record.entity)
              }
              onSelect={select}
              onFavorite={onFavorite}
              onPromote={onPromote}
            />
          ))
        )}
      </section>
      <DetailView
        record={
          records.find(
            (record) =>
              selected && entityId(record.entity) === entityId(selected.entity),
          ) ?? selected
        }
        onExport={onExport}
        onForgeHometown={onForgeHometown}
        onForgeFaction={onForgeFaction}
      />
    </div>
  );
}
