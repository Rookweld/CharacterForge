import { useEffect, useRef, useState } from 'react';
import {
  type ArtDirection,
  api,
  type VisualGenerationBrief,
  type VisualRecipeDefinition,
  type VisualRole,
} from './api';
import { ComfyProvider } from './ComfyProvider';

const emptyDirection: ArtDirection = {
  medium_style: '',
  palette: '',
  materials: '',
  camera_composition: '',
  atmosphere: '',
  avoid: '',
};
const directionLabels: Record<keyof ArtDirection, string> = {
  medium_style: 'Medium / style',
  palette: 'Palette',
  materials: 'Materials',
  camera_composition: 'Camera / composition',
  atmosphere: 'Art atmosphere',
  avoid: 'Avoid / prohibited traits',
};
const human = (text: string) => text.replaceAll('_', ' ');
export function VisualForge({
  id,
  entityType,
  writeVersion,
}: {
  id: string;
  entityType: string;
  writeVersion: number;
}) {
  const [open, setOpen] = useState(false);
  const [recipes, setRecipes] = useState<VisualRecipeDefinition[]>([]);
  const [role, setRole] = useState<VisualRole | ''>('');
  const [direction, setDirection] = useState<ArtDirection>({
    ...emptyDirection,
  });
  const [brief, setBrief] = useState<VisualGenerationBrief | null>(null);
  const [stale, setStale] = useState(false);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState('');
  const sequence = useRef(0);
  const prompt = useRef<HTMLTextAreaElement>(null);
  useEffect(() => {
    if (!open) return;
    let active = true;
    void api
      .visualRecipes(entityType)
      .then((items) => {
        if (active) {
          setRecipes(items);
          setRole(items[0]?.role ?? '');
        }
      })
      .catch(() => {
        if (active)
          setMessage(
            'Visual roles could not be loaded. Close and reopen Visual Forge to retry.',
          );
      });
    return () => {
      active = false;
    };
  }, [open, entityType]);
  useEffect(
    () => () => {
      ++sequence.current;
    },
    [],
  );
  const run = async (work: () => Promise<void>) => {
    const request = ++sequence.current;
    setBusy(true);
    setMessage('');
    try {
      await work();
    } catch (error) {
      if (request === sequence.current)
        setMessage(
          typeof error === 'object' && error && 'message' in error
            ? String(error.message)
            : 'Visual brief operation failed.',
        );
    } finally {
      if (request === sequence.current) setBusy(false);
    }
  };
  const compile = () => {
    if (!role) return;
    const request = sequence.current + 1;
    void run(async () => {
      const result = await api.compileVisualBrief(id, role, direction);
      if (request !== sequence.current) return;
      setBrief(result);
      setStale(false);
      setMessage('Visual brief compiled. No image was generated.');
    });
  };
  const sourceChanged =
    stale ||
    Boolean(brief && writeVersion > brief.context.source.write_version);
  const exportBrief = (format: 'json' | 'markdown') => {
    if (!brief) return;
    const path = window.prompt(
      `Save Visual Brief ${format.toUpperCase()} as`,
      format === 'json' ? 'visual-brief.json' : 'visual-brief.md',
    );
    if (!path?.trim()) return;
    const request = sequence.current + 1;
    void run(async () => {
      const changed = await api.visualBriefStale(brief.brief_hash);
      if (request !== sequence.current) return;
      if (changed) {
        setStale(true);
        setMessage(
          'Source changed. Recompile before exporting; the old preview is retained.',
        );
        return;
      }
      await api.exportVisualBrief(brief.brief_hash, path.trim(), format);
      if (request === sequence.current)
        setMessage('Visual brief export saved.');
    });
  };
  return (
    <section aria-label="Visual Forge">
      <button type="button" onClick={() => setOpen(!open)}>
        Visual Forge
      </button>
      {open && (
        <div className="panel context-card">
          <h3>Visual Forge foundation</h3>
          <p>
            Visual briefs work offline. Local ComfyUI is optional; no provider
            is contacted until you choose Check Connection or Generate Image.
          </p>
          <label>
            Visual role
            <select
              aria-label="Visual role"
              value={role}
              disabled={busy}
              onChange={(event) => setRole(event.target.value as VisualRole)}
            >
              {recipes.map((recipe) => (
                <option key={recipe.recipe_id} value={recipe.role}>
                  {human(recipe.role)}
                </option>
              ))}
            </select>
          </label>
          {(Object.keys(directionLabels) as (keyof ArtDirection)[]).map(
            (key) => (
              <label key={key}>
                {directionLabels[key]}
                <input
                  aria-label={directionLabels[key]}
                  value={direction[key]}
                  maxLength={512}
                  disabled={busy}
                  onChange={(event) =>
                    setDirection({ ...direction, [key]: event.target.value })
                  }
                />
              </label>
            ),
          )}
          <button type="button" disabled={busy || !role} onClick={compile}>
            {brief ? 'Recompile Visual Brief' : 'Compile Visual Brief'}
          </button>
          {message && <p role="status">{message}</p>}
          <ComfyProvider
            entityId={id}
            brief={brief}
            sourceChanged={sourceChanged}
          />
          {brief && (
            <section aria-label="Visual brief preview">
              {sourceChanged && (
                <p role="alert">
                  The saved source changed. This preview still refers to the old
                  source state. Recompile before export.
                </p>
              )}
              {(brief.context.role !== role ||
                JSON.stringify(brief.context.art_direction) !==
                  JSON.stringify(direction)) && (
                <p>
                  Role or art direction changed. Compile to update this frozen
                  preview.
                </p>
              )}
              <p>
                Source: {brief.context.source.entity_type} ·{' '}
                {human(brief.context.source.canon_state)} · write version{' '}
                {brief.context.source.write_version} ·{' '}
                {brief.context.source.canon_revision === null
                  ? 'current persisted state; no revision number inferred'
                  : `Canon revision ${brief.context.source.canon_revision}`}
              </p>
              <p>
                Role: {human(brief.context.role)} · Recipe:{' '}
                {brief.context.visual_recipe_id} v
                {brief.context.visual_recipe_version}
              </p>
              <ul>
                {brief.context.facts.map((fact) => (
                  <li key={fact.field}>
                    <strong>{human(fact.field)}:</strong>{' '}
                    {fact.values.join(' / ')}
                  </li>
                ))}
              </ul>
              <details>
                <summary>Brief provenance</summary>
                <p>Entity: {brief.context.source.entity_id}</p>
                <p>Source body hash: {brief.context.source.source_body_hash}</p>
                <p>Context hash: {brief.context.context_hash}</p>
                <p>Brief hash: {brief.brief_hash}</p>
                <pre>
                  {JSON.stringify(brief.context.source.generation, null, 2)}
                </pre>
              </details>
              <section aria-label="Subject classification">
                <h4>Subject classification</h4>
                <p>
                  Subject: {human(brief.subject_classification.subject_kind)} ·
                  Species mode: {brief.subject_classification.species_mode} ·
                  Literalization risk:{' '}
                  {brief.subject_classification.literalization_risk} · Human
                  anatomy safeguard:{' '}
                  {brief.subject_classification.humanoid_guardrail
                    ? 'applied'
                    : 'not applied; canonical form retained'}
                </p>
                <h4>Safeguards applied</h4>
                <ul>
                  {brief.safeguards_applied.map((s) => (
                    <li key={s}>{s}</li>
                  ))}
                </ul>
              </section>
              <label>
                Compiled positive prompt
                <textarea
                  ref={prompt}
                  aria-label="Visual prompt"
                  readOnly
                  value={brief.prompt_text}
                  rows={16}
                />
              </label>
              <label>
                Compiled negative prompt
                <textarea
                  aria-label="Compiled negative prompt"
                  readOnly
                  value={brief.negative_prompt_text}
                  rows={5}
                />
              </label>
              <details>
                <summary>Identity and symbolism</summary>
                <p>Identity: {brief.prompt_semantics.identity.join(' / ')}</p>
                <p>
                  Flavor / symbolism:{' '}
                  {brief.prompt_semantics.flavor_symbolism.join(' / ')}
                </p>
              </details>
              <p>
                Select the prompt to copy it with your usual keyboard shortcut.
                Export JSON includes the complete structured brief and
                provenance.
              </p>
              <div className="button-row">
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => {
                    const request = sequence.current + 1;
                    void run(async () => {
                      const changed = await api.visualBriefStale(
                        brief.brief_hash,
                      );
                      if (request !== sequence.current) return;
                      setStale(changed);
                      setMessage(
                        changed
                          ? 'Selecting the frozen old prompt; source changed. Recompile to use current facts.'
                          : 'Frozen prompt selected for copying.',
                      );
                      prompt.current?.select();
                    });
                  }}
                >
                  Select prompt
                </button>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => exportBrief('json')}
                >
                  Export Brief JSON
                </button>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => exportBrief('markdown')}
                >
                  Export Brief Markdown
                </button>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => {
                    const request = sequence.current + 1;
                    void run(async () => {
                      const changed = await api.visualBriefStale(
                        brief.brief_hash,
                      );
                      if (request === sequence.current) {
                        setStale(changed);
                        setMessage(
                          changed
                            ? 'Source changed; recompile this brief.'
                            : 'Source still matches the frozen brief.',
                        );
                      }
                    });
                  }}
                >
                  Check source
                </button>
              </div>
            </section>
          )}
        </div>
      )}
    </section>
  );
}
