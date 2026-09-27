import {
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { App } from '../src/App';
import {
  api,
  type CharacterCandidate,
  type EntityRecord,
  type ForgePayload,
  type Project,
  type Revision,
} from '../src/api';

const clickNamedButton = (name: string, index: number) => {
  const button = screen.getAllByRole('button', { name })[index];
  if (!button) throw new Error(`Expected button ${name} at index ${index}`);
  fireEvent.click(button);
};

vi.mock('../src/api', async (load) => {
  const actual = await load<typeof import('../src/api')>();
  return {
    ...actual,
    api: Object.fromEntries(
      Object.keys(actual.api).map((key) => [key, vi.fn()]),
    ),
  };
});

const project: Project = {
  id: 'project-id',
  name: 'Ashen Coast',
  path: 'C:\\Worlds\\ashen-coast.sqlite3',
  createdAtUnixMs: 1,
  updatedAtUnixMs: 1,
};
const dna = {
  recipe: {
    root_seed: '000000000000002a',
    universe_preset: 'dark-fantasy',
    creative_axes: {
      levels: {
        bleak: 50,
        mythic: 50,
        chaos: 50,
        epic: 50,
        ambiguity: 50,
        surreal: 50,
        technology: 50,
        mysticism: 50,
      },
    },
    constraints: {},
    field_controls: {
      '03-name': { nonce: 0, locked: false },
      '04-title': { nonce: 0, locked: false },
      '01-archetype': { nonce: 0, locked: false },
      '02-role': { nonce: 0, locked: false },
      '05-appearance': { nonce: 0, locked: false },
      '06-personality': { nonce: 0, locked: false },
      '07-desire': { nonce: 0, locked: false },
      '08-fear': { nonce: 0, locked: false },
      '09-flaw': { nonce: 0, locked: false },
      '10-secret': { nonce: 0, locked: false },
      '11-skills': { nonce: 0, locked: false },
      '12-abilities': { nonce: 0, locked: false },
      '13-faction-hint': { nonce: 0, locked: false },
      '14-signature-gear': { nonce: 0, locked: false },
      '15-relationship-hook': { nonce: 0, locked: false },
      '16-backstory-hook': { nonce: 0, locked: false },
      '17-story-hooks': { nonce: 0, locked: false },
      '16-clue': { nonce: 0, locked: false },
      '17-briefing': { nonce: 0, locked: false },
    },
  },
};
const envelope = (id: string, type: 'character' | 'quest' = 'character') => ({
  id,
  project_id: project.id,
  entity_type: type,
  name: type === 'character' ? 'Mara Vale' : 'The Bell Below',
  schema_version: 1,
  canon_state: 'candidate' as const,
  origin: 'generated',
  favorite: false,
  created_at_unix_ms: 1,
  updated_at_unix_ms: 1,
  tags: ['preset:dark-fantasy'],
  generation_provenance: { dna, action: 'forge' },
});
const character: ForgePayload = {
  kind: 'character_candidate',
  value: {
    dna,
    field_origins: { '03-name': 'generated', '04-title': 'generated' },
    character: {
      envelope: envelope('character-1'),
      aliases: [],
      title: 'Mara Vale',
      archetype: 'Lantern keeper',
      role: 'Guide',
      appearance: 'Coal dust on a silver coat.',
      faction_hint: 'The Night Registry',
      signature_gear: 'A key that warms near lies',
      personality: ['Patient', 'Wary'],
      desire: 'Find the missing district',
      fear: 'Being forgotten',
      flaw: 'Withholds the truth',
      secret: 'She has already been there',
      skills: ['Wayfinding', 'Negotiation'],
      abilities: ['Hears doors remember'],
      relationship_hook: 'Her apprentice knows the old route',
      backstory_hook: 'She filed her own disappearance',
      story_hooks: [
        'The road returns',
        'The registry is blank',
        'A bell answers',
      ],
    },
  },
};
const quest: ForgePayload = {
  kind: 'quest_candidate',
  value: {
    dna,
    field_origins: {},
    clue: 'A route is stamped on the wrong side of the map.',
    briefing:
      'The night clerk has filed a report for a street removed from every city plan.',
    quest: {
      envelope: envelope('quest-1', 'quest'),
      quest_type: 'Investigation',
      premise: 'A city record now lists an impossible street.',
      patron: 'The municipal night clerk',
      objective: 'Find who is issuing valid addresses from the missing street.',
      stakes: 'Residents may be erased from public services.',
      opposition: 'A registrar who remembers a different city.',
      complications: ['The street appears only on utility notices.'],
      twist: 'The filed reports are being delivered from tomorrow.',
      moral_pressure:
        'Publishing the truth would invalidate the residents’ homes.',
      locations: ['Records annex'],
      rewards: ['A signed address that exists nowhere'],
      failure_consequences: ['The street becomes officially forgotten'],
      follow_up_hooks: ['A new stop appears on the night bus'],
    },
  },
};
const record = (entity: ForgePayload, writeVersion = 1): EntityRecord => ({
  entity,
  writeVersion,
});

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(api.project).mockResolvedValue(null);
  vi.mocked(api.defaultProjectDirectory).mockResolvedValue(
    'C:\\Users\\Test\\Documents\\Character Forge',
  );
  vi.mocked(api.recentProjects).mockResolvedValue([]);
  vi.mocked(api.forgetRecentProject).mockResolvedValue(undefined);
  vi.mocked(api.createProject).mockResolvedValue(project);
  vi.mocked(api.openProject).mockResolvedValue(project);
  vi.mocked(api.closeProject).mockResolvedValue(undefined);
  vi.mocked(api.exportProject).mockResolvedValue(undefined);
  vi.mocked(api.createBackup).mockResolvedValue(undefined);
  vi.mocked(api.importProject).mockResolvedValue(project);
  vi.mocked(api.restoreBackup).mockResolvedValue(undefined);
  vi.mocked(api.exportEntity).mockResolvedValue(undefined);
  vi.mocked(api.listEntities).mockResolvedValue([]);
  vi.mocked(api.getEntity).mockImplementation(async (id) =>
    id === 'character-1' ? record(character) : record(quest),
  );
  vi.mocked(api.forgeCharacter).mockResolvedValue(character);
  vi.mocked(api.forgeQuest).mockResolvedValue(quest);
  vi.mocked(api.setLock).mockResolvedValue(character);
  vi.mocked(api.reroll).mockResolvedValue(character);
  vi.mocked(api.rerollUnlocked).mockResolvedValue(character);
  vi.mocked(api.editField).mockResolvedValue(character);
  vi.mocked(api.saveCandidate).mockResolvedValue(record(character));
  vi.mocked(api.favorite).mockResolvedValue(record(character));
  vi.mocked(api.promote).mockResolvedValue(
    record({
      ...character,
      value: {
        ...character.value,
        character: {
          ...character.value.character,
          envelope: {
            ...character.value.character.envelope,
            canon_state: 'canon',
          },
        },
      },
    }),
  );
  vi.mocked(api.history).mockResolvedValue([]);
  vi.mocked(api.snapshot).mockResolvedValue(character);
  vi.mocked(api.revisions).mockResolvedValue([]);
  vi.mocked(api.userSettings).mockResolvedValue({
    version: 1,
    reducedMotion: false,
  });
  vi.mocked(api.updateUserSettings).mockImplementation(
    async (settings) => settings,
  );
  vi.mocked(api.undo).mockResolvedValue({
    entity: character,
    canUndo: false,
    canRedo: true,
  });
  vi.mocked(api.redo).mockResolvedValue({
    entity: character,
    canUndo: true,
    canRedo: false,
  });
  vi.mocked(api.restoreHistoryToWorkingCopy).mockResolvedValue({
    entity: character,
    canUndo: false,
    canRedo: false,
  });
  vi.mocked(api.loadCanonRevisionWorkingCopy).mockResolvedValue({
    entity: character,
    canUndo: false,
    canRedo: false,
  });
  vi.mocked(api.reviseCanon).mockResolvedValue(record(character));
});

