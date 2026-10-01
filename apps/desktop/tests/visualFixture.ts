import type { ArtDirection, VisualGenerationBrief } from '../src/api';

const direction: ArtDirection = {
  medium_style: '',
  palette: '',
  materials: '',
  camera_composition: '',
  atmosphere: '',
  avoid: '',
};
export const brief: VisualGenerationBrief = {
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
