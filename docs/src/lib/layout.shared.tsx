import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';

export function baseOptions(): BaseLayoutProps {
  return {
    githubUrl: "https://github.com/axtraz/transpic",
    nav: {
      title: 'Transpic Docs',
      transparentMode: "always",
    },
  };
}
