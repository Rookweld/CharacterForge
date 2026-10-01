import {
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { api, type ConnectionView, type ForgeRelationship } from '../src/api';
import { Connections } from '../src/Connections';

vi.mock('../src/api', async (load) => {
  const actual = await load<typeof import('../src/api')>();
  return {
    ...actual,
    api: Object.fromEntries(
      Object.keys(actual.api).map((key) => [key, vi.fn()]),
    ),
  };
});

const definitions = [
  {
    key: 'born_in',
    relationshipType: 'born_in',
    source: 'character',
    target: 'location',
    directed: true,
    sourcePhrase: 'Born in',
    inversePhrase: 'Birthplace of',
  },
  {
    key: 'member_of',
    relationshipType: 'member_of',
    source: 'character',
    target: 'faction',
    directed: true,
    sourcePhrase: 'Member of',
    inversePhrase: 'Member:',
  },
  {
    key: 'owns',
    relationshipType: 'owns',
    source: 'character',
    target: 'item',
    directed: true,
    sourcePhrase: 'Owns',
    inversePhrase: 'Owned by',
  },
  {
    key: 'opposed_by',
    relationshipType: 'opposed_by',
    source: 'quest',
    target: 'creature',
    directed: true,
    sourcePhrase: 'Opposed by',
    inversePhrase: 'Opposes quest',
  },
] satisfies ConnectionView['definition'][];

const view = (
  index = 0,
  role: ConnectionView['role'] = 'source',
  state: ForgeRelationship['state'] = 'candidate',
): ConnectionView => {
  const definition = definitions[index];
  if (!definition) throw new Error('Unknown test definition');
  return {
    relationship: {
      id: 'relationship-1',
      project_id: 'project-1',
      relationship_type: definition.key,
      source_id: 'source-1',
      target_id: 'target-1',
      state,
      origin: 'generated',
      created_at_unix_ms: 1,
      updated_at_unix_ms: 2,
    },
    writeVersion: 7,
    role,
    relatedId: role === 'source' ? 'target-1' : 'source-1',
    relatedType: role === 'source' ? definition.target : definition.source,
    relatedName: 'Related name',
    relatedAvailability: 'available',
    definition,
  };
};
beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(api.relationshipsForEntity).mockResolvedValue([view()]);
  vi.mocked(api.promoteRelationship).mockResolvedValue(8);
  vi.mocked(api.archiveRelationship).mockResolvedValue(8);
  vi.mocked(api.relationshipRevisions).mockResolvedValue([]);
});

