/** A step on the space scale in primitives.css. Layout props accept only these. */
export type Space = '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8';

export const space = (step: Space) => `var(--space-${step})`;
