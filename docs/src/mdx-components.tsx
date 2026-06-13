import * as TabsComponents from "fumadocs-ui/components/tabs";
import defaultMdxComponents from "fumadocs-ui/mdx";
import type { MDXComponents } from "mdx/types";
import { GithubInfo } from "@/components/github-info";

export function getMDXComponents(components?: MDXComponents): MDXComponents {
    return {
        GithubInfo,
        ...defaultMdxComponents,
        ...TabsComponents,
        ...components,
    } satisfies MDXComponents;
}
