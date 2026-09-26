import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { App } from '../src/App';

describe('desktop foundation shell', () => {
  it('renders the application identity and baseline readiness', () => {
    render(<App />);

    expect(
      screen.getByRole('heading', { name: 'Character Forge / Quest Forge' }),
    ).toBeInTheDocument();
    expect(
      screen.getByText('The desktop foundation is ready.'),
    ).toBeInTheDocument();
  });
});
