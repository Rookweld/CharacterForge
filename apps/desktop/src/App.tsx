import { useCallback, useEffect, useRef, useState } from 'react';
import {
  type Axis,
  api,
  axisMeta,
  type Character,
  type EntityRecord,
  errorMessage,
  type FieldValue,
  type ForgePayload,
  type HistoryRecord,
  type Project,
  presets,
  type Quest,
  type RecentProject,
  type Revision,
  type UiError,
  type UserSettings,
  type WorkingState,
} from './api';

type Section =
  | 'forge'
  | 'characters'
  | 'quests'
  | 'canon'
  | 'history'
  | 'project';
type Mode = 'character' | 'quest';
type ProjectStartMode = 'create' | 'open' | 'import';
type Field = { key: string; label: string; value: FieldValue; list?: boolean };
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
  if (payload.kind === 'character') {
    const character = raw as unknown as Character;
    return {
      envelope: character.envelope,
      character,
      dna: required(
        character.envelope.generation_provenance,
        'Character generation metadata',
      ).dna,
      field_origins: {},
      clue: undefined,
      briefing: undefined,
    };
  }
  const quest = raw as unknown as Quest;
  return {
    envelope: quest.envelope,
    quest,
    dna: required(
      quest.envelope.generation_provenance,
      'Quest generation metadata',
    ).dna,
    field_origins: {},
    clue: undefined,
    briefing: undefined,
  };
}
function isCharacter(payload: ForgePayload) {
  return payload.kind === 'character_candidate' || payload.kind === 'character';
}
function payloadTitle(payload: ForgePayload) {
  const value = payloadEntity(payload);
  if (isCharacter(payload))
    return required(value.character, 'Character payload').title;
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
  return payloadEntity(payload).dna.recipe.field_controls;
}
function fieldControl(payload: ForgePayload, field: string) {
  const suffix =
    field === 'name' ? '03-name' : `-${field.replaceAll('_', '-')}`;
  const key = Object.keys(fieldsOf(payload)).find(
    (value) => value === suffix || value.endsWith(suffix),
  );
  return key ? fieldsOf(payload)[key] : undefined;
}
function originOf(payload: ForgePayload, field: string) {
  const key = field === 'name' ? '03-name' : field.replaceAll('_', '-');
  const control = Object.keys(fieldsOf(payload)).find((value) =>
    value.endsWith(`-${key}`),
  );
  return payload.kind.endsWith('candidate') && control
    ? payloadEntity(payload).field_origins[control]
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
    if (project && ['characters', 'quests', 'canon'].includes(section))
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
    } else if (constraints.inspiration?.trim())
      activeConstraints.inspiration = constraints.inspiration.trim();
    void run(
      () =>
        mode === 'character'
          ? api.forgeCharacter(preset, numericSeed, axes, activeConstraints)
          : api.forgeQuest(preset, numericSeed, axes, activeConstraints),
      (value) => {
        setCandidate(value);
        setCanUndo(false);
        setCanRedo(false);
        setDirty(true);
        setSelectedRecord(null);
        setNotice(
          `${mode === 'character' ? 'Character' : 'Quest'} forged from seed ${numericSeed}.`,
        );
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
  const label = current
    ? (labels[payloadEntity(current).dna.recipe.universe_preset] ??
      payloadEntity(current).dna.recipe.universe_preset)
    : labels[preset];
  const canonRecords = records.filter(
    (record) => payloadEntity(record.entity).envelope.canon_state === 'canon',
  );

  const save = async () => {
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
                {(['character', 'quest'] as Mode[]).map((item) => (
                  <button
                    type="button"
                    key={item}
                    aria-pressed={mode === item}
                    className={mode === item ? 'selected' : ''}
                    onClick={() => setMode(item)}
                  >
                    {item === 'character' ? 'Character' : 'Quest'}
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
                {busy
                  ? 'Forging…'
                  : `Forge ${mode === 'character' ? 'Character' : 'Quest'}`}
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
                <div className="field-list">
                  {allFields.map((field) => {
                    const controlKey =
                      field.key === 'name'
                        ? '03-name'
                        : Object.keys(fieldsOf(current)).find((key) =>
                            key.endsWith(`-${field.key.replaceAll('_', '-')}`),
                          );
                    const control = controlKey
                      ? fieldsOf(current)[controlKey]
                      : undefined;
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
                            <textarea
                              aria-label={`Edit ${field.label}`}
                              value={editValue}
                              onChange={(event) =>
                                setEditValue(event.target.value)
                              }
                              onKeyDown={(event) => {
                                if (event.key === 'Escape') setEditing(null);
                              }}
                            />
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
                              <span className="icon-button-label">Reroll</span>
                            </button>
                            <button
                              type="button"
                              aria-label={`Edit ${field.label}`}
                              className="icon-button"
                              disabled={busy}
                              onClick={() => startEdit(field)}
                            >
                              ✎ <span className="icon-button-label">Edit</span>
                            </button>
                          </div>
                        )}
                      </article>
                    );
                  })}
                </div>
              )}
            </section>
            <aside className="context-panel">
              <section className="panel context-card">
                <p className="eyebrow">SESSION</p>
                <h2>{current ? 'Ready to shape' : 'A new beginning'}</h2>
                <p className="muted">
                  {current
                    ? `Seed ${payloadEntity(current).dna.recipe.root_seed} · ${label}`
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
                        {payloadEntity(current).dna.recipe.root_seed}
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
                      payloadEntity(current).envelope.canon_state === 'canon'
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
                  <small>Changes to saved work are kept when you save.</small>
                </section>
              )}
              <section className="panel context-card">
                <p className="eyebrow">PROJECT LIBRARY</p>
                <p className="library-count">
                  {records.filter((row) => isCharacter(row.entity)).length}{' '}
                  <span className="library-count-label">Characters</span>
                </p>
                <p className="library-count">
                  {records.filter((row) => !isCharacter(row.entity)).length}{' '}
                  <span className="library-count-label">Quests</span>
                </p>
                <button
                  type="button"
                  className="quiet full"
                  onClick={() =>
                    navigateToSection(
                      mode === 'character' ? 'characters' : 'quests',
                    )
                  }
                >
                  Browse {mode === 'character' ? 'Characters' : 'Quests'} →
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
          />
        )}
        {section === 'quests' && (
          <LibraryView
            title="Quests"
            records={records.filter((row) => !isCharacter(row.entity))}
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
            <DetailView record={selectedRecord} />
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
              <dd>{records.filter((r) => !isCharacter(r.entity)).length}</dd>
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
      canon: 'Canon',
      history: 'History',
      project: 'Project',
    } as const
  )[section];
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
    <p className="field-value">{field.value || 'No text added.'}</p>
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
  const character = isCharacter(record.entity);
  const summary = character
    ? required(entity.character, 'Character payload').archetype
    : required(entity.quest, 'Quest payload').objective;
  return (
    <article className="record-row">
      <button
        type="button"
        className={`record-main ${selected ? 'selected' : ''}`}
        aria-pressed={selected}
        onClick={() => onSelect(record)}
      >
        <span className="record-glyph">{character ? '♙' : '⚑'}</span>
        <span>
          <strong>{payloadTitle(record.entity)}</strong>
          <small>{summary}</small>
          <em>
            {labels[entity.dna.recipe.universe_preset] ??
              entity.dna.recipe.universe_preset}{' '}
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
}: {
  record: EntityRecord | null;
  revisions?: Revision[];
  onExport?: (record: EntityRecord, format: 'json' | 'markdown') => void;
}) {
  if (!record)
    return (
      <section className="detail-panel panel">
        <Empty text="Select a saved item to see the complete structured result." />
      </section>
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
        {labels[payloadEntity(record.entity).dna.recipe.universe_preset] ??
          payloadEntity(record.entity).dna.recipe.universe_preset}{' '}
        · seed {payloadEntity(record.entity).dna.recipe.root_seed}
      </p>
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
      />
    </div>
  );
}
