import {
  cleanup,
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
  type Character,
  type CharacterCandidate,
  type EntityRecord,
  type FactionCandidate,
  type ForgePayload,
  type ForgeResultBundle,
  type ItemCandidate,
  type LocationCandidate,
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
    kind: 'character' as const,
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
const envelope = (
  id: string,
  type: 'character' | 'quest' | 'location' | 'faction' | 'item' = 'character',
) => ({
  id,
  project_id: project.id,
  entity_type: type,
  name:
    type === 'character'
      ? 'Mara Vale'
      : type === 'location'
        ? 'North Bell Lodge'
        : type === 'faction'
          ? 'The Ashen Concord'
          : 'The Bell Below',
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
const manualCharacter: ForgePayload = {
  kind: 'character',
  value: {
    ...((character.value as CharacterCandidate).character as Character),
    envelope: {
      ...envelope('character-manual'),
      origin: 'manual',
      generation_provenance: null,
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
const locationDna = {
  recipe: {
    ...dna.recipe,
    universe_preset: 'dark-fantasy',
    field_controls: {
      '01-location-profile': { nonce: 0, locked: false },
      '02-name': { nonce: 0, locked: false },
      '03-visual-identity': { nonce: 0, locked: false },
      '04-atmosphere': { nonce: 0, locked: false },
      '05-purpose': { nonce: 0, locked: false },
      '06-summary': { nonce: 0, locked: false },
      '07-history': { nonce: 0, locked: false },
      '08-inhabitants-summary': { nonce: 0, locked: false },
      '09-notable-features': { nonce: 0, locked: false },
      '10-danger': { nonce: 0, locked: false },
      '11-secret': { nonce: 0, locked: false },
      '12-story-hooks': { nonce: 0, locked: false },
    },
  },
};
const locationCandidate: LocationCandidate = {
  dna: locationDna,
  field_origins: Object.fromEntries(
    Object.keys(locationDna.recipe.field_controls).map((key) => [
      key,
      'generated',
    ]),
  ),
  location: {
    envelope: {
      ...envelope('location-1', 'location'),
      generation_provenance: { dna: locationDna, action: 'forge' },
    },
    location_profile: 'settlement',
    summary: 'A frontier settlement beside the north road.',
    visual_identity: 'Pale stone walls surround the watch lodge.',
    atmosphere: 'A quiet watchfulness settles over the road.',
    purpose: 'It shelters travelers and maintains the warning bell.',
    history: 'The lodge grew around an old road marker.',
    inhabitants_summary: 'Road wardens and local families live here.',
    notable_features: ['North bell tower', 'Road marker', 'Covered well'],
    danger: 'The eastern road becomes unsafe after dusk.',
    secret: 'The bell rings before anyone touches it.',
    story_hooks: ['A missing patrol', 'An unmarked road', 'A bell at noon'],
  },
};
const locationPayload: ForgePayload = {
  kind: 'location_candidate',
  value: locationCandidate,
};
const factionDna = {
  recipe: {
    ...dna.recipe,
    kind: 'faction' as const,
    field_controls: {
      '01-faction-profile': { nonce: 0, locked: false },
      '02-name': { nonce: 0, locked: false },
      '03-identity': { nonce: 0, locked: false },
      '04-ideology': { nonce: 0, locked: false },
      '05-primary-goal': { nonce: 0, locked: false },
      '06-methods': { nonce: 0, locked: false },
      '07-culture': { nonce: 0, locked: false },
      '08-resources': { nonce: 0, locked: false },
      '09-public-face': { nonce: 0, locked: false },
      '10-hidden-truth': { nonce: 0, locked: false },
      '11-symbols': { nonce: 0, locked: false },
      '12-reputation': { nonce: 0, locked: false },
      '13-story-hooks': { nonce: 0, locked: false },
    },
  },
};
const factionCandidate: FactionCandidate = {
  dna: factionDna,
  field_origins: Object.fromEntries(
    Object.keys(factionDna.recipe.field_controls).map((key) => [
      key,
      'generated',
    ]),
  ),
  faction: {
    envelope: {
      ...envelope('faction-1', 'faction'),
      name: 'The Ashen Concord',
      generation_provenance: { dna: factionDna, action: 'forge' },
    },
    faction_profile: 'guild',
    identity: 'A guild tending the old border beacons.',
    ideology: 'Every road deserves a witness.',
    primary_goal: 'Keep the winter passage open.',
    methods: 'They maintain beacons and escort supply wagons.',
    culture: 'Members mark each journey on a shared map.',
    resources: 'They control the beacon stations and local stores.',
    public_face: 'A practical guild of guides and repairers.',
    hidden_truth: 'Its oldest route records omit one vanished village.',
    symbols: 'A black lantern over a white road.',
    reputation: 'Reliable in storms, secretive about old maps.',
    story_hooks: [
      'A beacon is lit from inside.',
      'A route vanishes.',
      'A courier brings a blank map.',
    ],
  },
};
const factionPayload: ForgePayload = {
  kind: 'faction_candidate',
  value: factionCandidate,
};
const itemDna = {
  recipe: {
    ...dna.recipe,
    kind: 'item' as const,
    field_controls: {
      '01-item-profile': { nonce: 0, locked: false },
      '02-name': { nonce: 0, locked: false },
      '03-appearance': { nonce: 0, locked: false },
      '04-purpose': { nonce: 0, locked: false },
      '05-properties': { nonce: 0, locked: false },
      '06-origin': { nonce: 0, locked: false },
      '07-history': { nonce: 0, locked: false },
      '08-power-or-function': { nonce: 0, locked: false },
      '09-cost-or-drawback': { nonce: 0, locked: false },
      '10-rarity': { nonce: 0, locked: false },
      '11-condition': { nonce: 0, locked: false },
      '12-secret': { nonce: 0, locked: false },
      '13-story-hooks': { nonce: 0, locked: false },
    },
  },
};
const itemCandidate: ItemCandidate = {
  dna: itemDna,
  field_origins: Object.fromEntries(
    Object.keys(itemDna.recipe.field_controls).map((key) => [key, 'generated']),
  ),
  item: {
    envelope: {
      ...envelope('item-1', 'item'),
      name: 'The Winter Pledge',
      generation_provenance: { dna: itemDna, action: 'forge' },
    },
    item_profile: 'weapon',
    appearance: 'A guild tending the old border beacons.',
    purpose: 'Every road deserves a witness.',
    properties: 'Keep the winter passage open.',
    origin: 'They maintain beacons and escort supply wagons.',
    history: 'Members mark each journey on a shared map.',
    power_or_function: 'They control the beacon stations and local stores.',
    cost_or_drawback: 'A practical guild of guides and repairers.',
    rarity: 'Its oldest route records omit one vanished village.',
    condition: 'A black lantern over a white road.',
    secret: 'Reliable in storms, secretive about old maps.',
    story_hooks: [
      'A beacon is lit from inside.',
      'A route vanishes.',
      'A courier brings a blank map.',
    ],
  },
};
const itemPayload: ForgePayload = {
  kind: 'item_candidate',
  value: itemCandidate,
};
const hometownBundle: ForgeResultBundle = {
  primary: locationCandidate,
  supporting: [],
  relationship: {
    id: 'relationship-hometown',
    project_id: project.id,
    relationship_type: 'born_in',
    source_id: 'character-1',
    target_id: 'location-1',
    state: 'candidate',
    origin: 'generated',
    created_at_unix_ms: 1,
    updated_at_unix_ms: 1,
  },
  recipe: {
    format_version: 1,
    action_id: 'character.forge-hometown',
    action_version: 1,
    context_recipe_id: 'character-hometown-v1',
    context_recipe_version: 1,
    context_snapshot: {
      format_version: 1,
      context_recipe_id: 'character-hometown-v1',
      context_recipe_version: 1,
      source_entity_id: 'character-1',
      source_write_version: 1,
      source_canon_state: 'candidate',
      source_type: 'character',
      source_generation_kind: 'character',
      source_generator_version: 1,
      source_content_version: 1,
      facts: {
        name: 'Mara Vale',
        title: 'Mara Vale',
        archetype: 'Lantern keeper',
        role: 'Guide',
        faction_hint: 'The Night Registry',
        backstory_hook: 'She filed her own disappearance',
        desire: 'Find the missing district',
        secret: 'She has already been there',
      },
      inherited_preset: 'dark-fantasy',
      inherited_creative_axes: dna.recipe.creative_axes,
      source_body_hash: 'source-hash',
      context_hash: 'context-hash',
    },
    action_seed: 42,
    location_dna: locationDna,
  },
  result: {
    format_version: 1,
    primary_output_id: 'location-1',
    primary_output_type: 'location',
    relationship_ids: ['relationship-hometown'],
    relationship_type: 'born_in',
    source_entity_id: 'character-1',
    warnings: [],
  },
};
const record = (entity: ForgePayload, writeVersion = 1): EntityRecord => ({
  entity,
  writeVersion,
});

beforeEach(() => {
  vi.unstubAllGlobals();
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
  vi.mocked(api.forgeHometown).mockResolvedValue(hometownBundle);
  vi.mocked(api.saveForgeBundle).mockResolvedValue({
    location: record(locationPayload),
    relationship: hometownBundle.relationship,
    eventId: 1,
  });
  vi.mocked(api.discardForgeBundle).mockResolvedValue(undefined);
  vi.mocked(api.relationshipsForEntity).mockResolvedValue([]);
  vi.mocked(api.listEntities).mockResolvedValue([]);
  vi.mocked(api.getEntity).mockImplementation(async (id) =>
    id === 'character-1' ? record(character) : record(quest),
  );
  vi.mocked(api.forgeCharacter).mockResolvedValue(character);
  vi.mocked(api.forgeQuest).mockResolvedValue(quest);
  vi.mocked(api.forgeLocation).mockResolvedValue(quest);
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

  it('supports the standalone Location Forge mode and library', async () => {
    await openWorkspace();
    expect(
      screen.getByRole('button', { name: 'Locations' }),
    ).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Location' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Location' }));
    await waitFor(() =>
      expect(api.forgeLocation).toHaveBeenCalledWith(
        'dark-fantasy',
        42,
        expect.any(Object),
      ),
    );
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

  it('shows Location Name locked state from its 02-name control', async () => {
    const lockedLocation: ForgePayload = {
      kind: 'location_candidate',
      value: {
        ...locationCandidate,
        dna: {
          ...locationCandidate.dna,
          recipe: {
            ...locationCandidate.dna.recipe,
            field_controls: {
              ...locationCandidate.dna.recipe.field_controls,
              '02-name': { nonce: 0, locked: true },
            },
          },
        },
      },
    };
    vi.mocked(api.forgeLocation).mockResolvedValue(locationPayload);
    vi.mocked(api.setLock).mockResolvedValue(lockedLocation);

    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Location' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Location' }));
    await screen.findByText('North Bell Lodge', { selector: 'h2' });
    fireEvent.click(screen.getByRole('button', { name: 'Lock Name' }));

    await waitFor(() => expect(api.setLock).toHaveBeenCalledWith('name', true));
    const nameField = screen
      .getByRole('heading', { name: 'Name' })
      .closest('article');
    expect(nameField).not.toBeNull();
    expect(
      within(nameField as HTMLElement).getByText('Locked'),
    ).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Unlock Name' })).toHaveAttribute(
      'aria-pressed',
      'true',
    );
  });

  it('locks the Location profile using its domain action key', async () => {
    const lockedLocation: ForgePayload = {
      kind: 'location_candidate',
      value: {
        ...locationCandidate,
        dna: {
          ...locationCandidate.dna,
          recipe: {
            ...locationCandidate.dna.recipe,
            field_controls: {
              ...locationCandidate.dna.recipe.field_controls,
              '01-location-profile': { nonce: 0, locked: true },
            },
          },
        },
      },
    };
    vi.mocked(api.forgeLocation).mockResolvedValue(locationPayload);
    vi.mocked(api.setLock).mockResolvedValue(lockedLocation);

    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Location' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Location' }));
    await screen.findByText('North Bell Lodge', { selector: 'h2' });
    fireEvent.click(
      screen.getByRole('button', { name: 'Lock Location profile' }),
    );

    await waitFor(() =>
      expect(api.setLock).toHaveBeenCalledWith('profile', true),
    );
    const profileField = screen
      .getByRole('heading', { name: 'Location profile' })
      .closest('article');
    expect(profileField).not.toBeNull();
    expect(
      within(profileField as HTMLElement).getByText('Locked'),
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Unlock Location profile' }),
    ).toHaveAttribute('aria-pressed', 'true');
  });

  it('rerolls and edits Location profile through its domain action key', async () => {
    vi.mocked(api.forgeLocation).mockResolvedValue(locationPayload);
    vi.mocked(api.reroll).mockResolvedValue(locationPayload);
    vi.mocked(api.editField).mockResolvedValue(locationPayload);

    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Location' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Location' }));
    await screen.findByText('North Bell Lodge', { selector: 'h2' });

    fireEvent.click(
      screen.getByRole('button', { name: 'Reroll Location profile' }),
    );
    await waitFor(() => expect(api.reroll).toHaveBeenCalledWith('profile'));

    fireEvent.click(
      screen.getByRole('button', { name: 'Edit Location profile' }),
    );
    fireEvent.change(
      screen.getByRole('textbox', { name: 'Edit Location profile' }),
      {
        target: { value: 'ruins' },
      },
    );
    fireEvent.click(screen.getByRole('button', { name: 'Save edit' }));
    await waitFor(() =>
      expect(api.editField).toHaveBeenCalledWith('profile', 'ruins'),
    );
  });

  it('locks Faction Name and reflects the actual 02-name control', async () => {
    const locked: ForgePayload = {
      kind: 'faction_candidate',
      value: {
        ...factionCandidate,
        dna: {
          ...factionCandidate.dna,
          recipe: {
            ...factionCandidate.dna.recipe,
            field_controls: {
              ...factionCandidate.dna.recipe.field_controls,
              '02-name': { nonce: 0, locked: true },
            },
          },
        },
      },
    };
    vi.mocked(api.forgeFaction).mockResolvedValue(factionPayload);
    vi.mocked(api.setLock)
      .mockResolvedValueOnce(locked)
      .mockResolvedValueOnce(factionPayload);

    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Faction' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Faction' }));
    await screen.findByText('The Ashen Concord', { selector: 'h2' });
    fireEvent.click(screen.getByRole('button', { name: 'Lock Name' }));

    await waitFor(() => expect(api.setLock).toHaveBeenCalledWith('name', true));
    const nameField = screen
      .getByRole('heading', { name: 'Name' })
      .closest('article');
    expect(nameField).not.toBeNull();
    expect(
      within(nameField as HTMLElement).getByText('Locked'),
    ).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Unlock Name' })).toHaveAttribute(
      'aria-pressed',
      'true',
    );
  });

  it('locks and rerolls the typed Faction Profile action key', async () => {
    const locked: ForgePayload = {
      kind: 'faction_candidate',
      value: {
        ...factionCandidate,
        dna: {
          ...factionCandidate.dna,
          recipe: {
            ...factionCandidate.dna.recipe,
            field_controls: {
              ...factionCandidate.dna.recipe.field_controls,
              '01-faction-profile': { nonce: 0, locked: true },
            },
          },
        },
      },
    };
    vi.mocked(api.forgeFaction).mockResolvedValue(factionPayload);
    vi.mocked(api.setLock)
      .mockResolvedValueOnce(locked)
      .mockResolvedValueOnce(factionPayload);
    vi.mocked(api.reroll).mockResolvedValue(factionPayload);

    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Faction' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Faction' }));
    await screen.findByText('The Ashen Concord', { selector: 'h2' });
    fireEvent.click(
      screen.getByRole('button', { name: 'Lock Faction Profile' }),
    );

    await waitFor(() =>
      expect(api.setLock).toHaveBeenCalledWith('faction_profile', true),
    );
    const profileField = screen
      .getByRole('heading', { name: 'Faction Profile' })
      .closest('article');
    expect(profileField).not.toBeNull();
    expect(
      within(profileField as HTMLElement).getByText('Locked'),
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Unlock Faction Profile' }),
    ).toHaveAttribute('aria-pressed', 'true');

    fireEvent.click(
      screen.getByRole('button', { name: 'Unlock Faction Profile' }),
    );
    await waitFor(() =>
      expect(api.setLock).toHaveBeenLastCalledWith('faction_profile', false),
    );
    const rerollProfile = screen.getByRole('button', {
      name: 'Reroll Faction Profile',
    });
    await waitFor(() => expect(rerollProfile).toBeEnabled());
    fireEvent.click(rerollProfile);
    await waitFor(() =>
      expect(api.reroll).toHaveBeenCalledWith('faction_profile'),
    );
  });

  it('locks Item Name and reflects the actual 02-name control', async () => {
    const locked: ForgePayload = {
      kind: 'item_candidate',
      value: {
        ...itemCandidate,
        dna: {
          ...itemCandidate.dna,
          recipe: {
            ...itemCandidate.dna.recipe,
            field_controls: {
              ...itemCandidate.dna.recipe.field_controls,
              '02-name': { nonce: 0, locked: true },
            },
          },
        },
      },
    };
    vi.mocked(api.forgeItem).mockResolvedValue(itemPayload);
    vi.mocked(api.setLock)
      .mockResolvedValueOnce(locked)
      .mockResolvedValueOnce(itemPayload);

    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Item' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Item' }));
    await screen.findByText('The Winter Pledge', { selector: 'h2' });
    fireEvent.click(screen.getByRole('button', { name: 'Lock Name' }));

    await waitFor(() => expect(api.setLock).toHaveBeenCalledWith('name', true));
    const nameField = screen
      .getByRole('heading', { name: 'Name' })
      .closest('article');
    expect(nameField).not.toBeNull();
    expect(
      within(nameField as HTMLElement).getByText('Locked'),
    ).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Unlock Name' })).toHaveAttribute(
      'aria-pressed',
      'true',
    );
  });

  it('locks and rerolls the typed Item Profile action key', async () => {
    const locked: ForgePayload = {
      kind: 'item_candidate',
      value: {
        ...itemCandidate,
        dna: {
          ...itemCandidate.dna,
          recipe: {
            ...itemCandidate.dna.recipe,
            field_controls: {
              ...itemCandidate.dna.recipe.field_controls,
              '01-item-profile': { nonce: 0, locked: true },
            },
          },
        },
      },
    };
    vi.mocked(api.forgeItem).mockResolvedValue(itemPayload);
    vi.mocked(api.setLock)
      .mockResolvedValueOnce(locked)
      .mockResolvedValueOnce(itemPayload);
    vi.mocked(api.reroll).mockResolvedValue(itemPayload);

    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Item' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Item' }));
    await screen.findByText('The Winter Pledge', { selector: 'h2' });
    fireEvent.click(screen.getByRole('button', { name: 'Lock Item Profile' }));

    await waitFor(() =>
      expect(api.setLock).toHaveBeenCalledWith('item_profile', true),
    );
    const profileField = screen
      .getByRole('heading', { name: 'Item Profile' })
      .closest('article');
    expect(profileField).not.toBeNull();
    expect(
      within(profileField as HTMLElement).getByText('Locked'),
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Unlock Item Profile' }),
    ).toHaveAttribute('aria-pressed', 'true');

    fireEvent.click(
      screen.getByRole('button', { name: 'Unlock Item Profile' }),
    );
    await waitFor(() =>
      expect(api.setLock).toHaveBeenLastCalledWith('item_profile', false),
    );
    const rerollProfile = screen.getByRole('button', {
      name: 'Reroll Item Profile',
    });
    await waitFor(() => expect(rerollProfile).toBeEnabled());
    fireEvent.click(rerollProfile);
    await waitFor(() =>
      expect(api.reroll).toHaveBeenCalledWith('item_profile'),
    );
  });

  it('edits Item Profile through only supported select values', async () => {
    vi.mocked(api.forgeItem).mockResolvedValue(itemPayload);
    vi.mocked(api.editField).mockResolvedValue(itemPayload);
    await openWorkspace();
    fireEvent.click(screen.getByRole('button', { name: 'Item' }));
    fireEvent.click(screen.getByRole('button', { name: 'Forge Item' }));
    await screen.findByText('The Winter Pledge', { selector: 'h2' });
    fireEvent.click(screen.getByRole('button', { name: 'Edit Item Profile' }));
    const select = screen.getByRole('combobox', { name: 'Edit Item Profile' });
    expect(within(select).getAllByRole('option')).toHaveLength(11);
    expect(
      screen.queryByRole('textbox', { name: 'Edit Item Profile' }),
    ).not.toBeInTheDocument();
    fireEvent.change(select, { target: { value: 'invalid-profile' } });
    fireEvent.click(screen.getByRole('button', { name: 'Save edit' }));
    expect(api.editField).not.toHaveBeenCalled();
    fireEvent.change(select, { target: { value: 'book' } });
    fireEvent.click(screen.getByRole('button', { name: 'Save edit' }));
    await waitFor(() =>
      expect(api.editField).toHaveBeenCalledWith('item_profile', 'book'),
    );
  });

  it('opens the Item library and typed saved detail', async () => {
    await openWorkspace([record(itemPayload)]);
    fireEvent.click(screen.getByRole('button', { name: /^Items/ }));
    expect(
      screen.getByRole('heading', { name: 'Items', level: 1 }),
    ).toBeInTheDocument();
    fireEvent.click(
      await screen.findByRole('button', {
        name: /The Winter Pledge.*candidate/i,
      }),
    );
    expect(
      await screen.findByRole('heading', { name: 'Item Profile' }),
    ).toBeInTheDocument();
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

  it('forges and atomically saves a Hometown bundle from the selected Character', async () => {
    const reviewedLocation: ForgePayload = {
      kind: 'location_candidate',
      value: {
        ...locationCandidate,
        dna: {
          ...locationCandidate.dna,
          recipe: {
            ...locationCandidate.dna.recipe,
            field_controls: {
              ...locationCandidate.dna.recipe.field_controls,
              '02-name': { nonce: 0, locked: true },
            },
          },
        },
      },
    };
    vi.mocked(api.setLock).mockResolvedValue(reviewedLocation);
    await openWorkspace([record(character)]);
    fireEvent.click(screen.getByRole('button', { name: /^Characters/ }));
    fireEvent.click(await screen.findByRole('button', { name: /Mara Vale/ }));
    fireEvent.click(
      await screen.findByRole('button', { name: 'Forge Hometown' }),
    );
    await waitFor(() =>
      expect(api.forgeHometown).toHaveBeenCalledWith('character-1', 42),
    );
    expect(
      await screen.findByText(
        /Proposed relationship: Born in North Bell Lodge/,
      ),
    ).toBeInTheDocument();
    expect(screen.getByText(/Context hash: context-hash/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Lock Name' }));
    await waitFor(() => expect(api.setLock).toHaveBeenCalledWith('name', true));
    fireEvent.click(screen.getByRole('button', { name: 'Save Bundle' }));
    await waitFor(() => expect(api.saveForgeBundle).toHaveBeenCalledTimes(1));
    expect(api.saveCandidate).not.toHaveBeenCalled();
    expect(await screen.findByText(/saved together/)).toBeInTheDocument();
  });

  it('explains unsupported manual provenance and discards an unsaved Hometown', async () => {
    await openWorkspace([record(manualCharacter)]);
    fireEvent.click(screen.getByRole('button', { name: /^Characters/ }));
    fireEvent.click(await screen.findByRole('button', { name: /Mara Vale/ }));
    expect(
      screen.getByRole('button', { name: 'Forge Hometown' }),
    ).toBeDisabled();
    expect(
      screen.getByText(/no supported generated ForgeDNA/),
    ).toBeInTheDocument();

    cleanup();
    await openWorkspace([record(character)]);
    fireEvent.click(screen.getByRole('button', { name: /^Characters/ }));
    fireEvent.click(await screen.findByRole('button', { name: /Mara Vale/ }));
    fireEvent.click(
      await screen.findByRole('button', { name: 'Forge Hometown' }),
    );
    await waitFor(() =>
      expect(
        screen.getAllByRole('button', { name: 'Discard Bundle' }),
      ).toHaveLength(2),
    );
    clickNamedButton('Discard Bundle', 0);
    await waitFor(() =>
      expect(api.discardForgeBundle).toHaveBeenCalledTimes(1),
    );
    expect(api.saveForgeBundle).not.toHaveBeenCalled();
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

  it('ignores an older history snapshot response after a newer selection', async () => {
    vi.mocked(api.project).mockResolvedValue(project);
    vi.mocked(api.history).mockResolvedValue([
      {
        runId: 'event-older',
        entityId: 'character-1',
        action: 'manual_edit',
        createdAtUnixMs: 1000,
      },
      {
        runId: 'event-newer',
        entityId: 'character-1',
        action: 'reroll_field',
        createdAtUnixMs: 2000,
      },
    ]);
    const olderSnapshot: ForgePayload = {
      ...character,
      value: {
        ...character.value,
        character: {
          ...character.value.character,
          title: 'Older snapshot response',
        },
      },
    };
    const newerSnapshot: ForgePayload = {
      ...character,
      value: {
        ...character.value,
        character: {
          ...character.value.character,
          title: 'Newer snapshot response',
        },
      },
    };
    const completions = new Map<string, (value: ForgePayload | null) => void>();
    vi.mocked(api.snapshot).mockImplementation(
      (eventId) =>
        new Promise((resolve) => {
          completions.set(eventId, resolve);
        }),
    );
    const { container } = render(<App />);
    await screen.findByRole('heading', { name: 'Forge' });
    fireEvent.click(screen.getByRole('button', { name: 'History' }));
    await screen.findByRole('heading', { name: 'History', level: 1 });
    await waitFor(() =>
      expect(container.querySelectorAll('.history-row')).toHaveLength(2),
    );
    const rows = container.querySelectorAll<HTMLButtonElement>('.history-row');
    fireEvent.click(rows.item(1));
    fireEvent.click(rows.item(0));
    await waitFor(() => expect(completions.size).toBe(2));

    completions.get('event-newer')?.(newerSnapshot);
    await waitFor(() =>
      expect(container.querySelector('.detail-panel')?.textContent).toContain(
        'Newer snapshot response',
      ),
    );
    completions.get('event-older')?.(olderSnapshot);
    await waitFor(() =>
      expect(container.querySelector('.detail-panel')?.textContent).toContain(
        'Newer snapshot response',
      ),
    );
    expect(container.querySelector('.detail-panel')?.textContent).not.toContain(
      'Older snapshot response',
    );
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

  it('keeps a canceled Canon save transient and requires explicit confirmation before revising', async () => {
    const historical = character;
    const currentCanon: ForgePayload = {
      ...character,
      value: {
        ...character.value,
        character: {
          ...character.value.character,
          title: 'Current Canon C',
          envelope: {
            ...character.value.character.envelope,
            canon_state: 'canon',
          },
        },
      },
    };
    vi.mocked(api.listEntities).mockResolvedValue([record(currentCanon)]);
    vi.mocked(api.getEntity).mockResolvedValue(record(currentCanon));
    vi.mocked(api.revisions).mockResolvedValue([
      {
        revisionNumber: 1,
        reason: 'Canon A',
        entity: historical,
        createdAtUnixMs: 1,
      },
    ]);
    vi.mocked(api.loadCanonRevisionWorkingCopy).mockResolvedValue({
      entity: historical,
      canUndo: false,
      canRedo: false,
    });
    const confirm = vi.fn().mockReturnValue(false);
    const prompt = vi.fn().mockReturnValue('revisit A');
    vi.stubGlobal('confirm', confirm);
    vi.stubGlobal('prompt', prompt);

    await openWorkspace([record(currentCanon)]);
    fireEvent.click(screen.getByRole('button', { name: 'Canon' }));
    fireEvent.click(
      await screen.findByRole('button', { name: /Current Canon C/ }),
    );
    fireEvent.click(await screen.findByRole('button', { name: 'Inspect' }));
    fireEvent.click(
      screen.getByRole('button', { name: 'Use as working copy' }),
    );
    await waitFor(() =>
      expect(api.loadCanonRevisionWorkingCopy).toHaveBeenCalledWith(
        'character-1',
        1,
      ),
    );
    fireEvent.click(
      await screen.findByRole('button', { name: 'Save Canon revision' }),
    );

    expect(confirm).toHaveBeenCalledWith(
      expect.stringContaining('Current Canon C'),
    );
    expect(confirm).toHaveBeenCalledWith(expect.stringContaining('Mara Vale'));
    expect(prompt).not.toHaveBeenCalled();
    expect(api.reviseCanon).not.toHaveBeenCalled();
    expect(
      screen.getByText(/Current Canon was not changed/),
    ).toBeInTheDocument();

    confirm.mockReturnValue(true);
    fireEvent.click(
      screen.getByRole('button', { name: 'Save Canon revision' }),
    );
    await waitFor(() =>
      expect(api.reviseCanon).toHaveBeenCalledWith('revisit A'),
    );
    expect(prompt).toHaveBeenCalledTimes(1);
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
