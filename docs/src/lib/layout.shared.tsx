import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';

export const appName = "Transpic Docs";

export function baseOptions(): BaseLayoutProps {
  return {
    githubUrl: "https://github.com/axtraz/transpic",
    nav: {
      title: appName,
      transparentMode: "always",
    },
  };
}
