import { useCallback, useEffect, useRef, useState } from 'react';
import {
  api,
  type ComfyDiscovery,
  type ComfySettings,
  type ComfyStatus,
  type VisualGenerationBrief,
} from './api';

const defaults: ComfySettings = {
  checkpoint: '',
  seed: 1,
  width: 512,
  height: 512,
  steps: 20,
  cfg: 7,
  sampler: '',
  scheduler: '',
};
export function validSettings(s: ComfySettings) {
  return Boolean(
    s.checkpoint &&
      s.sampler &&
      s.scheduler &&
      Number.isSafeInteger(s.seed) &&
      s.seed >= 0 &&
      [s.width, s.height].every(
        (n) => Number.isInteger(n) && n >= 256 && n <= 1024 && n % 64 === 0,
      ) &&
      s.width * s.height <= 786432 &&
      Number.isInteger(s.steps) &&
      s.steps >= 1 &&
      s.steps <= 40 &&
      Number.isFinite(s.cfg) &&
      s.cfg >= 1 &&
      s.cfg <= 15,
  );
}
const problem = (e: unknown) =>
  typeof e === 'object' && e && 'message' in e
    ? String(e.message)
    : 'Local ComfyUI operation failed. Check Connection to retry.';

export function ComfyProvider({
  brief,
  sourceChanged,
  entityId,
}: {
  brief: VisualGenerationBrief | null;
  sourceChanged: boolean;
  entityId?: string;
}) {
  const [url, setUrl] = useState('http://127.0.0.1:8000');
  const [discovery, setDiscovery] = useState<ComfyDiscovery | null>(null);
  const [settings, setSettings] = useState<ComfySettings>({ ...defaults });
  const [job, setJob] = useState<ComfyStatus | null>(null);
  const [image, setImage] = useState('');
  const [busy, setBusy] = useState(false);
  const [checking, setChecking] = useState(false);
  const [message, setMessage] = useState('');
  const alive = useRef(true);
  const imageSequence = useRef(0);
  const sequence = useRef(0);
  useEffect(() => {
    alive.current = true;
    return () => {
      alive.current = false;
      ++sequence.current;
    };
  }, []);
  useEffect(() => {
    if (!brief) return;
    const [width, height] =
      brief.aspect_intent === 'portrait'
        ? [512, 768]
        : brief.aspect_intent === 'landscape'
          ? [768, 512]
          : [512, 512];
    setSettings((s) => ({ ...s, width, height }));
  }, [brief]);
  const loadImage = useCallback(async (id: string) => {
    const imageTicket = ++imageSequence.current;
    const bytes = await api.comfyImage(id);
    if (!alive.current || imageTicket !== imageSequence.current) return;
    // In-memory data URL; no filesystem or remote browser image request.
    if (bytes.length > 8 * 1024 * 1024)
      throw new Error('Image exceeds preview bound.');
    let binary = '';
    for (let n = 0; n < bytes.length; n += 8192)
      binary += String.fromCharCode(...bytes.slice(n, n + 8192));
    setImage(`data:image/png;base64,${btoa(binary)}`);
  }, []);
  useEffect(() => {
    if (!entityId) return;
    let active = true;
    const ticket = sequence.current;
    void Promise.resolve(api.comfyResume(entityId))
      .then(async (resumed) => {
        if (
          !active ||
          !alive.current ||
          ticket !== sequence.current ||
          !resumed
        )
          return;
        setJob(resumed);
        setMessage(resumed.message);
        if (resumed.state === 'completed') await loadImage(resumed.prompt_id);
      })
      .catch((e) => {
        if (active && alive.current && ticket === sequence.current)
          setMessage(problem(e));
      });
    return () => {
      active = false;
    };
  }, [entityId, loadImage]);
  useEffect(() => {
    if (!job || !['queued', 'running'].includes(job.state)) return;
    let active = true;
    const timer = window.setTimeout(() => {
      void api
        .comfyStatus(job.prompt_id)
        .then(async (next) => {
          if (!active || !alive.current) return;
          setJob(next);
          setMessage(next.message);
          if (next.state === 'completed') await loadImage(next.prompt_id);
        })
        .catch((e) => {
          if (active && alive.current) {
            setJob((j) => (j ? { ...j, state: 'failed' } : j));
            setMessage(problem(e));
          }
        });
    }, 1500);
    return () => {
      active = false;
      window.clearTimeout(timer);
    };
  }, [job, loadImage]);
  const check = async () => {
    const ticket = ++sequence.current;
    setChecking(true);
    setDiscovery(null);
    setMessage('Checking local ComfyUI…');
    try {
      const d = await api.comfyCheck(url);
      if (!alive.current || ticket !== sequence.current) return;
      setDiscovery(d);
      setMessage(d.message);
      setSettings((s) => ({
        ...s,
        checkpoint: d.checkpoints[0] ?? '',
        sampler: d.samplers.includes('euler') ? 'euler' : (d.samplers[0] ?? ''),
        scheduler: d.schedulers.includes('normal')
          ? 'normal'
          : (d.schedulers[0] ?? ''),
      }));
    } catch (e) {
      if (alive.current && ticket === sequence.current) setMessage(problem(e));
    } finally {
      if (alive.current && ticket === sequence.current) setChecking(false);
    }
  };
  const generating = Boolean(job && ['queued', 'running'].includes(job.state));
  const ready = Boolean(
    brief &&
      discovery?.state === 'ready' &&
      validSettings(settings) &&
      !sourceChanged &&
      !busy &&
      !checking &&
      !generating,
  );
  const generate = async (variation = false) => {
    if (!ready || !brief || !discovery) return;
    const effective = variation
      ? { ...settings, seed: settings.seed + 1 }
      : settings;
    if (!validSettings(effective)) {
      setMessage('Choose a valid explicit seed.');
      return;
    }
    setSettings(effective);
    setBusy(true);
    setMessage(`Queueing explicit seed ${effective.seed}…`);
    const ticket = ++sequence.current;
    try {
      if (await api.visualBriefStale(brief.brief_hash)) {
        if (alive.current)
          setMessage(
            'Source changed. Recompile the Visual Brief before generation.',
          );
        return;
      }
      const queued = await api.comfyGenerate(
        discovery.server_url,
        brief.brief_hash,
        effective,
      );
      if (alive.current && ticket === sequence.current) {
        ++imageSequence.current;
        setJob(queued);
        setImage('');
        setMessage(queued.message);
      }
    } catch (e) {
      if (alive.current && ticket === sequence.current) setMessage(problem(e));
    } finally {
      if (alive.current && ticket === sequence.current) setBusy(false);
    }
  };
  const staleResult = Boolean(
    job?.stale ||
      (job?.result && brief && job.result.brief_hash !== brief.brief_hash) ||
      sourceChanged,
  );
  return (
    <section aria-label="Local ComfyUI provider">
      <h4>Provider: Local ComfyUI</h4>
      <p>
        Start ComfyUI, then Check Connection. Only HTTP loopback hosts are
        allowed. No models are downloaded. Results remain transient; Save Image
        exports an ordinary file.
      </p>
      <label>
        ComfyUI server address
        <input
          aria-label="ComfyUI server address"
          value={url}
          disabled={checking || busy || generating}
          onChange={(e) => {
            setUrl(e.target.value);
            setDiscovery(null);
            ++sequence.current;
          }}
        />
      </label>
      <button
        type="button"
        disabled={checking || busy || generating}
        onClick={() => void check()}
      >
        Check Connection
      </button>
      <p role="status">
        {checking ? 'checking_provider' : (discovery?.state ?? 'idle')} ·{' '}
        {message}
      </p>
      {discovery?.state === 'ready' && (
        <div>
          <label>
            Checkpoint
            <select
              aria-label="Checkpoint"
              disabled={busy || generating}
              value={settings.checkpoint}
              onChange={(e) =>
                setSettings({ ...settings, checkpoint: e.target.value })
              }
            >
              {discovery.checkpoints.map((m) => (
                <option key={m}>{m}</option>
              ))}
            </select>
          </label>
          {(['seed', 'width', 'height', 'steps', 'cfg'] as const).map((key) => (
            <label key={key}>
              {key}
              <input
                aria-label={`ComfyUI ${key}`}
                type="number"
                value={settings[key]}
                disabled={busy || generating}
                onChange={(e) =>
                  setSettings({ ...settings, [key]: Number(e.target.value) })
                }
              />
            </label>
          ))}
          {(['sampler', 'scheduler'] as const).map((key) => (
            <label key={key}>
              {key}
              <select
                aria-label={`ComfyUI ${key}`}
                value={settings[key]}
                disabled={busy || generating}
                onChange={(e) =>
                  setSettings({ ...settings, [key]: e.target.value })
                }
              >
                {(key === 'sampler'
                  ? discovery.samplers
                  : discovery.schedulers
                ).map((v) => (
                  <option key={v}>{v}</option>
                ))}
              </select>
            </label>
          ))}
          <p>
            Seed 0–9007199254740991; dimensions 256–1024 in multiples of 64, at
            most 786432 pixels; steps 1–40; CFG 1–15. Generic checkpoints may
            require a different future workflow. No alpha output is promised.
          </p>
        </div>
      )}
      {sourceChanged && (
        <p>Source changed. Recompile before image generation.</p>
      )}
      <button type="button" disabled={!ready} onClick={() => void generate()}>
        Generate Image
      </button>
      {job && (
        <section aria-label="ComfyUI result">
          <p>
            Generation: {job.state} · owned prompt {job.prompt_id}
          </p>
          {job.state === 'completed' && job.result && (
            <>
              {staleResult && (
                <p role="alert">
                  Generated from an older source state. This result retains its
                  original brief.
                </p>
              )}
              {image && (
                <img
                  src={image}
                  alt="Generated concept preview"
                  style={{ maxWidth: '100%', maxHeight: 512 }}
                />
              )}
              <p>
                Checkpoint {job.result.settings.checkpoint} · seed{' '}
                {job.result.settings.seed} · {job.result.settings.width}×
                {job.result.settings.height} · steps {job.result.settings.steps}{' '}
                · CFG {job.result.settings.cfg} · {job.result.settings.sampler}/
                {job.result.settings.scheduler}
              </p>
              <details>
                <summary>Image provenance</summary>
                <pre>{JSON.stringify(job.result, null, 2)}</pre>
              </details>
              <button
                type="button"
                disabled={busy}
                onClick={() => {
                  const path = window.prompt(
                    'Save Image as',
                    `visual-${job.result?.settings.seed}.png`,
                  );
                  if (!path?.trim()) return;
                  setBusy(true);
                  void api
                    .comfySave(job.prompt_id, path.trim())
                    .then(() => {
                      if (alive.current)
                        setMessage(
                          'Image export saved. No Entity or Canon was changed.',
                        );
                    })
                    .catch((e) => {
                      if (alive.current) setMessage(problem(e));
                    })
                    .finally(() => {
                      if (alive.current) setBusy(false);
                    });
                }}
              >
                Save Image
              </button>
              <button
                type="button"
                disabled={!ready || settings.seed >= Number.MAX_SAFE_INTEGER}
                onClick={() => void generate(true)}
              >
                Generate Variation (seed {settings.seed + 1})
              </button>
            </>
          )}
          <button
            type="button"
            disabled={busy || generating}
            onClick={() => {
              void api
                .comfyDiscard(job.prompt_id)
                .then(() => {
                  if (alive.current) {
                    ++imageSequence.current;
                    setJob(null);
                    setImage('');
                  }
                })
                .catch((e) => {
                  if (alive.current) setMessage(problem(e));
                });
            }}
          >
            Discard transient result
          </button>
          {generating && (
            <p>
              No global Cancel is exposed. Leaving this view stops app polling;
              ComfyUI may finish the queued job in its own output directory.
            </p>
          )}
        </section>
      )}
    </section>
  );
}
