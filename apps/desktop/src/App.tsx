import { useCallback, useEffect, useState } from 'react';
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
  type Revision,
  type UiError,
} from './api';

type Section =
  | 'forge'
  | 'characters'
  | 'quests'
  | 'canon'
  | 'history'
  | 'project';
type Mode = 'character' | 'quest';
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
  const [projectPath, setProjectPath] = useState('');
  const [portablePath, setPortablePath] = useState('');
  const [importDestination, setImportDestination] = useState('');
  const [editing, setEditing] = useState<string | null>(null);
  const [editValue, setEditValue] = useState('');
  const [snapshot, setSnapshot] = useState<ForgePayload | null>(null);

  const refreshProject = useCallback(async () => {
    setLoading(true);
    try {
      const active = await api.project();
      setProject(active);
      if (active) {
        setProjectName(active.name);
        setProjectPath(active.path);
        const portableDefault = active.path.replace(
          /\.sqlite3$/i,
          '.forgeproject',
        );
        setPortablePath(
          portableDefault === active.path
            ? `${active.path}.forgeproject`
            : portableDefault,
        );
        setImportDestination(
          active.path.replace(/\.sqlite3$/i, '.imported.sqlite3'),
        );
      }
    } catch (reason) {
      setError(errorMessage(reason as UiError).message);
    } finally {
      setLoading(false);
    }
  }, []);
  useEffect(() => {
    void refreshProject();
  }, [refreshProject]);

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

  const run = async <T,>(
    operation: () => Promise<T>,
    onSuccess: (value: T) => void,
  ) => {
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
  };
  const createProject = () =>
    void run(
      () => api.createProject(projectPath, projectName),
      (value) => {
        setProject(value);
        setSection('forge');
        setCandidate(null);
        setDirty(false);
        setNotice('Project created and ready to forge.');
      },
    );
  const openProject = () =>
    void run(
      () => api.openProject(projectPath),
      (value) => {
        setProject(value);
        setSection('forge');
        setCandidate(null);
        setDirty(false);
        setSelectedRecord(null);
        setNotice('Project opened.');
      },
    );
  const exportProject = () =>
    void run(
      () => api.exportProject(portablePath),
      () => setNotice('Project exported successfully.'),
    );
  const createBackup = () =>
    void run(
      () => api.createBackup(portablePath),
      () => setNotice('Backup created successfully.'),
    );
  const importProject = () =>
    void run(
      () => api.importProject(portablePath, importDestination),
      (value) => {
        setProject(value);
        setProjectPath(value.path);
        setSection('forge');
        setCandidate(null);
        setDirty(false);
        setSelectedRecord(null);
        setNotice('Project imported and opened.');
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
      () => api.restoreBackup(portablePath),
      () => {
        setCandidate(null);
        setDirty(false);
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
    await run(
      () => api.getEntity(entityId(record.entity)),
      (fresh) => {
        if (fresh) {
          setSelectedRecord(fresh);
          setCandidate(fresh.entity);
          setDirty(false);
          setRevisions([]);
          void api
            .revisions(entityId(fresh.entity))
            .then(setRevisions)
            .catch(() => undefined);
        }
      },
    );
  };
  const promote = (record: EntityRecord) =>
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
          'Promoted to canon. The previous state is preserved as a revision.',
        );
        void api.revisions(entityId(updated.entity)).then(setRevisions);
      },
    );
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
    if (item.runId)
      void run(
        () => api.snapshot(item.runId),
        (value) => setSnapshot(value),
      );
  };
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
            Your work is stored locally in a project database you choose.
          </p>
          <label>
            Project name
            <input
              value={projectName}
              onChange={(event) => setProjectName(event.target.value)}
            />
          </label>
          <label>
            Project database path
            <input
              value={projectPath}
              onChange={(event) => setProjectPath(event.target.value)}
              placeholder="C:\\Worlds\\my-world.sqlite3"
            />
          </label>
          <div className="button-row">
            <button
              type="button"
              className="primary"
              disabled={busy || !projectName.trim() || !projectPath.trim()}
              onClick={createProject}
            >
              {busy ? 'Creating…' : 'Create project'}
            </button>
            <button
              type="button"
              disabled={busy || !projectPath.trim()}
              onClick={openProject}
            >
              Open project
            </button>
          </div>
          <p className="path-note">
            Choose a new <code>.sqlite3</code> path to create a project, or
            enter an existing project database path to open it.
          </p>
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
                setSection(id);
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
              onClick={() => setSection('project')}
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
              <div
                className="mode-switch"
                role="tablist"
                aria-label="Forge type"
              >
                {(['character', 'quest'] as Mode[]).map((item) => (
                  <button
                    type="button"
                    key={item}
                    role="tab"
                    aria-selected={mode === item}
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
              <details className="axis-details" open>
                <summary>
                  Creative axes <span>8 controls</span>
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
                <span aria-hidden="true">✦</span>
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
                    {candidate ? 'CURRENT CANDIDATE' : 'YOUR NEXT IDEA'}
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
                              <span>{locked ? 'Unlock' : 'Lock'}</span>
                            </button>
                            <button
                              type="button"
                              aria-label={`Reroll ${field.label}`}
                              className="icon-button"
                              disabled={locked || busy}
                              onClick={() =>
                                updateCandidate(
                                  () => api.reroll(field.key),
                                  `${field.label} rerolled.`,
                                )
                              }
                            >
                              ↻ <span>Reroll</span>
                            </button>
                            <button
                              type="button"
                              aria-label={`Edit ${field.label}`}
                              className="icon-button"
                              disabled={busy}
                              onClick={() => startEdit(field)}
                            >
                              ✎ <span>Edit</span>
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
                      <span>Preset</span>
                      <strong>{label}</strong>
                    </div>
                    <div>
                      <span>Seed</span>
                      <strong>
                        {payloadEntity(current).dna.recipe.root_seed}
                      </strong>
                    </div>
                    <div>
                      <span>Fields</span>
                      <strong>{allFields.length} structured details</strong>
                    </div>
                  </div>
                )}
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
                      ? 'Save changes'
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
                      setSection('history');
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
                  <span>Characters</span>
                </p>
                <p className="library-count">
                  {records.filter((row) => !isCharacter(row.entity)).length}{' '}
                  <span>Quests</span>
                </p>
                <button
                  type="button"
                  className="quiet full"
                  onClick={() =>
                    setSection(mode === 'character' ? 'characters' : 'quests')
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
                <Empty text="Nothing in canon yet. Promote a saved candidate when it feels right." />
              ) : (
                canonRecords.map((record) => (
                  <RecordRow
                    key={entityId(record.entity)}
                    record={record}
                    onSelect={selectRecord}
                    onFavorite={favorite}
                    onPromote={promote}
                  />
                ))
              )}
            </section>
            <DetailView record={selectedRecord} revisions={revisions} />
            {selectedRecord && <RevisionPanel revisions={revisions} />}
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
                <Empty text="Generation and revision activity will appear here." />
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
                      <strong>{friendlyAction(item.action)}</strong>
                      <small>
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
              <p className="eyebrow">REVISION SNAPSHOT</p>
              {snapshot ? (
                <DetailView record={{ entity: snapshot, writeVersion: 0 }} />
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
            <dl>
              <dt>Project database</dt>
              <dd>{project.path}</dd>
              <dt>Characters</dt>
              <dd>{records.filter((r) => isCharacter(r.entity)).length}</dd>
              <dt>Quests</dt>
              <dd>{records.filter((r) => !isCharacter(r.entity)).length}</dd>
              <dt>Created</dt>
              <dd>{dateLabel(project.createdAtUnixMs)}</dd>
            </dl>
            <div className="project-archive-actions">
              <h3>Portable project and backups</h3>
              <label htmlFor="portable-path">Project file path</label>
              <input
                id="portable-path"
                value={portablePath}
                onChange={(event) => setPortablePath(event.target.value)}
                placeholder="C:\\Projects\\MyWorld.forgeproject"
              />
              <div className="button-row">
                <button
                  type="button"
                  disabled={busy || !portablePath.trim()}
                  onClick={exportProject}
                >
                  Export Project
                </button>
                <button
                  type="button"
                  disabled={busy || !portablePath.trim()}
                  onClick={createBackup}
                >
                  Create Backup
                </button>
                <button
                  type="button"
                  disabled={busy || !portablePath.trim()}
                  onClick={restoreBackup}
                >
                  Restore Backup…
                </button>
              </div>
              <label htmlFor="import-destination">
                New project database path
              </label>
              <input
                id="import-destination"
                value={importDestination}
                onChange={(event) => setImportDestination(event.target.value)}
                placeholder="C:\\Projects\\ImportedWorld.sqlite3"
              />
              <button
                type="button"
                disabled={
                  busy || !portablePath.trim() || !importDestination.trim()
                }
                onClick={importProject}
              >
                Import Project…
              </button>
              {busy && <p role="status">Working with project file…</p>}
            </div>
            <button
              type="button"
              className="danger"
              onClick={() =>
                void run(api.closeProject, () => {
                  setProject(null);
                  setCandidate(null);
                  setDirty(false);
                  setSelectedRecord(null);
                  setProjectPath('');
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
  return action
    .replaceAll('_', ' ')
    .replace(/\b\w/g, (letter) => letter.toUpperCase());
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
function Empty({ text }: { text: string }) {
  return (
    <div className="empty-state">
      <span aria-hidden="true">✦</span>
      <p>{text}</p>
    </div>
  );
}
function RecordRow({
  record,
  onSelect,
  onFavorite,
  onPromote,
}: {
  record: EntityRecord;
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
        className="record-main"
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
function RevisionPanel({ revisions }: { revisions: Revision[] }) {
  return (
    <section className="revision-panel panel">
      <p className="eyebrow">CANON REVISIONS</p>
      {revisions.length ? (
        revisions.map((revision) => (
          <article key={revision.revisionNumber}>
            <strong>Revision {revision.revisionNumber}</strong>
            <small>
              {revision.reason} · {dateLabel(revision.createdAtUnixMs)}
            </small>
          </article>
        ))
      ) : (
        <p className="muted">No canon revisions yet.</p>
      )}
    </section>
  );
}
function LibraryView({
  title,
  records,
  loading,
  empty,
  onSelect,
  onFavorite,
  onPromote,
  onExport,
}: {
  title: string;
  records: EntityRecord[];
  loading: boolean;
  empty: string;
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
          <Empty text={empty} />
        ) : (
          records.map((record) => (
            <RecordRow
              key={entityId(record.entity)}
              record={record}
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