async function openWorkspace(items: EntityRecord[] = []) {
  vi.mocked(api.project).mockResolvedValue(project);
  vi.mocked(api.listEntities).mockResolvedValue(items);
  render(<App />);
  await screen.findByRole('heading', { name: 'Forge' });
}
async function openProjectSettings() {
  await openWorkspace();
  fireEvent.click(
    within(
      screen.getByRole('navigation', { name: 'Main navigation' }),
    ).getByRole('button', { name: 'Project' }),
  );
  await screen.findByRole('heading', { name: 'Move a project between places' });
}
async function forgeCharacterResult() {
  await openWorkspace();
  fireEvent.click(screen.getByRole('button', { name: 'Forge Character' }));
  await screen.findByText('Mara Vale', { selector: 'h2' });
}

describe('desktop creative workflows', () => {
  it('exports the project and confirms success', async () => {
    await openProjectSettings();
    fireEvent.change(screen.getByLabelText('Export project to'), {
      target: { value: 'C:\\Worlds\\backup.forgeproject' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Export Project' }));
    await waitFor(() =>
      expect(api.exportProject).toHaveBeenCalledWith(
        'C:\\Worlds\\backup.forgeproject',
      ),
    );
    expect(
      await screen.findByText('Project exported successfully.'),
    ).toBeInTheDocument();
  });

  it('shows archive validation errors without losing the open project', async () => {
    vi.mocked(api.exportProject).mockRejectedValue({ kind: 'unsafe_archive' });
    await openProjectSettings();
    fireEvent.change(screen.getByLabelText('Export project to'), {
      target: { value: 'unsafe.forgeproject' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Export Project' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('unsafe path');
    expect(
      screen.getByRole('heading', { name: project.name }),
    ).toBeInTheDocument();
  });

  it('creates a local backup with success feedback', async () => {
    await openProjectSettings();
    fireEvent.change(screen.getByLabelText('Create backup at'), {
      target: { value: 'backup.forgeproject' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Create Backup' }));
    await waitFor(() =>
      expect(api.createBackup).toHaveBeenCalledWith('backup.forgeproject'),
    );
    expect(
      await screen.findByText('Backup created successfully.'),
    ).toBeInTheDocument();
  });

  it('disables archive actions while an operation is running', async () => {
    let finish!: () => void;
    vi.mocked(api.exportProject).mockReturnValue(
      new Promise<void>((resolve) => {
        finish = resolve;
      }),
    );
    await openProjectSettings();
    fireEvent.change(screen.getByLabelText('Export project to'), {
      target: { value: 'working.forgeproject' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Export Project' }));
    expect(await screen.findByRole('status')).toHaveTextContent(
      'Working with project file',
    );
    expect(
      screen.getByRole('button', { name: 'Creating backup…' }),
    ).toBeDisabled();
    finish();
    await waitFor(() =>
      expect(
        screen.getByRole('button', { name: 'Create Backup' }),
      ).toBeEnabled(),
    );
    expect(
      screen.queryByText('Working with project file…'),
    ).not.toBeInTheDocument();
  });

  it('imports to a new path and opens the imported project', async () => {
    await openProjectSettings();
    fireEvent.change(screen.getByLabelText('Import portable project'), {
      target: { value: 'world.forgeproject' },
    });
    fireEvent.change(screen.getByLabelText('Import into folder'), {
      target: { value: 'C:\\Worlds' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Import Project' }));
    await waitFor(() =>
      expect(api.importProject).toHaveBeenCalledWith(
        'world.forgeproject',
        'C:\\Worlds',
      ),
    );
    expect(
      await screen.findByRole('heading', { name: 'Forge' }),
    ).toBeInTheDocument();
    expect(
      await screen.findByText('Project imported and opened.'),
    ).toBeInTheDocument();
  });

  it('requires restore confirmation and preserves the current view after failure', async () => {
    vi.stubGlobal(
      'confirm',
      vi.fn(() => true),
    );
    vi.mocked(api.restoreBackup).mockRejectedValue({ kind: 'invalid_archive' });
    await openProjectSettings();
    fireEvent.change(screen.getByLabelText('Restore from backup file'), {
      target: { value: 'backup.forgeproject' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Restore Backup…' }));
    await waitFor(() =>
      expect(api.restoreBackup).toHaveBeenCalledWith('backup.forgeproject'),
    );
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'not a valid Forge project',
    );
    expect(
      screen.getByRole('heading', { name: project.name }),
    ).toBeInTheDocument();
    vi.unstubAllGlobals();
  });

  it('shows a project creation and open screen without an active project', async () => {
    render(<App />);
    expect(
      await screen.findByRole('heading', { name: 'Begin with a project' }),
    ).toBeInTheDocument();
    expect(
      screen.getAllByRole('button', { name: 'Create Project' })[0],
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Open Project' }),
    ).toBeInTheDocument();
  });

  it('creates a project from its name and containing folder', async () => {
    render(<App />);
    await screen.findByRole('heading', { name: 'Begin with a project' });
    fireEvent.change(screen.getByLabelText('Project name'), {
      target: { value: 'My New World' },
    });
    fireEvent.change(screen.getByLabelText('Project location'), {
      target: { value: 'C:\\Worlds' },
    });
    clickNamedButton('Create Project', 1);
    await waitFor(() =>
      expect(api.createProject).toHaveBeenCalledWith(
        'C:\\Worlds',
        'My New World',
      ),
    );
    expect(
      await screen.findByRole('heading', { name: 'Forge' }),
    ).toBeInTheDocument();
  });

  it('opens only a live project file from the Open Project workflow', async () => {
    render(<App />);
    await screen.findByRole('heading', { name: 'Begin with a project' });
    clickNamedButton('Open Project', 0);
    fireEvent.change(screen.getByLabelText('Existing project file'), {
      target: { value: 'C:\\Worlds\\Ashen Coast.sqlite3' },
    });
    clickNamedButton('Open Project', 1);
    await waitFor(() =>
      expect(api.openProject).toHaveBeenCalledWith(
        'C:\\Worlds\\Ashen Coast.sqlite3',
      ),
    );
  });

  it('keeps portable import separate from opening a live project', async () => {
    render(<App />);
    await screen.findByRole('heading', { name: 'Begin with a project' });
    clickNamedButton('Import Project', 0);
    fireEvent.change(screen.getByLabelText('Portable project file'), {
      target: { value: 'C:\\Worlds\\Moved.forgeproject' },
    });
    clickNamedButton('Import Project', 1);
    await waitFor(() =>
      expect(api.importProject).toHaveBeenCalledWith(
        'C:\\Worlds\\Moved.forgeproject',
        'C:\\Users\\Test\\Documents\\Character Forge',
      ),
    );
    expect(api.openProject).not.toHaveBeenCalled();
  });

  it('opens a recent project and allows missing entries to be forgotten', async () => {
    vi.mocked(api.recentProjects).mockResolvedValue([
      {
        name: 'Ashen Coast',
        path: project.path,
        lastOpenedAtUnixMs: 1234,
        exists: true,
      },
      {
        name: 'Old World',
        path: 'C:\\Worlds\\Old.sqlite3',
        lastOpenedAtUnixMs: 1000,
        exists: false,
      },
    ]);
    render(<App />);
    await screen.findByRole('heading', { name: 'Recent Projects' });
    fireEvent.click(screen.getByRole('button', { name: /^Ashen Coast/ }));
    await waitFor(() =>
      expect(api.openProject).toHaveBeenCalledWith(project.path),
    );
  });

  it('explains a stale recent project and removes only its recent entry', async () => {
    vi.mocked(api.recentProjects).mockResolvedValue([
      {
        name: 'Old World',
        path: 'C:\\Worlds\\Old.sqlite3',
        lastOpenedAtUnixMs: 1000,
        exists: false,
      },
    ]);
    render(<App />);
    const openRecent = await screen.findByRole('button', {
      name: /Old World File not found at this location/,
    });
    fireEvent.click(openRecent);
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'couldn’t be found at its last known location',
    );
    fireEvent.click(
      screen.getByRole('button', {
        name: 'Forget Old World from Recent Projects',
      }),
    );
    await waitFor(() =>
      expect(api.forgetRecentProject).toHaveBeenCalledWith(
        'C:\\Worlds\\Old.sqlite3',
      ),
    );
    expect(await screen.findByRole('status')).toHaveTextContent(
      'The project file was not deleted.',
    );
  });

  it('opens the workspace with active project context', async () => {
    await openWorkspace();
    expect(screen.getAllByText('Ashen Coast')).toHaveLength(2);
    expect(
      screen.getByRole('navigation', { name: 'Main navigation' }),
    ).toBeInTheDocument();
  });

  it('switches between Character and Quest Forge modes', async () => {
    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Quest' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Quest' }));
    await screen.findByText('The Bell Below', { selector: 'h2' });
    expect(api.forgeQuest).toHaveBeenCalled();
  });

  it('selects a canonical preset', async () => {
    await openWorkspace();
    fireEvent.change(screen.getByLabelText('Universe'), {
      target: { value: 'occult' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Forge Character' }));
    await waitFor(() =>
      expect(api.forgeCharacter).toHaveBeenCalledWith(
        'occult',
        42,
        expect.any(Object),
        {},
      ),
    );
  });

  it('updates a creative axis accessibly', async () => {
    await openWorkspace();
    fireEvent.change(screen.getByRole('slider', { name: 'Bleakness' }), {
      target: { value: '82' },
    });
    expect(screen.getByText('82')).toBeInTheDocument();
  });

  it('renders the complete generated Character result', async () => {
    await forgeCharacterResult();
    expect(screen.getByText('Lantern keeper')).toBeInTheDocument();
    expect(
      screen.getByText('She filed her own disappearance'),
    ).toBeInTheDocument();
  });

  it('shows structured generation errors in an alert', async () => {
    vi.mocked(api.forgeCharacter).mockRejectedValue({
      kind: 'validation',
      message: 'invalid preset',
    });
    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Forge Character' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'invalid preset',
    );
  });

  it('communicates locked field state and submits the real lock action', async () => {
    vi.mocked(api.setLock).mockResolvedValue({
      ...character,
      value: {
        ...character.value,
        dna: {
          ...dna,
          recipe: {
            ...dna.recipe,
            field_controls: {
              ...dna.recipe.field_controls,
              '08-fear': { nonce: 0, locked: true },
            },
          },
        },
      },
    });
    await forgeCharacterResult();
    fireEvent.click(screen.getByRole('button', { name: 'Lock Fear' }));
    await waitFor(() => expect(api.setLock).toHaveBeenCalledWith('fear', true));
    expect(
      await screen.findByText('Fear locked. Rerolls will leave it unchanged.'),
    ).toBeInTheDocument();
  });

  it('shows the canonical Name lock and renders preserved values from a reroll response', async () => {
    const source = character.value as CharacterCandidate;
    const withNameLock = (
      value: CharacterCandidate,
      locked: boolean,
      archetype = value.character.archetype,
    ): ForgePayload => ({
      kind: 'character_candidate',
      value: {
        ...value,
        dna: {
          ...value.dna,
          recipe: {
            ...value.dna.recipe,
            field_controls: {
              ...value.dna.recipe.field_controls,
              '03-name': { nonce: 0, locked },
            },
          },
        },
        character: {
          ...value.character,
          archetype,
          envelope: { ...value.character.envelope, name: 'Mara Vale' },
        },
      },
    });
    const locked = withNameLock(source, true);
    const rerolled = withNameLock(source, true, 'A different archetype');
    vi.mocked(api.setLock).mockResolvedValue(locked);
    vi.mocked(api.reroll).mockResolvedValue(rerolled);

    await forgeCharacterResult();
    fireEvent.click(screen.getByRole('button', { name: 'Lock Name' }));
    await waitFor(() => expect(api.setLock).toHaveBeenCalledWith('name', true));
    expect(
      await screen.findByRole('button', { name: 'Unlock Name' }),
    ).toHaveAttribute('aria-pressed', 'true');

    fireEvent.click(screen.getByRole('button', { name: 'Reroll Archetype' }));
    await waitFor(() => expect(api.reroll).toHaveBeenCalledWith('archetype'));
    expect(
      await screen.findByText('A different archetype'),
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Unlock Name' }),
    ).toBeInTheDocument();
    expect(screen.getAllByText('Mara Vale').length).toBeGreaterThan(0);
  });

  it('rerolls a single requested field', async () => {
    await forgeCharacterResult();
    fireEvent.click(screen.getByRole('button', { name: 'Reroll Name' }));
    await waitFor(() => expect(api.reroll).toHaveBeenCalledWith('name'));
  });

  it('supports edit, confirm, and cancel actions', async () => {
    await forgeCharacterResult();
    fireEvent.click(screen.getByRole('button', { name: 'Edit Secret' }));
    const editor = screen.getByRole('textbox', { name: 'Edit Secret' });
    fireEvent.change(editor, {
      target: { value: 'A secret written by the player.' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save edit' }));
    await waitFor(() =>
      expect(api.editField).toHaveBeenCalledWith(
        'secret',
        'A secret written by the player.',
      ),
    );
    fireEvent.click(screen.getByRole('button', { name: 'Edit Fear' }));
    fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(
      screen.queryByRole('textbox', { name: 'Edit Fear' }),
    ).not.toBeInTheDocument();
  });

  it('saves the candidate and reflects persisted status', async () => {
    await forgeCharacterResult();
    fireEvent.click(screen.getByRole('button', { name: 'Save candidate' }));
    await waitFor(() => expect(api.saveCandidate).toHaveBeenCalled());
    expect(
      await screen.findByText('Saved to this project.'),
    ).toBeInTheDocument();
  });

  it('renders Characters from project entities and opens a detail', async () => {
    await openWorkspace([record(character)]);
    fireEvent.click(screen.getByRole('button', { name: /^Characters/ }));
    expect(
      await screen.findByText('Mara Vale', { selector: 'strong' }),
    ).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: /Mara Vale/ }));
    await waitFor(() =>
      expect(api.getEntity).toHaveBeenCalledWith('character-1'),
    );
    expect(await screen.findByText('Withholds the truth')).toBeInTheDocument();
  });

  it('exports a saved entity in structured JSON format', async () => {
    vi.stubGlobal(
      'prompt',
      vi.fn(() => 'Mara Vale.json'),
    );
    await openWorkspace([record(character)]);
    fireEvent.click(screen.getByRole('button', { name: /^Characters/ }));
    fireEvent.click(await screen.findByRole('button', { name: /Mara Vale/ }));
    fireEvent.click(await screen.findByRole('button', { name: 'Export JSON' }));
    await waitFor(() =>
      expect(api.exportEntity).toHaveBeenCalledWith(
        'character-1',
        'Mara Vale.json',
        'json',
      ),
    );
    expect(await screen.findByText('JSON export saved.')).toBeInTheDocument();
    vi.unstubAllGlobals();
  });

  it('renders persisted Quests in their library', async () => {
    await openWorkspace([record(quest)]);
    fireEvent.click(screen.getByRole('button', { name: 'Quests' }));
    expect(
      await screen.findByText('The Bell Below', { selector: 'strong' }),
    ).toBeInTheDocument();
    expect(
      screen.getByText(
        'Find who is issuing valid addresses from the missing street.',
      ),
    ).toBeInTheDocument();
  });

  it('guides empty Character and Quest libraries back to the matching Forge mode', async () => {
    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Characters' }));
    fireEvent.click(
      await screen.findByRole('button', { name: 'Forge a Character' }),
    );
    expect(screen.getByRole('button', { name: 'Character' })).toHaveAttribute(
      'aria-pressed',
      'true',
    );
    fireEvent.click(screen.getByRole('button', { name: 'Quests' }));
    fireEvent.click(
      await screen.findByRole('button', { name: 'Forge a Quest' }),
    );
    expect(screen.getByRole('button', { name: 'Quest' })).toHaveAttribute(
      'aria-pressed',
      'true',
    );
  });

  it('promotes a saved candidate through an intentional action', async () => {
    vi.stubGlobal(
      'confirm',
      vi.fn(() => true),
    );
    await openWorkspace([record(character)]);
    fireEvent.click(screen.getByRole('button', { name: /^Characters/ }));
    fireEvent.click(await screen.findByRole('button', { name: /Mara Vale/ }));
    await waitFor(() => expect(api.getEntity).toHaveBeenCalled());
    fireEvent.click(screen.getByRole('button', { name: 'Promote' }));
    expect(window.confirm).toHaveBeenCalledWith(
      expect.stringContaining('established project material'),
    );
    await waitFor(() =>
      expect(api.promote).toHaveBeenCalledWith('character-1', 1),
    );
    vi.unstubAllGlobals();
  });

  it('shows history entries and loads their older snapshot', async () => {
    vi.mocked(api.history).mockResolvedValue([
      {
        runId: 'run-1',
        entityId: 'character-1',
        action: 'manual_edit',
        createdAtUnixMs: 1234,
      },
    ]);
    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'History' }));
    fireEvent.click(
      await screen.findByRole('button', { name: /Edited a result/ }),
    );
    await waitFor(() => expect(api.snapshot).toHaveBeenCalledWith('run-1'));
    expect(
      await screen.findByRole('button', { name: 'Use as working copy' }),
    ).toBeInTheDocument();
  });

  it('exposes session undo and redo as separate from persisted save', async () => {
    vi.mocked(api.reroll).mockResolvedValue({
      ...character,
      value: {
        ...character.value,
        character: { ...character.value.character, title: 'Rerolled title' },
      },
    });
    await forgeCharacterResult();
    expect(
      screen.getByRole('button', { name: 'Undo last session change' }),
    ).toBeDisabled();
    fireEvent.click(screen.getByRole('button', { name: 'Reroll Name' }));
    fireEvent.click(
      await screen.findByRole('button', { name: 'Undo last session change' }),
    );
    await waitFor(() => expect(api.undo).toHaveBeenCalled());
    expect(api.saveCandidate).not.toHaveBeenCalled();
  });

  it('uses an inspected history snapshot as a new working copy', async () => {
    vi.mocked(api.history).mockResolvedValue([
      {
        runId: 'run-restore',
        entityId: 'character-1',
        action: 'manual_edit',
        createdAtUnixMs: 1234,
      },
    ]);
    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'History' }));
    fireEvent.click(
      await screen.findByRole('button', { name: /Edited a result/ }),
    );
    fireEvent.click(
      await screen.findByRole('button', { name: 'Use as working copy' }),
    );
    await waitFor(() =>
      expect(api.restoreHistoryToWorkingCopy).toHaveBeenCalledWith(
        'run-restore',
      ),
    );
    expect(
      await screen.findByText(/History snapshot opened as a working copy/),
    ).toBeInTheDocument();
  });

  it('inspects structured prior canon revisions and offers a working-copy action', async () => {
    const canon = record({
      ...character,
      value: {
        ...character.value,
        character: {
          ...character.value.character,
          envelope: {
            ...character.value.character.envelope,
            canon_state: 'canon',
          },
        },
      },
    });
    const prior: Revision = {
      revisionNumber: 1,
      reason: 'promoted to canon',
      entity: character,
      createdAtUnixMs: 1234,
    };
    vi.mocked(api.listEntities).mockResolvedValue([canon]);
    vi.mocked(api.getEntity).mockResolvedValue(canon);
    vi.mocked(api.revisions).mockResolvedValue([prior]);
    await openWorkspace([canon]);
    fireEvent.click(screen.getByRole('button', { name: 'Canon' }));
    fireEvent.click(await screen.findByRole('button', { name: /Mara Vale/ }));
    fireEvent.click(await screen.findByRole('button', { name: 'Inspect' }));
    expect(
      screen.getAllByText('READ-ONLY CANON REVISION').length,
    ).toBeGreaterThan(0);
    fireEvent.click(
      screen.getByRole('button', { name: 'Use as working copy' }),
    );
    await waitFor(() =>
      expect(api.loadCanonRevisionWorkingCopy).toHaveBeenCalledWith(
        'character-1',
        1,
      ),
    );
  });

  it('opens persisted application settings and saves reduced motion preference', async () => {
    await openProjectSettings();
    fireEvent.click(screen.getByRole('checkbox', { name: 'Reduce motion' }));
    await waitFor(() =>
      expect(api.updateUserSettings).toHaveBeenCalledWith({
        version: 1,
        reducedMotion: true,
      }),
    );
  });

  it('explains stale-write conflicts and requires reload', async () => {
    vi.mocked(api.reroll).mockRejectedValue({
      kind: 'stale_write',
      expected: 1,
      actual: 2,
    });
    await forgeCharacterResult();
    fireEvent.click(screen.getByRole('button', { name: 'Reroll Name' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'changed since you opened it',
    );
  });
});
