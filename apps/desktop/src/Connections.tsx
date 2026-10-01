import { useEffect, useRef, useState } from 'react';
import {
  api,
  type ConnectionView,
  errorMessage,
  type RelationshipRevision,
  type UiError,
} from './api';

export function Connections({ entityId }: { entityId: string }) {
  const [views, setViews] = useState<ConnectionView[] | null>(null);
  const [error, setError] = useState('');
  const [busy, setBusy] = useState(false);
  const [history, setHistory] = useState<{
    id: string;
    revisions: RelationshipRevision[];
  } | null>(null);
  const mounted = useRef(false);
  const requestVersion = useRef(0);

  useEffect(() => {
    let active = true;
    const request = ++requestVersion.current;
    mounted.current = true;
    void api.relationshipsForEntity(entityId).then(
      (result) => {
        if (active && request === requestVersion.current) setViews(result);
      },
      (failure: UiError) => {
        if (active && request === requestVersion.current)
          setError(errorMessage(failure).message);
      },
    );
    return () => {
      active = false;
      mounted.current = false;
    };
  }, [entityId]);

  const refresh = async () => {
    const request = ++requestVersion.current;
    try {
      const result = await api.relationshipsForEntity(entityId);
      if (mounted.current && request === requestVersion.current) {
        setViews(result);
        setHistory(null);
      }
    } catch (failure) {
      if (mounted.current && request === requestVersion.current) setViews(null);
      throw failure;
    }
  };
  const mutate = async (
    view: ConnectionView,
    action: 'promote' | 'archive',
  ) => {
    setBusy(true);
    setError('');
    try {
      const operation =
        action === 'promote'
          ? api.promoteRelationship
          : api.archiveRelationship;
      await operation(view.relationship.id, view.writeVersion);
      if (mounted.current) await refresh();
    } catch (failure) {
      if (mounted.current) {
        const message = errorMessage(failure as UiError);
        setError(
          message.stale
            ? 'This connection changed since you opened it. Reloaded the latest saved connections; review before trying again.'
            : message.message,
        );
        if (message.stale) {
          try {
            await refresh();
          } catch {
            setError(
              'This connection changed. Could not reload connections; use Refresh connections before trying again.',
            );
            setViews(null);
          }
        }
      }
    } finally {
      if (mounted.current) setBusy(false);
    }
  };
  const inspect = async (id: string) => {
    setBusy(true);
    setError('');
    try {
      const revisions = await api.relationshipRevisions(id);
      if (mounted.current) setHistory({ id, revisions });
    } catch (failure) {
      if (mounted.current) setError(errorMessage(failure as UiError).message);
    } finally {
      if (mounted.current) setBusy(false);
    }
  };
  const row = (view: ConnectionView) => (
    <article key={view.relationship.id}>
      <p>
        {view.role === 'source'
          ? view.definition.sourcePhrase
          : view.definition.inversePhrase}{' '}
        {view.relatedName ?? 'Missing entity'} · {view.relatedType} ·{' '}
        {view.relationship.state === 'canon'
          ? 'Canon'
          : view.relationship.state === 'candidate'
            ? 'Candidate'
            : 'Archived'}{' '}
        · {view.relationship.origin}
      </p>
      {view.relatedAvailability !== 'available' && (
        <p>
          {view.relatedAvailability === 'archived'
            ? 'Related entity is archived.'
            : `Related entity is missing (${view.relatedId}).`}
        </p>
      )}
      <div className="button-row">
        {view.relationship.state === 'candidate' && (
          <button
            type="button"
            disabled={busy}
            onClick={() => void mutate(view, 'promote')}
          >
            Promote connection to Canon
          </button>
        )}
        {view.relationship.state !== 'archived' && (
          <button
            type="button"
            disabled={busy}
            onClick={() => void mutate(view, 'archive')}
          >
            Archive connection
          </button>
        )}
        <button
          type="button"
          disabled={busy}
          onClick={() => void inspect(view.relationship.id)}
        >
          Inspect connection history
        </button>
      </div>
      {history?.id === view.relationship.id && (
        <section aria-label="Connection history">
          <h4>Current connection · {view.relationship.state}</h4>
          <p>
            {view.relationship.relationship_type} · source{' '}
            {view.relationship.source_id} → target {view.relationship.target_id}{' '}
            · {view.relationship.origin} · write version {view.writeVersion} ·{' '}
            {new Date(view.relationship.updated_at_unix_ms).toLocaleString()}
          </p>
          <h4>Historical snapshots · read-only</h4>
          {history.revisions.length === 0 && (
            <p>No connection revisions yet.</p>
          )}
          {history.revisions.map((revision) => (
            <article key={revision.revisionNumber}>
              <strong>
                Revision {revision.revisionNumber} · {revision.reason}
              </strong>
              <p>
                {revision.relationship.state} ·{' '}
                {revision.relationship.relationship_type} · source{' '}
                {revision.relationship.source_id} → target{' '}
                {revision.relationship.target_id} ·{' '}
                {revision.relationship.origin} ·{' '}
                {new Date(revision.createdAtUnixMs).toLocaleString()}
              </p>
            </article>
          ))}
        </section>
      )}
    </article>
  );
  const active =
    views?.filter((view) => view.relationship.state !== 'archived') ?? [];
  const archived =
    views?.filter((view) => view.relationship.state === 'archived') ?? [];
  return (
    <section className="detail-connections" aria-label="Connections">
      <h3>Connections</h3>
      <button
        type="button"
        disabled={busy}
        onClick={() => {
          setBusy(true);
          setError('');
          void refresh()
            .catch((failure: UiError) => {
              if (mounted.current) setError(errorMessage(failure).message);
            })
            .finally(() => {
              if (mounted.current) setBusy(false);
            });
        }}
      >
        Refresh connections
      </button>
      {error && <p role="alert">{error}</p>}
      {views === null ? (
        <p>Connections have not been loaded.</p>
      ) : (
        active.length === 0 && <p>No connections yet.</p>
      )}
      {active.map(row)}
      {archived.length > 0 && (
        <details>
          <summary>
            Archived connections · read-only ({archived.length})
          </summary>
          {archived.map(row)}
        </details>
      )}
    </section>
  );
}
