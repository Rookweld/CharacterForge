import {
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import {
  type ArtDirection,
  api,
  type VisualGenerationBrief,
  type VisualRecipeDefinition,
  type VisualRole,
} from '../src/api';
import { VisualForge } from '../src/VisualForge';

vi.mock('../src/api', async (load) => {
  const actual = await load<typeof import('../src/api')>();
  return {
    ...actual,
    api: Object.fromEntries(
      Object.keys(actual.api).map((key) => [key, vi.fn()]),
    ),
  };
});
const roles: Record<string, VisualRole[]> = {
  character: ['portrait', 'full_body', 'character_sheet'],
  location: ['establishing_shot', 'environment_concept', 'interior_concept'],
  faction: ['emblem', 'banner', 'uniform_concept'],
  item: ['item_concept', 'inventory_icon', 'prop_sheet'],
  creature: ['creature_concept', 'full_body', 'anatomy_detail'],
};
const direction: ArtDirection = {
  medium_style: '',
  palette: '',
  materials: '',
  camera_composition: '',
  atmosphere: '',
  avoid: '',
};
const brief: VisualGenerationBrief = {
  format_version: 1,
  context: {
    format_version: 1,
    visual_recipe_id: 'visual-character-portrait-v1',
    visual_recipe_version: 1,
    role: 'portrait',
    facts: [
      { field: 'name', values: ['Saved Witness'] },
      { field: 'appearance', values: ['Silver hair'] },
    ],
    art_direction: direction,
    context_hash: 'context-hash',
    source: {
      entity_id: 'source-id',
      project_id: 'project-id',
      entity_type: 'character',
      write_version: 1,
      canon_state: 'candidate',
      canon_revision: null,
      source_body_hash: 'body-hash',
      generation: null,
    },
  },
  subject_description: 'Saved Witness portrait',
  must_include: [],
  supporting_details: [],
  composition_intent: 'Portrait',
  aspect_intent: 'portrait',
  background_intent: 'Neutral',
  transparent_background_meaningful: false,
  avoid: [],
  consistency_notes: [],
  prompt_text: 'Saved Witness\nSilver hair',
  brief_hash: 'brief-hash',
};
beforeEach(() => {
  vi.resetAllMocks();
  vi.stubGlobal('prompt', vi.fn());
  vi.mocked(api.visualRecipes).mockImplementation(async (kind) =>
    (roles[kind] ?? []).map(
      (role): VisualRecipeDefinition => ({
        role,
        recipe_id: `visual-${kind}-${role}-v1`,
        version: 1,
        source_type: kind,
        composition_intent: 'Subject',
        aspect_intent: 'portrait',
        transparent_background_meaningful: false,
        environment_present: false,
      }),
    ),
  );
  vi.mocked(api.compileVisualBrief).mockResolvedValue(brief);
  vi.mocked(api.visualBriefStale).mockResolvedValue(false);
  vi.mocked(api.exportVisualBrief).mockResolvedValue(undefined);
});
async function open(kind = 'character') {
  render(<VisualForge id="source-id" entityType={kind} writeVersion={1} />);
  fireEvent.click(screen.getByRole('button', { name: 'Visual Forge' }));
  await waitFor(() => expect(screen.getAllByRole('option')).toHaveLength(3));
}
async function compile() {
  await open();
  fireEvent.click(screen.getByRole('button', { name: 'Compile Visual Brief' }));
  await screen.findByRole('textbox', { name: 'Visual prompt' });
}
describe('Visual Forge foundation', () => {
  it.each(Object.keys(roles))(
    'loads only the three supported roles for %s',
    async (kind) => {
      await open(kind);
      expect(api.visualRecipes).toHaveBeenCalledWith(kind);
      expect(
        screen
          .getAllByRole('option')
          .map((option) => option.getAttribute('value')),
      ).toEqual(roles[kind]);
      expect(
        screen.getByText(
          'Visual briefs work offline. Local ComfyUI is optional; no provider is contacted until you choose Check Connection or Generate Image.',
        ),
      ).toBeInTheDocument();
      expect(
        screen.getByRole('button', { name: 'Generate Image' }),
      ).toBeDisabled();
    },
  );
  it('previews exact visible facts and explicit direction without fake image success', async () => {
    vi.mocked(api.compileVisualBrief).mockImplementation(
      async (_id, _role, input) => ({
        ...brief,
        context: { ...brief.context, art_direction: input },
        prompt_text: `Silver hair / ${input.palette}`,
      }),
    );
    await open();
    fireEvent.change(screen.getByRole('textbox', { name: 'Palette' }), {
      target: { value: 'Iron and amber' },
    });
    fireEvent.click(
      screen.getByRole('button', { name: 'Compile Visual Brief' }),
    );
    expect(
      await screen.findByRole('textbox', { name: 'Visual prompt' }),
    ).toHaveValue('Silver hair / Iron and amber');
    expect(api.compileVisualBrief).toHaveBeenCalledWith(
      'source-id',
      'portrait',
      { ...direction, palette: 'Iron and amber' },
    );
    expect(
      screen.getByText('Visual brief compiled. No image was generated.'),
    ).toBeInTheDocument();
    expect(screen.queryByRole('img')).not.toBeInTheDocument();
  });
  it('retains the old snapshot when stale and supports explicit recompile', async () => {
    await compile();
    vi.mocked(api.visualBriefStale).mockResolvedValue(true);
    fireEvent.click(screen.getByRole('button', { name: 'Check source' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'old source state',
    );
    expect(screen.getByRole('textbox', { name: 'Visual prompt' })).toHaveValue(
      brief.prompt_text,
    );
    vi.mocked(api.compileVisualBrief).mockResolvedValue({
      ...brief,
      context: {
        ...brief.context,
        source: { ...brief.context.source, write_version: 2 },
      },
      prompt_text: 'Reviewed current source',
    });
    fireEvent.click(
      screen.getByRole('button', { name: 'Recompile Visual Brief' }),
    );
    await waitFor(() =>
      expect(
        screen.getByRole('textbox', { name: 'Visual prompt' }),
      ).toHaveValue('Reviewed current source'),
    );
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
  });
  it('exports JSON and Markdown only through the selected path and frozen brief identity', async () => {
    await compile();
    const prompt = vi
      .spyOn(window, 'prompt')
      .mockReturnValue('C:\\operator\\brief.json');
    fireEvent.click(screen.getByRole('button', { name: 'Export Brief JSON' }));
    await waitFor(() =>
      expect(api.exportVisualBrief).toHaveBeenCalledWith(
        'brief-hash',
        'C:\\operator\\brief.json',
        'json',
      ),
    );
    prompt.mockReturnValue('C:\\operator\\brief.md');
    await waitFor(() =>
      expect(
        screen.getByRole('button', { name: 'Export Brief Markdown' }),
      ).toBeEnabled(),
    );
    fireEvent.click(
      screen.getByRole('button', { name: 'Export Brief Markdown' }),
    );
    await waitFor(() =>
      expect(api.exportVisualBrief).toHaveBeenCalledWith(
        'brief-hash',
        'C:\\operator\\brief.md',
        'markdown',
      ),
    );
    prompt.mockRestore();
    expect(api.saveCandidate).not.toHaveBeenCalled();
    expect(api.promote).not.toHaveBeenCalled();
  });
  it('rejects stale export without replacing the old preview or faking success', async () => {
    await compile();
    vi.mocked(api.visualBriefStale).mockResolvedValue(true);
    const prompt = vi
      .spyOn(window, 'prompt')
      .mockReturnValue('C:\\operator\\stale.json');
    fireEvent.click(screen.getByRole('button', { name: 'Export Brief JSON' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'Recompile before export',
    );
    expect(api.exportVisualBrief).not.toHaveBeenCalled();
    expect(screen.getByRole('textbox', { name: 'Visual prompt' })).toHaveValue(
      brief.prompt_text,
    );
    prompt.mockRestore();
  });
  it('shows export readback errors and never claims saved output', async () => {
    await compile();
    vi.mocked(api.exportVisualBrief).mockRejectedValue({
      message: 'Source changed during export check',
    });
    const prompt = vi
      .spyOn(window, 'prompt')
      .mockReturnValue('C:\\operator\\error.json');
    fireEvent.click(screen.getByRole('button', { name: 'Export Brief JSON' }));
    expect(
      await screen.findByText('Source changed during export check'),
    ).toBeInTheDocument();
    expect(
      screen.queryByText('Visual brief export saved.'),
    ).not.toBeInTheDocument();
    prompt.mockRestore();
  });
  it('ignores compile responses after the source panel unmounts', async () => {
    let resolve: (value: VisualGenerationBrief) => void = () => undefined;
    vi.mocked(api.compileVisualBrief).mockImplementation(
      () =>
        new Promise((done) => {
          resolve = done;
        }),
    );
    await open();
    fireEvent.click(
      screen.getByRole('button', { name: 'Compile Visual Brief' }),
    );
    cleanup();
    resolve(brief);
    await Promise.resolve();
    expect(
      screen.queryByRole('textbox', { name: 'Visual prompt' }),
    ).not.toBeInTheDocument();
  });
});