describe('Connections lifecycle', () => {
  it.each(
    definitions.flatMap(
      (definition, index) =>
        [
          [index, 'source', definition.sourcePhrase, definition.target],
          [index, 'target', definition.inversePhrase, definition.source],
        ] as const,
    ),
  )('displays definition %s from %s', async (index, role, phrase, type) => {
    vi.mocked(api.relationshipsForEntity).mockResolvedValue([
      view(index, role),
    ]);
    render(<Connections entityId="entity-1" />);
    expect(
      await screen.findByText(
        `${phrase} Related name · ${type} · Candidate · generated`,
      ),
    ).toBeInTheDocument();
    expect(api.relationshipsForEntity).toHaveBeenCalledWith('entity-1');
  });

  it('promotes using persisted version and waits for actual reload before changing state', async () => {
    let finish!: (value: ConnectionView[]) => void;
    vi.mocked(api.relationshipsForEntity)
      .mockResolvedValueOnce([view()])
      .mockImplementationOnce(
        () =>
          new Promise((resolve) => {
            finish = resolve;
          }),
      );
    render(<Connections entityId="source-1" />);
    fireEvent.click(
      await screen.findByRole('button', {
        name: 'Promote connection to Canon',
      }),
    );
    await waitFor(() =>
      expect(api.promoteRelationship).toHaveBeenCalledWith('relationship-1', 7),
    );
    await waitFor(() =>
      expect(api.relationshipsForEntity).toHaveBeenCalledTimes(2),
    );
    expect(screen.getByText(/· Candidate ·/)).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Promote connection to Canon' }),
    ).toBeDisabled();
    finish([{ ...view(0, 'source', 'canon'), writeVersion: 8 }]);
    expect(await screen.findByText(/· Canon ·/)).toBeInTheDocument();
    expect(
      screen.queryByRole('button', { name: 'Promote connection to Canon' }),
    ).not.toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: 'Archive connection' }),
    ).toBeEnabled();
    expect(api.promote).not.toHaveBeenCalled();
  });

  it.each(['candidate', 'canon'] as const)(
    'archives %s using the stored version and retains inspectable history',
    async (state) => {
      const original = view(0, 'source', state);
      vi.mocked(api.relationshipsForEntity)
        .mockResolvedValueOnce([original])
        .mockResolvedValue([
          { ...view(0, 'source', 'archived'), writeVersion: 8 },
        ]);
      vi.mocked(api.relationshipRevisions).mockResolvedValue([
        {
          revisionNumber: 1,
          reason: 'archive relationship',
          relationship: original.relationship,
          createdAtUnixMs: 3,
        },
      ]);
      render(<Connections entityId="source-1" />);
      fireEvent.click(
        await screen.findByRole('button', { name: 'Archive connection' }),
      );
      await waitFor(() =>
        expect(api.archiveRelationship).toHaveBeenCalledWith(
          'relationship-1',
          7,
        ),
      );
      const summary = await screen.findByText(
        'Archived connections · read-only (1)',
      );
      fireEvent.click(summary);
      expect(
        screen.queryByRole('button', { name: 'Archive connection' }),
      ).not.toBeInTheDocument();
      expect(
        screen.queryByRole('button', { name: 'Promote connection to Canon' }),
      ).not.toBeInTheDocument();
      fireEvent.click(
        screen.getByRole('button', { name: 'Inspect connection history' }),
      );
      const history = await screen.findByRole('region', {
        name: 'Connection history',
      });
      expect(
        within(history).getByText('Historical snapshots · read-only'),
      ).toBeInTheDocument();
      expect(
        within(history).getByText('Revision 1 · archive relationship'),
      ).toBeInTheDocument();
      expect(
        within(history).getAllByText(/source source-1 → target target-1/)[0],
      ).toBeInTheDocument();
      expect(within(history).queryByRole('button')).not.toBeInTheDocument();
      expect(api.relationshipRevisions).toHaveBeenCalledWith('relationship-1');
    },
  );

  it.each(['promote', 'archive'] as const)(
    'reports stale %s without inventing a new state',
    async (action) => {
      const operation =
        action === 'promote'
          ? api.promoteRelationship
          : api.archiveRelationship;
      vi.mocked(operation).mockRejectedValue({
        kind: 'stale_write',
        expected: 7,
        actual: 9,
      });
      vi.mocked(api.relationshipsForEntity)
        .mockResolvedValueOnce([view()])
        .mockResolvedValue([{ ...view(), writeVersion: 9 }]);
      render(<Connections entityId="source-1" />);
      fireEvent.click(
        await screen.findByRole('button', {
          name:
            action === 'promote'
              ? 'Promote connection to Canon'
              : 'Archive connection',
        }),
      );
      expect(await screen.findByRole('alert')).toHaveTextContent(
        'This connection changed',
      );
      await waitFor(() =>
        expect(api.relationshipsForEntity).toHaveBeenCalledTimes(2),
      );
      expect(screen.getByText(/· Candidate ·/)).toBeInTheDocument();
      expect(
        screen.queryByText(/· Canon ·|· Archived ·/),
      ).not.toBeInTheDocument();
      await waitFor(() =>
        expect(
          screen.getByRole('button', { name: 'Archive connection' }),
        ).toBeEnabled(),
      );
      fireEvent.click(
        screen.getByRole('button', { name: 'Archive connection' }),
      );
      await waitFor(() =>
        expect(api.archiveRelationship).toHaveBeenCalledWith(
          'relationship-1',
          9,
        ),
      );
    },
  );

  it('keeps archived assertions read-only and reports a missing endpoint', async () => {
    vi.mocked(api.relationshipsForEntity).mockResolvedValue([
      {
        ...view(0, 'target', 'archived'),
        relatedName: null,
        relatedAvailability: 'missing',
      },
    ]);
    render(<Connections entityId="target-1" />);
    fireEvent.click(
      await screen.findByText('Archived connections · read-only (1)'),
    );
    expect(
      screen.getByText('Related entity is missing (source-1).'),
    ).toBeInTheDocument();
    expect(
      screen.queryByRole('button', { name: 'Promote connection to Canon' }),
    ).not.toBeInTheDocument();
    expect(
      screen.queryByRole('button', { name: 'Archive connection' }),
    ).not.toBeInTheDocument();
  });

  it('reports archived related entities and refreshes without duplicating rows', async () => {
    vi.mocked(api.relationshipsForEntity).mockResolvedValue([
      { ...view(), relatedAvailability: 'archived' },
    ]);
    render(<Connections entityId="source-1" />);
    expect(
      await screen.findByText('Related entity is archived.'),
    ).toBeInTheDocument();
    fireEvent.click(
      screen.getByRole('button', { name: 'Refresh connections' }),
    );
    await waitFor(() =>
      expect(api.relationshipsForEntity).toHaveBeenCalledTimes(2),
    );
    expect(screen.getAllByText(/Born in Related name/)).toHaveLength(1);
  });

  it('has an honest empty state', async () => {
    vi.mocked(api.relationshipsForEntity).mockResolvedValue([]);
    render(<Connections entityId="source-1" />);
    expect(await screen.findByText('No connections yet.')).toBeInTheDocument();
    expect(
      screen.queryByRole('button', { name: 'Archive connection' }),
    ).not.toBeInTheDocument();
  });

  it('ignores a late response from a previously selected endpoint', async () => {
    let finish!: (value: ConnectionView[]) => void;
    vi.mocked(api.relationshipsForEntity)
      .mockImplementationOnce(
        () =>
          new Promise((resolve) => {
            finish = resolve;
          }),
      )
      .mockResolvedValue([]);
    const rendered = render(<Connections key="old" entityId="source-1" />);
    rendered.rerender(<Connections key="new" entityId="target-1" />);
    expect(await screen.findByText('No connections yet.')).toBeInTheDocument();
    finish([view()]);
    await waitFor(() =>
      expect(
        screen.queryByText(/Born in Related name/),
      ).not.toBeInTheDocument(),
    );
  });
});

