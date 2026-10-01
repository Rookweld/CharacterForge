import {
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { api, type ComfyDiscovery, type ComfyStatus } from '../src/api';
import { ComfyProvider, validSettings } from '../src/ComfyProvider';
import { brief } from './visualFixture';

vi.mock('../src/api', async (load) => {
  const actual = await load<typeof import('../src/api')>();
  return {
    ...actual,
    api: Object.fromEntries(
      Object.keys(actual.api).map((key) => [key, vi.fn()]),
    ),
  };
});
const discovery: ComfyDiscovery = {
  state: 'ready',
  message: 'Local ready',
  server_url: 'http://127.0.0.1:8000',
  server_version: '0.22.2',
  checkpoints: ['test.safetensors', 'other.ckpt'],
  samplers: ['euler'],
  schedulers: ['normal'],
  missing_nodes: [],
};
const queued: ComfyStatus = {
  state: 'queued',
  prompt_id: 'owned-id',
  stale: false,
  message: 'Queued owned job',
  result: null,
};
const completed: ComfyStatus = {
  ...queued,
  state: 'completed',
  message: 'Completed owned job',
  result: {
    provider: 'comfyui_local',
    server_version: '0.22.2',
    brief_hash: brief.brief_hash,
    prompt_id: 'owned-id',
    settings: {
      checkpoint: 'test.safetensors',
      seed: 1,
      width: 512,
      height: 768,
      steps: 20,
      cfg: 7,
      sampler: 'euler',
      scheduler: 'normal',
    },
    output: { filename: 'TheForge_0001.png', subfolder: '', type: 'output' },
    mime_type: 'image/png',
    byte_length: 8,
    content_blake3: 'image-hash',
    completed_unix_ms: 123,
    warnings: [],
  },
};
beforeEach(() => {
  vi.resetAllMocks();
  vi.stubGlobal('prompt', vi.fn());
  vi.mocked(api.comfyCheck).mockResolvedValue(discovery);
  vi.mocked(api.comfyGenerate).mockResolvedValue(queued);
  vi.mocked(api.comfyStatus).mockResolvedValue(completed);
  vi.mocked(api.comfyImage).mockResolvedValue([
    137, 80, 78, 71, 13, 10, 26, 10,
  ]);
  vi.mocked(api.visualBriefStale).mockResolvedValue(false);
  vi.mocked(api.comfySave).mockResolvedValue(undefined);
  vi.mocked(api.comfyDiscard).mockResolvedValue(undefined);
});
afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});
async function ready() {
  fireEvent.click(screen.getByRole('button', { name: 'Check Connection' }));
  await screen.findByRole('combobox', { name: 'Checkpoint' });
}
async function generate() {
  await ready();
  fireEvent.click(screen.getByRole('button', { name: 'Generate Image' }));
  await screen.findByText(/Generation: queued/);
}
describe('B10 Local ComfyUI', () => {
  it('requires a compiled brief and explicit discovery', async () => {
    render(<ComfyProvider brief={null} sourceChanged={false} />);
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeDisabled();
    expect(api.comfyCheck).not.toHaveBeenCalled();
    await ready();
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeDisabled();
  });
  it('renders unreachable state without blocking briefs', async () => {
    vi.mocked(api.comfyCheck).mockResolvedValue({
      ...discovery,
      state: 'unavailable',
      message: 'Start ComfyUI, then Check Connection.',
      checkpoints: [],
    });
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    fireEvent.click(screen.getByRole('button', { name: 'Check Connection' }));
    await screen.findByText(/unavailable/);
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeDisabled();
    expect(api.comfyGenerate).not.toHaveBeenCalled();
  });
  it('renders reachable no-model state', async () => {
    vi.mocked(api.comfyCheck).mockResolvedValue({
      ...discovery,
      state: 'no_model',
      message:
        'ComfyUI is available, but no compatible local checkpoint was found.',
      checkpoints: [],
    });
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    fireEvent.click(screen.getByRole('button', { name: 'Check Connection' }));
    await screen.findByText(/no compatible local checkpoint/);
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeDisabled();
    expect(screen.queryByRole('combobox', { name: 'Checkpoint' })).toBeNull();
  });
  it('uses discovered models and bounded aspect suggestions', async () => {
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await ready();
    expect(screen.getByRole('combobox', { name: 'Checkpoint' })).toHaveValue(
      'test.safetensors',
    );
    expect(
      screen.getByRole('spinbutton', { name: 'ComfyUI height' }),
    ).toHaveValue(768);
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeEnabled();
    fireEvent.change(
      screen.getByRole('spinbutton', { name: 'ComfyUI width' }),
      { target: { value: '4096' } },
    );
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeDisabled();
  });
  it('blocks a newly stale saved source before queue', async () => {
    vi.mocked(api.visualBriefStale).mockResolvedValue(true);
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Generate Image' }));
    await screen.findByText(/Recompile the Visual Brief before generation/);
    expect(api.comfyGenerate).not.toHaveBeenCalled();
  });
  it('disables generation when known source changed', async () => {
    render(<ComfyProvider brief={brief} sourceChanged />);
    await ready();
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeDisabled();
  });
  it('tracks owned queued, running and completed preview', async () => {
    vi.mocked(api.comfyStatus)
      .mockResolvedValueOnce({
        ...queued,
        state: 'running',
        message: 'Owned running',
      })
      .mockResolvedValue(completed);
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await generate();
    await screen.findByText(/Generation: running/, {}, { timeout: 3000 });
    await screen.findByRole(
      'img',
      { name: 'Generated concept preview' },
      { timeout: 4000 },
    );
    expect(api.comfyStatus).toHaveBeenCalledWith('owned-id');
    expect(api.comfyImage).toHaveBeenCalledWith('owned-id');
    expect(screen.getByRole('img')).toHaveAttribute(
      'src',
      'data:image/png;base64,iVBORw0KGgo=',
    );
    expect(api.promote).not.toHaveBeenCalled();
    expect(api.saveCandidate).not.toHaveBeenCalled();
    expect(screen.queryByRole('button', { name: 'Cancel' })).toBeNull();
  });
  it('renders provider execution failure without fake preview', async () => {
    vi.mocked(api.comfyStatus).mockResolvedValue({
      ...queued,
      state: 'failed',
      message: 'ComfyUI execution failed.',
    });
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await generate();
    await screen.findByText(/Generation: failed/, {}, { timeout: 3000 });
    expect(screen.queryByRole('img')).toBeNull();
    expect(api.comfyImage).not.toHaveBeenCalled();
  });
  it('saves an owned image to an explicit user path and surfaces variation seed', async () => {
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await generate();
    await screen.findByRole('img', {}, { timeout: 3000 });
    vi.spyOn(window, 'prompt').mockReturnValue('C:\\disposable\\visual.png');
    fireEvent.click(screen.getByRole('button', { name: 'Save Image' }));
    await waitFor(() =>
      expect(api.comfySave).toHaveBeenCalledWith(
        'owned-id',
        'C:\\disposable\\visual.png',
      ),
    );
    await screen.findByText(/Image export saved/);
    fireEvent.click(
      screen.getByRole('button', { name: 'Generate Variation (seed 2)' }),
    );
    await waitFor(() =>
      expect(api.comfyGenerate).toHaveBeenLastCalledWith(
        'http://127.0.0.1:8000',
        brief.brief_hash,
        expect.objectContaining({ seed: 2 }),
      ),
    );
    expect(
      screen.getByRole('spinbutton', { name: 'ComfyUI seed' }),
    ).toHaveValue(2);
  });
  it('retains old-source result with an explicit warning', async () => {
    vi.mocked(api.comfyStatus).mockResolvedValue({ ...completed, stale: true });
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await generate();
    await screen.findByText(
      /Generated from an older source state/,
      {},
      { timeout: 3000 },
    );
    expect(screen.getByRole('button', { name: 'Save Image' })).toBeEnabled();
  });
  it('reports export failure and permits explicit discard', async () => {
    vi.mocked(api.comfySave).mockRejectedValue({
      message: 'Destination exists.',
    });
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await generate();
    await screen.findByRole('img', {}, { timeout: 3000 });
    vi.spyOn(window, 'prompt').mockReturnValue('existing.png');
    fireEvent.click(screen.getByRole('button', { name: 'Save Image' }));
    await screen.findByText(/Destination exists/);
    expect(screen.getByRole('img')).toBeInTheDocument();
    fireEvent.click(
      screen.getByRole('button', { name: 'Discard transient result' }),
    );
    await waitFor(() => expect(screen.queryByRole('img')).toBeNull());
    expect(api.comfyDiscard).toHaveBeenCalledWith('owned-id');
  });
  it('ignores late discovery after unmount', async () => {
    let resolve!: (d: ComfyDiscovery) => void;
    vi.mocked(api.comfyCheck).mockReturnValue(
      new Promise((r) => {
        resolve = r;
      }),
    );
    const view = render(<ComfyProvider brief={brief} sourceChanged={false} />);
    fireEvent.click(screen.getByRole('button', { name: 'Check Connection' }));
    view.unmount();
    resolve(discovery);
    await Promise.resolve();
    expect(api.comfyGenerate).not.toHaveBeenCalled();
  });
  it('invalidates discovery after a server-address edit', async () => {
    render(<ComfyProvider brief={brief} sourceChanged={false} />);
    await ready();
    fireEvent.change(
      screen.getByRole('textbox', { name: 'ComfyUI server address' }),
      { target: { value: 'http://192.168.0.1:8000' } },
    );
    expect(
      screen.getByRole('button', { name: 'Generate Image' }),
    ).toBeDisabled();
  });
  it('restores an owned transient result after reopening the same entity view', async () => {
    vi.mocked(api.comfyResume).mockResolvedValue(completed);
    render(
      <ComfyProvider entityId="source-id" brief={null} sourceChanged={false} />,
    );
    await screen.findByRole('img');
    expect(api.comfyResume).toHaveBeenCalledWith('source-id');
    expect(api.comfyGenerate).not.toHaveBeenCalled();
  });
  it('validates numeric bounds including seed and pixels', () => {
    const s = completed.result?.settings;
    if (!s) throw new Error('Missing test settings');
    expect(validSettings(s)).toBe(true);
    for (const invalid of [
      { ...s, seed: -1 },
      { ...s, seed: Number.MAX_SAFE_INTEGER + 1 },
      { ...s, cfg: NaN },
      { ...s, steps: 0 },
      { ...s, width: 1024, height: 1024 },
      { ...s, height: 513 },
    ])
      expect(validSettings(invalid)).toBe(false);
  });
});
