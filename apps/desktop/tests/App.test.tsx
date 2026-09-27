import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { App } from '../src/App';
import {
  api,
  type EntityRecord,
  type ForgePayload,
  type Project,
} from '../src/api';

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
  vi.mocked(api.createProject).mockResolvedValue(project);
  vi.mocked(api.openProject).mockResolvedValue(project);
  vi.mocked(api.closeProject).mockResolvedValue(undefined);
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
});

async function openWorkspace(items: EntityRecord[] = []) {
  vi.mocked(api.project).mockResolvedValue(project);
  vi.mocked(api.listEntities).mockResolvedValue(items);
  render(<App />);
  await screen.findByRole('heading', { name: 'Forge' });
}
async function forgeCharacterResult() {
  await openWorkspace();
  fireEvent.click(screen.getByRole('button', { name: 'Forge Character' }));
  await screen.findByText('Mara Vale', { selector: 'h2' });
}

describe('desktop creative workflows', () => {
  it('shows a project creation and open screen without an active project', async () => {
    render(<App />);
    expect(
      await screen.findByRole('heading', { name: 'Begin with a project' }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Create project' }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Open project' }),
    ).toBeInTheDocument();
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
    fireEvent.click(screen.getByRole('tab', { name: 'Quest' }));
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

  it('promotes a saved candidate through an intentional action', async () => {
    await openWorkspace([record(character)]);
    fireEvent.click(screen.getByRole('button', { name: /^Characters/ }));
    fireEvent.click(await screen.findByRole('button', { name: /Mara Vale/ }));
    await waitFor(() => expect(api.getEntity).toHaveBeenCalled());
    fireEvent.click(screen.getByRole('button', { name: 'Promote' }));
    await waitFor(() =>
      expect(api.promote).toHaveBeenCalledWith('character-1', 1),
    );
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
    fireEvent.click(await screen.findByRole('button', { name: /Manual Edit/ }));
    await waitFor(() => expect(api.snapshot).toHaveBeenCalledWith('run-1'));
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