describe('Connections reload integrity', () => {
  it('ignores an initial response that finishes after a newer refresh', async () => {
    let finish!: (value: ConnectionView[]) => void;
    vi.mocked(api.relationshipsForEntity)
      .mockImplementationOnce(
        () =>
          new Promise((resolve) => {
            finish = resolve;
          }),
      )
      .mockResolvedValue([{ ...view(0, 'source', 'canon'), writeVersion: 8 }]);
    render(<Connections entityId="source-1" />);
    fireEvent.click(
      screen.getByRole('button', { name: 'Refresh connections' }),
    );
    expect(await screen.findByText(/· Canon ·/)).toBeInTheDocument();
    finish([view()]);
    await waitFor(() =>
      expect(screen.queryByText(/· Candidate ·/)).not.toBeInTheDocument(),
    );
    expect(
      screen.queryByRole('button', { name: 'Promote connection to Canon' }),
    ).not.toBeInTheDocument();
  });
  it('removes mutation controls when stale-state reload fails', async () => {
    vi.mocked(api.promoteRelationship).mockRejectedValue({
      kind: 'stale_write',
      expected: 7,
      actual: 8,
    });
    vi.mocked(api.relationshipsForEntity)
      .mockResolvedValueOnce([view()])
      .mockRejectedValue({
        kind: 'storage',
        message: 'Could not read connections.',
      });
    render(<Connections entityId="source-1" />);
    fireEvent.click(
      await screen.findByRole('button', {
        name: 'Promote connection to Canon',
      }),
    );
    await waitFor(() =>
      expect(screen.getByRole('alert')).toHaveTextContent(
        'Could not reload connections',
      ),
    );
    expect(
      screen.queryByRole('button', { name: 'Promote connection to Canon' }),
    ).not.toBeInTheDocument();
    expect(
      screen.queryByRole('button', { name: 'Archive connection' }),
    ).not.toBeInTheDocument();
    expect(screen.queryByText('No connections yet.')).not.toBeInTheDocument();
  });
  it('requires a reload after a successful mutation whose readback fails', async () => {
    vi.mocked(api.relationshipsForEntity)
      .mockResolvedValueOnce([view()])
      .mockRejectedValue({
        kind: 'storage',
        message: 'Could not read connections.',
      });
    render(<Connections entityId="source-1" />);
    fireEvent.click(
      await screen.findByRole('button', {
        name: 'Promote connection to Canon',
      }),
    );
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'Could not read connections.',
    );
    expect(
      screen.queryByRole('button', { name: 'Promote connection to Canon' }),
    ).not.toBeInTheDocument();
    expect(screen.queryByText(/· Canon ·/)).not.toBeInTheDocument();
  });
});
