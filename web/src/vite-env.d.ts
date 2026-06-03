/// <reference types="vite/client" />
/// <reference types="vitest" />

declare module '*.css';

// jsdom ships without bundled types in this Bun test setup.
declare module 'jsdom';
